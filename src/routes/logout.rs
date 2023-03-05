use yew::prelude::*;

use crate::hooks::user_context::use_user_context;

#[function_component]
pub fn Logout() -> Html {
    let user_ctx = use_user_context();
    user_ctx.logout();

    html! {
        <p>{"Logging out"}</p>
        
    }
}
