use yew::prelude::*;
use yew_router::prelude::*;

use common::models::user::Usuario;

use crate::shadow_clone;
use crate::hooks::user_context::{use_user_context, UseUserContextHandle};
use crate::routes::AppRoute;
use crate::utils::{remove_class, add_class, has_class, toggle_class};

use gloo::utils::{document, document_element};


#[function_component]
pub fn NavBar() -> Html {
    // Context
    let user_ctx = use_user_context();

    // Expand/Collapse sidebar
    let aside_mobile_toggle = {
        Callback::from(move |_| {
            if let Some(icon) = document().get_element_by_id("navbar-toggle-sidebar-button") {
                let e = document_element(); 
                if has_class(&e, "has-aside-mobile-expanded") {
                    remove_class(&e, "has-aside-mobile-expanded");
                } else {
                    add_class(&e, "has-aside-mobile-expanded");
                }
                
                toggle_class(&icon, "fa-bars");
                toggle_class(&icon, "fa-xmark");
            }
        })
    };

    // Expand/Collapse Navbar mobile menu
    let navbar_menu_mobile_toggle = 
        Callback::from(move |_| {
            if let Some(element) = document().get_element_by_id("navbar-menu") {
                if let Some(dropdownicon) = document().get_element_by_id("navbar-toggle-menu-button") {
                    // Toggle menu expansion
                    toggle_class(&element, "is-active");
                    // Toggle icon
                    toggle_class(&dropdownicon, "fa-ellipsis-vertical");
                    toggle_class(&dropdownicon, "fa-xmark");
                }
            }
        });


    // HTML
    html!{
        <nav id="navbar-main" class="navbar is-fixed-top has-shadow">
            { 
                navbar_brand(user_ctx.clone(), aside_mobile_toggle) 
            }
            {
                navbar_brand_right(navbar_menu_mobile_toggle)
            }

            <div id="navbar-menu" class="navbar-menu fadeIn animated faster">
            {
                if user_ctx.is_authenticated() {
                    let user = user_ctx.get_user().unwrap();
                    navbar_end_logged_in(user)
                } else {
                    navbar_end_logged_out()
                }
            }
            </div>
        </nav>
    }
}


fn navbar_brand(user_ctx: UseUserContextHandle, onclick: Callback<MouseEvent>) -> Html {

    //let image_logo_url = "https://bulma.io/images/bulma-logo.png";
    let image_logo_url = "cpv_logo.png";

    // Redirect home on logo click
    let onclick_go_home = {
        shadow_clone![user_ctx];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            user_ctx.redirect_home();
        })
    };


    html!{
        <div class="navbar-brand">
            if user_ctx.is_authenticated() {
            <a class="navbar-item is-hidden-desktop jb-aside-mobile-toggle" {onclick}>
                <span class="icon"><i id="navbar-toggle-sidebar-button" class="fa-solid fa-bars"></i></span>
            </a>
            }
            <div class="navbar-item" onclick={onclick_go_home}>
                <img src={image_logo_url} width="112" height="28" />
            </div>
        </div>
    }
}


fn navbar_brand_right(onclick: Callback<MouseEvent>) -> Html {
    html!{
        <div class="navbar-brand is-right">
            <a class="navbar-item is-hidden-desktop jb-navbar-menu-toggle" data-target="navbar-menu" {onclick}>
                <span class="icon"><i id="navbar-toggle-menu-button" class="fa-solid fa-ellipsis-vertical"></i></span>
            </a>
        </div>
   }
}


fn navbar_end_logged_in(user: Usuario) -> Html {

    let nombres = user.nombres.clone();
    let imagen_url = user.imagen_url("http://127.0.0.1:8000/");
    
    html!{
        <div class="navbar-end">
            <div class="navbar-item has-dropdown has-dropdown-with-icons has-divider has-user-avatar is-hoverable">
                <a class="navbar-link is-arrowless">
                    <div class="is-user-avatar">
                        <img src={ imagen_url } />
                    </div>
                    <div class="is-user-name">
                        <span>{ nombres }</span>
                    </div>
                    <span class="icon"><i class="fa-solid fa-chevron-down"></i></span>
                </a>
                <div class="navbar-dropdown is-right">
                    <div class="navbar-item">
                        <Link<AppRoute> to={AppRoute::UserProfile}>
                            <span class="icon"><i class="fa-solid fa-user"></i></span>
                            { "Perfil" }
                        </Link<AppRoute>>
                    </div>
                    <div class="navbar-item">
                        <Link<AppRoute> to={AppRoute::Logout}>
                            <span class="icon"><i class="fa-solid fa-power-off"></i></span>
                            { "Cerrar sesion" }
                        </Link<AppRoute>>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn navbar_end_logged_out() -> Html {
    html!{
        <div class="navbar-end">
            <div class="navbar-item">
                <div class="buttons">
                    <Link<AppRoute> to={AppRoute::Login} classes="button is-light">
                        { "Iniciar sesion" }
                    </Link<AppRoute>>
                    <Link<AppRoute> to={AppRoute::Signup} classes="button is-primary">
                        { "Registrar" }
                    </Link<AppRoute>>
                </div>
            </div>
        </div>
    }
}
