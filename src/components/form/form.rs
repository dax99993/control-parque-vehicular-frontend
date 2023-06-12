use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FormProps {
    #[prop_or(String::from("post"))]
    pub method: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub onsubmit: Callback<SubmitEvent>,
}

#[function_component]
pub fn Form(props: &FormProps) -> Html {
    let props = props.clone();
    html!{
        <form class={props.classes} method={ props.method } onsubmit={props.onsubmit}>
            { props.children }
        </form>
    }
}



#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FormFieldProps {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
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
