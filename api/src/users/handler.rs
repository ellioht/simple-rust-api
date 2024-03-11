use warp::{http::StatusCode, Reply};

use crate::users::service;
use crate::users::models::{UserCreateRequest};

pub async fn get_users_handler() -> Result<impl Reply, warp::Rejection> {
    let users = service::get_users().await;
    Ok(warp::reply::json(&users))
}

pub async fn create_user_handler(req: UserCreateRequest) -> Result<impl Reply, warp::Rejection> {
    let response = format!("Received name: {}", req.name);
    Ok(warp::reply::with_status(response, StatusCode::OK))
}