use yew::prelude::*;

use crate::types::vehicule::Vehicule;
use crate::hooks::user_context::use_user_context;

use crate::routes::admin::vehicule::reducer::{VehiculeReducer, VehiculeAction};

use crate::shadow_clone;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
   pub vehicule: Vehicule, 
   pub dispatcher: UseReducerDispatcher<VehiculeReducer>,
}

#[function_component]
pub fn VehiculeTableRow(props: &Props) -> Html {
    shadow_clone!(props);

    //TODO request vehicule picture
    // by constructing a global URL_BASE
    
    let user_ctx = use_user_context();

    let click_show = {
        shadow_clone![props];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = props.vehicule.vehicule_id.clone();
            props.dispatcher.dispatch(VehiculeAction::ShowInfo(id));
        })
    };
    
    let click_delete = {
        shadow_clone![props];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = props.vehicule.vehicule_id.clone();
            props.dispatcher.dispatch(VehiculeAction::DeleteVehicule(id));
        })
    };
    

    let click_edit = {
        shadow_clone![props];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let id = props.vehicule.vehicule_id.clone();
            props.dispatcher.dispatch(VehiculeAction::UpdateInfo(id));
        })
    };


    {
    let vehicule = props.vehicule.clone();
    html!{
        <tr>
        <td class="is-checkbox-cell">
          <label class="b-checkbox checkbox">
            <input type="checkbox" value={"false"} />
            <span class="check"></span>
          </label>
        </td>
        <td class="is-image-cell">
            <div class="image">
                <img src="https://avatars.dicebear.com/v2/initials/rebecca-bauch.svg" class="is-rounded"/>
            </div>
        </td>

        <td data-label="Marca">{&vehicule.branch}</td>
        <td data-label="Modelo">{&vehicule.model}</td>
        <td data-label="Año">{&vehicule.year}</td>
        <td data-label="Nombre economico">{&vehicule.short_name}</td>
        <td data-label="Numero de tarjeta">{&vehicule.number_card}</td>
        <td data-label="Numero de placa">{&vehicule.number_plate}</td>

        if user_ctx.is_admin() {
        <td data-label="Estado">{
            if vehicule.is_available() {
                "disponible"
            } else if vehicule.is_occupied() {
                "ocupado"
            } else if vehicule.is_maintenance() {
                "mantenimiento"
            } else { "" }
        }</td>
        <td data-label="Activo">{
            if vehicule.is_active() {
                "si" 
            } else {
                "no"
            }
        }</td>
        <td data-label="Ultima modificacion">{&vehicule.updated_at}</td>
        <td data-label="Fecha de creacion">{&vehicule.created_at}</td>


        <td class="is-actions-cell">
            <div class="buttons is-right">
                <button class="button is-small is-primary" type="button" onclick={click_show}>
                    <span class="icon"><i class="fa-solid fa-eye"></i></span>
                    <span>{"Ver"}</span>
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
        }

        </tr>
    }
    }
}
