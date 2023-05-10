use yew::prelude::*;

use common::models::vehicule::Vehiculo;

use crate::shadow_clone;
use super::super::reducers::{VehiculeTableAction, VehiculeTableReducer};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
   pub vehiculo: Vehiculo, 
   pub dispatcher: UseReducerDispatcher<VehiculeTableReducer>,
}

#[function_component]
pub fn VehiculeTableRow(props: &Props) -> Html {
    let Props { vehiculo, dispatcher } = props;

    //TODO request vehiculo picture
    // by constructing a global URL_BASE
    let imagen_url = vehiculo.imagen_url("http://127.0.0.1:8000/");

    
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


    html!{
        <tr>
        <td class="is-image-cell">
            <figure class="is-flex is-align-items-center is-justify-content-center image is-128x128">
                <img src={imagen_url.clone()} onclick={click_show.clone()} />
            </figure>
        </td>

        <td data-label="Marca">{&vehiculo.marca}</td>
        <td data-label="Modelo">{&vehiculo.modelo}</td>
        <td data-label="Año">{&vehiculo.año}</td>
        <td data-label="Nombre economico">{&vehiculo.nombre_economico}</td>
        <td data-label="Numero de tarjeta">{&vehiculo.numero_tarjeta}</td>
        <td data-label="Numero de placa">{&vehiculo.numero_placa}</td>

        <td data-label="Estado">{ &vehiculo.estado }</td>
        <td class="has-text-centered" data-label="Activo">{ vehiculo.activo.to_string() }</td>
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
