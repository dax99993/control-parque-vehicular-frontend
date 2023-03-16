use yew::prelude::*;


#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer class="footer">
            <div class="container-fluid">
                <div class="level">
                    <div class="level-item">
                    <p>
                        <strong>{"CPV"}</strong>
                        {" por "}
                        <a href="http://opensource.org/licenses/mit-license.php">{ "Alguna organizacion o individuo" }</a>
                    </p>
                    </div>
                    <div class="level-item">
                    <p>
                        {"El codigo pertenece a "}
                        <a href="https://cozcyt.gob.mx/labsol/"> {"LABSOL"} </a>
                        {"."}
                    </p>
                    </div>
                    <div class="level-item">
                    <p>
                        { "El sitio web pertenece a " }
                        <a href="https://cozcyt.gob.mx/">{"CoZCyT"}</a>
                        {"."}
                    </p>
                    </div>
                </div>
            </div>
        </footer>
    }
}
