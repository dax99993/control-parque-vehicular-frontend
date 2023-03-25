use yew::prelude::*;

use crate::shadow_clone;
use crate::{hooks::user_context::use_user_context, routes::AppRoute};
use crate::types::vehicule::Vehicule;
use crate::components::modal::{open_modal};

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct VehiculeItemProps {
    pub vehicule: Vehicule,
}

#[function_component]
pub fn VehiculeItem(props: &VehiculeItemProps) -> Html {
    let user_ctx = use_user_context();
 
    //let click_trash = open_modal();

    let click_trash = {
        shadow_clone![user_ctx, props];
        Callback::from(move |_: MouseEvent| {
            user_ctx.redirect_to(AppRoute::VehiculesDelete { 
                id: props.vehicule.clone().vehicule_id.to_string() 
            });
        })
    };

    let click_eye = {
        shadow_clone![user_ctx, props];
        Callback::from(move |_: MouseEvent| {
            user_ctx.redirect_to(AppRoute::VehiculesEdit { 
                id: props.vehicule.clone().vehicule_id.to_string() 
            });
        })
    };

    shadow_clone!(props);
    html! {
        <>
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

        <td class="is-actions-cell">
            <div class="buttons is-right">
                <button class="button is-small is-primary" type="button" onclick={click_eye}>
                    <span class="icon"><i class="fa-solid fa-eye"></i></span>
                </button>
                <button class="button is-small is-danger jb-modal" data-target="sample-modal" type="button" onclick={click_trash}>
                    <span class="icon"><i class="fa-solid fa-trash-can"></i></span>
                </button>
            </div>
        </td>
        </tr>
        </>
    }

}

