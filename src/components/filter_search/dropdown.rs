use yew::prelude::*;
//use yew_hooks::
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DropDownProps {
    #[prop_or("Selecciona Filtro".to_string())]
    pub dropdown_button_label: String,
    #[prop_or_default]
    pub dropdown_items_labels: Vec<String>,

    pub selected_state: UseStateHandle<Option<String>>,
}

#[function_component]
pub fn DropDown(props: &DropDownProps) -> Html {
    let props = props.clone();
    let selected = props.selected_state.clone();

    let active = use_state(|| false);

    let onclick_activate_dropdown = {
        let active = active.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            active.set(!(*active));
        })
    };

    // Close dropdown when selecting an item
    {
        let active = active.clone();
        use_effect_with_deps(move |_| {
            active.set(false);
        }
        , selected.clone())
    }

    let dropdown_classes = 
        classes![
            "dropdown",
            if *active {"is-active"} else {""},
        ];

    let dropdown_current_button_label = {
        if selected.deref().is_some() { 
            selected.deref().clone().unwrap()
        } else {
            props.dropdown_button_label
        }
    };

        html!{
            <div class={dropdown_classes}>
                <div class="dropdown-trigger">
                    <button class="button" aria-haspopup="true" aria-controls="dropdown-menu" onclick={onclick_activate_dropdown}>
                    <span>{ dropdown_current_button_label }</span>
                    <span class="icon is-small">
                        <i class="fas fa-angle-down" aria-hidden="true"></i>
                    </span>
                    </button>
                </div>

                <div class="dropdown-menu" id="dropdown-menu" role="menu">
                    <div class="dropdown-content">
                    {
                        vec_string_to_dropdown_items(props.dropdown_items_labels.clone(), selected.clone())
                    }
                    </div>
                </div>
            </div>
        }
}

fn vec_string_to_dropdown_items(fields: Vec<String>, selected_state: UseStateHandle<Option<String>>) -> Vec<Html> {
    fields.into_iter().map(|f| {
        html!{
            <DropDownItem
                label={f}
                selected_state={selected_state.clone()}
            />
        }
    }).collect()
}


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DropDownItemProps {
    #[prop_or("label".to_string())]
    pub label: String,
    pub selected_state: UseStateHandle<Option<String>>,
}

#[function_component]
fn DropDownItem(props: &DropDownItemProps) -> Html {
    let props = props.clone();

    let is_active = props.selected_state.deref().is_some() && 
                props.selected_state.deref().clone().unwrap() == props.label;

    let dropdown_item_classes = 
        classes![
            "dropdown-item",
            if is_active { "is-active" } else { "" }
        ];

    let onclick_activate_item= {
        let props = props.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            log::debug!("you click an item! {}", &props.label);
            props.selected_state.set(Some(props.label.clone()));
        })
    };


    html!{
        <a class={dropdown_item_classes} onclick={onclick_activate_item}>
            {props.label}
        </a>
    }
}
