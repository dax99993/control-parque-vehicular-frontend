use yew::prelude::*;

use gloo::utils::{document, document_element};
use crate::utils::{remove_class, add_class};

pub fn open_modal() -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        if let Some(element) = document().get_element_by_id("sample-modal") {
            add_class(&element, "is-active");
        }
        let element = document_element();
        add_class(&element, "is-clipped");
    })
}

pub fn close_modal() -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        if let Some(element) = document().get_element_by_id("sample-modal") {
            remove_class(&element, "is-active");
        }
        let element = document_element();
        remove_class(&element, "is-clipped");
    })
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct ModalProps {
    #[prop_or(Some(String::from("Confirmar accion")))]
    pub title: Option<String>,
    pub body: Html,
    pub footer: Option<Html>,
}


#[function_component]
pub fn Modal(props: &ModalProps) -> Html {
    let props = props.clone();

    html! {
        <div id="sample-modal" class="modal">
            <div class="modal-background jb-modal-close"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                if props.title.is_some() {
                    <p class="modal-card-title">{ props.title }</p>
                }
                <button class="delete jb-modal-close" aria-label="close" id="modal-close-button" onclick={close_modal()}></button>
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
            <button class="modal-close is-large jb-modal-close" aria-label="close" id="modal-close-outside-button" onclick={close_modal()}></button>
      </div>
    }
}
/*
<button class="button jb-modal-close" id="modal-close-cancel-button" onclick={ close_modal() }>{ props.left_button_label }</button>
<button class="button is-danger jb-modal-close" onclick={ props.right_button_onclick }>{ props.right_button_label }</button>
*/
