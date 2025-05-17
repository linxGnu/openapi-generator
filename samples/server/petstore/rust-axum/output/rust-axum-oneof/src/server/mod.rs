use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use chrono::Utc;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header::CONTENT_TYPE};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, apis::event, models};

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::EventDispatcher + apis::default::Default + 'static,
{
    // build our application with a route
    Router::new()
        .route("/", post(foo::<I, A>))
        .with_state(api_impl)
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct FooBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::Message,
}

#[tracing::instrument(skip_all)]
fn foo_validation(
    body: models::Message,
) -> std::result::Result<(models::Message,), ValidationErrors> {
    let b = FooBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}

/// Foo - POST /
#[tracing::instrument(skip_all)]
async fn foo<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::Message>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::EventDispatcher + apis::default::Default,
{
    let start_at = Utc::now();

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || foo_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let mut event = event::Event::default();
    let result = api_impl
        .as_ref()
        .foo(&mut event, method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::default::FooResponse::Status200_Re(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
        }
    };
    if let Ok(resp) = resp.as_ref() {
        if !event.is_empty() {
            event.insert(
                event::convention::EVENT_TIMESTAMP.to_string(),
                format!("{start_at:?}"),
            );
            event.insert(
                event::convention::EVENT_SERVICE.to_string(),
                api_impl.as_ref().service_name(),
            );
            event.insert(
                event::convention::EVENT_STATUS_CODE.to_string(),
                resp.status().as_u16().to_string(),
            );
            event.insert(
                event::convention::EVENT_ACTION.to_string(),
                "foo".to_string(),
            );
            event.insert(
                event::convention::EVENT_LATENCY_SECS.to_string(),
                format!(
                    "{:.6}",
                    Utc::now().signed_duration_since(start_at).as_seconds_f64()
                ),
            );
            api_impl.as_ref().dispatch(event).await;
        }
    }

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[allow(dead_code)]
#[inline]
fn response_with_status_code_only(code: StatusCode) -> Result<Response, StatusCode> {
    Response::builder()
        .status(code)
        .body(Body::empty())
        .map_err(|_| code)
}
