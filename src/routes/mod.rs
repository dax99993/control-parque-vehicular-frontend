pub mod login;
pub mod logout;
pub mod home;
pub mod register;


use yew::prelude::*;
use yew_router::prelude::*;


use login::Login;
use logout::Logout;
use register::Register;
use home::Home;


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
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Register => html! { <Register /> },
        AppRoute::Login=> html! { <Login /> },
        AppRoute::Logout=> html! { <Logout /> },
        AppRoute::Home=> html! { <Home /> },
        AppRoute::NotFound=> html! { "Page not found" },
    }
}


