use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::Link;
use yew_hooks::use_async;

use crate::components::collapsible::FormCollapsible;
use crate::hooks::user_context::use_user_context;
use crate::routes::AppRoute;

use crate::services;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::ops::Deref;


use crate::components::toast::{use_toaster, Toast, ToastType, ToastPosition};

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

    let toaster = use_toaster().expect("No toaster viewer");

    let date = NodeRef::default();

    let cb = {
        //let date = date.clone();
        Callback::from(|e: MouseEvent| {
            //e.prevent_default();

            let input = e.target_unchecked_into::<HtmlInputElement>();
            log::debug!("{}", input.value());
        })
    };

    let cb_toast = {
        //let date = date.clone();
        let toaster = toaster.clone();    
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let toast = Toast {
                body: "Message".into(),
                position: ToastPosition::TopLeft,
                r#type: ToastType::Info,
                timeout: Some(chrono::Duration::milliseconds(2000)),
            };
            toaster.toast(toast);
        })
    };

    html! {
    <>
        <p>{"Home user"}</p>

        <FormCollapsible/>

        <form>
            <input ref={date} type="date" lang="es-VE" onclick={cb}/>
        </form>

        <button class="button" onclick={cb_toast}>
            {"Open toast"}
        </button>

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
