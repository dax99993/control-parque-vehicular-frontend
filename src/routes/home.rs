use yew::prelude::*;
use yew_router::components::Link;

use crate::hooks::user_context::use_user_context;
use crate::routes::AppRoute;

use crate::components::form::collapsible::FormCollapsible;
use crate::components::upload::file_upload::UploadFile;
use crate::components::upload::Upload;

use crate::services::vehicule::request_admin_get_vehicules;

#[function_component]
pub fn HomeView() -> Html {
    let user_ctx = use_user_context();


    html! {
        if user_ctx.is_authenticated() {
            <HomeLoggedInView/>
            <UploadFile accept={vec!["image/*".into(), "video/*".into()]} multiple={false} max_files={2}/>
            <Upload request_url={"someurl"}/>
        } else {
            <HomeLoggedOutView/>
        }
    }
}

#[function_component]
fn HomeLoggedInView() -> Html {
    html! {
    <>
        <p>{"Home user"}</p>
        
        <FormCollapsible/>

    </>
    }
}

#[function_component]
fn HomeLoggedOutView() -> Html {
    html!{
        <section class="hero is-fullheight is-info is-medium is-bold">
            <div class="hero-body">
                <div class="container has-text-centered">
                    <h1 class="title">{"Control Parque Vehicular"}</h1>
                    <h1 class="title">{"Iniciar sesion o crear cuenta"}</h1>
                    <br/>
                    <h2 class="subtitle has-text-danger-dark">{"Home Anonymous user"}</h2>

                    <Link<AppRoute> to={AppRoute::Login} classes="button is-rounded is-info is-outlined is-inverted">
                    {"Iniciar sesion"}
                    </Link<AppRoute>>

                    <Link<AppRoute> to={AppRoute::Register} classes="button is-rounded is-primary is-outlined is-inverted">
                    {"Registrar cuenta"}
                    </Link<AppRoute>>
                </div>
            </div>
        </section>
    }
}
