use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::user_context::use_user_context;

#[function_component]
pub fn Tabs() -> Html {
    let user_ctx = use_user_context();
    let state = use_state(|| false);
    let onclick = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| {
            state.set(!*state); 
        })
    };

    if true {
        normal_view(state, onclick)
    } else {
        html! {

        }
    }
}

fn normal_view(state: UseStateHandle<bool>, onclick: Callback<MouseEvent>) -> Html {
    html! {
        <ybc::Tabs classes={classes!(
                "is-centered",
                "is-boxed",
                "is-small",
                "is-toggle"
                )}>
          <ul>
            <li class={
                classes!(
                    if *state { Some("is-active") } else { None }
                )
            }
            >
              <a>
                <ybc::Icon classes={classes!(
                        "is-small"
                        )}>
                    <i class="fas fa-car" aria-hidden="true"></i>
                </ybc::Icon>
                <span>{"Vehiculos"}</span>
              </a>
            </li>
            <li>
              <a>
                <span class="icon is-small"><i class="fas fa-key" aria-hidden="true"></i></span>
                <span>{"Peticiones"}</span>
              </a>
            </li>
          </ul>
        </ybc::Tabs>
    }

}

fn admin_view(state: UseStateHandle<bool>, onclick: Callback<MouseEvent>) -> Html {
    html! {
        <div class="tabs is-centered is-boxed is-small is-toggle">
          <ul>
            <li class="is-active">
              <a>
                <span class="icon is-small"><i class="fas fa-user" aria-hidden="true"></i></span>
                <span>{"Usuarios"}</span>
              </a>
            </li>
            <li class={
                classes!(
                    if *state { Some("is-active") } else { None }
                )
            }
            >
              <a>
                <span class="icon is-small"><i class="fas fa-car" aria-hidden="true"></i></span>
                <span>{"Vehiculos"}</span>
              </a>
            </li>
            <li>
              <a>
                <span class="icon is-small"><i class="fas fa-key" aria-hidden="true"></i></span>
                <span>{"Peticiones"}</span>
              </a>
            </li>
            <li>
              <a>
                <span class="icon is-small"><i class="far fa-file-alt" aria-hidden="true"></i></span>
                <span>{"Reportes"}</span>
              </a>
            </li>
          </ul>
        </div>
    }
}
