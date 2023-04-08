use yew::prelude::*;

use crate::types::vehicule::Vehicule;
use crate::hooks::user_context::use_user_context;
use crate::routes::AppRoute;
use crate::utils::modal::open_modal;
use crate::context::vehicule::{VehiculeItemContext, VehiculeItemAction};

use crate::shadow_clone;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
   pub vehicule: Vehicule, 
}

#[function_component]
pub fn VehiculeTableRow(props: &Props) -> Html {
    shadow_clone!(props);

    //TODO request vehicule picture
    // by constructing a global URL_BASE
    
    let user_ctx = use_user_context();
    let vehicule_ctx = use_context::<VehiculeItemContext>().unwrap();

    let click_show = {
        shadow_clone![vehicule_ctx, props];
        //vehicule_ctx.vehicule_id = props.vehicule.vehicule_id.to_string();
        open_modal("vehicule-details-modal".to_string())
    };
    
    let click_delete = {
        shadow_clone![vehicule_ctx, props];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if (*vehicule_ctx).vehicule_id.is_none() {
                let id = props.vehicule.vehicule_id.to_string();
                vehicule_ctx.dispatch(VehiculeItemAction::Delete(id));
            }
            //log::debug!("Vehicule delete context {:?}", vehicule_ctx);
            open_modal("vehicule-delete-modal".to_string()).emit(e);
        })
    };
    

    let click_edit = {
        shadow_clone![user_ctx, props];
        Callback::from(move |_: MouseEvent| {
            user_ctx.redirect_to(AppRoute::VehiculesEdit { 
                id: props.vehicule.clone().vehicule_id.to_string() 
            });
        })
    };


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

        <td data-label="Marca">{&props.vehicule.branch}</td>
        <td data-label="Modelo">{&props.vehicule.clone().model}</td>
        <td data-label="Año">{&props.vehicule.clone().year}</td>
        <td data-label="Estado">{&props.vehicule.clone().status}</td>
        <td data-label="Nombre economico">{&props.vehicule.clone().short_name}</td>
        <td data-label="Numero de tarjeta">{&props.vehicule.clone().number_card}</td>

        <td class="is-actions-cell">
            <div class="buttons is-right">
                <button class="button is-small jb-modal" type="button" onclick={click_show}>
                    <span class="icon"><i class="fa-solid fa-eye"></i></span>
                    <span>{"Ver"}</span>
                </button>

                <button class="button is-info is-small is-primary" type="button" onclick={click_edit}>
                    <span class="icon"><i class="fa-solid fa-pen"></i></span>
                    <span>{"Editar"}</span>
                </button>

                <button class="button is-danger is-small jb-modal" type="button" onclick={click_delete}>
                    <span class="icon"><i class="fa-solid fa-trash-can"></i></span>
                    <span>{"Borrar"}</span>
                </button>

            </div>
        </td>
        </tr>
    }
}
