use gloo::utils::document;
use yew::prelude::*;
use yew_router::prelude::*;
//use yew_hooks::prelude::*;

use crate::hooks::user_context::use_user_context;
use crate::routes::AppRoute;
use crate::utils::toggle_class;


#[function_component]
pub fn Sidebar() -> Html {
    let user_ctx = use_user_context();

    let sidebar_node_ref = use_node_ref();
    

    html! {
        if user_ctx.is_admin() {
        <aside class="aside is-placed-left is-expanded" ref={sidebar_node_ref}>
            { sidebar_top() }
            { sidebar_menu() }
        </aside>
        }
    }
}

fn sidebar_top() -> Html {
    html!{
        <div class="aside-tools">
            <div class="aside-tools-label">
                <span><b>{"Admin"}</b></span>
            </div>
        </div>
    }
}

fn sidebar_menu() -> Html {
    let onclick_vehicule = toggle_menu("vehicule-menu".to_owned());
    let onclick_user = toggle_menu("user-menu".to_owned());

    html!{
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
                                <span class="menu-item-label">{"Ver Vehiculos"}</span>
                            </Link<AppRoute>>
                        </li>
                        <li>
                            <Link<AppRoute> to={AppRoute::VehiculesRegister} >
                                <span class="menu-item-label">{"Registrar Vehiculo"}</span>
                            </Link<AppRoute>>
                        </li>
                    </ul>
                </li>
                <li id="user-menu">
                    <a class="has-icon has-dropdown-icon" onclick={onclick_user}>
                        <span class="icon"><i class="fa-solid fa-users"></i></span>
                        <span class="menu-item-label">{"Usuarios"}</span>
                        <div class="dropdown-icon">
                          <span class="icon"><i class="fa-solid fa-angle-down"></i></span>
                        </div>
                    </a>
                    <ul>
                        <li>
                            <Link<AppRoute> to={AppRoute::Users} >
                                    <span class="menu-item-label">{"Ver Usuarios"}</span>
                            </Link<AppRoute>>
                        </li>
                        <li>
                            <Link<AppRoute> to={AppRoute::Users} >
                                <span class="menu-item-label">{"Registrar usuario"}</span>
                            </Link<AppRoute>>
                        </li>
                    </ul>
                </li>
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

            toggle_class(children.clone(), "fa-angle-up");
            toggle_class(children, "fa-angle-down");
            toggle_class(element, "is-active");
        }
    })
}
