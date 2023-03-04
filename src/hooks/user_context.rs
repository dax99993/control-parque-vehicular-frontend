use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// State handle for the ['use_user_context'] hook
pub struct UseUserContextHandle {
    inner: UseStateHandle<()>,
    navigator: Navigator,
}


impl UseUserContextHandle {
    pub fn login(&self) {

        // Return to home page
        self.navigator.push(&AppRoute::Home)
    }

    pub fn logout(&self) {

        // Return to home page
        self.navigator.push(&AppRoute::Home)
    }
}



#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<()>>().unwrap();
    let navigator = use_navigator().unwrap();

    UseUserContextHandle { inner, navigator }
}
