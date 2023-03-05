use yew::prelude::*;

use crate::hooks::user_context::use_user_context;


#[function_component]
pub fn Home() -> Html {

    let user_ctx = use_user_context();


    html! {
        if user_ctx.is_authenticated() {
            <p>{"Home user"}</p>
        } else {
            <p>{"Home Anonymous user"}</p>
        }
    }
}
