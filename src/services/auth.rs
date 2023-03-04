use crate::{types::user::LoginUser, api_response::ApiResponse};

use super::request::request_post;




pub async fn login(login_user: LoginUser) -> Result<ApiResponse, reqwasm::Error> {
    request_post("http:/127.0.0.1:8000/api/auth/login".to_string(), login_user).await
}
