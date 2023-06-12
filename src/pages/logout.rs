use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::shadow_clone;
use crate::components::toast::{use_toaster, ToastPosition, ToastType, Toast};
use crate::hooks::user_context::use_user_context;
//use crate::services::auth::request_logout;
use crate::services::auth::request_logout;

#[function_component]
pub fn LogoutView() -> Html {
    //Context
    let user_ctx = use_user_context();
    let toaster = use_toaster().expect("No hay toastViewer");

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
                    let toast = Toast { position: ToastPosition::TopCenter,
                    body: "Sesion cerrada".into(),
                    timeout: Some(chrono::Duration::milliseconds(3000)),
                    r#type: ToastType::Success,
                    };
                    toaster.toast(toast);
                }
                if let Some(response) = &request_logout.error {
                    log::error!("Logout request failed {:?}", &response);
                    let toast = Toast { position: ToastPosition::TopRight,
                    body: "Fallo al cerrar sesion".into(),
                    timeout: Some(chrono::Duration::milliseconds(3000)),
                    r#type: ToastType::Danger,
                    };
                    toaster.toast(toast);
                    user_ctx.redirect_home();
                }
            }, 
            request_logout.clone());
    }

    html!{}
}
