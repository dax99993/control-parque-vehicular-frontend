use yew::prelude::*;
use yew_hooks::prelude::*;

use web_sys::HtmlInputElement;

use crate::hooks::user_context::use_user_context;
use crate::components::main_section::MainSection;
use crate::components::form::form::{Form, FormField, TextInputField};
use crate::components::card::{Card, CardContent};

use crate::services::vehicule::{request_admin_get_vehicule_with_id, request_admin_update_vehicule};
use crate::types::vehicule::{Vehicule, UpdateVehicule};

use crate::{oninput_macro, shadow_clone};
use crate::utils::FormFieldState;

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
    // States
    let updated_vehicule = use_state(|| UpdateVehicule::default());  
    let updated_vehicule_valid = use_state(|| bool::default());  

    let vehicule = use_state(|| Vehicule::default());  

    let branch = use_state(|| FormFieldState::default());
    let oninput_branch = oninput_macro!(branch, validate);

    let model = use_state(|| FormFieldState::default());
    let oninput_model = oninput_macro!(model, validate);

    let year = use_state(|| FormFieldState::default());
    let oninput_year = oninput_macro!(year, validate_number);

    let number_plate = use_state(|| FormFieldState::default());
    let oninput_number_plate = oninput_macro!(number_plate, validate);

    let short_name = use_state(|| FormFieldState::default());
    let oninput_short_name = oninput_macro!(short_name, validate);

    let number_card= use_state(|| FormFieldState::default());
    let oninput_number_card = oninput_macro!(number_card, validate);

    // ------- request vehicule information ------
    let request_vehicule_with_id = {
        shadow_clone!(props);
        use_async(async {
            request_admin_get_vehicule_with_id(props.id).await
        })
    };

    // Request vehicule information on rendering
    {
        shadow_clone!(request_vehicule_with_id);
        use_effect_with_deps(move |_| {
            request_vehicule_with_id.run();
        }, ())
    }

    {
        shadow_clone!(vehicule, request_vehicule_with_id);
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        use_effect_with_deps(move |request_vehicule_with_id| {
            if let Some(response) = &request_vehicule_with_id.data {
                log::debug!("{:?}", response);
                if let Some(veh) = &response.data {
                    vehicule.set(veh.clone()); 

                    let mut tmp = (*branch).clone();
                    tmp.set_value(veh.branch.clone());
                    branch.set(tmp);
                    
                    let mut tmp = (*model).clone();
                    tmp.set_value(veh.model.clone());
                    model.set(tmp);

                    let mut tmp = (*year).clone();
                    tmp.set_value(veh.year.to_string().clone());
                    year.set(tmp);

                    let mut tmp = (*number_plate).clone();
                    tmp.set_value(veh.number_plate.clone());
                    number_plate.set(tmp);

                    let mut tmp = (*number_card).clone();
                    tmp.set_value(veh.number_card.clone());
                    number_card.set(tmp);

                    let mut tmp = (*short_name).clone();
                    tmp.set_value(veh.short_name.clone());
                    short_name.set(tmp);
                }
            }
        },
        request_vehicule_with_id.clone())
    }

    // ------- request vehicule update information ------
    // create request to update information
    let request_update_vehicule = {
        shadow_clone![props, updated_vehicule];
        use_async(async move {
            request_admin_update_vehicule(props.id, (*updated_vehicule).clone()).await
        })
    };
    // create request to update picture 

    let picture_url = format!("http://127.0.0.1:8000/api/images?filename=vehicules/default-picture.jpeg");

    {
    let props = props.clone();
    shadow_clone![props, vehicule];
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

                                <FormField label="Marca">
                                    <TextInputField 
                                        error_msg="Campo Obligatorio"
                                        oninput={oninput_branch.clone()}
                                        value={(*branch).value.clone()}
                                        has_error={(*branch).valid}
                                    />
                                </FormField> 

                                <FormField label="Modelo">
                                    <TextInputField 
                                        placeholder="e.g. Leaf"
                                        error_msg="Campo Obligatorio"
                                        oninput={oninput_model.clone()}
                                        value={(*model).value.clone()}
                                        has_error={(*model).valid}
                                    />
                                </FormField>

                                <FormField label="Año">
                                    <TextInputField 
                                        placeholder="e.g. 2016"
                                        error_msg="Campo Obligatorio"
                                        oninput={oninput_year.clone()}
                                        value={(*year).value.clone()}
                                        has_error={(*year).valid}
                                    />
                                </FormField>

                                <FormField label="Placa">
                                    <TextInputField 
                                        placeholder="e.g. ABCD XYZ 123"
                                        error_msg="Campo Obligatorio"
                                        oninput={oninput_number_plate.clone()}
                                        value={(*number_plate).value.clone()}
                                        has_error={(*number_plate).valid}
                                    />
                                </FormField>

                                <FormField label="Numero de tarjeta">
                                    <TextInputField 
                                        placeholder="e.g. 12345678asd"
                                        error_msg="Campo Obligatorio"
                                        oninput={oninput_number_card.clone()}
                                        value={(*number_card).value.clone()}
                                        has_error={(*number_card).valid}
                                        icon_left={"fa-solid fa-address-card"}
                                    />
                                </FormField>

                                <FormField label="Nombre economico">
                                    <TextInputField 
                                        placeholder="e.g. Leaf 202"
                                        error_msg="Campo Obligatorio"
                                        oninput={oninput_short_name.clone()}
                                        value={(*short_name).value.clone()}
                                        has_error={(*short_name).valid}
                                    />
                                </FormField>


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
                                <img src={picture_url} alt="John Doe" />
                            </div>

                            <hr/>

                            { vehicule_to_readonly_form((*vehicule).clone()) }

                        </CardContent>
                    </Card>


                </div>
            </div>

        </MainSection>
    }
    }
}


fn vehicule_to_readonly_form(vehicule: Vehicule) -> Html {
    html!{
        <Form method="get">

            <div class="field">
                <label class="label">{ "id" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.vehicule_id.to_string() } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Marca" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.branch } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Modelo" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.model } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "A;o" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.year.to_string()  } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Placa" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.number_plate  } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Nombre economico" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.short_name  } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Numero de tarjeta" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.number_card  } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Estado" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.status  } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Activo" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.active.to_string()  } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Ultima modificacion" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.updated_at.to_string()  } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Fecha de creacion" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.created_at.to_string()  } class="input is-static"/>
                </div>
            </div>

        </Form>
    }

}

fn validate(s: String) -> bool {
    !s.is_empty()
}

fn validate_number(s: String) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}
