use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::components::Link;

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

    let open_modal = Callback::from(|e: MouseEvent| {
        e.prevent_default();
        if let Ok(Some(e)) = gloo::utils::document().query_selector("[data-modal]") {
        //if let Ok(Some(e)) = gloo::utils::document().query_selector("#dialog-modal") {
            let dialog = e.unchecked_into::<web_sys::HtmlDialogElement>();
            let _ = dialog.show_modal();
        }
    });

    let close_modal = Callback::from(|e: MouseEvent| {
        if let Ok(Some(e)) = gloo::utils::document().query_selector("[data-modal]") {
        //if let Ok(Some(e)) = gloo::utils::document().query_selector("#dialog-modal") {
            let dialog = e.unchecked_into::<web_sys::HtmlDialogElement>();
            dialog.close();
        }
    });

    let onkey_log = Callback::from(|e: KeyboardEvent| {
        log::debug!("you press the key {}",e.key());
    });

    html! {
    <>
        <p>{"Home user"}</p>

        <button onclick={open_modal}>{"open modal"}</button>
        <dialog id={"dialog-modal"} onkeypress={onkey_log} data-modal={""}>
            <div class="card">
            <header class="card-header">
                <div class="card-header-icon">
                    <button class="delete" onclick={close_modal.clone()}></button>
                </div>
            </header>
            <div class="card-content">{"This is a modal"}</div>
            <footer class="card-footer">
                <button class="button is-danger" onclick={close_modal}>{"Close"}</button>
            </footer>
            </div>
        </dialog>
        
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
