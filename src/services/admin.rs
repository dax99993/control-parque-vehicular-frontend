use crate::api_response::ApiResponse;

use crate::services::request::{request_get, request_post, request_delete, request_patch, request_multipart_patch, request_image};
use crate::error::Error;

// Users
use common::models::user::{Usuario, ActualizaUsuario};

pub async fn request_admin_get_users() -> Result<ApiResponse::<Vec<Usuario>>, Error> {
    request_get(format!("/api/users")).await
}

pub async fn request_admin_get_user_with_id(id: String) -> Result<ApiResponse::<Usuario>, Error> {
    request_get(format!("/api/users/{id}")).await
}

/*
pub async fn request_admin_get_user_picture(filename: String) -> Result<ApiResponse::<Usuario>, Error> {
    request_get(format!("/api/users/{filename}")).await
}
*/

pub async fn request_admin_delete_user(id: String) -> Result<ApiResponse::<()>, Error> {
    request_delete(format!("/api/users/{id}")).await
}

pub async fn request_admin_update_user(id: String, updated_user: ActualizaUsuario) -> Result<ApiResponse::<Usuario>, Error> {
    request_patch(format!("/api/users/{id}"), updated_user).await
}

pub async fn request_admin_update_user_picture(id: String, picture: reqwest::multipart::Form) -> Result<ApiResponse::<Usuario>, Error> {
    request_multipart_patch(format!("/api/users/picture/{id}"), picture).await
}


// General user routes
//pub async fn request_imagen_usuario(imagen: &str) -> Result<Vec<u8>, Error> {
pub async fn request_imagen_usuario(imagen: String) -> Result<Vec<u8>, Error> {
    request_image(format!("/api/users/picture/{imagen}")).await
}


// Vehicules
use common::models::vehicule::{Vehiculo, NuevoVehiculo, ActualizaVehiculo};

pub async fn request_admin_create_vehicule(nuevo_vehicule: NuevoVehiculo) -> Result<ApiResponse::<Vehiculo>, Error> {
    request_post(format!("/api/vehicules"), nuevo_vehicule).await
}

pub async fn request_admin_get_vehicule_with_id(id: String) -> Result<ApiResponse::<Vehiculo>, Error> {
    request_get(format!("/api/vehicules/{id}")).await
}

pub async fn request_admin_update_vehicule(id: String, updated_vehicule: ActualizaVehiculo) -> Result<ApiResponse::<Vehiculo>, Error> {
    request_patch(format!("/api/vehicules/{id}"), updated_vehicule).await
}

pub async fn request_admin_update_vehicule_picture(id: String, picture: reqwest::multipart::Form) -> Result<ApiResponse::<Vehiculo>, Error> {
    request_multipart_patch(format!("/api/vehicules/picture/{id}"), picture).await
}

pub async fn request_admin_delete_vehicule(id: String) -> Result<ApiResponse::<()>, Error> {
    request_delete(format!("/api/vehicules/{id}")).await
}

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

pub async fn request_vehicule_picture(imagen: String) -> Result<Vec<u8>, Error> {
    request_image(format!("/api/vehicules/picture/{imagen}")).await
}

// Departments
use common::models::department::Departamento;

pub async fn request_admin_delete_department(id: String) -> Result<ApiResponse::<()>, Error> {
    request_delete(format!("/api/users/{id}")).await
}

// All users
pub async fn request_get_departments() -> Result<ApiResponse::<Vec<Departamento>>, Error> {
    request_get(format!("/api/departments")).await
}
