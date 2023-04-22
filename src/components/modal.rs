use yew::prelude::*;

use crate::utils::modal::close_modal_cb;
use crate::shadow_clone;


#[derive(Debug, PartialEq, Clone, Properties)]
pub struct ModalProps {
    pub id: String,
    pub title: Option<String>,
    pub body: Html,
    pub footer: Option<Html>,
    pub onclose: Option<Callback<MouseEvent>>,
}


#[function_component]
pub fn Modal(props: &ModalProps) -> Html {
    let ModalProps { id, title, body, footer, onclose } = props;

    let onclick_close = {
        shadow_clone![id, onclose];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(onclose) = onclose.clone() {
                onclose.emit(e.clone());
            }
            close_modal_cb(id.clone()).emit(e);
        })
    };

    {
    shadow_clone![id, title, body, footer];
    html! {
        <div id={id} class="modal">
            <div class="modal-background jb-modal-close"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                if title.is_some() {
                    <p class="modal-card-title">{ title }</p>
                }
                <button class="delete" aria-label="close" onclick={onclick_close.clone()}></button>
                </header>
                <section class="modal-card-body">
                {
                  body
                }
                </section>
                if footer.is_some() {
                <footer class="modal-card-foot">
                    { footer }
                </footer>
                }
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={onclick_close.clone()}></button>
      </div>
    }
    }
}
