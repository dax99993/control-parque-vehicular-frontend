use yew::prelude::*;
use yew_router::prelude::*;
use yew_hooks::use_async;

use common::models::user::Usuario;

use crate::shadow_clone;
use crate::hooks::user_context::{use_user_context, UseUserContextHandle};
use crate::routes::AppRoute;
use crate::utils::{remove_class, add_class, has_class, toggle_class};

use gloo::utils::{document, document_element};
use crate::features::profile::services::request_profile_image;


use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::ops::Deref;



#[function_component]
pub fn NavBar() -> Html {
    // Context
    let user_ctx = use_user_context();

    // HTML
    html!{
        <nav id="navbar-main" class="navbar is-fixed-top has-shadow">
            <NavBarBrand/>
            <NavBarBrandRight/>

            <div id="navbar-menu" class="navbar-menu fadeIn animated faster">
                if user_ctx.is_authenticated() {
                    <NavBarEndLoggedIn/>
                } else {
                    <NavBarEndLoggedOut/>
                }
            </div>
        </nav>
    }
}


#[function_component]
fn NavBarBrand() -> Html {
    //Context
    let user_ctx = use_user_context();


    //Callbacks
    // Redirect home on logo click
    let onclick_go_home = {
        shadow_clone![user_ctx];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            user_ctx.redirect_home();
        })
    };

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


    // Variables
    //let image_logo_url = "https://bulma.io/images/bulma-logo.png";
    let image_logo_url = "cpv_logo.png";

    
    //Html
    html!{
        <div class="navbar-brand">
            if user_ctx.is_authenticated() {
            <a class="navbar-item is-hidden-desktop jb-aside-mobile-toggle" onclick={aside_mobile_toggle}>
                <span class="icon"><i id="navbar-toggle-sidebar-button" class="fa-solid fa-bars"></i></span>
            </a>
            }
            <div class="navbar-item" onclick={onclick_go_home}>
                <img src={image_logo_url} width="112" height="28" />
            </div>
        </div>
    }
}


#[function_component]
fn NavBarBrandRight() -> Html {

    //Callbacks
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
    
    html!{
        <div class="navbar-brand is-right">
            <a class="navbar-item is-hidden-desktop jb-navbar-menu-toggle" data-target="navbar-menu" onclick={navbar_menu_mobile_toggle}>
                <span class="icon"><i id="navbar-toggle-menu-button" class="fa-solid fa-ellipsis-vertical"></i></span>
            </a>
        </div>
   }
}


#[function_component]
fn NavBarEndLoggedIn() -> Html {
    //Context
    let user_ctx = use_user_context();


    //States
    let image = use_state(|| vec![]);


    //Hooks
    let request_image = use_async(async {
        request_profile_image().await
    });

    {
        let request_image = request_image.clone();
        use_effect_with_deps(move |_| {
            request_image.run();
        }, ())
    }

    {
        let image = image.clone();
        use_effect_with_deps(move |request| {
            if let Some(response) = &request.data {
                image.set(response.clone()); 
            }
        }, request_image.clone())
    }

    let nombres = user_ctx.get_user().unwrap().nombres.clone();
    
    html!{
        <div class="navbar-end">
            <div class="navbar-item has-dropdown has-dropdown-with-icons has-divider has-user-avatar is-hoverable">
                <a class="navbar-link is-arrowless">
                    <div class="is-user-avatar">
                        if !image.deref().is_empty() {
                            <img src={ format!("data:image/jpeg;base64,{}", STANDARD.encode(&image.deref())) } />
                        }
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

#[function_component]
fn NavBarEndLoggedOut() -> Html {
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
