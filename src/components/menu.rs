use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::user_context::use_user_context;
use crate::routes::AppRoute;




#[function_component]
pub fn Menu() -> Html {
    let user_ctx = use_user_context();

    html! {
        if user_ctx.is_admin() {
            <AdminMenu>
            </AdminMenu>
        }

    }
}


#[function_component]
fn AdminMenu() -> Html {


    html! {
        <ybc::Menu>
            <ybc::MenuLabel text={"Vehiculos"}>
            </ybc::MenuLabel>
            <ybc::MenuList>
                <Link<AppRoute> to={AppRoute::Vehicules} >
                    <li><a>{"Ver vehiculos"}</a></li>
                </Link<AppRoute>>
                <li><a>{"Editar Vehiculos"}</a></li>
                <li><a>{"Borrar Vehiculos"}</a></li>
            </ybc::MenuList>
        </ybc::Menu>
    }
}
