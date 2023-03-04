use yew::prelude::*;
use yew_router::prelude::*;


use crate::components::navbar::NavBar;
use crate::components::footer::Footer;
use crate::components::tabs::Tabs;
use crate::routes::{switch, AppRoute};


/// The root app component
#[function_component]
pub fn App() -> Html {
    html! {
        <HashRouter>
            <NavBar />
            <Tabs />
            <Switch<AppRoute> render={switch} />
            <Footer />
        </HashRouter>
    }
}


