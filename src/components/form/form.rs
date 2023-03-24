use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FormProps {
    pub method: String,
    pub children: Children,
}

#[function_component]
pub fn Form(props: &FormProps) -> Html {
    let props = props.clone();
    html!{
        <form method={ props.method }>
            { props.children }
        </form>
    }
}



#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FormFieldProps {
    pub label: Option<String>,
    pub children: Children,
}

#[function_component]
pub fn FormField(props: &FormFieldProps) -> Html {
    let props = props.clone();
    html!{
        <div class="field is-horizontal">
            <div class="field-label is-normal">
                if props.label.is_some() {
                <label class="label">{ props.label }</label>
                }
            </div>
            <div class="field-body">
                <div class="field">
                    { props.children }
                </div>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FormInputFieldProps {
    pub input_type: String,
    pub placeholder: Option<String>,
    pub value: Option<String>,
    pub danger_msg: Option<String>,
    pub oninput: Callback<InputEvent>,
    pub valid: bool,
    pub icon_left: Option<String>,
    pub icon_right: Option<String>,
}

use crate::components::form::icon::Icon;

#[function_component]
pub fn FormInputField(props: &FormInputFieldProps) -> Html {
    let props = props.clone();
    html!{
        <>
        <div class={classes!["control",
            props.icon_left.is_some().then(|| "has-icons-left"),
            props.icon_right.is_some().then(|| "has-icons-right"),
        ]}>
            <input class={classes!["input", if !props.valid { "is-danger" } else { "" }] }
                type = { props.input_type }
                placeholder = { props.placeholder }
                value = { props.value }
                oninput = { props.oninput }
            />
            <Icon icon={ props.icon_right } position="is-right" />
            <Icon icon={ props.icon_left } position="is-left" />
        </div>
        if !props.valid {
            <p class="help is-danger">{ props.danger_msg }</p>
        }
        </>
    }
}
