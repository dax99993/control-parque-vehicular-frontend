use yew::prelude::*;

use common::models::vehicule::Vehicule;

use crate::shadow_clone;
use super::super::reducers::{VehiculeAction, VehiculeReducer};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
   pub vehicule: Vehicule, 
   pub dispatcher: UseReducerDispatcher<VehiculeReducer>,
}

#[function_component]
pub fn VehiculeTableRow(props: &Props) -> Html {
    let Props { vehicule, dispatcher } = props;

    //TODO request vehicule picture
    // by constructing a global URL_BASE
    let picture_url = vehicule.get_picture_url("http://127.0.0.1:8000/");

    
    let click_show = {
        shadow_clone![vehicule, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = vehicule.vehicule_id.clone();
            dispatcher.dispatch(VehiculeAction::ShowPicture(id));
        })
    };
    
    let click_delete = {
        shadow_clone![vehicule, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = vehicule.vehicule_id.clone();
            dispatcher.dispatch(VehiculeAction::DeleteVehicule(id));
        })
    };
    

    let click_edit = {
        shadow_clone![vehicule, dispatcher];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = vehicule.vehicule_id.clone();
            dispatcher.dispatch(VehiculeAction::UpdateInfo(id));
        })
    };


    html!{
        <tr>
        <td class="is-image-cell">
            <figure class="image is-16by9">
                <img src={picture_url} />
            </figure>
        </td>

        <td data-label="Marca">{&vehicule.branch}</td>
        <td data-label="Modelo">{&vehicule.model}</td>
        <td data-label="Año">{&vehicule.year}</td>
        <td data-label="Nombre economico">{&vehicule.short_name}</td>
        <td data-label="Numero de tarjeta">{&vehicule.number_card}</td>
        <td data-label="Numero de placa">{&vehicule.number_plate}</td>

        <td data-label="Estado">{ vehicule.status_to_spanish() }</td>
        <td class="has-text-centered" data-label="Activo">{ vehicule.active_to_spanish() }</td>
        <td data-label="Ultima modificacion">{&vehicule.updated_at}</td>
        <td data-label="Fecha de creacion">{&vehicule.created_at}</td>


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
