use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::dropdown::DropDown;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FilterSearchProps{
    #[prop_or("Selecciona Filtro".to_string())]
    pub default_filter_label: String,
    #[prop_or_default]
    pub filter_fields: Vec<String>,
    pub selected_filter_state: UseStateHandle<Option<String>>,
    pub search_state: UseStateHandle<Option<String>>,
}


#[function_component]
pub fn FilterSearch(props: &FilterSearchProps) -> Html {
    let props = props.clone(); 

    //let selected_filter = use_state(|| None::<String>);
    let selected_filter_state = props.selected_filter_state.clone();
    let filter_fields = props.filter_fields.clone();

    let search = props.search_state.clone();
    let search_ref = use_node_ref();

    let onclick_get_search = {
        let search = search.clone();
        let search_ref = search_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // Get search field value
            if let Some(input) = search_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                log::debug!("search field = {}", value);
                let search_value = if value.is_empty() {
                    None
                } else { 
                    Some(value.to_string())
                };
                search.set(search_value);
            }
        })
    };

    html!{
        <nav class="level">
            <div class="level-left">
                <div class="level-item">
                    <div class="field has-addons">

                        <DropDown dropdown_button_label={props.default_filter_label.clone()} selected_state={selected_filter_state} dropdown_items_labels={filter_fields.clone()}/>

                        <p class="control">
                            <input ref={search_ref} class="input" type="text" placeholder={"Encontrar vehiculo"}/>
                        </p>

                        <p class="control">
                            <button class="button" onclick={onclick_get_search}>
                                {"Buscar"}
                            </button>
                        </p>

                    </div>
                </div>
            </div>
        </nav>
    }
}
