use common::models::user::{LoginUsuario, Usuario, SignupUsuario};
use crate::api_response::ApiResponse;

use crate::services::request::{request_get, request_post};
use crate::error::Error;


pub async fn request_login(login_usuario: LoginUsuario) -> Result<ApiResponse::<String>, Error> {
    request_post(format!("api/auth/login"), login_usuario).await
}

//pub async fn request_me() -> Result<ApiResponse::<FilteredUser>, Error> {
pub async fn request_me() -> Result<ApiResponse::<Usuario>, Error> {
    request_get(format!("api/users/me")).await
}

pub async fn request_logout() -> Result<ApiResponse::<()>, Error> {
    request_get(format!("api/auth/logout")).await
}

pub async fn request_signup(signup_usuario: SignupUsuario) -> Result<ApiResponse::<()>, Error> {
    request_post(format!("api/auth/signup"), signup_usuario).await
}
