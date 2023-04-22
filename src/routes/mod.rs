pub mod home;
pub mod notfound;
pub mod auth;

//pub mod vehicule;
//use self::vehicule::get::GetVehiculesView;

pub mod admin;
//pub mod normal;
pub mod general;


use yew::prelude::*;
use yew_router::prelude::*;


use home::HomeView;
use notfound::NotFoundView;

use self::auth::login::LoginView;
use self::auth::logout::LogoutView;
use self::auth::register::RegisterView;

use self::general::vehicule::VehiculesView;


///App Routes
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/logout")]
    Logout,
    // Vehicule routes
    #[at("/vehicules")]
    Vehicules,
    // User routes
    #[at("/users")]
    Users,
    // Request routes
    #[at("/requests")]
    Requests,
    // Report routes
    #[at("/reports")]
    Reports,
    // Report routes
    #[at("/admin")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(route: AppRoute) -> Html {
    match route {
        // Auth routes
        AppRoute::Register => html! { <RegisterView/> },
        AppRoute::Login => html! { <LoginView/> },
        AppRoute::Logout => html! { <LogoutView/> },
        // Home route
        AppRoute::Home => html! { <HomeView/> },
        // Vehicule routes
        AppRoute::Vehicules => html! { <VehiculesView/> },
        // User routes
        AppRoute::Users => html! { {"users"} },
        // Request routes
        AppRoute::Requests => html! { {"requests"} },
        // Report routes
        AppRoute::Reports => html! { {"reports"} },
        // Nofound route
        AppRoute::NotFound => html! { <NotFoundView/> },
        // Admin route
        AppRoute::Admin => html! { <p>{"admin"}</p> },
    }
}


