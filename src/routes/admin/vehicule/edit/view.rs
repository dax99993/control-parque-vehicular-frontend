use yew::prelude::*;
use yew_hooks::prelude::*;

use uuid::Uuid;
use common::models::vehicule::Vehicule;

use crate::hooks::user_context::use_user_context;
use crate::services::vehicule::request_admin_get_vehicule_with_id;

use crate::shadow_clone;

use super::form::EditVehiculeForm;
use super::show::EditVehiculeShow;

use crate::components::main_section::MainSection;



#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditVehiculeProps {
    pub id: Uuid,
}

#[function_component]
pub fn EditVehiculeView(props: &EditVehiculeProps) -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }
    // States
    let id = use_state(|| props.id.to_string());  
    let vehicule = use_state(|| Vehicule::default());  
    //let updated_vehicule = use_state(|| UpdateVehicule::default());  
    //let updated_vehicule_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));


    // ------- request vehicule information ------
    let request_vehicule_with_id = {
        shadow_clone!(id);
        use_async(async move {
            request_admin_get_vehicule_with_id((*id).clone()).await
        })
    };

    // Request vehicule information on rendering
    {
        shadow_clone!(request_vehicule_with_id);
        use_effect_with_deps(move |_| {
            request_vehicule_with_id.run();
        }, ())
    }

    // Request vehicule from id
    {
        shadow_clone![request_vehicule_with_id, vehicule];
        use_effect_with_deps(move |request_vehicule| {
            if let Some(response) = &request_vehicule.data {
                log::debug!("Successful get vehicule {:?}", response);
                if let Some(veh) = &response.data {
                    vehicule.set(veh.clone()); 
                }
                if let Some(response) = &request_vehicule.error {
                    log::error!("get vehicule request failed {:?}", response);
                }
            }
        },
        request_vehicule_with_id.clone()
        );
    }


    // HTML
    {
        shadow_clone![vehicule];
        html!{
        <MainSection route="Admin" subroute="Vehiculos" title="Editar Vehiculo">
            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <EditVehiculeForm vehicule={vehicule.clone()}/>
                </div>
                <div class="tile is-parent">
                    <EditVehiculeShow vehicule={vehicule.clone()}/>
                </div>
            </div>
        </MainSection>
        }
    }
}






