use yew::prelude::*;

use crate::hooks::user_context::use_user_context;
use crate::components::main_section::MainSection;
use crate::components::form::form::{Form, FormField, FormInputField};
use crate::components::card::{Card, CardContent};


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
    // TODO: request vehiculo information
    // create request to update information
    // create request to update picture 


    let props = props.clone();
    html!{
        <MainSection route="Admin" subroute="Vehiculos" title="Editar Vehiculo">
            <div class="tile is-ancestor">
                <div class="tile is-parent">

                    <Card classes={classes!["tile", "is-child"]}
                        header_icon_left={"fa-solid fa-car-side"} header_title={"Editar vehiculo"}
                    >
                        <CardContent>
                            <Form method="get">

                                <FormField label="Imagen">
                                    <div class="field file">
                                        <label class="upload control">
                                            <a class="button is-primary">
                                                  <span class="icon"><i class="fa-solid fa-upload"></i></span>
                                                  <span>{ "Selecciona archivo" }</span>
                                            </a>
                                            <input type="file" />
                                        </label>
                                    </div>
                                </FormField> 

                                <hr/>

                                <p>{ props.clone().id }</p>
                            </Form>
                        </CardContent>
                    </Card>

                </div>

                <div class="tile is-parent">

                    <Card classes={classes!["tile", "is-child"]}
                        header_icon_left={"fa-solid fa-car"} header_title={"Vehiculo"}
                    >
                        <CardContent>
                            <div class="is-user-avatar image has-max-width is-aligned-center">
                                <img src="https://avatars.dicebear.com/v2/initials/john-doe.svg" alt="John Doe" />
                            </div>

                            <hr/>

                            <Form method="get">
                                <p>{ props.id }</p>
                            </Form>
                        </CardContent>
                    </Card>

                </div>
            </div>

        </MainSection>
    }
}
