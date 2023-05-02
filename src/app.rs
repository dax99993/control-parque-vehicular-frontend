use yew::prelude::*;
use yew_router::prelude::*;


use crate::layout::navbar::NavBar;
use crate::layout::footer::Footer;
use crate::layout::sidebar::Sidebar;
use crate::context::UserContextProvider;
use crate::routes::{switch, AppRoute};


/// The root app component
#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <UserContextProvider>
                <NavBar />
                <Sidebar/>
                <Switch<AppRoute> render={switch} />
                <Footer />
            </UserContextProvider>
        </BrowserRouter>
    }
}


