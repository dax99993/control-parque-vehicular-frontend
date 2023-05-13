use common::models::user::Usuario;

use crate::api_response::ApiResponse;

use crate::services::request::{request_get, request_delete};
use crate::error::Error;


// Admin routes
pub async fn request_admin_get_users() -> Result<ApiResponse::<Vec<Usuario>>, Error> {
    request_get(format!("api/users")).await
}

pub async fn request_admin_delete_user(id: String) -> Result<ApiResponse::<()>, Error> {
    request_delete(format!("api/users/{id}")).await
}
