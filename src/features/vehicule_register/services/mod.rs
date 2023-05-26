use common::models::vehicule::{Vehiculo, NuevoVehiculo};

use crate::api_response::ApiResponse;
use crate::services::request::request_post;
use crate::error::Error;

// Admin route
pub async fn request_admin_create_vehicule(nuevo_vehicule: NuevoVehiculo) -> Result<ApiResponse::<Vehiculo>, Error> {
    request_post(format!("/api/vehicules"), nuevo_vehicule).await
}
