use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::collapsible::FormCollapsible;
use crate::hooks::user_context::use_user_context;
use crate::routes::AppRoute;

#[function_component]
pub fn HomeView() -> Html {
    let user_ctx = use_user_context();

    html! {
        if user_ctx.is_authenticated() {
            <HomeLoggedInView/>
        } else {
            <HomeLoggedOutView/>
        }
    }
}


#[function_component]
fn HomeLoggedInView() -> Html {

    html! {
    <>
        <FormCollapsible/>
    </>
    }
}


#[function_component]
fn HomeLoggedOutView() -> Html {
    html!{
        <section class="hero-background is-fullheight is-medium is-bold">
            <div class="hero-body">
                <div class="container">
                    <div class="columns is-centered">
                        <div class="column is-6-dektop is-5-widescreen">
                            <div class="box">
                                <div class="columns is-centered">
                                    <div class="column is-full has-text-centered ">
                                        <h1 class="title">{"Control Parque Vehicular"}</h1>
                                    </div>
                                </div>

                                <div class="columns is-centered has-text-centered">
                                    <div class="column">
                                        <Link<AppRoute> to={AppRoute::Login} classes="button is-rounded is-info">
                                        {"Iniciar sesion"}
                                        </Link<AppRoute>>
                                    </div>

                                    <div class="column">
                                        <Link<AppRoute> to={AppRoute::Signup} classes="button is-rounded is-primary">
                                        {"Registrar cuenta"}
                                        </Link<AppRoute>>
                                    </div>
                                </div>

                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
