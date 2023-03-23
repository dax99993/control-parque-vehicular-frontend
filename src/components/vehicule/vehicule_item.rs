use yew::prelude::*;

use crate::hooks::user_context::use_user_context;
use crate::types::vehicule::Vehicule;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct VehiculeItemProps {
    pub veh: Option<Vehicule>,
}

#[function_component]
pub fn VehiculeItem(props: &VehiculeItemProps) -> Html {
    let user_ctx = use_user_context();
 
    let click_trash = open_modal();

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

        if props.veh.is_some() {
        <td data-label="Marca">{&props.veh.clone().unwrap().branch}</td>
        <td data-label="Modelo">{&props.veh.clone().unwrap().model}</td>
        <td data-label="Año">{&props.veh.clone().unwrap().year}</td>
        <td data-label="Estado">{&props.veh.clone().unwrap().status}</td>
        }

        <td class="is-actions-cell">
            <div class="buttons is-right">
                <button class="button is-small is-primary" type="button">
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


/*
fn admin_view() {

}

fn normal_view() {

}
*/

use gloo::utils::{document, document_element};
use crate::utils::toggle_class;

//fn open_modal(menu_id: String) -> Callback<MouseEvent> {
fn open_modal() -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        if let Some(element) = document().get_element_by_id("sample-modal") {
            toggle_class(element, "is-active");
        }
        let element = document_element();
        toggle_class(element, "is-clipped");
    })
}
