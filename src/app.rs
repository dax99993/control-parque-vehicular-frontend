use yew::prelude::*;
use yew_router::prelude::*;


use crate::components::navbar::NavBar;
use crate::components::footer::Footer;
use crate::components::user_context_provider::UserContextProvider;
use crate::components::sidebar::Sidebar;
use crate::components::main_section::MainSection;
use crate::routes::{switch, AppRoute};


/// The root app component
#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <UserContextProvider>
                <NavBar />
                <MainSection>
                    <Sidebar/>
                    <Switch<AppRoute> render={switch} />
                </MainSection>
                <Footer />
            </UserContextProvider>
        </BrowserRouter>
    }
}


