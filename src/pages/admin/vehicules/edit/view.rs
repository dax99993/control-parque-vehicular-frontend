use yew::prelude::*;
use yew_hooks::prelude::*;

use common::models::vehicule::Vehiculo;
use uuid::Uuid;

use crate::shadow_clone;
use crate::hooks::user_context::use_user_context;
use crate::layout::main_section::MainSection;
use super::{EditVehiculeShow, EditVehiculeForm};

use crate::services::admin::request_admin_get_vehicule_with_id;



#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditVehiculeProps {
    pub id: Uuid,
}


#[function_component]
pub fn EditVehiculeView(props: &EditVehiculeProps) -> Html {
    // Context
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }

    // States
    //let id = use_state(|| props.id.to_string());  
    let vehiculo = use_state(|| Vehiculo::default());  


    // ------- request vehicule information ------
    let request_vehicule_with_id = {
        //shadow_clone!(id);
        let id = props.id.clone();
        use_async(async move {
            //request_admin_get_vehicule_with_id((*id).clone()).await
            request_admin_get_vehicule_with_id(id.to_string()).await
        })
    };

    // Request vehicule information on rendering
    {
        shadow_clone![request_vehicule_with_id];
        use_effect_with_deps(move |_| {
            request_vehicule_with_id.run();
        }, ())
    }


    // Request vehicule from id
    {
        shadow_clone![vehiculo];
        use_effect_with_deps(move |request_vehicule| {
            if let Some(response) = &request_vehicule.data {
                log::debug!("Peticion obtener vehiculo exitosa {:?}", response);
                if let Some(veh) = &response.data {
                    vehiculo.set(veh.clone()); 
                }
                if let Some(response) = &request_vehicule.error {
                    log::error!("Peticion obtener vehiculo fallo {:?}", response);
                }
            }
        },
        request_vehicule_with_id.clone()
        );
    }


    // HTML
    {
        //shadow_clone![vehiculo];
        html!{
        <MainSection route="Admin" subroute="Vehiculos" title="Editar Vehiculo">
            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <EditVehiculeForm estado_vehiculo={vehiculo.clone()}/>
                </div>
                <div class="tile is-parent">
                    <EditVehiculeShow estado_vehiculo={vehiculo.clone()}/>
                </div>
            </div>
        </MainSection>
        }
    }
}
