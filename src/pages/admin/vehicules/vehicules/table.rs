use yew::prelude::*;

use yew::platform::spawn_local;

use common::models::vehicule::Vehiculo;

use crate::shadow_clone;
use crate::services::admin::request_vehicule_picture;
use super::reducer::{VehiculeTableAction, VehiculeTableReducer};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::ops::Deref;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TableRowProps {
   pub vehiculo: Vehiculo, 
   pub dispatcher: UseReducerDispatcher<VehiculeTableReducer>,
}

#[function_component]
pub fn VehiculeTableRow(props: &TableRowProps) -> Html {
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
                let response = request_vehicule_picture(imagen_filename).await;
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
            dispatcher.dispatch(VehiculeTableAction::ShowVehiculePicture(id));
        })
    };
    
    let click_delete = {
        shadow_clone![vehiculo, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = vehiculo.vehiculo_id.clone();
            dispatcher.dispatch(VehiculeTableAction::DeleteVehicule(id));
        })
    };
    

    let click_edit = {
        shadow_clone![vehiculo, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = vehiculo.vehiculo_id.clone();
            dispatcher.dispatch(VehiculeTableAction::UpdateVehicule(id));
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
        <td class="has-text-centered" data-label="Activo">{ vehiculo.activo_a_palabra() }</td>
        <td data-label="Ultima modificacion">{&vehiculo.modificado_en}</td>
        <td data-label="Fecha de creacion">{&vehiculo.creado_en}</td>


        <td class="is-actions-cell">
            <div class="buttons is-right">
                <button class="button is-small is-primary" type="button" onclick={click_show}>
                    <span class="icon"><i class="fa-solid fa-eye"></i></span>
                    <span>{"Imagen"}</span>
                </button>

                <button class="button is-info is-small" type="button" onclick={click_edit}>
                    <span class="icon"><i class="fa-solid fa-pen"></i></span>
                    <span>{"Editar"}</span>
                </button>

                <button class="button is-danger is-small" type="button" onclick={click_delete}>
                    <span class="icon"><i class="fa-solid fa-trash-can"></i></span>
                    <span>{"Borrar"}</span>
                </button>
            </div>
        </td>

        </tr>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children, 
}

#[function_component]
pub fn VehiculeTable(props: &TableProps) -> Html {
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
                        <th>{"Activo"}</th>
                        <th>{"Ultima modificacion"}</th>
                        <th>{"Fecha de creacion"}</th>
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


