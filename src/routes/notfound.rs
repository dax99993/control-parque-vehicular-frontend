use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;


#[function_component]
pub fn NotFoundView() -> Html {

    html!{
        <section class="hero is-fullheight is-info is-medium is-bold">
            <div class="hero-body">
                <div class="container has-text-centered">
                    <h1 class="title">{"Error 404"}</h1>
                    <h1 class="title">{"La pagina que intenta acceder no existe"}</h1>
                    <br/>
                    <Link<AppRoute> to={AppRoute::Home} classes="button is-rounded is-info is-outlined is-inverted">
                    {"Ir a pagina principal"}
                    </Link<AppRoute>>
                </div>
            </div>
        </section>
    }
}
