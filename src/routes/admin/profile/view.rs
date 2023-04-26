use yew::prelude::*;
use yew_hooks::prelude::*;

use super::reducer::{ProfileReducer, ProfileActions};

use crate::components::main_section::MainSection;
use super::password::ChangePasswordForm;

#[function_component]
pub fn AdminProfilePage() -> Html {
    let reducer = use_reducer(ProfileReducer::default);

    //For now using user stored in context but might request again for consistency in db
    //let user = 


    html!{
        <MainSection route="Admin" subroute="Perfil" title="Editar Perfil">
            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <p class="tile">{"form"}</p>
                </div>
                <div class="tile is-parent">
                    <p class="tile">{"info"}</p>
                </div>
            </div>
            <ChangePasswordForm/>
        </MainSection>
    }
}



