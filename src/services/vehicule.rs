use crate::types::vehicule::{Vehicule, FilteredVehicule};
use crate::api_response::ApiResponse;

use super::request::{request_get, request_post};
use crate::error::Error;

const BASE_URL: &str = "http://127.0.0.1:8000";


pub async fn request_normal_get_vehicules() -> Result<ApiResponse::<Vec<FilteredVehicule>>, Error> {
    request_get(format!("{BASE_URL}/api/vehicules")).await
}

pub async fn request_admin_get_vehicules() -> Result<ApiResponse::<Vec<Vehicule>>, Error> {
    request_get(format!("{BASE_URL}/api/vehicules")).await
}
