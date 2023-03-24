use yew::prelude::*;

use super::icon::Icon;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct FormItemProps {
    pub label: String,
    #[prop_or(String::from("text"))]
    pub input_type: String,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub icon_left: Option<String>,
    #[prop_or_default]
    pub icon_right: Option<String>,
    #[prop_or_default]
    pub error: Option<String>,

    //pub validate_fn: Callback<HtmlInputElement, bool>,
    //pub oninput: Callback<InputEvent, String>,
    pub oninput: Callback<InputEvent>,
}


#[function_component]
pub fn FormItem(props: &FormItemProps) -> Html {

    let props = props.clone();
    let id=format!("{}-field", &props.label);
    

    html! {
        <div class="field">
            <label class="label">{ props.label }</label>
            <div class={classes!(
                    "control",
                    props.icon_left.is_some().then(|| "has-icons-left"),
                    props.icon_right.is_some().then(|| "has-icons-right"),
                    )}>
                <input id={id}
                    class="input"
                    type={ props.input_type }
                    placeholder={ props.placeholder }
                    oninput={props.oninput}
                />
                <Icon icon={ props.icon_right } position="is-right" />
                <Icon icon={ props.icon_left } position="is-left" />
            </div>
                if props.error.is_some() {
                    <p class="help is-danger">{ props.error }</p>
                }
        </div>
    }
}


