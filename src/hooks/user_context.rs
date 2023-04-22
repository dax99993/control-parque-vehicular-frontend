use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::request::store_token;
//use crate::types::user::FilteredUser;
use crate::types::user::User;

use std::ops::Deref;



/// State handle for the ['use_user_context'] hook
#[derive(Debug, PartialEq)]
pub struct UseUserContextHandle {
    inner: UseStateHandle<Option<User>>,
    //inner: UseStateHandle<Option<FilteredUser>>,
    navigator: Navigator,
}

impl UseUserContextHandle {
    pub fn login(&self, user: &User) {
        // Store token to local storage
        //store_token(Some(token));
        // Set state to logged in User
        self.inner.set(Some(user.clone()));
        // Return to home page
        self.navigator.push(&AppRoute::Home)
    }

    pub fn logout(&self) {
        // Delete token from local storage
        store_token(None);
        // Set new state now without a User
        self.inner.set(None);
        // Return to home page
        self.navigator.push(&AppRoute::Home)
    }

    pub fn is_authenticated(&self) -> bool {
        self.inner.is_some()
    }

    pub fn redirect_home(&self) {
        self.navigator.push(&AppRoute::Home)
    }

    pub fn is_admin(&self) -> bool {
        if let Some(user) = (*self.inner).clone() {
            return user.is_admin();
        } else {
            return false;
        }
    }

    pub fn redirect_to<R>(&self, route: R)
    where 
        R: Routable
    {
        self.navigator.push(&route)
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            navigator: self.navigator.clone(),
        }
    }
}
impl Deref for UseUserContextHandle {
    type Target = Option<User>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}



#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<Option<User>>>().unwrap();
    let navigator = use_navigator().unwrap();

    UseUserContextHandle { inner, navigator }
}
