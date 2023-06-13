use crate::api_response::ApiResponse;

use crate::services::request::{request_get, request_post, request_delete, request_patch, request_multipart_patch, request_image};
use crate::error::Error;

// Users
use common::models::user::{Usuario, ActualizaUsuario};


/*
pub async fn request_admin_update_user(id: String, updated_user: ActualizaUsuario) -> Result<ApiResponse::<Usuario>, Error> {
    request_patch(format!("/api/users/{id}"), updated_user).await
}

pub async fn request_admin_update_user_picture(id: String, picture: reqwest::multipart::Form) -> Result<ApiResponse::<Usuario>, Error> {
    request_multipart_patch(format!("/api/users/picture/{id}"), picture).await
}
*/

// Profile
use common::models::user::CambiarMiPassword;
pub async fn request_imagen_perfil() -> Result<Vec<u8>, Error> {
    request_image(format!("/api/users/me/picture")).await
}

pub async fn request_cambiar_password(password: CambiarMiPassword) -> Result<ApiResponse::<()>, Error> {
    request_post(format!("/api/users/me/change-password"), password).await
}

pub async fn request_actualizar_imagen_perfil(picture: reqwest::multipart::Form) -> Result<ApiResponse::<Usuario>, Error> {
    request_multipart_patch(format!("/api/users/me/picture"), picture).await
}


// General user routes
pub async fn request_imagen_usuario(imagen: String) -> Result<Vec<u8>, Error> {
    request_image(format!("/api/users/picture/{imagen}")).await
}


// Vehicules
use common::models::vehicule::Vehiculo;

pub async fn request_get_vehiculo_con_id(id: String) -> Result<ApiResponse::<Vehiculo>, Error> {
    request_get(format!("/api/vehicules/{id}")).await
}

pub async fn request_get_vehiculos(
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

pub async fn request_imagen_vehiculo(imagen: String) -> Result<Vec<u8>, Error> {
    request_image(format!("/api/vehicules/picture/{imagen}")).await
}

// Departments
use common::models::department::Departamento;

// All users
pub async fn request_get_departamentos() -> Result<ApiResponse::<Vec<Departamento>>, Error> {
    request_get(format!("/api/departments")).await
}

// Vehicule Request

