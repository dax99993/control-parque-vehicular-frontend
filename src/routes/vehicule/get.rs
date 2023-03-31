use yew::prelude::*;
use yew_hooks::use_async;

use crate::hooks::user_context::use_user_context;
use crate::services::vehicule::{request_normal_get_vehicules, request_admin_get_vehicules};
use crate::shadow_clone;
use crate::types::vehicule::{Vehicule, FilteredVehicule};

use crate::components::main_section::MainSection;
use crate::components::card::{Card, CardContent};
use crate::components::vehicule::table::{VehiculeTable, VehiculeTableRow};
use crate::components::modal::Modal;
use crate::components::pagination::Pagination;
//use crate::routes::AppRoute;




#[function_component]
pub fn GetVehicules() -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }

    html! {
        if user_ctx.is_admin() {
            <GetVehiculesAdminView />
        } else {
            <GetVehiculesNormalView />
        }
    }
}


#[function_component]
fn GetVehiculesAdminView() -> Html {

    let vehicules = use_state(|| Vec::<Vehicule>::new());

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

    // Re-fetch api when clicking on button
    let onclick = {
        shadow_clone!(request_vehicule_admin);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); 
            request_vehicule_admin.run();
        })
    };

    // Update vehicule vector when fetching from api
    {
        shadow_clone![vehicules, request_vehicule_admin];
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

    

    {
        shadow_clone!(vehicules);
        html!{
            <MainSection route="Admin" subroute="Vehiculos" title="Vehiculos">

                <Card header_icon_left={ "fa-solid fa-car" } header_title={ "Vehiculos" } classes={classes!["has-table"]}
                    header_icon_right={ "fa-solid fa-rotate-right" } header_icon_right_onclick={ onclick } 
                >
                    <CardContent>
                        <VehiculeTable>
                            {
                                vehicule_to_vehicule_table_row((*vehicules).clone())
                            }
                        </VehiculeTable>
                    </CardContent>


                </Card>

                <Pagination total_pages=4 />
                    
                <Modal 
                    body={html!{<p>{"un "}<b>{"mensaje"}</b></p>}}
                    right_button_label={"Borrar"}
                    right_button_onclick={ Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        log::debug!("Right modal click");
                        //user_ctx.redirect_to(AppRoute::VehiculesDelete);
                        })
                    }
                >
                </Modal>

            </MainSection>
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

    let _onclick = {
        let request_vehicule = request_vehicule.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); 
            request_vehicule.run();
        })
    };
    

    html!{
        <>
        </>
    }
}

fn vehicule_to_vehicule_table_row(vehicules: Vec<Vehicule>) -> Vec<Html> {
    vehicules.into_iter().map(|v| {
        html!{
            <VehiculeTableRow
                vehicule={v}>
            </VehiculeTableRow>
        }
    })
    .collect()
}
