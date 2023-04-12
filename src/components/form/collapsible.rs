use yew::prelude::*;

use crate::components::collapsible::Collapsible;


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
