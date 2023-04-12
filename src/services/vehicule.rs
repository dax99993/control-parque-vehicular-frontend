use crate::types::vehicule::{Vehicule, FilteredVehicule, NewVehicule, UpdateVehicule};
use crate::api_response::ApiResponse;

use super::request::{request_get, request_post, request_delete, request_patch};
use crate::error::Error;

const BASE_URL: &str = "http://127.0.0.1:8000";
//const BASE_URL_VEHICULES: &str = "http://127.0.0.1:8000/api/vehicules";


// Admin routes

pub async fn request_admin_get_vehicules() -> Result<ApiResponse::<Vec<Vehicule>>, Error> {
    request_get(format!("{BASE_URL}/api/vehicules")).await
}

pub async fn request_admin_get_vehicule_with_id(id: String) -> Result<ApiResponse::<Vehicule>, Error> {
    request_get(format!("{BASE_URL}/api/vehicules/{id}")).await
}

pub async fn request_admin_create_vehicule(new_vehicule: NewVehicule) -> Result<ApiResponse::<Vehicule>, Error> {
    request_post(format!("{BASE_URL}/api/vehicules"), new_vehicule).await
}

pub async fn request_admin_delete_vehicule(id: String) -> Result<ApiResponse::<()>, Error> {
    request_delete(format!("{BASE_URL}/api/vehicules/{id}")).await
}

pub async fn request_admin_update_vehicule(id: String, updated_vehicule: UpdateVehicule) -> Result<ApiResponse::<Vehicule>, Error> {
    request_patch(format!("{BASE_URL}/api/vehicules/{id}"), updated_vehicule).await
}

// Normal user routes
pub async fn request_normal_get_vehicules() -> Result<ApiResponse::<Vec<FilteredVehicule>>, Error> {
    request_get(format!("{BASE_URL}/api/vehicules")).await
}

pub async fn request_normal_get_vehicule_with_id(id: String) -> Result<ApiResponse::<FilteredVehicule>, Error> {
    request_get(format!("{BASE_URL}/api/vehicules/{id}")).await
}


// general routes



