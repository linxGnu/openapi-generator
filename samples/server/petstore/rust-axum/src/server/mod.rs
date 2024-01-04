use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;

use crate:: header;

#[allow(unused_imports)]
use crate::models;

use crate::{Api,
     AddPetResponse,
     DeletePetResponse,
     FindPetsByStatusResponse,
     FindPetsByTagsResponse,
     GetPetByIdResponse,
     UpdatePetResponse,
     UpdatePetWithFormResponse,
     UploadFileResponse,
     DeleteOrderResponse,
     GetInventoryResponse,
     GetOrderByIdResponse,
     PlaceOrderResponse,
     CreateUserResponse,
     CreateUsersWithArrayInputResponse,
     CreateUsersWithListInputResponse,
     DeleteUserResponse,
     GetUserByNameResponse,
     LoginUserResponse,
     LogoutUserResponse,
     UpdateUserResponse
};

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: Api + 'static,
{
    // build our application with a route
    Router::new()
        .route("/v2/pet",
            post(add_pet::<I, A>).put(update_pet::<I, A>)
        )
        .route("/v2/pet/:pet_id",
            delete(delete_pet::<I, A>).get(get_pet_by_id::<I, A>).post(update_pet_with_form::<I, A>)
        )
        .route("/v2/pet/:pet_id/uploadImage",
            post(upload_file::<I, A>)
        )
        .route("/v2/pet/findByStatus",
            get(find_pets_by_status::<I, A>)
        )
        .route("/v2/pet/findByTags",
            get(find_pets_by_tags::<I, A>)
        )
        .route("/v2/store/inventory",
            get(get_inventory::<I, A>)
        )
        .route("/v2/store/order",
            post(place_order::<I, A>)
        )
        .route("/v2/store/order/:order_id",
            delete(delete_order::<I, A>).get(get_order_by_id::<I, A>)
        )
        .route("/v2/user",
            post(create_user::<I, A>)
        )
        .route("/v2/user/:username",
            delete(delete_user::<I, A>).get(get_user_by_name::<I, A>).put(update_user::<I, A>)
        )
        .route("/v2/user/createWithArray",
            post(create_users_with_array_input::<I, A>)
        )
        .route("/v2/user/createWithList",
            post(create_users_with_list_input::<I, A>)
        )
        .route("/v2/user/login",
            get(login_user::<I, A>)
        )
        .route("/v2/user/logout",
            get(logout_user::<I, A>)
        )
        .with_state(api_impl)
}

