use std::ops::Deref;

use web_sys::Element;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::hooks::user_context::use_user_context;

use gloo::utils::{document, document_element};

#[function_component]
pub fn NavBar() -> Html {
    let user_ctx = use_user_context();
    

    let aside_mobile_toggle = {
        Callback::from(move |_| {
           // Toggle sidebar expansion
           let e = document_element();
           toggle_class(e.clone(), "has-aside-mobile-expanded");
           log::debug!("document class {:?}", e.class_name());
           // Toggle icon
           let el = document().get_elements_by_class_name("jb-aside-mobile-toggle").item(0).unwrap();
           let dropdownicon = el.get_elements_by_class_name("icon").item(0).unwrap()
               .get_elements_by_class_name("fa-solid").item(0).unwrap();
           toggle_class(dropdownicon.clone(), "fa-bars");
           toggle_class(dropdownicon.clone(), "fa-xmark");
           log::debug!("navbar left dropdown classes {:?}", el.class_name());
        })
    };

    let navbar_menu_mobile_toggle = 
        Callback::from(move |_| {
           // Toggle menu expansion
           let el = document().get_element_by_id("navbar-menu").unwrap();
           toggle_class(el.clone(), "is-active");
           log::debug!("navbar right classes {:?}", el.class_name());
           // Toggle icon
           let el = document().get_elements_by_class_name("jb-navbar-menu-toggle").item(0).unwrap();
           let dropdownicon = el.get_elements_by_class_name("icon").item(0).unwrap()
               .get_elements_by_class_name("fa-solid").item(0).unwrap();
           toggle_class(dropdownicon.clone(), "fa-ellipsis-vertical");
           toggle_class(dropdownicon.clone(), "fa-xmark");
           log::debug!("navbar right dropdown classes {:?}", el.class_name());
        });


    html!{
        <nav id="navbar-main" class="navbar is-fixed-top has-shadow">
            { 
                navbar_brand(aside_mobile_toggle) 
            }
            {
                navbar_brand_right(navbar_menu_mobile_toggle)
            }

            <div id="navbar-menu" class="navbar-menu fadeIn animated faster">
            {
                if user_ctx.is_authenticated() {
                    let first_name = (*user_ctx).clone().unwrap().first_name;
                    navbar_end_logged_in(first_name)
                } else {
                    navbar_end_logged_out()
                }
            }
            </div>
        </nav>
    }
}

fn navbar_brand(onclick: Callback<MouseEvent>) -> Html {
    html!{
        <div class="navbar-brand">
            <a class="navbar-item is-hidden-desktop jb-aside-mobile-toggle" {onclick}>
                <span class="icon"><i class="fa-solid fa-bars"></i></span>
            </a>
            <div class="navbar-item" href="https://bulma.io">
                <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28" />
            </div>
        </div>
    }
}

fn navbar_brand_right(onclick: Callback<MouseEvent>) -> Html {
    html!{
        <div class="navbar-brand is-right">
            <a class="navbar-item is-hidden-desktop jb-navbar-menu-toggle" data-target="navbar-menu" {onclick}>
                <span class="icon"><i class="fa-solid fa-ellipsis-vertical"></i></span>
            </a>
        </div>
   }
}


fn navbar_end_logged_in(first_name: String) -> Html {
    html!{
        <div class="navbar-end">
            <div class="navbar-item has-dropdown has-dropdown-with-icons has-divider has-user-avatar is-hoverable">
                <a class="navbar-link is-arrowless">
                    <div class="is-user-avatar">
                        <img class="is-rounded" src="https://avatars.dicebear.com/v2/initials/john-doe.svg" alt="John Doe"/>
                    </div>
                    <div class="is-user-name">
                        <span>{ first_name }</span>
                    </div>
                    <span class="icon"><i class="fa-solid fa-chevron-down"></i></span>
                </a>
                <div class="navbar-dropdown is-right">
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
                    <Link<AppRoute> to={AppRoute::Register} classes="button is-primary">
                        { "Registrar" }
                    </Link<AppRoute>>
                </div>
            </div>
        </div>
    }
}

fn toggle_class(e: Element, class: &str) {
    let e_classes = e.class_name();
    let mut classes: Vec<&str> = e_classes
        .split_whitespace()
        .collect();
    if classes.contains(&class) {
        classes.retain(|&c| c != class);
    } else {
        classes.push(class);
    };
    e.set_class_name(&classes.join(" "));
}
