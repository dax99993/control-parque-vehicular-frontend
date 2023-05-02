use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonType {
    Button,
    Submit,
    Reset,
}


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Type of button for this component
    #[prop_or(ButtonType::Button)]
    pub r#type: ButtonType,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}


#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let props = props.clone();

    let button_type = match props.r#type {
        ButtonType::Button => "button",
        ButtonType::Submit => "submit",
        ButtonType::Reset => "reset",
    };

    let class = classes![
        "button",
        props.classes.clone(),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static"),
    ];

    html!{
        <button type={ button_type } {class} onclick={props.onclick} disabled={props.disabled} >
            { props.children }
        </button>
    }
}
