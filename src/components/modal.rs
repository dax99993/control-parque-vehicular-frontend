use yew::prelude::*;

use gloo::utils::{document, document_element};
use crate::utils::modal::{open_modal, close_modal};


#[derive(Debug, PartialEq, Clone, Properties)]
pub struct ModalProps {
    #[prop_or(String::from("sample-modal"))]
    pub id: String,
    #[prop_or(Some(String::from("Confirmar accion")))]
    pub title: Option<String>,
    pub body: Html,
    pub footer: Option<Html>,
}


#[function_component]
pub fn Modal(props: &ModalProps) -> Html {
    let props = props.clone();



    html! {
        <div id={props.id.clone()} class="modal">
            <div class="modal-background jb-modal-close"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                if props.title.is_some() {
                    <p class="modal-card-title">{ props.title }</p>
                }
                <button class="delete jb-modal-close" aria-label="close" id="modal-close-button" onclick={close_modal(props.id.clone())}></button>
                </header>
                <section class="modal-card-body">
                {
                  props.body
                }
                </section>
                if props.footer.is_some() {
                <footer class="modal-card-foot">
                    { props.footer }
                </footer>
                }
            </div>
            <button class="modal-close is-large jb-modal-close" aria-label="close" id="modal-close-outside-button" onclick={close_modal(props.id)}></button>
      </div>
    }
}
/*
<button class="button jb-modal-close" id="modal-close-cancel-button" onclick={ close_modal() }>{ props.left_button_label }</button>
<button class="button is-danger jb-modal-close" onclick={ props.right_button_onclick }>{ props.right_button_label }</button>
*/
