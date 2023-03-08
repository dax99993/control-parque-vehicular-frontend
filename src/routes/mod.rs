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
    #[at("/vehicules")]
    Vehicules,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(route: AppRoute) -> Html {
    match route {
        // Auth routes
        AppRoute::Register => html! { <Register /> },
        AppRoute::Login=> html! { <Login /> },
        AppRoute::Logout=> html! { <Logout /> },
        // Home route
        AppRoute::Home=> html! { <Home /> },
        // Vehicule routes
        AppRoute::Vehicules=> html! { <GetVehicules/> },
        // User routes
        // Request routes
        // Report routes
        // Nofound route
        AppRoute::NotFound=> html! { "Page not found" },
    }
}


