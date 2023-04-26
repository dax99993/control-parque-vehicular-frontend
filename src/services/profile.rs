use common::models::user::{User, ChangePasswordMe};
use crate::api_response::ApiResponse;

use super::request::{request_get, request_post, request_delete, request_patch, request_multipart_patch};
use crate::error::Error;

// General user routes
pub async fn request_change_password(password: ChangePasswordMe) -> Result<ApiResponse::<()>, Error> {
    request_post(format!("api/users/me/change_password"), password).await
}

/*
pub async fn request_update_profile(updated_profile) -> Result<ApiResponse::<User>, Error> {
    request_patch(format!("api/users/me")).await
}
*/

pub async fn request_update_profile_picture(picture: reqwest::multipart::Form) -> Result<ApiResponse::<User>, Error> {
    request_multipart_patch(format!("api/users/me/picture"), picture).await
}

