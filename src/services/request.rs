use reqwasm::{Error, http::Request};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use gloo::storage::{LocalStorage, Storage};

use crate::api_response::ApiResponse;

const TOKEN_KEY: &str = "yew.token";

fn store_token(token: Option<String>) {
    if let Some(t) = token {
        LocalStorage::set(TOKEN_KEY, t).expect("failed to set token");
    } else {
        LocalStorage::delete(TOKEN_KEY) ;
    }
}

fn get_token() -> Option<String> {
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

    let result = request.send().await;

    let response = result?;
    //log::debug!("{:?}", response);
    
    response.json::<ApiResponse::<T>>().await
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

pub async fn request_post<B>(url: String, body: B) -> Result<ApiResponse, Error> 
where
    B: Serialize + 'static,
{
    request(HttpMethod::Post, url, body).await
}

pub async fn request_patch<B>(url: String, body: B) -> Result<ApiResponse, Error> 
where
    B: Serialize + 'static + Into<JsValue>,
{
    request(HttpMethod::Patch, url, body).await
}
