
use web_sys::HtmlInputElement;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FormProps {
    pub method: String,
    pub children: Children,
    pub classes: Option<Classes>,
}

#[function_component]
pub fn Form(props: &FormProps) -> Html {
    let props = props.clone();
    html!{
        <form class={props.classes} method={ props.method }>
            { props.children }
        </form>
    }
}



#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FormFieldProps {
    pub label: Option<String>,
    pub children: Children,
    #[prop_or(true)]
    pub is_horizontal: bool,
}

#[function_component]
pub fn FormField(props: &FormFieldProps) -> Html {
    let props = props.clone();
    html!{
        <div class={classes!["field", props.is_horizontal.then(|| "is-horizontal")]}>
            if props.label.is_some() {
                if props.is_horizontal {
                <div class="field-label is-normal">
                    <label class="label">{ props.label }</label>
                </div>
                } else {
                <label class="label">{ props.label }</label>
                }
            }
            <div class="field-body">
                <div class="field">
                    { props.children }
                </div>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TextInputFieldProps {
    #[prop_or(String::from("text"))]
    pub input_type: String,
    pub placeholder: Option<String>,
    pub value: Option<String>,
    pub oninput: Callback<InputEvent>,
    pub msg: Option<String>,
    pub error_msg: Option<String>,
    pub has_error: bool,
    pub icon_left: Option<String>,
    pub icon_right: Option<String>,
}

use crate::{components::form::icon::Icon, shadow_clone};

#[function_component]
pub fn TextInputField(props: &TextInputFieldProps) -> Html {
    let props = props.clone();
    html!{
        <>
        <div class={classes!["control",
            props.icon_left.is_some().then(|| "has-icons-left"),
            props.icon_right.is_some().then(|| "has-icons-right"),
        ]}>
            <input class={classes!["input", if props.has_error { "is-danger" } else { "" }] }
                type = {props.input_type}
                placeholder = { props.placeholder }
                value = { props.value }
                oninput = { props.oninput }
            />
            <Icon icon={ props.icon_right } position="is-right" />
            <Icon icon={ props.icon_left } position="is-left" />
        </div>
        if props.has_error {
            <p class="help is-danger">{ props.error_msg }</p>
        } else {
            <p class="help">{ props.msg }</p>
        }
        </>
    }
}


use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use validator::ValidationErrors;

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
            <Icon icon={ props.icon_left } position="is-left" />
            if !error_message.is_empty() && props.icon_right.is_some() {
                <Icon icon={ props.icon_right } position="is-right" />
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
    let is_init = use_state(|| true);

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
