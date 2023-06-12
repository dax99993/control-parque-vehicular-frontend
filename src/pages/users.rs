use yew::prelude::*;

use crate::hooks::user_context::use_user_context;
use super::admin::users::AdminUsersView;


#[function_component]
pub fn UsersView() -> Html {
    // Context
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }

    html! {
        <AdminUsersView/>
    }
}
