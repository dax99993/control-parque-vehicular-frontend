use common::models::user::{Usuario, ActualizaUsuario};

use crate::api_response::ApiResponse;

use crate::services::request::{request_get, request_delete, request_patch, request_multipart_patch};
use crate::error::Error;


// Admin routes
pub async fn request_admin_get_users() -> Result<ApiResponse::<Vec<Usuario>>, Error> {
    request_get(format!("api/users")).await
}

pub async fn request_admin_get_user_with_id(id: String) -> Result<ApiResponse::<Usuario>, Error> {
    request_get(format!("api/users/{id}")).await
}

pub async fn request_admin_delete_user(id: String) -> Result<ApiResponse::<()>, Error> {
    request_delete(format!("api/users/{id}")).await
}

pub async fn request_admin_update_user(id: String, updated_user: ActualizaUsuario) -> Result<ApiResponse::<Usuario>, Error> {
    request_patch(format!("api/users/{id}"), updated_user).await
}

pub async fn request_admin_update_user_picture(id: String, picture: reqwest::multipart::Form) -> Result<ApiResponse::<Usuario>, Error> {
    request_multipart_patch(format!("api/users/picture/{id}"), picture).await
}
