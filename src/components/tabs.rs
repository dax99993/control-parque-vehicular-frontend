use yew::prelude::*;
use yew_router::prelude::*;


#[function_component]
pub fn Tabs() -> Html {
    let state = use_state(|| false);
    let onclick ={
        let state = state.clone();
        Callback::from(move |e: MouseEvent| {
            state.set(!*state); 
        })
    };
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
