use yew::prelude::*;
use uuid::Uuid;

use super::admin::users::AdminUserEditView;
use crate::hooks::user_context::use_user_context;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub id: Uuid,
}


#[function_component]
pub fn EditUserView(props: &Props) -> Html {
    // Context
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }

    html!{
        <AdminUserEditView usuario_id={props.id}/>
    }

}
