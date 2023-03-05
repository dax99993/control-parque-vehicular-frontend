use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::request::store_token;
use crate::types::user::FilteredUser;
//use crate::types::user::User;

#[derive(PartialEq)]
pub struct User {
    pub email: String,
}

/// State handle for the ['use_user_context'] hook
pub struct UseUserContextHandle {
    //inner: UseStateHandle<Option<User>>,
    inner: UseStateHandle<Option<FilteredUser>>,
    navigator: Navigator,
}

impl UseUserContextHandle {
    //pub fn login(&self, user: &FilteredUser, token: String) {
    pub fn login(&self, user: &FilteredUser) {
        // Store token to local storage
        //store_token(Some(token));
        // Set state to logged in User
        //self.inner.set(Some(user));
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
}



#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    //let inner = use_context::<UseStateHandle<Option<User>>>().unwrap();
    let inner = use_context::<UseStateHandle<Option<FilteredUser>>>().unwrap();
    let navigator = use_navigator().unwrap();

    UseUserContextHandle { inner, navigator }
}
