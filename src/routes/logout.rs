//use std::ops::Deref;

use yew::prelude::*;
//use yew_hooks::prelude::*;

use crate::hooks::user_context::use_user_context;
//use crate::services::auth::request_logout;

#[function_component]
pub fn Logout() -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    } else {
        user_ctx.logout();
    }
    
    /*
    let update = use_update();

    let logout_request = {
        use_async(async move {
            request_logout().await
        })
    };
    

    
    {
        let logout_request = logout_request.clone();
        use_effect_once(move || {
            logout_request.run(); 
            move || {
            if let Some(response) = &logout_request.data {
                log::debug!("Logout response {}", &response);
                // execute logout routine (delete local token, routing to home)
                update();
                user_ctx.logout();
            }
            }
        });
    }
    */




    html! {
        <p>{"Logging out"}</p>
    }
}
