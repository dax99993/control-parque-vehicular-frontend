use serde::{Serialize, de::DeserializeOwned};
use gloo::storage::{LocalStorage, Storage};

use crate::api_response::ApiResponse;
use crate::error::Error;


pub const API_ROOT: &str = "http://localhost:8000";
const TOKEN_KEY: &str = "yew.token";


pub fn store_token(token: Option<String>) {
    if let Some(t) = token {
        LocalStorage::set(TOKEN_KEY, t).expect("failed to set token");
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }
}

pub fn get_token() -> Option<String> {
    match LocalStorage::get(TOKEN_KEY) {
        Ok(token) => Some(token),
        Err(_) => None
    }
}



pub async fn request_get<T>(url: String) -> Result<ApiResponse<T>, Error> 
where
    T: serde::de::DeserializeOwned
{
    request(reqwest::Method::GET, url, "").await
}

pub async fn request_delete(url: String) -> Result<ApiResponse, Error> {
    request(reqwest::Method::DELETE, url, "").await
}

pub async fn request_post<B, T>(url: String, body: B) -> Result<ApiResponse<T>, Error> 
where
    B: Serialize + 'static,
    T: DeserializeOwned
{
    request(reqwest::Method::POST, url, body).await
}

pub async fn request_patch<B, T>(url: String, body: B) -> Result<ApiResponse<T>, Error> 
where
    B: Serialize + 'static,
    T: DeserializeOwned
{
    request(reqwest::Method::PATCH, url, body).await
}


pub async fn request<B, T>(
    method: reqwest::Method,
    url: String,
    body: B)
-> Result<ApiResponse::<T>, Error> 
where
    B: Serialize + 'static,
    T: DeserializeOwned
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PATCH;
    let url = format!("{}{}", API_ROOT, url);

    let mut builder = reqwest::Client::new().request(method, &url);

    if let Some(token) = get_token() {
       builder = builder.bearer_auth(token);
    }

    if allow_body {
        builder = builder.json(&body);
    }


    match builder.send().await {
        Ok(response) => {
            match response.status().as_u16() {
                // Succesfull response
                200..=299 => {
                    response.json::<ApiResponse::<T>>().await
                        .map_err(|_| Error::DeserializeError)
                }

                400 => {
                    let message = response.json::<ApiResponse::<T>>().await
                        .map_err(|_| Error::DeserializeError)?
                        .message;
                    let msg = match message {
                        Some(m) => m.to_string(),
                        None => String::new(),
                    };

                    Err(Error::BadRequestError(msg))
                },
                401 => Err(Error::UnathorizedError),
                403 => Err(Error::ForbiddenError),
                404 => Err(Error::NotFoundError),
                500 => Err(Error::InternalServerError),
                _ => Err(Error::UnexpectedError),
            }
        }
        Err(_) => Err(Error::FailedRequestError),
    }
    
}

pub async fn request_multipart<T>(
    method: reqwest::Method,
    url: String,
    body: reqwest::multipart::Form)
-> Result<ApiResponse::<T>, Error> 
where
    T: DeserializeOwned
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PATCH;
    let url = format!("{}{}", API_ROOT, url);

    let mut builder = reqwest::Client::new().request(method, &url);

    if let Some(token) = get_token() {
       builder = builder.bearer_auth(token);
    }

    if allow_body {
        builder = builder.multipart(body);
    }


    match builder.send().await {
        Ok(response) => {
            match response.status().as_u16() {
                // Succesfull response
                200..=299 => {
                    response.json::<ApiResponse::<T>>().await
                        .map_err(|_| Error::DeserializeError)
                }

                400 => {
                    let message = response.json::<ApiResponse::<T>>().await
                        .map_err(|_| Error::DeserializeError)?
                        .message;
                    let msg = match message {
                        Some(m) => m.to_string(),
                        None => String::new(),
                    };

                    Err(Error::BadRequestError(msg))
                },
                401 => Err(Error::UnathorizedError),
                403 => Err(Error::ForbiddenError),
                404 => Err(Error::NotFoundError),
                500 => Err(Error::InternalServerError),
                _ => Err(Error::UnexpectedError),
            }
        }
        Err(_) => Err(Error::FailedRequestError),
    }
}



pub async fn request_multipart_post<T>(url: String, body: reqwest::multipart::Form) -> Result<ApiResponse<T>, Error> 
where
    T: DeserializeOwned
{
    request_multipart(reqwest::Method::POST, url, body).await
}

pub async fn request_multipart_patch<T>(url: String, body: reqwest::multipart::Form) -> Result<ApiResponse<T>, Error> 
where
    T: DeserializeOwned
{
    request_multipart(reqwest::Method::PATCH, url, body).await
}


pub async fn request_image(url: String) -> Result<Vec<u8>, Error> {
    let url = format!("{}{}", API_ROOT, url);

    //let mut builder = reqwest::Client::new().request(method, &url);
    let mut builder = reqwest::Client::new().request(reqwest::Method::GET, &url);

    if let Some(token) = get_token() {
       builder = builder.bearer_auth(token);
    }

    match builder.send().await {
        Ok(response) => {
            match response.status().as_u16() {
                // Succesfull response
                200..=299 => {
                    match response.bytes().await {
                        Ok(bytes) => {
                            Ok(bytes.as_ref().to_vec())
                        },
                        Err(_) => {
                            Err(Error::DeserializeError)
                        },
                    }
                }

                400 => {
                    Err(Error::BadRequestError(String::new()))
                },
                401 => Err(Error::UnathorizedError),
                403 => Err(Error::ForbiddenError),
                404 => Err(Error::NotFoundError),
                500 => Err(Error::InternalServerError),
                _ => Err(Error::UnexpectedError),
            }
        }
        Err(_) => Err(Error::FailedRequestError),
    }
    
}
