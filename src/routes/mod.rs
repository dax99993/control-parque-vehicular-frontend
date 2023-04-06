pub mod home;
pub mod notfound;
pub mod auth;
pub mod vehicule;


use yew::prelude::*;
use yew_router::prelude::*;


use home::HomeView;
use notfound::NotFoundView;


use self::auth::login::LoginView;
use self::auth::logout::LogoutView;
use self::auth::register::RegisterView;

use self::vehicule::delete::VehiculeDeleteView;
use self::vehicule::get::GetVehiculesView;
use self::vehicule::register::RegisterVehiculeView;
use self::vehicule::edit::EditVehiculeView;


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
    #[at("/vehicules/register")]
    VehiculesRegister,
    #[at("/vehicules/edit/:id")]
    VehiculesEdit { id: String },
    #[at("/vehicules/delete/:id")]
    VehiculesDelete{ id: String },
    // User routes
    #[at("/users")]
    Users,
    // Request routes
    #[at("/requests")]
    Requests,
    // Report routes
    #[at("/reports")]
    Reports,
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
        AppRoute::Vehicules => html! { <GetVehiculesView/> },
        AppRoute::VehiculesRegister => html! { <RegisterVehiculeView/> },
        AppRoute::VehiculesEdit { id } => html! { <EditVehiculeView id={id}/> },
        AppRoute::VehiculesDelete { id } => html! { <VehiculeDeleteView id={id}/> },
        // User routes
        AppRoute::Users => html! { {"users"} },
        // Request routes
        AppRoute::Requests => html! { {"requests"} },
        // Report routes
        AppRoute::Reports => html! { {"reports"} },
        // Nofound route
        AppRoute::NotFound => html! { <NotFoundView/> },
    }
}


