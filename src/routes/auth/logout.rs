use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::hooks::user_context::use_user_context;
use crate::services::auth::request_logout;
use crate::shadow_clone;

#[function_component]
pub fn LogoutView() -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }
    

    // api logout request 
    let request_logout = {
        use_async(async move {
            request_logout().await
        })
    };
    
    // api request on rendering
    {
        shadow_clone!(request_logout);
        use_effect_with_deps(move |_| {
            request_logout.run();
        }, 
        ());
    }

    // perform logout routine
    {
        shadow_clone!(request_logout);
        use_effect_with_deps(
            move |request_logout| {
                if let Some(response) = &request_logout.data {
                    log::debug!("Logout response {}", &response);
                    user_ctx.logout();
                }
            }, 
            request_logout.clone());
    }


    html!{}
}
