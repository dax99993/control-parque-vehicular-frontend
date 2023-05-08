use yew::prelude::*;

use uuid::Uuid;

use crate::features::vehicule_edit::EditVehiculeAdminView;
use crate::hooks::user_context::use_user_context;



// Maybe should move this component to features 

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditVehiculeProps {
    pub id: Uuid,
}


#[function_component]
pub fn EditVehiculeView(props: &EditVehiculeProps) -> Html {
    // Context
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }

    html!{
        <EditVehiculeAdminView id={props.id}/>
    }

}
