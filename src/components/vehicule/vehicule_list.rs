use yew::prelude::*;


use crate::hooks::user_context::use_user_context;
use super::vehicule_item::VehiculeItem;
use crate::components::tabs::Tabs;

#[function_component]
pub fn VehiculeList() -> Html {
    let user_ctx = use_user_context();

    if user_ctx.is_authenticated() {
        
    }
    
    html! {
        <>
            <Tabs >
            </Tabs>
            <VehiculeItem>
            </VehiculeItem>
        </>

    }

}


fn admin_view() {

}

fn normal_view() {

}
