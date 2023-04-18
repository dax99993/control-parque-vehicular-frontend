use crate::types::vehicule::{Vehicule, FilteredVehicule, NewVehicule, UpdateVehicule};
use crate::api_response::ApiResponse;

use super::request::{request_get, request_post, request_delete, request_patch, request_multipart_patch};
use crate::error::Error;


// Admin routes

pub async fn request_admin_get_vehicules() -> Result<ApiResponse::<Vec<Vehicule>>, Error> {
    request_get(format!("api/vehicules")).await
}

pub async fn request_admin_get_vehicule_with_id(id: String) -> Result<ApiResponse::<Vehicule>, Error> {
    request_get(format!("api/vehicules/{id}")).await
}

pub async fn request_admin_create_vehicule(new_vehicule: NewVehicule) -> Result<ApiResponse::<Vehicule>, Error> {
    request_post(format!("api/vehicules"), new_vehicule).await
}

pub async fn request_admin_delete_vehicule(id: String) -> Result<ApiResponse::<()>, Error> {
    request_delete(format!("api/vehicules/{id}")).await
}

pub async fn request_admin_update_vehicule(id: String, updated_vehicule: UpdateVehicule) -> Result<ApiResponse::<Vehicule>, Error> {
    request_patch(format!("api/vehicules/{id}"), updated_vehicule).await
}

pub async fn request_admin_update_vehicule_picture(id: String, picture: reqwest::multipart::Form) -> Result<ApiResponse::<Vehicule>, Error> {
    request_multipart_patch(format!("api/vehicules/picture/{id}"), picture).await
}


// Normal user routes
pub async fn request_normal_get_vehicules() -> Result<ApiResponse::<Vec<FilteredVehicule>>, Error> {
    request_get(format!("api/vehicules")).await
}

pub async fn request_normal_get_vehicule_with_id(id: String) -> Result<ApiResponse::<FilteredVehicule>, Error> {
    request_get(format!("api/vehicules/{id}")).await
}

