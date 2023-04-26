pub mod home;
pub mod notfound;
pub mod auth;

pub mod admin;
//pub mod normal;
pub mod general;


use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;


use home::HomeView;
use notfound::NotFoundView;

use self::auth::login::LoginView;
use self::auth::logout::LogoutView;
use self::auth::register::RegisterView;

use self::general::vehicule::VehiculesView;
use self::general::profile::ProfileView;

use self::admin::vehicule::edit::view::EditVehiculeView;
use self::admin::vehicule::register::RegisterVehiculeView;




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
    VehiculeAdd,
    #[at("/vehicules/:id")]
    VehiculeEdit {id: Uuid},
    //VehiculesEdit {id: String},
    // User routes
    #[at("/users")]
    Users,
    #[at("/users/profile")]
    UserProfile,
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
        AppRoute::VehiculeAdd => html! { <RegisterVehiculeView /> },
        AppRoute::VehiculeEdit { id } => html! { <EditVehiculeView {id}/> },
        // User routes
        AppRoute::Users => html! { {"users"} },
        AppRoute::UserProfile => html! { <ProfileView/> },
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


