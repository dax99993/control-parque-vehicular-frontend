use yew::prelude::*;

use crate::components::card::{Card, CardContent};
use super::form::VehiculeRegisterForm;
use crate::hooks::user_context::use_user_context;
use crate::layout::main_section::MainSection;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    //pub vehicule_dispatcher: UseReducerDispatcher<VehiculeReducer>,
}


#[function_component]
pub fn RegisterVehiculeView(_props: &Props) -> Html {
    // Context
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }


    html!{
    <MainSection route="Admin" subroute="Vehiculos" title="Agregar Vehiculo">
        <Card header_icon_left={ "fa-solid fa-ballot" } header_title={ "Registro de Vehiculo" }>
            <CardContent>
                <VehiculeRegisterForm/>
            </CardContent>
        </Card>
    </MainSection>
    }
}
