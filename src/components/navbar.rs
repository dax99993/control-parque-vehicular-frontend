use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;


#[function_component]
pub fn NavBar() -> Html {
    let logout= true;
    let menu_state = use_state(|| false);
    let toggle_menu = {
        let menu_state = menu_state.clone();
        Callback::from(move |_| {
            menu_state.set(! *menu_state);
        })
    };
    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <a class="navbar-item">
                    <Link<AppRoute> to={AppRoute::Home} >
                        <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28" />
                    </Link<AppRoute>>
                </a>

                <div class={classes!(
                        "navbar-burger",
                        "burger",
                        (*menu_state).then(|| Some("is-active"))
                    )}
                    onclick={toggle_menu}
                >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </div>
            </div>

            <div class={classes!(
                    "navbar-menu",
                    (*menu_state).then(|| Some("is-active"))
                    )}
            >
                <div class="navbar-start">

                <a class="navbar-item">
                {"Documentation"}
                </a>

                <div class="navbar-item has-dropdown is-hoverable">
                    <a class="navbar-link">
                        {"More"}
                    </a>

                    <div class="navbar-dropdown">
                        <a class="navbar-item">
                            {"About"}
                        </a>
                        <a class="navbar-item">
                            {"Jobs"}
                        </a>
                        <a class="navbar-item">
                            {"Contact"}
                        </a>
                        <hr class="navbar-divider" />
                        <a class="navbar-item">
                            {"Report an issue"}
                        </a>
                    </div>
                </div>
            </div>

            {
                if logout {
                    logged_out_view()
                } else {
                    logged_in_view()
                }
            }

            </div>
        </nav>
    }

}


fn logged_out_view() -> Html {
    html! {
        <div class="navbar-end">
            <div class="navbar-item">
                <div class="buttons">
                    <Link<AppRoute> to={AppRoute::Login} classes="button is-light">
                        { "Iniciar sesion" }
                    </Link<AppRoute>>
                    <Link<AppRoute> to={AppRoute::Register} classes="button is-primary">
                        { "Registrar" }
                    </Link<AppRoute>>
                </div>
            </div>
        </div>
    }
}

fn logged_in_view() -> Html {
    html! {
        <div class="navbar-end">
            <div class="navbar-item">
                <div class="buttons">
                    <Link<AppRoute> to={AppRoute::Register} classes="button is-light">
                        { "Configuracion" }
                    </Link<AppRoute>>
                    <Link<AppRoute> to={AppRoute::Login} classes="button is-primary">
                        { "Cerrar sesion" }
                    </Link<AppRoute>>
                </div>
            </div>
        </div>
    }
}
