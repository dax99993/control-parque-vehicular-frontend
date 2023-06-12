use yew::prelude::*;
use super::toast::{ToastPosition, ToastType};

#[derive(Clone, PartialEq, Properties)]
pub struct ToastContainerProps {
    #[prop_or_default]
    pub position: ToastPosition,
    #[prop_or_default]
    pub children: Children,
}



/// A component to align and positioned toasts
///
/// There are 6 available positions that toasts can be placed
/// This component should be (on body level of HTML document) ???
#[function_component]
pub fn ToastContainer(props: &ToastContainerProps) -> Html {

    let position = props.position.as_attribute();

    html!{
        <div class="toaster-container" data-position={position}>
            { props.children.clone() }
        </div>
    }
}


#[derive(Clone, PartialEq, Properties)]
pub struct ToastComponentProps {
    //#[prop_or_default]
    //pub classes: Classes,
    #[prop_or_default]
    pub r#type: ToastType,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(true)]
    pub dimissible: bool,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
}


/// A component to show toast
///
/// There are 6 available positions that toasts can be placed
#[function_component]
pub fn ToastComponent(props: &ToastComponentProps) -> Html {
    let mut classes = classes!("toaster", "notification");
    classes.extend(props.r#type.as_classes());
    //classes.extend(props.classes.clone());

    html!{
        if let Some(onclose) = props.onclose.as_ref() {
            <div class={classes} onclick={onclose.clone().reform(|_|())}>
                <button class="delete" onclick={onclose.reform(|_|())}></button>
                {props.children.clone()}
            </div>
        } else {
            <div class={classes} >
                {props.children.clone()}
            </div>
        }
    }
}
