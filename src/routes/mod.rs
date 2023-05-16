use yew::prelude::*;
use yew_router::prelude::*;

use uuid::Uuid;


use crate::pages::HomeView;
use crate::pages::NotFoundView;

use crate::pages::LoginView;
use crate::pages::LogoutView;
use crate::pages::SignupView;

use crate::pages::VehiculesView;
use crate::pages::EditVehiculeView;
use crate::pages::RegisterVehiculeView;

use crate::pages::ProfileView;

use crate::pages::UsersView;
use crate::pages::EditUserView;



///App Routes
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/logout")]
    Logout,
    // Vehicule routes
    #[at("/vehicules")]
    Vehicules,
    #[at("/vehicules/register")]
    VehiculeAdd,
    #[at("/vehicules/:id")]
    VehiculeEdit {id: Uuid},
    // User routes
    #[at("/users")]
    Users,
    #[at("/users/:id")]
    UserEdit {id: Uuid},
    #[at("/users/profile")]
    UserProfile,
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
        AppRoute::Signup => html! { <SignupView/> },
        AppRoute::Login => html! { <LoginView/> },
        AppRoute::Logout => html! { <LogoutView/> },
        // Home route
        AppRoute::Home => html! { <HomeView/> },
        // Vehicule routes
        AppRoute::Vehicules => html! { <VehiculesView/> },
        AppRoute::VehiculeAdd => html! { <RegisterVehiculeView /> },
        AppRoute::VehiculeEdit { id } => html! { <EditVehiculeView {id}/> },
        // User routes
        AppRoute::Users => html! { <UsersView/> },
        AppRoute::UserEdit { id } => html! { <EditUserView {id} />  },
        // Profile routes
        AppRoute::UserProfile => html! { <ProfileView/> },
        // Request routes
        AppRoute::Requests => html! { {"requests"} },
        // Report routes
        AppRoute::Reports => html! { {"reports"} },
        // Nofound route
        AppRoute::NotFound => html! { <NotFoundView/> },
    }
}


