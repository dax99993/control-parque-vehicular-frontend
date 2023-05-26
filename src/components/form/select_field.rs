use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::rc::Rc;
use std::cell::RefCell;
use validator::ValidationErrors;

use crate::shadow_clone;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SelectFieldValidatedProps {
    // value, display text
    pub options: Vec<(String, String)>,
    pub msg: Option<String>,
    pub selected: Option<String>,
    //pub icon_left: Option<String>,
    //pub icon_right: Option<String>,

    pub name: String,
    pub input_ref: NodeRef,
    pub handle_onchange: Callback<String>,
    pub handle_on_input_blur: Callback<(String, String)>,
    pub errors: Rc<RefCell<ValidationErrors>>
}

#[function_component]
pub fn SelectFieldValidated(props: &SelectFieldValidatedProps) -> Html {
    shadow_clone!(props);
    //States
    let is_init = use_state(|| true);
    log::debug!("select field selected\n{:?}\nvalues\n{:?}", props.selected, props.options);

    // Callbacks
    let onclick = {
        shadow_clone!(props);
        shadow_clone!(is_init);
        Callback::from(move |e: Event| {
            is_init.set(false);
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            log::debug!("dropdown menu value: {}", value);
            props.handle_onchange.emit(value);
        })
    };

    let on_blur = {
        let handle_on_input_blur = props.handle_on_input_blur.clone();
        let cloned_input_name = props.name.clone();
        shadow_clone!(is_init);
        Callback::from(move |event: FocusEvent| {
            is_init.set(false);
            let input_name = cloned_input_name.clone();
            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            handle_on_input_blur.emit((input_name, value));
        })
    };


    //HTML
    html!{
    <div class="select is-small">
        <select onchange={onclick} onblur={on_blur}>
        {
            props.options.iter().map(|(value, text)| {
                shadow_clone!(value);
                let selected = if *is_init && props.selected.is_some() {
                    props.selected.clone().unwrap() == value
                } else { false };
                html!{
                    <option {value} {selected}>{text}</option>
                }
            }).collect::<Html>()
        }
        </select>
    </div>
    }
}
