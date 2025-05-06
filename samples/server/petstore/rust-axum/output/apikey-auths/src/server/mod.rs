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
pub fn new<I, A, C>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::EventDispatcher
        + apis::payments::Payments<Claims = C>
        + apis::payments::PaymentsAuthorization<Claims = C>
        + apis::ApiKeyAuthHeader<Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + 'static,
    C: Send + Sync + 'static,
{
    // build our application with a route
    Router::new()
        .route("/v71/paymentMethods", get(get_payment_methods::<I, A, C>))
        .route(
            "/v71/paymentMethods/{id}",
            get(get_payment_method_by_id::<I, A, C>),
        )
        .route("/v71/payments", post(post_make_payment::<I, A, C>))
        .with_state(api_impl)
}

#[tracing::instrument(skip_all)]
fn get_payment_method_by_id_validation(
    path_params: models::GetPaymentMethodByIdPathParams,
) -> std::result::Result<(models::GetPaymentMethodByIdPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}

/// GetPaymentMethodById - GET /v71/paymentMethods/{id}
#[tracing::instrument(skip_all)]
async fn get_payment_method_by_id<I, A, C>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetPaymentMethodByIdPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::EventDispatcher
        + apis::payments::Payments<Claims = C>
        + apis::payments::PaymentsAuthorization<Claims = C>,
{
    let start_at = Utc::now();
    let mut event = event::Event::default();
    event.insert(
        event::convention::EVENT_TIMESTAMP.to_string(),
        format!("{start_at:?}"),
    );

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || get_payment_method_by_id_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_payment_method_by_id(&mut event, method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::payments::GetPaymentMethodByIdResponse::Status200_OK(body) => {
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
            apis::payments::GetPaymentMethodByIdResponse::Status422_UnprocessableEntity(body) => {
                let mut response = response.status(422);
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
                event::convention::EVENT_SERVICE.to_string(),
                api_impl.as_ref().service_name(),
            );
            event.insert(
                event::convention::EVENT_STATUS_CODE.to_string(),
                resp.status().as_u16().to_string(),
            );
            event.insert(
                event::convention::EVENT_ACTION.to_string(),
                "get_payment_method_by_id".to_string(),
            );
            event.insert(
                event::convention::EVENT_LATENCY.to_string(),
                Utc::now().signed_duration_since(start_at).to_string(),
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
fn get_payment_methods_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}

/// GetPaymentMethods - GET /v71/paymentMethods
#[tracing::instrument(skip_all)]
async fn get_payment_methods<I, A, C>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::EventDispatcher
        + apis::payments::Payments<Claims = C>
        + apis::payments::PaymentsAuthorization<Claims = C>,
{
    let start_at = Utc::now();
    let mut event = event::Event::default();
    event.insert(
        event::convention::EVENT_TIMESTAMP.to_string(),
        format!("{start_at:?}"),
    );

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_payment_methods_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_payment_methods(&mut event, method, host, cookies)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::payments::GetPaymentMethodsResponse::Status200_OK(body) => {
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
                event::convention::EVENT_SERVICE.to_string(),
                api_impl.as_ref().service_name(),
            );
            event.insert(
                event::convention::EVENT_STATUS_CODE.to_string(),
                resp.status().as_u16().to_string(),
            );
            event.insert(
                event::convention::EVENT_ACTION.to_string(),
                "get_payment_methods".to_string(),
            );
            event.insert(
                event::convention::EVENT_LATENCY.to_string(),
                Utc::now().signed_duration_since(start_at).to_string(),
            );
            api_impl.as_ref().dispatch(event).await;
        }
    }

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct PostMakePaymentBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::Payment,
}

#[tracing::instrument(skip_all)]
fn post_make_payment_validation(
    body: Option<models::Payment>,
) -> std::result::Result<(Option<models::Payment>,), ValidationErrors> {
    if let Some(body) = &body {
        let b = PostMakePaymentBodyValidator { body };
        b.validate()?;
    }

    Ok((body,))
}

/// PostMakePayment - POST /v71/payments
#[tracing::instrument(skip_all)]
async fn post_make_payment<I, A, C>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<Option<models::Payment>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::EventDispatcher
        + apis::payments::Payments<Claims = C>
        + apis::payments::PaymentsAuthorization<Claims = C>
        + apis::CookieAuthentication<Claims = C>,
{
    let start_at = Utc::now();
    let mut event = event::Event::default();
    event.insert(
        event::convention::EVENT_TIMESTAMP.to_string(),
        format!("{start_at:?}"),
    );

    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "X-API-Key")
        .await;
    let claims = None.or(claims_in_cookie);
    let Some(claims) = claims else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || post_make_payment_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let authorization = api_impl
        .as_ref()
        .post_make_payment_authorize(&method, &host, &cookies, &claims, &body)
        .await;
    match authorization {
        Ok(authorization) => match authorization {
            apis::Authorization::Authorized => {}
            apis::Authorization::Forbidden => {
                return response_with_status_code_only(StatusCode::FORBIDDEN);
            }
        },
        Err(_) => {
            return response_with_status_code_only(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    let result = api_impl
        .as_ref()
        .post_make_payment(&mut event, method, host, cookies, claims, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::payments::PostMakePaymentResponse::Status200_OK(body) => {
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
            apis::payments::PostMakePaymentResponse::Status422_UnprocessableEntity(body) => {
                let mut response = response.status(422);
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
                event::convention::EVENT_SERVICE.to_string(),
                api_impl.as_ref().service_name(),
            );
            event.insert(
                event::convention::EVENT_STATUS_CODE.to_string(),
                resp.status().as_u16().to_string(),
            );
            event.insert(
                event::convention::EVENT_ACTION.to_string(),
                "post_make_payment".to_string(),
            );
            event.insert(
                event::convention::EVENT_LATENCY.to_string(),
                Utc::now().signed_duration_since(start_at).to_string(),
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
