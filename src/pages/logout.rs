use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::shadow_clone;
use crate::hooks::user_context::use_user_context;
//use crate::services::auth::request_logout;
use crate::features::authentication::services::auth::request_logout;

#[function_component]
pub fn LogoutView() -> Html {
    //Context
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }

    // Api logout request 
    let request_logout = {
        use_async(async move {
            request_logout().await
        })
    };
    
    // Api request on rendering
    {
        shadow_clone!(request_logout);
        use_effect_with_deps(move |_| {
            request_logout.run();
        }, 
        ());
    }

    // Perform logout routine
    {
        use_effect_with_deps(
            move |request_logout| {
                if let Some(response) = &request_logout.data {
                    log::debug!("Logout response {}", &response);
                    user_ctx.logout();
                }
                if let Some(response) = &request_logout.error {
                    log::error!("Logout request failed {:?}", &response);
                    user_ctx.redirect_home();
                }
            }, 
            request_logout.clone());
    }

    html!{}
}
