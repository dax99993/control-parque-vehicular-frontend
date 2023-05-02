use common::models::vehicule::{Vehicule, UpdateVehicule};

use crate::api_response::ApiResponse;
use crate::services::request::{request_get, request_patch, request_multipart_patch};
use crate::error::Error;


// Admin routes

pub async fn request_admin_get_vehicule_with_id(id: String) -> Result<ApiResponse::<Vehicule>, Error> {
    request_get(format!("api/vehicules/{id}")).await
}


pub async fn request_admin_update_vehicule(id: String, updated_vehicule: UpdateVehicule) -> Result<ApiResponse::<Vehicule>, Error> {
    request_patch(format!("api/vehicules/{id}"), updated_vehicule).await
}

pub async fn request_admin_update_vehicule_picture(id: String, picture: reqwest::multipart::Form) -> Result<ApiResponse::<Vehicule>, Error> {
    request_multipart_patch(format!("api/vehicules/picture/{id}"), picture).await
}
