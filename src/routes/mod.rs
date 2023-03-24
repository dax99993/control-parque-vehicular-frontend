pub mod login;
pub mod logout;
pub mod home;
pub mod register;
pub mod vehicule;


use yew::prelude::*;
use yew_router::prelude::*;


use login::Login;
use logout::Logout;
use register::Register;
use home::Home;


use self::vehicule::get::GetVehicules;
use self::vehicule::register::RegisterVehicule;
use self::vehicule::edit::EditVehicule;


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
        AppRoute::Register => html! { <Register /> },
        AppRoute::Login => html! { <Login /> },
        AppRoute::Logout => html! { <Logout /> },
        // Home route
        AppRoute::Home => html! { <Home /> },
        // Vehicule routes
        AppRoute::Vehicules => html! { <GetVehicules/> },
        AppRoute::VehiculesRegister => html! { <RegisterVehicule/> },
        AppRoute::VehiculesEdit { id } => html! { <EditVehicule id={id}/> },
        // User routes
        AppRoute::Users => html! { {"users"} },
        // Request routes
        AppRoute::Requests => html! { {"requests"} },
        // Report routes
        AppRoute::Reports => html! { {"reports"} },
        // Nofound route
        AppRoute::NotFound => html! { "Page not found" },
    }
}


