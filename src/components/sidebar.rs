use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::user_context::use_user_context;
use crate::routes::AppRoute;



#[function_component]
pub fn Sidebar() -> Html {
    let user_ctx = use_user_context();



    html! {
        if user_ctx.is_admin() {
        <aside class="aside is-placed-left is-expanded">
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
    let active = true;
    html!{
        <div class="menu is-menu-main">
            <p class="menu-label">{"Administracion"}</p>
            <ul class="menu-list">
                <li >
                    <a class="has-icon has-dropdown-icon is-active">
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
                        <a href="#void">
                            <span>{"Sub-item Two"}</span>
                        </a>
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
    }
}
