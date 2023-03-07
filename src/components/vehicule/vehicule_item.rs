use yew::prelude::*;

use crate::hooks::user_context::use_user_context;


#[function_component]
pub fn VehiculeItem() -> Html {
    let user_ctx = use_user_context();

    if user_ctx.is_authenticated() {
        
    }
    
    html! {
        <p>{"Vehicule item"}</p>

    }

}


/*
fn admin_view() {

}

fn normal_view() {

}
*/
