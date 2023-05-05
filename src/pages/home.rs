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

                    <Link<AppRoute> to={AppRoute::Signup} classes="button is-rounded is-primary is-outlined is-inverted">
                    {"Registrar cuenta"}
                    </Link<AppRoute>>
                </div>
            </div>
        </section>
    }
}
