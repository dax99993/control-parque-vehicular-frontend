use yew::prelude::*;

use crate::hooks::user_context::use_user_context;
use crate::components::main_section::MainSection;
use crate::components::form::form::{Form, FormField, FormInputField};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditVehiculeProps {
    pub id: String,
}

#[function_component]
pub fn EditVehicule(props: &EditVehiculeProps) -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }


    let props = props.clone();
    html!{
        <MainSection route="Admin" subroute="Vehiculos" title="Editar Vehiculo">

            <div class="card">
                <header class="card-header">
                    <p class="card-header-title">
                        <span class="icon"><i class="fa-solid fa-ballot"></i></span>
                        { "Editar registro" }
                    </p>
                </header>

                <div class="card-content">
                    <Form method="get">
                        <p>{ props.id }</p>
                    </Form>
                </div>
            </div>

        </MainSection>
    }
}
