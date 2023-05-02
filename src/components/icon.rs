use yew::prelude::*;


#[derive(Debug, Clone, PartialEq)]
pub enum IconPosition {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct IconProps {
    #[prop_or(IconPosition::Left)]
    pub position: IconPosition,
    #[prop_or_default]
    pub icon: Option<String>,
}

#[function_component]
pub fn Icon(props: &IconProps) -> Html {
    let props = props.clone();

    let position = match props.position {
        IconPosition::Left => { "is-left" },
        IconPosition::Right=> { "is-right" },
    };

    html!{
        if props.icon.is_some() {
        <span class={classes!(
                "icon",
                "is-small",
                position
                )}
        >
            <i class={ props.icon }></i>
        </span>
        }
    }
}
