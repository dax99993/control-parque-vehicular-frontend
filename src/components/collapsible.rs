use yew::prelude::*;
use gloo::utils::document;

use crate::shadow_clone;
use crate::utils::{add_class, remove_class};


/*
pub enum CollapsibleMsg {
   Expand,
   Collapse,
}

#[derive(Debug, PartialEq, Properties)]
pub struct CollapsibleProps {
    #[prop_or_default]
    pub children: Children,
    pub id: String,
}

pub struct Collapsible {
    id: String,
}

impl Component for Collapsible {
    type Message = CollapsibleMsg;
    type Properties = CollapsibleProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            id: props.id.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <p>{ &self.id }</p> 
        }
    }
}
*/



#[derive(Debug, PartialEq, Properties)]
pub struct CollapsibleProps {
    #[prop_or_default]
    pub children: Children,
    pub id: String,
    pub expanded: bool,
}

#[function_component]
pub fn Collapsible(props: &CollapsibleProps) -> Html {
    //let props = props.clone();
    let CollapsibleProps { children, id, expanded } = props;

    {
    shadow_clone![id, expanded];
    use_effect_with_deps(move |(id, expanded)| {
            if let Some(element) = document().get_element_by_id(&id) {
                if expanded.clone() {
                    //element.set_attribute("aria-expanded", "false");
                    if let Ok(_) = element.set_attribute("style", "height: 0px") {
                        add_class(&element, "is-active");
                    }
                } else {
                    //element.set_attribute("aria-expanded", "true");
                    let height = element.scroll_height();
                    if let Ok(_) = element.set_attribute("style", &format!("height: {}px", height)) {
                        remove_class(&element, "is-active");
                    }
                }
            }
        },
        (id, expanded).clone()
    );
    }

    shadow_clone![id, children];
    html! {
        <div id={id} class="is-collapsible">
            {children}
        </div>
    }
}


