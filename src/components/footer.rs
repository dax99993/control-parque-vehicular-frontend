use yew::prelude::*;




#[function_component]
pub fn Footer() -> Html {
    html! {
        <ybc::Footer>
            <ybc::Content
                classes={classes!("has-text-centered")}
            >
                <p>
                    <strong>{"CPV"}</strong>
                    {" por "}
                    <a href="http://opensource.org/licenses/mit-license.php">{ "Alguna organizacion o individuo" }</a>
                    {". El codigo pertenece a "}
                    <a href="https://cozcyt.gob.mx/labsol/"> {"LABSOL"} </a>
                    { ". El sitio web pertenece a " }
                    <a href="https://cozcyt.gob.mx/">{"CoZyT"}</a>
                    {"."}
                </p>
            </ybc::Content>
        </ybc::Footer>
    }
}
