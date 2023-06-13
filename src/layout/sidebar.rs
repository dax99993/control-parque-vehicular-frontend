use gloo::utils::{document, document_element};
use yew::prelude::*;
use yew_router::prelude::*;

use yew_hooks::use_click_away;

use crate::hooks::user_context::use_user_context;
use crate::routes::AppRoute;
use crate::shadow_clone;
use crate::utils::{toggle_class, remove_class, add_class, has_class};


//TODO: Create a sidebar for normal user and move current one to SidebarAdmin

#[function_component]
pub fn Sidebar() -> Html {
    // Context
    let user_ctx = use_user_context();

    // States
    let sidebar_node_ref = use_node_ref();

    
    // Dont show sidebar if not logged in
    {
        shadow_clone!(user_ctx);
        use_effect_with_deps(move |user_ctx| {
            let element = document_element();

            if user_ctx.is_authenticated() {
                add_class(&element, "has-aside-left");
                add_class(&element, "has-aside-mobile-transition");
                //add_class(element.clone(), "has-aside-mobile-expanded");
            } else {
                remove_class(&element, "has-aside-left");
                remove_class(&element, "has-aside-mobile-transition");
                //remove_class(element.clone(), "has-aside-mobile-expanded");
            }
        },
        user_ctx.clone())
    }

    // Close sidebar on mobile when clicking outside of it
    /*
    use_click_away(sidebar_node_ref.clone(), move |_: Event| {
        //collapse_sidebar();
        if let Some(icon) = document().get_element_by_id("navbar-toggle-sidebar-button") {
            //toggle_class(&document_element(), "has-aside-mobile-expanded");
            let e = document_element(); 
            if has_class(&e, "has-aside-mobile-expanded") {
                remove_class(&e, "has-aside-mobile-expanded");
            }
            toggle_class(&icon, "fa-bars");
            toggle_class(&icon, "fa-xmark");
        }
    });
    */


    html! {
        <aside class="aside is-placed-left is-expanded" ref={sidebar_node_ref}>
            if user_ctx.is_admin() {
                { admin_sidebar() }
            } else if user_ctx.is_authenticated() {
                { normal_sidebar() }
            }
        </aside>
    }
}


fn admin_sidebar() -> Html {
    let onclick_vehicule = toggle_menu("vehicule-menu".to_owned());

    html!{
        <>
        <div class="aside-tools">
            <div class="aside-tools-label">
                <span><b>{"Admin"}</b></span>
            </div>
        </div>

        <div class="menu is-menu-main">
            <p class="menu-label">{"Administracion"}</p>
            <ul class="menu-list">
                <li id="vehicule-menu">
                    <a class="has-icon has-dropdown-icon" onclick={onclick_vehicule}>
                        <span class="icon"><i class="fa-solid fa-car"></i></span>
                        <span class="menu-item-label">{"Vehiculos"}</span>
                        <div class="dropdown-icon">
                          <span class="icon"><i class="fa-solid fa-angle-down"></i></span>
                        </div>
                    </a>
                    <ul>
                        <li>
                            <Link<AppRoute> to={AppRoute::Vehicules} >
                                    <span class="menu-item-label">{"Ver vehiculos"}</span>
                            </Link<AppRoute>>
                        </li>
                        <li>
                            <Link<AppRoute> to={AppRoute::VehiculeAdd} >
                                <span class="menu-item-label">{"Registrar vehiculo"}</span>
                            </Link<AppRoute>>
                        </li>
                    </ul>
                </li>
                <Link<AppRoute> to={AppRoute::Users} >
                        <span class="icon"><i class="fa-solid fa-users"></i></span>
                        <span class="menu-item-label">{"Usuarios"}</span>
                </Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Requests} >
                        <span class="icon"><i class="fa-solid fa-id-card"></i></span>
                        <span class="menu-item-label">{"Peticiones"}</span>
                </Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Reports} >
                        <span class="icon"><i class="fa-sharp fa-regular fa-file-pdf"></i></span>
                        <span class="menu-item-label">{"Reportes"}</span>
                </Link<AppRoute>>
            </ul>
        </div>
        </>
    }
}


fn normal_sidebar() -> Html {
    html!{
        <>
        <div class="aside-tools">
            <div class="aside-tools-label">
                <span><b>{"Usuario"}</b></span>
            </div>
        </div>

        <div class="menu is-menu-main">
            <p class="menu-label">{"Menu"}</p>
            <ul class="menu-list">
                <Link<AppRoute> to={AppRoute::Vehicules} >
                        <span class="icon"><i class="fa-solid fa-car"></i></span>
                        <span class="menu-item-label">{"Vehiculos"}</span>
                </Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Requests} >
                        <span class="icon"><i class="fa-solid fa-id-card"></i></span>
                        <span class="menu-item-label">{"Peticiones"}</span>
                </Link<AppRoute>>
            </ul>
        </div>
        </>
    }
}

fn toggle_menu(menu_id: String) -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        if let Some(element) = document().get_element_by_id(&menu_id) {
            let children = element
                .get_elements_by_class_name("dropdown-icon")
                .item(0).unwrap()
                .get_elements_by_class_name("fa-solid")
                .item(0).unwrap();

            toggle_class(&children, "fa-angle-up");
            toggle_class(&children, "fa-angle-down");
            toggle_class(&element, "is-active");
        }
    })
}
