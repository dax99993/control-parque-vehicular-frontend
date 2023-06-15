use yew::prelude::*;

use crate::hooks::user_context::use_user_context;
use crate::pages::admin::vehicules::AdminVehiculeView;


#[function_component]
pub fn VehiculesView() -> Html {
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }

    html! {
        <AdminVehiculeView/>
    }
}
