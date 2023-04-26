use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::use_navigator;

use common::models::vehicule::Vehicule;

use crate::shadow_clone;
use crate::hooks::user_context::use_user_context;


use crate::components::main_section::MainSection;
use crate::components::card::{Card, CardContent};
use crate::components::vehicule::table::{VehiculeTable, VehiculeTableRow};
use crate::components::modal::Modal;
use crate::components::pagination::Pagination;

use super::reducer::{VehiculeReducer, VehiculeAction};

use crate::services::vehicule::request_admin_get_vehicules;


#[function_component]
pub fn AdminVehiculePage() -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }

    // hooks
    let reducer = use_reducer(VehiculeReducer::default);
    let current_page = use_state(|| reducer.current_page);
    let navigator = use_navigator();

    // Add navigator
    {
        shadow_clone![reducer, navigator];
        use_effect_with_deps(move |nav| {
            reducer.dispatch(VehiculeAction::AddNavigator(nav.clone()));
        },
        navigator);
    }


    /*
    {
        let reducer = reducer.clone();
        use_effect_with_deps(move |_| {
            reducer.dispatch(VehiculeAction::GetVehicules); 
        },
        ());
    }
    */
    
    // Api fetch request
    let request_vehicule_admin = {
        use_async(async {
            request_admin_get_vehicules().await
        })
    };

    // Fetch api when rendered
    {
        shadow_clone!(request_vehicule_admin);
        use_effect_with_deps(move |_| {
            request_vehicule_admin.run()
        },
        ());
    }

    // Update vehicule vector when fetching from api
    {
        shadow_clone![reducer, request_vehicule_admin];
        use_effect_with_deps(
            move |request_vehicule| {
                if let Some(api_response) = &request_vehicule.data {
                    log::debug!("vehicules successful api response\n {:?}", &api_response);
                    if let Some(vec_vehicules) = api_response.data.clone() {
                        reducer.dispatch(VehiculeAction::GetVehicules(vec_vehicules));
                    }
                }
                if let Some(api_response) = &request_vehicule.error {
                    log::warn!("vehicules failed api response\n {:?}", &api_response);
                }
            },
            request_vehicule_admin.clone() 
        );
    }
    

    // Re-fetch api when clicking on button
    let onclick_add_vehicule = {
        shadow_clone!(reducer);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); 
            reducer.dispatch(VehiculeAction::AddVehicule);
        })
    };

    // Effect for keeping in sync Pagination state with reducer
    {
        shadow_clone![reducer, current_page];
        shadow_clone![request_vehicule_admin];
        use_effect_with_deps(move |(current_page, _)| {
        //use_effect_with_deps(move |current_page| {
            log::debug!("effect pagination page = {:?}", current_page);
            reducer.dispatch(VehiculeAction::GoToPage(**current_page));
        },
        (current_page.clone(), request_vehicule_admin.clone())
        //current_page.clone()
        );
    }


    html! {
        <MainSection route="Admin" subroute="Vehiculos" title="Vehiculos">
            <Card classes={classes!["has-table"]}
                header_icon_left={ "fa-solid fa-car" } header_title={ "Vehiculos" } 
                header_icon_right={ "fa-solid fa-plus" } header_icon_right_label={ "Agregar vehiculo" }
                header_icon_right_onclick={ onclick_add_vehicule } 
            >
                <CardContent>
                    <VehiculeTable>
                        {
                            vehicule_to_vehicule_table_row(reducer.current_page_vehicules.clone(), reducer.dispatcher())
                        }
                    </VehiculeTable>
                </CardContent>

            </Card>

            <Pagination 
                total_pages = { reducer.total_pages }
                current_page_state = { current_page.clone() }
            />

            <Modal 
                id={"vehicule-modal"}
                title={reducer.modal_title.clone()}
                body={if reducer.modal_body.is_some() { reducer.modal_body.as_ref().unwrap().clone() } else {html!{}}}
                footer={reducer.modal_footer.clone()}
                onclose={
                    //if reducer.modal_onclick.is_some() 
                    //    { reducer.modal_onclick.as_ref().unwrap().clone() }
                    //else {
                        shadow_clone![reducer];
                        Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            reducer.dispatch(VehiculeAction::ResetModal);
                        })
                    //}
                }
            >
           </Modal>

        </MainSection>
    }
}


fn vehicule_to_vehicule_table_row(vehicules: Vec<Vehicule>, dispatcher: UseReducerDispatcher<VehiculeReducer>) -> Vec<Html> {
    vehicules.into_iter().map(|v| {
        html!{
            <VehiculeTableRow
                vehicule={v}
                dispatcher={dispatcher.clone()}
            >
            </VehiculeTableRow>
        }
    })
    .collect()
}
