use yew::prelude::*;

use yew::platform::spawn_local;

use common::models::vehicule::Vehiculo;

use crate::shadow_clone;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::ops::Deref;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children, 
}

#[function_component]
pub fn Table(props: &TableProps) -> Html {
    shadow_clone!(props);

    html!{
    <div class="b-table has-pagination">
        <div class="table-wrapper has-mobile-cards">
            <table class="table is-narrow is-striped is-hoverable is-fullwidth">
                <thead>
                    <tr>
                        <th></th>
                        <th>{"Marca"}</th>
                        <th>{"Modelo"}</th>
                        <th>{"Año"}</th>
                        <th>{"Nombre economico"}</th>
                        <th>{"Numero de tarjeta"}</th>
                        <th>{"Numero de placa"}</th>
                        <th>{"Estado"}</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                {
                    props.children
                }
                </tbody>
            </table>
        </div>
    </div>
    }
}


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TableRowProps {
   pub vehiculo: Vehiculo, 
   pub dispatcher: UseReducerDispatcher<TableReducer>,
}

#[function_component]
pub fn TableRow(props: &TableRowProps) -> Html {
    // Props
    let TableRowProps{ vehiculo, dispatcher } = props;

    //States
    let imagen = use_state(|| vec![]);

    // Hooks
    {
        let imagen = imagen.clone();
        use_effect_with_deps(move |vehiculo| {
            let imagen_filename = vehiculo.imagen.clone();
            spawn_local(async move {
                let response = crate::services::normal::request_imagen_vehiculo(imagen_filename).await;
                log::debug!("ejecutando peticion de imagen");
                match response {
                    Ok(bytes) => {
                        imagen.set(bytes.clone());
                    }
                    Err(_) => {
                        log::error!("peticion de imagen fallo");
                    }
                }
            });
        }, vehiculo.clone())
    }

    
    //Callbacks
    let click_show = {
        shadow_clone![vehiculo, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = vehiculo.vehiculo_id.clone();
            dispatcher.dispatch(TableAction::ShowVehiculePicture(id));
        })
    };
    
    let click_request = {
        shadow_clone![vehiculo, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = vehiculo.vehiculo_id.clone();
            dispatcher.dispatch(TableAction::RequestVehicule(id));
        })
    };


    //HTML
    html!{
        <tr>
        <td class="is-image-cell">
            <figure class="is-flex is-align-items-center is-justify-content-center image is-128x128">
                if !imagen.deref().is_empty() {
                    <img src={ format!("data:image/jpeg;base64,{}", STANDARD.encode(&imagen.deref())) } onclick={click_show.clone()}/>
                }
            </figure>
        </td>

        <td data-label="Marca">{&vehiculo.marca}</td>
        <td data-label="Modelo">{&vehiculo.modelo}</td>
        <td data-label="Año">{&vehiculo.año}</td>
        <td data-label="Nombre economico">{&vehiculo.nombre_economico}</td>
        <td data-label="Numero de tarjeta">{&vehiculo.numero_tarjeta}</td>
        <td data-label="Numero de placa">{&vehiculo.numero_placa}</td>

        <td data-label="Estado">{ &vehiculo.estado }</td>

        <td class="is-actions-cell">
            <div class="buttons is-right">
                <button class="button is-small is-primary" type="button" onclick={click_show}>
                    <span class="icon"><i class="fa-solid fa-eye"></i></span>
                    <span>{"Imagen"}</span>
                </button>

                <button class="button is-info is-small" type="button" onclick={click_request}>
                    <span class="icon"><i class="fa-solid fa-test"></i></span>
                    <span>{"Solicitar"}</span>
                </button>

            </div>
        </td>

        </tr>
    }
}

use uuid::Uuid;
use yew_router::prelude::Navigator;

use crate::routes::AppRoute;
use crate::utils::modal::{open_modal, close_modal};


pub enum TableAction {
    AddNavigator(Navigator),
    RequestVehicule(Uuid),
    ShowVehiculePicture(Uuid),
    ResetSelectedShow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableReducer {
    pub selected_vehicule_to_show_id: Option<Uuid>,
    pub navigator: Option<Navigator>,
}

impl Default for TableReducer {
    fn default() -> Self {
        Self {
            selected_vehicule_to_show_id: None,
            navigator: None, 
        }
    }
}

impl Reducible for TableReducer {
    type Action = TableAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut selected_vehicule_to_show_id = self.selected_vehicule_to_show_id.clone();
        let mut navigator = self.navigator.clone();

        
        match action {
            TableAction::AddNavigator(nav) => {
                navigator = Some(nav);
            }
            TableAction::RequestVehicule(id) => {
                if let Some(nav) = navigator.clone() {
                    //nav.push(&AppRoute::VehiculeRequest { id });
                    log::debug!("redirect to vehicule request with id {id}");
                } else {
                    log::error!("navigator is None!");
                }
            }
            TableAction::ShowVehiculePicture(id) => {
                open_modal("vehicule-picture-modal".to_string());
                selected_vehicule_to_show_id = Some(id);
            }
            TableAction::ResetSelectedShow => {
                selected_vehicule_to_show_id = None;
            }
        }

        Self {
            selected_vehicule_to_show_id,
            navigator,
        }.into()    
    }
}


pub fn vehicule_to_table_row(
    vehiculos: Vec<Vehiculo>,
    dispatcher: UseReducerDispatcher<TableReducer>,
) -> Vec<Html> {
    vehiculos.into_iter().map(|v| {
        html!{
            <TableRow
                vehiculo={v}
                dispatcher={dispatcher.clone()}
            >
            </TableRow>
        }
    })
    .collect()
}
