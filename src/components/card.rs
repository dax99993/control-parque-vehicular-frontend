use yew::prelude::*;

use crate::shadow_clone;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardProps {
    pub header_icon_left: Option<String>,
    pub header_icon_right: Option<String>,
    pub header_icon_right_label: Option<String>,
    pub header_icon_right_onclick: Option<Callback<MouseEvent>>,
    pub header_title: String,
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}


#[function_component]
pub fn Card(props: &CardProps) -> Html {
    shadow_clone!(props);

    html!{
        <div class={classes!["card", props.classes]}>
            <header class="card-header">
                <p class="card-header-title">
                    if props.header_icon_left.is_some() {
                    <span class="icon"><i class={ props.header_icon_left.unwrap() }></i></span>
                    }

                    { props.header_title }
                </p>
                if props.header_icon_right.is_some() || props.header_icon_right_label.is_some() {
                    <a href="#" class="card-header-icon" onclick={ props.header_icon_right_onclick }>
                        <button class="button is-outlined is-info is-small" type="button">
                        if props.header_icon_right.is_some() {
                            <span class="icon"><i class={ props.header_icon_right.unwrap() }></i></span>
                        }
                        if props.header_icon_right_label.is_some() {
                            <span>{ props.header_icon_right_label.unwrap() }</span>
                        }
                        </button>
                    </a>
                }
            </header>

            { props.children }
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardContentProps {
    pub children: Children,
}

#[function_component]
pub fn CardContent(props: &CardContentProps) -> Html {
    shadow_clone!(props);

    html!{
        <div class="card-content">
            { props.children }
        </div>
    }
}