/// Operation: AddPet - POST /pet
#[tracing::instrument(skip_all)]
async fn add_pet<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
  Json(body): Json<models::Pet>
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().add_pet(
      method,
      host,
      cookies,
            body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                AddPetResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = body;
                                                    response.body(Body::from(body_content))
                                                },
                                                AddPetResponse::InvalidInput
                                                => {
                                                    let mut response = response.status(405);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: DeletePet - DELETE /pet/{petId}
#[tracing::instrument(skip_all)]
async fn delete_pet<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
  Path(path_pet_id): Path<i64>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{
                // Header parameters
                let header_api_key = headers.get(HeaderName::from_static("api_key"));

                let header_api_key = match header_api_key {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header api_key - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };



  let result = api_impl.as_ref().delete_pet(
      method,
      host,
      cookies,
      header_api_key,
      path_pet_id,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                DeletePetResponse::InvalidPetValue
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: FindPetsByStatus - GET /pet/findByStatus
#[tracing::instrument(skip_all)]
async fn find_pets_by_status<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<HashMap<String, String>>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

    let query_status = query_params.iter().filter(|e| e.0 == "status").map(|e| e.1.clone())
                    .filter_map(|query_status| query_status.parse().ok())
                    .collect::<Vec<_>>();


  let result = api_impl.as_ref().find_pets_by_status(
      method,
      host,
      cookies,
      query_status,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                FindPetsByStatusResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = body;
                                                    response.body(Body::from(body_content))
                                                },
                                                FindPetsByStatusResponse::InvalidStatusValue
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: FindPetsByTags - GET /pet/findByTags
#[tracing::instrument(skip_all)]
async fn find_pets_by_tags<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<HashMap<String, String>>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

    let query_tags = query_params.iter().filter(|e| e.0 == "tags").map(|e| e.1.clone())
                    .filter_map(|query_tags| query_tags.parse().ok())
                    .collect::<Vec<_>>();


  let result = api_impl.as_ref().find_pets_by_tags(
      method,
      host,
      cookies,
      query_tags,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                FindPetsByTagsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = body;
                                                    response.body(Body::from(body_content))
                                                },
                                                FindPetsByTagsResponse::InvalidTagValue
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: GetPetById - GET /pet/{petId}
#[tracing::instrument(skip_all)]
async fn get_pet_by_id<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_pet_id): Path<i64>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().get_pet_by_id(
      method,
      host,
      cookies,
      path_pet_id,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                GetPetByIdResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = body;
                                                    response.body(Body::from(body_content))
                                                },
                                                GetPetByIdResponse::InvalidIDSupplied
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                                GetPetByIdResponse::PetNotFound
                                                => {
                                                    let mut response = response.status(404);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: UpdatePet - PUT /pet
#[tracing::instrument(skip_all)]
async fn update_pet<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
  Json(body): Json<models::Pet>
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().update_pet(
      method,
      host,
      cookies,
            body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                UpdatePetResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = body;
                                                    response.body(Body::from(body_content))
                                                },
                                                UpdatePetResponse::InvalidIDSupplied
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                                UpdatePetResponse::PetNotFound
                                                => {
                                                    let mut response = response.status(404);
                                                    response.body(Body::empty())
                                                },
                                                UpdatePetResponse::ValidationException
                                                => {
                                                    let mut response = response.status(405);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: UpdatePetWithForm - POST /pet/{petId}
#[tracing::instrument(skip_all)]
async fn update_pet_with_form<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_pet_id): Path<i64>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().update_pet_with_form(
      method,
      host,
      cookies,
      path_pet_id,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                UpdatePetWithFormResponse::InvalidInput
                                                => {
                                                    let mut response = response.status(405);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: UploadFile - POST /pet/{petId}/uploadImage
#[tracing::instrument(skip_all)]
async fn upload_file<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_pet_id): Path<i64>,
 State(api_impl): State<I>,
  body: Multipart,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().upload_file(
      method,
      host,
      cookies,
      path_pet_id,
          body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                UploadFileResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = serde_json::to_string(&body).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?;
                                                    response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: DeleteOrder - DELETE /store/order/{orderId}
#[tracing::instrument(skip_all)]
async fn delete_order<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_order_id): Path<String>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().delete_order(
      method,
      host,
      cookies,
      path_order_id,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOrderResponse::InvalidIDSupplied
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                                DeleteOrderResponse::OrderNotFound
                                                => {
                                                    let mut response = response.status(404);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: GetInventory - GET /store/inventory
#[tracing::instrument(skip_all)]
async fn get_inventory<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().get_inventory(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                GetInventoryResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = serde_json::to_string(&body).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?;
                                                    response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: GetOrderById - GET /store/order/{orderId}
#[tracing::instrument(skip_all)]
async fn get_order_by_id<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_order_id): Path<i64>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().get_order_by_id(
      method,
      host,
      cookies,
      path_order_id,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                GetOrderByIdResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = body;
                                                    response.body(Body::from(body_content))
                                                },
                                                GetOrderByIdResponse::InvalidIDSupplied
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                                GetOrderByIdResponse::OrderNotFound
                                                => {
                                                    let mut response = response.status(404);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: PlaceOrder - POST /store/order
#[tracing::instrument(skip_all)]
async fn place_order<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
  Json(body): Json<models::Order>
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().place_order(
      method,
      host,
      cookies,
            body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                PlaceOrderResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = body;
                                                    response.body(Body::from(body_content))
                                                },
                                                PlaceOrderResponse::InvalidOrder
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: CreateUser - POST /user
#[tracing::instrument(skip_all)]
async fn create_user<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
  Json(body): Json<models::User>
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().create_user(
      method,
      host,
      cookies,
            body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                CreateUserResponse::SuccessfulOperation
                                                => {
                                                    let mut response = response.status(0);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: CreateUsersWithArrayInput - POST /user/createWithArray
#[tracing::instrument(skip_all)]
async fn create_users_with_array_input<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
  Json(body): Json<Vec<models::User>>
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().create_users_with_array_input(
      method,
      host,
      cookies,
            body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                CreateUsersWithArrayInputResponse::SuccessfulOperation
                                                => {
                                                    let mut response = response.status(0);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: CreateUsersWithListInput - POST /user/createWithList
#[tracing::instrument(skip_all)]
async fn create_users_with_list_input<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
  Json(body): Json<Vec<models::User>>
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().create_users_with_list_input(
      method,
      host,
      cookies,
            body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                CreateUsersWithListInputResponse::SuccessfulOperation
                                                => {
                                                    let mut response = response.status(0);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: DeleteUser - DELETE /user/{username}
#[tracing::instrument(skip_all)]
async fn delete_user<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_username): Path<String>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().delete_user(
      method,
      host,
      cookies,
      path_username,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                DeleteUserResponse::InvalidUsernameSupplied
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                                DeleteUserResponse::UserNotFound
                                                => {
                                                    let mut response = response.status(404);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: GetUserByName - GET /user/{username}
#[tracing::instrument(skip_all)]
async fn get_user_by_name<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_username): Path<String>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().get_user_by_name(
      method,
      host,
      cookies,
      path_username,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                GetUserByNameResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = body;
                                                    response.body(Body::from(body_content))
                                                },
                                                GetUserByNameResponse::InvalidUsernameSupplied
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                                GetUserByNameResponse::UserNotFound
                                                => {
                                                    let mut response = response.status(404);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: LoginUser - GET /user/login
#[tracing::instrument(skip_all)]
async fn login_user<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<HashMap<String, String>>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

    let query_username = query_params.iter().filter(|e| e.0 == "username").map(|e| e.1.clone())
                    .next();
                let query_username = match query_username {
                    Some(query_username) => {
                        let query_username =
                            <String as std::str::FromStr>::from_str
                                (&query_username);
                        match query_username {
                            Ok(query_username) => Some(query_username),
                            Err(e) => return Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter username - doesn't match schema: {}", e))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR }),
                        }
                    },
                    None => None,
                };
                let query_username = match query_username {
                    Some(query_username) => query_username,
                    None => return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter username")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR }),
                };
    let query_password = query_params.iter().filter(|e| e.0 == "password").map(|e| e.1.clone())
                    .next();
                let query_password = match query_password {
                    Some(query_password) => {
                        let query_password =
                            <String as std::str::FromStr>::from_str
                                (&query_password);
                        match query_password {
                            Ok(query_password) => Some(query_password),
                            Err(e) => return Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter password - doesn't match schema: {}", e))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR }),
                        }
                    },
                    None => None,
                };
                let query_password = match query_password {
                    Some(query_password) => query_password,
                    None => return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter password")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR }),
                };


  let result = api_impl.as_ref().login_user(
      method,
      host,
      cookies,
      query_username,
      query_password,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                LoginUserResponse::SuccessfulOperation
                                                    {
                                                        body,
                                                        set_cookie,
                                                        x_rate_limit,
                                                        x_expires_after
                                                    }
                                                => {
                                                    if let Some(set_cookie) = set_cookie {
                                                    let set_cookie = match header::IntoHeaderValue(set_cookie).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling set_cookie header - {}", e))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                                                        }
                                                    };

                                                    
                                                    {
                                                      let mut response_headers = response.headers_mut().unwrap();
                                                      response_headers.insert(
                                                          HeaderName::from_static("set-cookie"),
                                                          set_cookie
                                                      );
                                                    }
                                                    }
                                                    if let Some(x_rate_limit) = x_rate_limit {
                                                    let x_rate_limit = match header::IntoHeaderValue(x_rate_limit).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling x_rate_limit header - {}", e))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                                                        }
                                                    };

                                                    
                                                    {
                                                      let mut response_headers = response.headers_mut().unwrap();
                                                      response_headers.insert(
                                                          HeaderName::from_static("x-rate-limit"),
                                                          x_rate_limit
                                                      );
                                                    }
                                                    }
                                                    if let Some(x_expires_after) = x_expires_after {
                                                    let x_expires_after = match header::IntoHeaderValue(x_expires_after).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling x_expires_after header - {}", e))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                                                        }
                                                    };

                                                    
                                                    {
                                                      let mut response_headers = response.headers_mut().unwrap();
                                                      response_headers.insert(
                                                          HeaderName::from_static("x-expires-after"),
                                                          x_expires_after
                                                      );
                                                    }
                                                    }
                                                    let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/xml").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }
                                                    let body_content = body;
                                                    response.body(Body::from(body_content))
                                                },
                                                LoginUserResponse::InvalidUsername
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: LogoutUser - GET /user/logout
#[tracing::instrument(skip_all)]
async fn logout_user<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().logout_user(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                LogoutUserResponse::SuccessfulOperation
                                                => {
                                                    let mut response = response.status(0);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

/// Operation: UpdateUser - PUT /user/{username}
#[tracing::instrument(skip_all)]
async fn update_user<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_username): Path<String>,
 State(api_impl): State<I>,
  Json(body): Json<models::User>
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{


  let result = api_impl.as_ref().update_user(
      method,
      host,
      cookies,
      path_username,
            body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                UpdateUserResponse::InvalidUserSupplied
                                                => {
                                                    let mut response = response.status(400);
                                                    response.body(Body::empty())
                                                },
                                                UpdateUserResponse::UserNotFound
                                                => {
                                                    let mut response = response.status(404);
                                                    response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

