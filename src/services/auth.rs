use crate::{types::user::{LoginUser, FilteredUser}, api_response::ApiResponse};

use super::request::{request_get, request_post};
use crate::error::Error;

const BASE_URL: &str = "http://127.0.0.1:8000";


pub async fn request_login(login_user: LoginUser) -> Result<ApiResponse::<String>, Error> {
    request_post(format!("{BASE_URL}/api/auth/login"), login_user).await
}

pub async fn request_me() -> Result<ApiResponse::<FilteredUser>, Error> {
    request_get(format!("{BASE_URL}/api/users/me")).await
}
