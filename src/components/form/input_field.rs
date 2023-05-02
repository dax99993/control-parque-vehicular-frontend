use yew::prelude::*;
use web_sys::HtmlInputElement;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use validator::ValidationErrors;

use crate::shadow_clone;
use crate::components::icon::{Icon, IconPosition};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct InputFieldValidatedProps {
    #[prop_or(String::from("text"))]
    pub input_type: String,
    pub placeholder: Option<String>,
    //pub value: Option<String>,
    pub msg: Option<String>,

    pub icon_left: Option<String>,
    pub icon_right: Option<String>,

    pub name: String,
    pub input_ref: NodeRef,
    pub handle_onchange: Callback<String>,
    pub handle_on_input_blur: Callback<(String, String)>,
    pub errors: Rc<RefCell<ValidationErrors>>
}

#[function_component]
pub fn InputFieldValidated(props: &InputFieldValidatedProps) -> Html {
    shadow_clone!(props);
    let is_init = use_state(|| true);

    let val_errors = props.errors.borrow();
    let errors = val_errors.field_errors().clone();
    let empty_errors = vec![];
    let error = match errors.get(&props.name.as_str()) {
        Some(error) => error,
        None => &empty_errors,
    };
    let error_message = match error.get(0) {
        Some(message) => message.to_string(),
        None => "".to_string(),
    };
    
    let onchange = {
        let handle_onchange = props.handle_onchange.clone();
        shadow_clone!(is_init);
        Callback::from(move |event: Event| {
            is_init.set(false);
            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            handle_onchange.emit(value);
        })
    };

    let on_blur = {
        let handle_on_input_blur = props.handle_on_input_blur.clone();
        let cloned_input_name = props.name.clone();
        shadow_clone!(is_init);
        Callback::from(move |event: FocusEvent| {
            is_init.set(false);
            let input_name = cloned_input_name.clone();
            //let target = event.target().unwrap();
            //let value = target.unchecked_into::<HtmlInputElement>().value();
            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            handle_on_input_blur.emit((input_name, value));
        })
    };

    html!{
        <>
        <div class={classes!["control",
            props.icon_left.is_some().then(|| "has-icons-left"),
            if !error_message.is_empty() && props.icon_right.is_some() { "has-icons-right" } else { "" },
        ]}>
            <input class={classes!["input", 
                if !error_message.is_empty() && !is_init.deref() { "is-danger" } 
                else if *is_init  { "" } 
                else { "is-success" }
                ]}

                type = {props.input_type}
                placeholder = { props.placeholder }
                ref={props.input_ref.clone()}
                onchange={onchange}
                onblur={on_blur}
            />
            <Icon icon={ props.icon_left } position={IconPosition::Left}/>
            if !error_message.is_empty() && props.icon_right.is_some() {
            <Icon icon={ props.icon_right } position={IconPosition::Right}/>
            }
        </div>
        if !error_message.is_empty() {
            <p class="help is-danger">{ error_message }</p>
        } else {
            <p class="help">{ props.msg }</p>
        }
        </>
    }
}