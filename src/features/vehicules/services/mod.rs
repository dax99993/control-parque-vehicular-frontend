//use common::models::vehicule::{Vehicule, FilteredVehicule};
use common::models::vehicule::Vehiculo;

use crate::api_response::ApiResponse;

use crate::services::request::{request_get, request_delete, request_image};
use crate::error::Error;

// General user routes
pub async fn request_vehicule_picture(file: String) -> Result<Vec<u8>, Error> {
    request_image(format!("/api/vehicules/picture/{file}")).await
}

// Admin routes

pub async fn request_admin_get_vehicules(
    pagina: usize,
    vehiculos_por_pagina: usize,
    filter: Option<String>,
    filter_value: Option<String>,
) -> Result<ApiResponse::<Vec<Vehiculo>>, Error> {
    // create query str
    let mut query = format!("pagina={pagina}&limite={vehiculos_por_pagina}");
    if let (Some(filter), Some(value)) = (filter, filter_value) {
        let f = format!("&{}={}", filter.to_lowercase().as_str(), value);
        query.push_str(&f);
    }

    log::debug!("query = {}", &query);
    request_get(format!("/api/vehicules?{}", query)).await
}

pub async fn request_admin_delete_vehicule(id: String) -> Result<ApiResponse::<()>, Error> {
    request_delete(format!("/api/vehicules/{id}")).await
}


// Normal user routes
/*
pub async fn request_normal_get_vehicules() -> Result<ApiResponse::<Vec<FilteredVehicule>>, Error> {
    request_get(format!("api/vehicules")).await
}

pub async fn request_normal_get_vehicule_with_id(id: String) -> Result<ApiResponse::<FilteredVehicule>, Error> {
    request_get(format!("api/vehicules/{id}")).await
}
*/
