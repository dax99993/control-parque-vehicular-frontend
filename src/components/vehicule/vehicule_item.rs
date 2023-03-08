use yew::prelude::*;

use crate::hooks::user_context::use_user_context;
use crate::types::vehicule::Vehicule;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct VehiculeItemProps {
    pub veh: Option<Vehicule>,
}

#[function_component]
pub fn VehiculeItem(props: &VehiculeItemProps) -> Html {
    let user_ctx = use_user_context();

    
    html! {
        <ybc::Media>
            <ybc::MediaLeft>
                <ybc::Image
                    size={Some(ybc::ImageSize::Is64x64)}>
                    <img src="https://w.wallhaven.cc/full/m9/wallhaven-m9xyg8.jpg" />
                </ybc::Image>
            </ybc::MediaLeft>
            <ybc::MediaContent>
                <ybc::Content>
                    <ul>
                        if props.veh.is_some() {
                            <li>
                                <p>{"Model: "} {&props.veh.clone().unwrap().model}</p>
                                <p>{"Year: "} {&props.veh.clone().unwrap().year}</p>
                            </li>
                        }
                    </ul>
                </ybc::Content>
            </ybc::MediaContent>

        </ybc::Media>
    }

}


/*
fn admin_view() {

}

fn normal_view() {

}
*/
