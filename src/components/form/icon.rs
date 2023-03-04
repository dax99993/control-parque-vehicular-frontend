use yew::prelude::*;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct IconProps {
    #[prop_or(String::from("is-left"))]
    pub position: String,
    #[prop_or_default]
    pub icon: Option<String>,
}

#[function_component]
pub fn Icon(props: &IconProps) -> Html {
    let props = props.clone();

    html!{
        if props.icon.is_some() {
        <span class={classes!(
                "icon",
                "is-small",
                props.position
                )}
        >
            <i class={ props.icon }></i>
        </span>
        }
    }
}
