//! User context provider.
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::shadow_clone;
use crate::error::Error;
use crate::services::auth::*;
use crate::services::request::{get_token, store_token};
use crate::types::user::User;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component]
pub fn UserContextProvider(props: &Props) -> Html {
    let user_ctx = use_state( || None::<User>);

    // Api fetch request
    let current_user = {
        use_async(async move {
            request_me().await 
        })
    };

    // Fetch api when mounted
    {
        shadow_clone!(current_user);
        use_mount(move || {
            if get_token().is_some() {
                current_user.run();
            }
        });
    }

    // Update context when current user changes (e.g login or logout)
    {
        shadow_clone!(user_ctx);
        use_effect_with_deps(
            move |current_user| {
                // Successeful get me request
                if let Some(response) = &current_user.data {
                    user_ctx.set(response.data.clone());
                }

                // fail get me request (e.g Invalid or unexisting user token)
                if let Some(error) = &current_user.error {
                    match error {
                        Error::UnathorizedError | Error::ForbiddenError => store_token(None),
                        _ => (),
                    }
                }
                || ()
            },
            current_user,
        )
    }

    html! {
        <ContextProvider<UseStateHandle<Option<User>>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<Option<User>>>>
    }
}
