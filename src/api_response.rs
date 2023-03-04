use serde::{Serialize, Deserialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T= ()>
{
    //#[serde(skip_serializing)]
    //status_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>
}
pub type ApiError = ApiResponse<()>;


impl<T: Serialize + std::fmt::Debug> std::fmt::Display for ApiResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

/*
impl<T: Serialize + std::fmt::Debug> ApiResponse<T> {
    pub fn new() -> Self {
        Self { 
            status_code: 200,
            status: Some("success".into()),
            //status: None,
            message: None,
            data: None
        }
    }

    pub fn with_status_code(mut self, status_code: u16) -> Self {
        self.status_code = status_code;
        self
    }


    pub fn with_status<S: Into<Cow<'static, str>>>(mut self, status: S) -> Self { 
        self.status= Some(status.into());
        self
    }

    pub fn with_message<S: Into<Cow<'static, str>>>(mut self, message: S) -> Self { 
        self.message = Some(message.into());
        self
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
}


pub fn e500() -> ApiError {
    ApiError::new()
            .with_status_code(500)
            .with_status("fail")
            .with_message("Server Error")
}

pub fn e400() -> ApiError {
    ApiError::new()
            .with_status_code(400)
            .with_status("fail")
}

pub fn e401() -> ApiError {
    ApiError::new()
            .with_status_code(401)
            .with_status("fail")
}

pub fn e403() -> ApiError {
    ApiError::new()
            .with_status_code(403)
            .with_status("fail")
}

pub fn e404() -> ApiError {
    ApiError::new()
            .with_status_code(404)
            .with_status("fail")
}

pub fn e409() -> ApiError {
    ApiError::new()
            .with_status_code(409)
            .with_status("fail")
}
*/
