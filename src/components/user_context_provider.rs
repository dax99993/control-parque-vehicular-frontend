//! User context provider.
use yew::prelude::*;
use yew_hooks::prelude::*;

//use crate::api_response::ApiResponse;
use crate::error::Error;
use crate::services::auth::*;
use crate::services::request::{get_token, store_token};
use crate::types::user::FilteredUser;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component]
pub fn UserContextProvider(props: &Props) -> Html {
    let user_ctx = use_state( || None::<FilteredUser>);
    let current_user = use_async(async move { request_me().await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_token().is_some() {
                current_user.run();
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(response) = &current_user.data {
                    user_ctx.set(response.data.clone());
                }

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
        <ContextProvider<UseStateHandle<Option<FilteredUser>>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<Option<FilteredUser>>>>
    }
}
