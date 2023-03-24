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
    pub danger_msg: Option<String>,
    pub oninput: Callback<InputEvent>,
    pub valid: bool,
}

#[function_component]
pub fn FormInputField(props: &FormInputFieldProps) -> Html {
    let props = props.clone();
    html!{
        <>
        <div class="control">
            <input class={classes!["input", if !props.valid { "is-danger" } else { "" }] }
                type = { props.input_type }
                placeholder  ={ props.placeholder }
                oninput = { props.oninput }
            />
        </div>
        if !props.valid {
            <p class="help is-danger">{ props.danger_msg }</p>
        }
        </>
    }
}
