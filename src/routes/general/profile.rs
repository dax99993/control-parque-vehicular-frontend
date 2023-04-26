use yew::prelude::*;

use crate::hooks::user_context::use_user_context;

use crate::routes::admin::profile::view::AdminProfilePage;

#[function_component]
pub fn ProfileView() -> Html {
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }

    html! {
        if user_ctx.is_admin() {
            <AdminProfilePage />
        } else {
            <p>{"normal profile page"}</p>
        }
    }
}
