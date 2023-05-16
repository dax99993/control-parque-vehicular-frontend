use yew::prelude::*;
use yew_hooks::use_async;
use uuid::Uuid;

//use crate::features::
use common::models::user::Usuario;
use crate::features::users::services::request_admin_get_user_with_id;

//use crate::features::users::com
use super::user_edit::AdminUserEditForm;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub usuario_id: Uuid,
}

#[function_component]
pub fn AdminUserEditView(props: &Props) -> Html {
    //Props
    let Props { usuario_id } = props;


    //States
    let usuario = use_state(|| Usuario::default());


    // Hooks

    let request_usuario = {
        let id = usuario_id.to_string();
        use_async(async move {
            request_admin_get_user_with_id(id).await
        })
    };

    // Fetch en primer render
    {
        let request_usuario = request_usuario.clone();
        use_effect_with_deps(move |_| {
            request_usuario.run();
        },
        ())
    }

    // Set usuario 
    {
        let usuario = usuario.clone();
        use_effect_with_deps(move |request| {
            if let Some(response) = &request.data {
                usuario.set(response.data.clone().unwrap());
            }
            if let Some(error) = &request.error{
                log::error!("request error: {:?}", &error);
            }
        },
        request_usuario.clone())
    }


    html!{
        <AdminUserEditForm usuario={usuario.clone()}/>
    }
}
