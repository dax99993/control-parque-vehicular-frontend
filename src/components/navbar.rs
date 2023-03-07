use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::hooks::user_context::{use_user_context, UseUserContextHandle};


#[function_component]
pub fn NavBar() -> Html {
    let user_ctx = use_user_context();

    let nav_brand = html! {
        <a>
        <Link<AppRoute> to={AppRoute::Home} >
            <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28" />
        </Link<AppRoute>>
        </a>
    };
    let nav_end = 
    html! {
            {
                if user_ctx.is_authenticated() {
                    logged_in_view()
                } else {
                    logged_out_view()
                }
            }
    };
    html! {
        <ybc::Navbar 
            navburger=true 
            navbrand={Some(nav_brand)} 
            navend={Some(nav_end)}
        >
        </ybc::Navbar>
    }

}


fn logged_out_view() -> Html {
    html! {
        <ybc::NavbarItem>
            <ybc::Buttons>
                <Link<AppRoute> to={AppRoute::Login} classes="button is-light">
                    { "Iniciar sesion" }
                </Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Register} classes="button is-primary">
                    { "Registrar" }
                </Link<AppRoute>>
            </ybc::Buttons>
        </ybc::NavbarItem>
    }
}

fn logged_in_view() -> Html {
    html! {
        <ybc::NavbarItem>
            <ybc::Buttons>
                <Link<AppRoute> to={AppRoute::Register} classes="button is-light">
                    { "Configuracion" }
                </Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Logout} classes="button is-primary">
                    { "Cerrar sesion" }
                </Link<AppRoute>>
            </ybc::Buttons>
        </ybc::NavbarItem>
    }
}
