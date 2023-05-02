use yew::prelude::*;

use gloo::utils::document;

use crate::shadow_clone;
use crate::utils::{add_class, remove_class};

#[derive(Debug, PartialEq, Properties)]
pub struct CollapsibleProps {
    #[prop_or_default]
    pub children: Children,
    pub id: String,
    pub expanded: bool,
}

#[function_component]
pub fn Collapsible(props: &CollapsibleProps) -> Html {
    //let props = props.clone();
    let CollapsibleProps { children, id, expanded } = props;

    {
    shadow_clone![id, expanded];
    use_effect_with_deps(move |(id, expanded)| {
            if let Some(element) = document().get_element_by_id(&id) {
                if expanded.clone() {
                    //element.set_attribute("aria-expanded", "false");
                    if let Ok(_) = element.set_attribute("style", "height: 0px") {
                        add_class(&element, "is-active");
                    }
                } else {
                    //element.set_attribute("aria-expanded", "true");
                    let height = element.scroll_height();
                    if let Ok(_) = element.set_attribute("style", &format!("height: {}px", height)) {
                        remove_class(&element, "is-active");
                    }
                }
            }
        },
        (id, expanded).clone()
    );
    }

    shadow_clone![id, children];
    html! {
        <div id={id} class="is-collapsible">
            {children}
        </div>
    }
}



#[function_component]
pub fn FormCollapsible() -> Html {

    let expanded = use_state(|| false);

    let onclick_expande = {
        let expanded = expanded.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            expanded.set(!(*expanded));
        })
    };

    let onclick_save = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            log::debug!("Save changes");
        })
    };

    html! {
        <div class="card">
            <header class={classes!["card-header", if !(*expanded) { "is-hidden" } else { "" }]} >
                <div class="card-header-title">
                    <div class="columns">
                        <div class="column is-half">
                            <p> {"Campo"} </p>
                        </div>
                        <div class="column is-offset-5">
                            <p> {"Valor"} </p>
                        </div>
                    </div>
                </div>
                <a href="#" class="card-header-icon">
                <button class="button is-outlined is-info is-small" type="button" onclick={onclick_expande.clone()} >
                    <span class="icon"><i class="fa-solid fa-pen"></i></span>
                    <span>{"Editar"}</span>
                </button>
                </a>
            </header>
            <Collapsible id="some-id" expanded={*expanded}>
                <div class="card-content">
                    <p>{"some content"}</p>
                </div>
                <div class="card-footer">
                    <div class="columns">
                        <div class="column">
                            <button class="button is-info is-small" type="button" onclick={onclick_save}>
                                <span class="icon"><i class="fa-solid fa-save"></i></span>
                                <span>{"Guardar"}</span>
                            </button>
                        </div>
                        <div class="column">
                            <button class="button is-outlined is-small" type="button" onclick={onclick_expande}>
                                <span class="icon"><i class="fa-solid fa-cross"></i></span>
                                <span>{"Cerrar"}</span>
                            </button>
                        </div>
                    </div>
                </div>
            </Collapsible>
        </div>
    }
}
