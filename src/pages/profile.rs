use yew::prelude::*;

use crate::pages::admin::profile::AdminProfileView;
use crate::pages::normal::profile::NormalProfileView;
use crate::hooks::user_context::use_user_context;


#[function_component]
pub fn ProfileView() -> Html {
    // Context
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }


    html! {
        if user_ctx.is_admin() {
            <AdminProfileView/>
        } else {
            <NormalProfileView/>
        }
    }
}
