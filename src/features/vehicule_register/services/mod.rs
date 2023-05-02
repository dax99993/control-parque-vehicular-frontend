use common::models::vehicule::{Vehicule, NewVehicule};

use crate::api_response::ApiResponse;
use crate::services::request::request_post;
use crate::error::Error;

// Admin route
pub async fn request_admin_create_vehicule(new_vehicule: NewVehicule) -> Result<ApiResponse::<Vehicule>, Error> {
    request_post(format!("api/vehicules"), new_vehicule).await
}
