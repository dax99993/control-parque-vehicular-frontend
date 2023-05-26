use common::models::user::{Usuario, CambiarMiPassword};
use crate::api_response::ApiResponse;

use crate::services::request::{request_post, request_patch, request_multipart_patch, request_image};
use crate::error::Error;

// General user routes
pub async fn request_profile_image() -> Result<Vec<u8>, Error> {
    request_image(format!("/api/users/me/picture")).await
}

pub async fn request_change_password(password: CambiarMiPassword) -> Result<ApiResponse::<()>, Error> {
    request_post(format!("/api/users/me/change-password"), password).await
}

/*
pub async fn request_update_profile(updated_profile) -> Result<ApiResponse::<User>, Error> {
    request_patch(format!("api/users/me")).await
}
*/

pub async fn request_update_profile_picture(picture: reqwest::multipart::Form) -> Result<ApiResponse::<Usuario>, Error> {
    request_multipart_patch(format!("/api/users/me/picture"), picture).await
}
