use yew::prelude::*;
use yew_hooks::use_async;

use crate::hooks::user_context::use_user_context;
use crate::routes::AppRoute;
use crate::services::vehicule::request_admin_delete_vehicule;
use crate::shadow_clone;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct VehiculeDeleteViewProps {
    pub id: String,
}

#[function_component]
pub fn VehiculeDeleteView(props: &VehiculeDeleteViewProps) -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() && !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }

    // Api delete vehicule request
    let request_delete_vehicule = {
        shadow_clone!(props);
        use_async(async {
           request_admin_delete_vehicule(props.id).await
        })
    };

    // Api make request on rendering
    {
        shadow_clone!(request_delete_vehicule);
        use_effect_with_deps(move |_| {
            request_delete_vehicule.run();
        }, 
        ());
    }

    // Perform redirect to home if successful delete
    // TODO: show message error (toast maybe) if database connection failed
    {
        shadow_clone!(request_delete_vehicule);
        use_effect_with_deps(
            move |request_delete_vehicule| {
                if let Some(response) = &request_delete_vehicule.data {
                    log::debug!("Delete response {:?}", response);
                    user_ctx.redirect_to(AppRoute::Vehicules);
                }
            }, 
            request_delete_vehicule.clone());
    }


    html!{}
}
