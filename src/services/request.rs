use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use gloo::storage::{LocalStorage, Storage};

use crate::api_response::ApiResponse;
use crate::error::Error;

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

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Delete,
    Post,
    Patch,
}


pub async fn request<B, T>(method: HttpMethod, url: String, body: B) -> Result<ApiResponse::<T>, Error> 
where
    B: Serialize + 'static,
    T: serde::de::DeserializeOwned
{
    let allow_body = method == HttpMethod::Post || method == HttpMethod::Patch;
    //let url = format!("{}{}", API_ROOT, url);
    let mut request = match method {
        HttpMethod::Get => Request::get(&url),
        HttpMethod::Delete => Request::delete(&url),
        HttpMethod::Patch => Request::patch(&url),
        HttpMethod::Post=> Request::post(&url),
    };


    let header = reqwasm::http::Headers::new();
    header.append("Content-Type", "application/json");
        
    if let Some(token) = get_token() {
       header.append("Authorization", &format!("Bearer {}", token));
    }

    request = request.headers(header);

    if allow_body {
        let body = serde_json::to_string(&body).unwrap();
        request = request.body(body);
    }

    let response = request.send().await
        .map_err(|_| Error::FailedRequestError)?;


    if response.ok() {
        response.json::<ApiResponse::<T>>().await
            .map_err(|_| Error::DeserializeError)
    } else {
        match response.status() {
            400 => Err(Error::BadRequestError),
            401 => Err(Error::UnathorizedError),
            403 => Err(Error::ForbiddenError),
            404 => Err(Error::NotFoundError),
            500 => Err(Error::InternalServerError),
            _ => Err(Error::UnexpectedError),
        }
    }
    
}


pub async fn request_get<T>(url: String) -> Result<ApiResponse<T>, Error> 
where
    T: serde::de::DeserializeOwned
{
    request(HttpMethod::Get, url, "").await
}

pub async fn request_delete(url: String) -> Result<ApiResponse, Error> {
    request(HttpMethod::Delete, url, "").await
}

pub async fn request_post<B, T>(url: String, body: B) -> Result<ApiResponse<T>, Error> 
where
    B: Serialize + 'static,
    T: serde::de::DeserializeOwned
{
    request(HttpMethod::Post, url, body).await
}

pub async fn request_patch<B, T>(url: String, body: B) -> Result<ApiResponse<T>, Error> 
where
    B: Serialize + 'static,
    T: serde::de::DeserializeOwned
{
    request(HttpMethod::Patch, url, body).await
}

