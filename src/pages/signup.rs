use yew::prelude::*;

use crate::features::authentication::components::SignupForm;
use crate::hooks::user_context::use_user_context;


#[function_component]
pub fn SignupView() -> Html {
    // Context
    let user_ctx = use_user_context();

    if user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }

    html! {
    <section class="hero-background is-fullheight">
        <div class="hero-body">
            <div class="container"> 
                <div class="columns is-centered ">
                <div class="column is-5-tablet is-4-desktop is-3-widescreen">
                    <SignupForm/>
                </div>
                </div>
            </div>
        </div>
    </section>
    }
}


use std::borrow::Cow;

fn validate_name<'a, T>(value: T) -> bool
where 
    T: Into<Cow<'a, str>>
{
    let val = value.into();
    if val.is_empty() || val.chars().all(char::is_whitespace) {
        return false;
    }

    for c in val.chars() {
        if c.is_digit(10)  {
            return false;
        }
    }

    return true;
}
