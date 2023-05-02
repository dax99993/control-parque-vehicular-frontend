use common::models::vehicule::{Vehicule, FilteredVehicule};

use crate::api_response::ApiResponse;

use crate::services::request::{request_get, request_delete};
use crate::error::Error;


// Admin routes
pub async fn request_admin_get_vehicules() -> Result<ApiResponse::<Vec<Vehicule>>, Error> {
    request_get(format!("api/vehicules")).await
}

pub async fn request_admin_delete_vehicule(id: String) -> Result<ApiResponse::<()>, Error> {
    request_delete(format!("api/vehicules/{id}")).await
}


// Normal user routes
pub async fn request_normal_get_vehicules() -> Result<ApiResponse::<Vec<FilteredVehicule>>, Error> {
    request_get(format!("api/vehicules")).await
}

pub async fn request_normal_get_vehicule_with_id(id: String) -> Result<ApiResponse::<FilteredVehicule>, Error> {
    request_get(format!("api/vehicules/{id}")).await
}

