use yew::prelude::*;


#[derive(Debug, PartialEq, Clone, Properties)]
pub struct ModalProps {
    pub body: Html,
    pub ActionButtonLabel: String,
}


#[function_component]
pub fn Modal(props: &ModalProps) -> Html {

    /*
    let elements = document_element().get_elements_by_class_name("jb-modal-close");

    for i = 0..elements.length() {
        if let Some(element) = elements.get_with_index(i) {
            
        }

    }
    */

    let onclick = close_modal();

    let props = props.clone();
    html! {
        <div id="sample-modal" class="modal">
            <div class="modal-background jb-modal-close"></div>
            <div class="modal-card">
              <header class="modal-card-head">
                <p class="modal-card-title">{ "Confirmar accion"}</p>
                <button class="delete jb-modal-close" aria-label="close" id="modal-close-button" onclick={onclick.clone()}></button>
              </header>
              <section class="modal-card-body">
              {
                  props.body
              }
              </section>
              <footer class="modal-card-foot">
                <button class="button jb-modal-close" id="modal-close-cancel-button" onclick={onclick.clone()}>{ "Cancelar" }</button>
                <button class="button is-danger jb-modal-close">{ props.ActionButtonLabel }</button>
              </footer>
            </div>
            <button class="modal-close is-large jb-modal-close" aria-label="close" id="modal-close-outside-button" onclick={onclick.clone()}></button>
      </div>
    }
}

use gloo::utils::{document, document_element};
use crate::utils::toggle_class;

fn close_modal() -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        if let Some(element) = document().get_element_by_id("sample-modal") {
            toggle_class(element, "is-active");
        }
        let element = document_element();
        toggle_class(element, "is-clipped");
    })
}
