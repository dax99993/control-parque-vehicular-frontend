use yew::prelude::*;
use yew_router::prelude::*;


use crate::layout::navbar::NavBar;
use crate::layout::footer::Footer;
use crate::layout::sidebar::Sidebar;
use crate::context::UserContextProvider;
use crate::routes::{switch, AppRoute};
use crate::components::toast::ToasterViewer;


/// The root app component
#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <UserContextProvider>
                <NavBar />
                <Sidebar/>
                <ToasterViewer>
                    <Switch<AppRoute> render={switch} />
                </ToasterViewer>
                <Footer />
            </UserContextProvider>
        </BrowserRouter>
    }
}


