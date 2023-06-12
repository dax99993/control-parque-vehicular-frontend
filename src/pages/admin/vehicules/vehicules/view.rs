use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yew::platform::spawn_local;

use std::ops::Deref;

use common::models::vehicule::Vehiculo;

use super::table::{VehiculeTable, VehiculeTableRow};
use super::reducer::{VehiculeTableAction, VehiculeTableReducer};
use crate::services::admin::{request_admin_get_vehicules, request_admin_delete_vehicule, request_vehicule_picture};

use crate::shadow_clone;
use crate::components::card::{Card, CardContent};
use crate::components::filter_search::FilterSearch;
use crate::components::modal::Modal;
use crate::components::pagination::Pagination;
use crate::hooks::user_context::use_user_context;
use crate::layout::main_section::MainSection;

use crate::utils::modal::{open_modal, close_modal};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;


#[function_component]
pub fn AdminVehiculeView() -> Html {
    // Context 
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }

    //States
    let table_reducer = use_reducer(VehiculeTableReducer::default);
    let vehiculos = use_state(|| vec![]);
    let imagen_vehiculo = use_state(|| vec![]);
    let current_page = use_state(|| 1);
    let vehiculos_por_pagina = use_state(|| 4);
    let navigator = use_navigator();
    let reload_table = use_state(|| false);

    let search_state = use_state(|| None::<String>);
    let selected_filter = use_state(|| None::<String>);
    let filter_fields = vec!["Marca".to_string(), "Modelo".to_string(), "Año".to_string()];


    // Hooks
    // Add navigator to table reducer for redirecting
    {
        shadow_clone![table_reducer];
        use_effect_with_deps(move |navigator| {
            if let Some(nav) = navigator.clone() {
                table_reducer.dispatch(VehiculeTableAction::AddNavigator(nav));
            }
        },
        navigator.clone())
    }

    
    // Effect for keeping in sync vehiculos with pagination
    {
        shadow_clone![vehiculos];
        use_effect_with_deps(move |(current_page, vehiculos_por_pagina, selected_filter, search_state, _)| {
            let page = **current_page;
            let limit = **vehiculos_por_pagina;
            let filter = (**selected_filter).clone();
            let search = (**search_state).clone();
            spawn_local(async move {
                log::debug!("obtener vehiculos en pagina {}", page);
                let response = request_admin_get_vehicules(page, limit, filter, search).await;
                match response {
                    Ok(res) => {
                        if let Some(v) = res.data {
                            vehiculos.set(v);
                        }
                    }
                    Err(_) => {
                        log::error!("peticion de obtener vehiculos fallo");
                    }
                }
            });
        },
        (current_page.clone(), vehiculos_por_pagina.clone(), selected_filter.clone(), search_state.clone(), reload_table.clone())
        );
    }

    // Fetch imagen del vehiculo cuando se selecciona un vehiculo
    {
        let vehiculos = vehiculos.clone();
        let imagen_vehiculo = imagen_vehiculo.clone();
        use_effect_with_deps(move |table_reducer| {
            match table_reducer.selected_vehicule_to_show_id {
                Some(id) => {
                    if let Some(vehiculo) = vehiculos.deref().iter().filter(|v| v.vehiculo_id.eq(&id)).map(|v| v).next() {
                        log::debug!("Vehiculo seleccionado {:?}", &vehiculo);
                        let imagen_filename = vehiculo.imagen.clone();
                        let imagen_vehiculo = imagen_vehiculo.clone();
                        spawn_local(async move {
                            let response = request_vehicule_picture(imagen_filename).await;
                            log::debug!("ejecutando peticion de imagen");
                            match response {
                                Ok(bytes) => {
                                    imagen_vehiculo.set(bytes.clone());
                                }
                                Err(_) => {
                                    log::error!("peticion de imagen fallo");
                                    imagen_vehiculo.set(vec![]);
                                }
                            }
                        });
                    }
                }
                None => { imagen_vehiculo.set(vec![]); }
            }
        },
        table_reducer.clone())
    }


    // Callbacks
    let onclick_delete = {
        shadow_clone![reload_table];
        let selected_vehicule_to_delete_id = table_reducer.selected_vehicule_to_delete_id.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // Execute api
            shadow_clone![reload_table];
            if let Some(id) = selected_vehicule_to_delete_id {
                spawn_local(async move {
                    log::debug!("se borrara el vehiculo con id {}", id.to_string());
                    let response = request_admin_delete_vehicule(id.to_string()).await;
                    match response {
                        Ok(_) => {
                            close_modal("vehicule-delete-modal".to_string());
                            // reload current page
                            reload_table.set(!reload_table.deref());
                        }
                        Err(_) => {
                            log::error!("Peticion de borrar vehiculo fallo");
                        }
                    }
                });
            }
        })
    };


    // Variables
    let total_pages = {
        if vehiculos.deref().len() < *vehiculos_por_pagina.deref() {
            *current_page 
        } else {
            *current_page + 1
        }
    };


    //HTML
    html! {
        <MainSection route="Admin" subroute="Vehiculos" title="Vehiculos">

            <FilterSearch filter_fields={filter_fields.clone()} selected_filter_state={selected_filter.clone()} search_state={search_state.clone()} />

            <div class="mb-3"/>

            <Card classes={classes!["has-table"]}
                header_icon_left={ "fa-solid fa-car" } header_title={ "Vehiculos" } 
                header_icon_right={ "fa-solid fa-rotate-right" } header_icon_right_label={ "Recargar tabla" }
                header_icon_right_onclick={ 
                    shadow_clone![reload_table]; 
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        reload_table.set(!reload_table.deref());
                    }) 
                }
            >
                <CardContent>
                    <VehiculeTable>
                        {
                            vehicule_to_vehicule_table_row(vehiculos.deref().clone(), table_reducer.dispatcher())
                        }
                    </VehiculeTable>
                </CardContent>

            </Card>

            <Pagination 
                total_pages = { total_pages }
                current_page_state = { current_page.clone() }
            />


            <Modal 
                id={"vehicule-delete-modal"}
                title={"".to_string()}
                body={ html!{<p><b>{ "Realmente desea borrar el vehiculo" }</b></p>} }
                footer={
                        html!{
                            <>
                            <button class="button jb-modal-close" onclick={
                                Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    close_modal("vehicule-delete-modal".to_string());
                                })
                            }>
                            { "Cancelar" }
                            </button>
                            <button class="button is-danger jb-modal-close" onclick={onclick_delete}>
                            { "Borrar" }
                            </button>
                            </>
                        }
                }
                onclose={
                        shadow_clone![table_reducer];
                        Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            table_reducer.dispatch(VehiculeTableAction::ResetSelectedDelete);
                        })
                }
            >
           </Modal>


           <Modal 
                id={"vehicule-picture-modal"}
                title={"".to_string()}
                body={
                    html!{
                        <img src={ format!("data:image/jpeg;base64,{}", STANDARD.encode(&imagen_vehiculo.deref())) } />
                    }
                }
                onclose={
                    shadow_clone![table_reducer];
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        table_reducer.dispatch(VehiculeTableAction::ResetSelectedShow);
                    })
                }
            >
            </Modal>

        </MainSection>
    }
}


fn vehicule_to_vehicule_table_row(vehiculos: Vec<Vehiculo>, dispatcher: UseReducerDispatcher<VehiculeTableReducer>) -> Vec<Html> {
    vehiculos.into_iter().map(|v| {
        html!{
            <VehiculeTableRow
                vehiculo={v}
                dispatcher={dispatcher.clone()}
            >
            </VehiculeTableRow>
        }
    })
    .collect()
}

/*
*/
