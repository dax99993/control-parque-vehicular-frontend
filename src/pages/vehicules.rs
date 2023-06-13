use yew::prelude::*;

use crate::hooks::user_context::use_user_context;
use crate::pages::admin::vehicules::AdminVehiculeView;
use crate::pages::normal::vehicules::NormalVehiculesView;


#[function_component]
pub fn VehiculesView() -> Html {
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }

    html! {
        if user_ctx.is_admin() {
            <AdminVehiculeView/>
        } else {
            <NormalVehiculesView/>
        }
    }
}
