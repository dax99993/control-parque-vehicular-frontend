use yew::Callback;
use web_sys::MouseEvent;
use gloo::utils::{document, document_element};

use super::{add_class, remove_class};

pub fn open_modal(modal_id: String) {
    if let Some(element) = document().get_element_by_id(&modal_id) {
        add_class(&element, "is-active");

        let element = document_element();
        add_class(&element, "is-clipped");
    }
}

pub fn close_modal(modal_id: String) {
    if let Some(element) = document().get_element_by_id(&modal_id) {
        remove_class(&element, "is-active");

        let element = document_element();
        remove_class(&element, "is-clipped");
    }
}

pub fn open_modal_cb(modal_id: String) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        open_modal(modal_id.clone());
    })
}

pub fn close_modal_cb(modal_id: String) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        close_modal(modal_id.clone());
    })
}
