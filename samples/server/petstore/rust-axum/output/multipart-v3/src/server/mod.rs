use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use chrono::Utc;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
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
        .route(
            "/multipart_related_request",
            post(multipart_related_request_post::<I, A>),
        )
        .route("/multipart_request", post(multipart_request_post::<I, A>))
        .route(
            "/multiple-identical-mime-types",
            post(multiple_identical_mime_types_post::<I, A>),
        )
        .with_state(api_impl)
}

#[tracing::instrument(skip_all)]
fn multipart_related_request_post_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}

/// MultipartRelatedRequestPost - POST /multipart_related_request
#[tracing::instrument(skip_all)]
async fn multipart_related_request_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    body: axum::body::Body,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::EventDispatcher + apis::default::Default,
{
    let start_at = Utc::now();

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || multipart_related_request_post_validation())
            .await
            .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let mut event = event::Event::default();
    let result = api_impl
        .as_ref()
        .multipart_related_request_post(&mut event, method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::default::MultipartRelatedRequestPostResponse::Status201_OK => {
                let mut response = response.status(201);
                response.body(Body::empty())
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
                "multipart_related_request_post".to_string(),
            );
            event.insert(
                event::convention::EVENT_LATENCY.to_string(),
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

#[tracing::instrument(skip_all)]
fn multipart_request_post_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}

/// MultipartRequestPost - POST /multipart_request
#[tracing::instrument(skip_all)]
async fn multipart_request_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    body: Multipart,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::EventDispatcher + apis::default::Default,
{
    let start_at = Utc::now();

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || multipart_request_post_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let mut event = event::Event::default();
    let result = api_impl
        .as_ref()
        .multipart_request_post(&mut event, method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::default::MultipartRequestPostResponse::Status201_OK => {
                let mut response = response.status(201);
                response.body(Body::empty())
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
                "multipart_request_post".to_string(),
            );
            event.insert(
                event::convention::EVENT_LATENCY.to_string(),
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

#[tracing::instrument(skip_all)]
fn multiple_identical_mime_types_post_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}

/// MultipleIdenticalMimeTypesPost - POST /multiple-identical-mime-types
#[tracing::instrument(skip_all)]
async fn multiple_identical_mime_types_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    body: axum::body::Body,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::EventDispatcher + apis::default::Default,
{
    let start_at = Utc::now();

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || multiple_identical_mime_types_post_validation())
            .await
            .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let mut event = event::Event::default();
    let result = api_impl
        .as_ref()
        .multiple_identical_mime_types_post(&mut event, method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::default::MultipleIdenticalMimeTypesPostResponse::Status200_OK => {
                let mut response = response.status(200);
                response.body(Body::empty())
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
                "multiple_identical_mime_types_post".to_string(),
            );
            event.insert(
                event::convention::EVENT_LATENCY.to_string(),
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
