use yew::prelude::*;
//use yew_hooks::prelude::*;


use super::info::AdminProfileInfo;
use super::change_password::ChangePassword;

use crate::hooks::user_context::use_user_context;
use crate::layout::main_section::MainSection;

#[function_component]
pub fn AdminProfileView() -> Html {
    // Context
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }

    let user = use_state(|| user_ctx.get_user().unwrap());
    //let reducer = use_reducer(ProfileReducer::default);

    //For now using user stored in context but might request again for consistency in db
    //let user = 


    html!{
        <MainSection route="Admin" subroute="Perfil" title="Editar Perfil">
            <div class="tile is-ancestor">
                <div class="tile is-parent">
                    <p class="tile">{"form"}</p>
                </div>
                <div class="tile is-parent">
                    <AdminProfileInfo user_state={user.clone()}/>
                </div>
            </div>
            <ChangePassword/>
        </MainSection>
    }
}



