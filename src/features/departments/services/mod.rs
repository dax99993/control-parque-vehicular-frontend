use common::models::department::Departamento;

use crate::api_response::ApiResponse;

use crate::services::request::{request_get, request_delete};
use crate::error::Error;


// Admin users

pub async fn request_admin_delete_department(id: String) -> Result<ApiResponse::<()>, Error> {
    request_delete(format!("api/users/{id}")).await
}


// All users
pub async fn request_get_departments() -> Result<ApiResponse::<Vec<Departamento>>, Error> {
    request_get(format!("api/departments")).await
}
