use common::models::vehicule::{Vehicule, FilteredVehicule};

use crate::api_response::ApiResponse;

use crate::services::request::{request_get, request_delete};
use crate::error::Error;


// Admin routes
/*
pub async fn request_admin_get_vehicules() -> Result<ApiResponse::<Vec<Vehicule>>, Error> {
    request_get(format!("api/vehicules")).await
}

pub async fn request_admin_get_vehicules(page: usize, items_per_page: usize) -> Result<ApiResponse::<Vec<Vehicule>>, Error> {
    let query = format!("page={page}&limit={items_per_page}");
    request_get(format!("api/vehicules?{}", query)).await
}
*/

pub async fn request_admin_get_vehicules(page: usize, items_per_page: usize, filter: Option<String>, filter_value: Option<String>) -> Result<ApiResponse::<Vec<Vehicule>>, Error> {
    let mut query = format!("page={page}&limit={items_per_page}");
    if let (Some(filter), Some(value)) = (filter, filter_value) {
        // parse filter from spanish to english
        let translated_filter = match filter.as_str() {
            "Marca" => "branch",
            "Modelo" => "model",
            "Año" => "year",
            _ => "EMPTY",
        };
        //let f = format!("&{}={}", filter, value);
        let f = format!("&{}={}", translated_filter, value);
        query.push_str(&f);
    }
    log::debug!("query = {}", &query);
    request_get(format!("api/vehicules?{}", query)).await
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

