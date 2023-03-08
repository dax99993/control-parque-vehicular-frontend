use yew::prelude::*;
use yew_hooks::use_async;

use crate::hooks::user_context::use_user_context;
use crate::services::vehicule::{request_normal_get_vehicules, request_admin_get_vehicules};
use crate::types::vehicule::{Vehicule, FilteredVehicule};
use crate::components::vehicule::vehicule_item::VehiculeItem;




#[function_component]
pub fn GetVehicules() -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }

    html! {
        if user_ctx.is_admin() {
            <GetVehiculesAdminView>
            </GetVehiculesAdminView>
        } else {
            <GetVehiculesNormalView>
            </GetVehiculesNormalView>
        }
    }
}


#[function_component]
fn GetVehiculesAdminView() -> Html {
    let vehicules = use_state(|| Vec::<Vehicule>::new());

    let request_vehicule_admin = {
        use_async(async {
            request_admin_get_vehicules().await
        })
    };

    {
        let vehicules = vehicules.clone();
        use_effect_with_deps(
            move |request_vehicule| {
                if let Some(response) = &request_vehicule.data {
                    log::debug!("vehicules response {}", &response);
                    if let Some(vec_vehicules) = response.data.clone() {
                        log::debug!("vehicules vec {:?}", &vec_vehicules);
                        // Store token to be able to make requests
                        vehicules.set(vec_vehicules);
                    }
                }
            },
            request_vehicule_admin.clone() 
        );
    }

    let onclick = {
        let request_vehicule_admin = request_vehicule_admin.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); 
            request_vehicule_admin.run();
        })
    };
    

    {
        let vehicules = vehicules.clone();
        html!{
            <>
            {"Admin Vehicules"}
            <br/>
            <ybc::Button
                classes={classes!("is-primary")}
                {onclick}
            >
                {"Get Vehicules"}
            </ybc::Button>
                
            <ybc::Container>
            { 
                    vehicule_to_vehicule_list((*vehicules).clone())
            }
            </ybc::Container>

                
            </>
        }
    }
}

#[function_component]
fn GetVehiculesNormalView() -> Html {
    let vehicules = use_state(|| Vec::<FilteredVehicule>::new());

    let request_vehicule = {
        use_async(async {
            request_normal_get_vehicules().await
        })
    };

    {
        let vehicules = vehicules.clone();
        use_effect_with_deps(
            move |request_vehicule| {
                if let Some(response) = &request_vehicule.data {
                    log::debug!("vehicules response {}", &response);
                    if let Some(vec_vehicules) = response.data.clone() {
                        log::debug!("vehicules vec {:?}", &vec_vehicules);
                        // Store token to be able to make requests
                        vehicules.set(vec_vehicules);
                    }
                }
            },
            request_vehicule.clone() 
        );
    }

    let onclick = {
        let request_vehicule = request_vehicule.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); 
            request_vehicule.run();
        })
    };
    

    html!{
        <>
        {"Normal Vehicules"}
        <ybc::Button
            {onclick}
        >
        </ybc::Button>
        </>
    }
}

fn vehicule_to_vehicule_list(vehicules: Vec<Vehicule>) -> Vec<Html> {
    vehicules.into_iter().map(|v| {
        html!{
            <VehiculeItem
                veh={Some(v)}>
            </VehiculeItem>
        }
    })
    .collect()
}
