use common::models::user::{LoginUser, User, SignupUser};
use crate::api_response::ApiResponse;

use super::request::{request_get, request_post};
use crate::error::Error;


pub async fn request_login(login_user: LoginUser) -> Result<ApiResponse::<String>, Error> {
    request_post(format!("api/auth/login"), login_user).await
}

//pub async fn request_me() -> Result<ApiResponse::<FilteredUser>, Error> {
pub async fn request_me() -> Result<ApiResponse::<User>, Error> {
    request_get(format!("api/users/me")).await
}

pub async fn request_logout() -> Result<ApiResponse::<()>, Error> {
    request_get(format!("api/auth/logout")).await
}

pub async fn request_signup(signup_user: SignupUser) -> Result<ApiResponse::<()>, Error> {
    request_post(format!("api/auth/signup"), signup_user).await
}
