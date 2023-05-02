pub mod api_response;
pub mod app;
pub mod components;
pub mod context;
pub mod error;
pub mod features;
pub mod hooks;
pub mod layout;
pub mod pages;
pub mod routes;
pub mod services;
pub mod types;
pub mod utils;


use crate::app::App;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    log::info!("Rendering Yew app");

    yew::Renderer::<App>::new().render();
}
