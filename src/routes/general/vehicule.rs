use yew::prelude::*;

use crate::hooks::user_context::use_user_context;

use crate::routes::admin::vehicule::main_view::AdminVehiculePage;


#[function_component]
pub fn VehiculesView() -> Html {
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }

    html! {
        if user_ctx.is_admin() {
            <AdminVehiculePage />
        } else {
            <p>{"Vehicule normal user page"}</p>
        }
    }
}
