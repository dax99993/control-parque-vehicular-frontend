use yew::prelude::*;
use yew_hooks::prelude::*;

use web_sys::HtmlInputElement;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use validator::{Validate, ValidationErrors};

use crate::hooks::user_context::use_user_context;
use crate::services::vehicule::{request_admin_get_vehicule_with_id, request_admin_update_vehicule};
use crate::types::vehicule::{Vehicule, UpdateVehicule};

use crate::shadow_clone;

use crate::components::main_section::MainSection;
use crate::components::form::form::{Form, FormField, InputFieldValidated};
use crate::components::card::{Card, CardContent};


fn get_input_callback(
    name: &'static str,
    form: UseStateHandle<UpdateVehicule>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = form.deref().clone();
        match name {
            "branch" => data.branch = Some(value),
            "model" => data.model = Some(value),
            // Maybe need parsing
            "year" => data.year = if let Ok(number) = value.parse::<i16>() {Some(number)} else { Some(-1) },
            "number_plate" => data.number_plate = Some(value),
            "short_name" => data.short_name = Some(value),
            "number_card" => data.number_card = Some(value),
            // Maybe need to create other function for dropdown list
            "status" => data.status = Some(value),
            // Maybe need to create other function for boolean checkbox
            //"active" => data.active= if let Ok(number) = value.parse::<i16>() {number} else { -1 },
            _ => (),
        }
        form.set(data);
    })
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditVehiculeProps {
    pub id: String,
}

#[function_component]
pub fn EditVehiculeView(props: &EditVehiculeProps) -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }
    // States
    let vehicule = use_state(|| Vehicule::default());  
    let updated_vehicule = use_state(|| UpdateVehicule::default());  
    let updated_vehicule_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));


    let onchange_branch = get_input_callback("branch", updated_vehicule.clone());
    let onchange_model = get_input_callback("model", updated_vehicule.clone());
    let onchange_year = get_input_callback("year", updated_vehicule.clone());
    let onchange_number_plate= get_input_callback("number_plate", updated_vehicule.clone());
    let onchange_short_name = get_input_callback("short_name", updated_vehicule.clone());
    let onchange_number_card = get_input_callback("number_card", updated_vehicule.clone());

    let branch = NodeRef::default();
    let model = NodeRef::default();
    let year = NodeRef::default();
    let number_plate = NodeRef::default();
    let short_name = NodeRef::default();
    let number_card = NodeRef::default();

    let validate_input_on_blur = {
        shadow_clone![updated_vehicule, updated_vehicule_validation];
        Callback::from(move |(name, value): (String, String)| {
            let mut data = updated_vehicule.deref().clone();
            match name.as_str() {
                "branch" => data.branch = Some(value),
                "model" => data.model = Some(value),
                // Maybe need parsing
                "year" => data.year = if let Ok(number) = value.parse::<i16>() {Some(number)} else { Some(-1) },
                "number_plate" => data.number_plate = Some(value),
                "short_name" => data.short_name = Some(value),
                "number_card" => data.number_card = Some(value),
                _ => (),
            }
            log::debug!("Onblur login data {:?}", &data); 
            updated_vehicule.set(data);

            match updated_vehicule.validate() {
                Ok(_) => {
                    updated_vehicule_validation 
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    log::debug!("Onblur update vehicule validation ok {:?}", &updated_vehicule_validation); 
                }
                Err(errors) => {
                    for(field_name, error) in errors.errors() {
                        if field_name == &name {
                            updated_vehicule_validation 
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                            log::debug!("Onblur update vehicule validation errors {:?}", &updated_vehicule_validation); 
                        }
                    }

                }
            }
        })
    };

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
        shadow_clone!(request_vehicule_with_id);
        shadow_clone!(vehicule, updated_vehicule);
        use_effect_with_deps(move |request_vehicule_with_id| {
            if let Some(response) = &request_vehicule_with_id.data {
                log::debug!("{:?}", response);
                if let Some(veh) = &response.data {
                    vehicule.set(veh.clone()); 

                    /*
                    let mut tmp_veh = (*updated_vehicule).clone();
                    
                    tmp_veh.branch = veh.branch.clone();
                    tmp_veh.model = veh.model.clone();
                    tmp_veh.year = veh.year.clone();;
                    tmp_veh.number_plate = veh.number_plate.clone();
                    tmp_veh.number_card = veh.number_card.clone();
                    tmp_veh.short_name = veh.short_name.clone();
                    tmp_veh.status = veh.status.clone();
                    tmp_veh.active= veh.active.clone();
                    */
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

    
    // reset all form fields
    let onreset = {
        shadow_clone![updated_vehicule, updated_vehicule_validation];
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            updated_vehicule.set(UpdateVehicule::default());
            updated_vehicule_validation.set(Rc::new(RefCell::new(ValidationErrors::new())));

            let branch = if let Some(element) = branch.cast::<HtmlInputElement>() { element }
            else {
                return;
            };
            let model = if let Some(element) = model.cast::<HtmlInputElement>() { element }
            else {
                return;
            };
            let year = if let Some(element) = year.cast::<HtmlInputElement>() { element }
            else {
                return;
            };
            let number_plate = if let Some(element) = number_plate.cast::<HtmlInputElement>() { element }
            else {
                return;
            };
            let short_name = if let Some(element) = short_name.cast::<HtmlInputElement>() { element }
            else {
                return;
            };
            let number_card = if let Some(element) = number_card.cast::<HtmlInputElement>() { element }
            else {
                return;
            };

            branch.set_value("");
            model.set_value("");
            year.set_value("");
            number_plate.set_value("");
            short_name.set_value("");
            number_card.set_value("");
        })
    };

    // Submit valid form
    let onsubmit = {
        shadow_clone![updated_vehicule, updated_vehicule_validation];
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        shadow_clone![request_update_vehicule];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            match updated_vehicule.validate() {
                Ok(_) => {
                    let branch = if let Some(element) = branch.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };
                    let model_element = if let Some(element) = model.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };
                    let year = if let Some(element) = year.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };
                    let number_plate = if let Some(element) = number_plate.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };
                    let short_name = if let Some(element) = short_name.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };
                    let number_card = if let Some(element) = number_card.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };
                    
                    branch.set_value("");
                    model_element.set_value("");
                    year.set_value("");
                    number_plate.set_value("");
                    short_name.set_value("");
                    number_card.set_value("");

                    // make request to database
                    request_update_vehicule.run();
                }
                Err(e) => {
                    updated_vehicule_validation.set(Rc::new(RefCell::new(e)));
                }
            }
        })
    };

    // HTML
    {
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
                                    <InputFieldValidated 
                                        placeholder={vehicule.deref().branch.clone()}
                                        msg="Colocar nombre economico del vehiculo"
                                        icon_right={"fa-solid fa-triangle-exclamation"}
                                        name="branch"
                                        input_ref={branch}
                                        handle_onchange={onchange_branch}
                                        handle_on_input_blur={validate_input_on_blur.clone()}
                                        errors={&*updated_vehicule_validation.clone()}
                                    />
                                </FormField> 

                                <FormField label="Modelo">
                                    <InputFieldValidated 
                                        msg="Colocar Modelo del vehiculo"
                                        icon_right={"fa-solid fa-triangle-exclamation"}
                                        name="model"
                                        input_ref={model}
                                        handle_onchange={onchange_model}
                                        handle_on_input_blur={validate_input_on_blur.clone()}
                                        errors={&*updated_vehicule_validation.clone()}
                                    />
                                </FormField>

                                <FormField label="Año">
                                    <InputFieldValidated 
                                        msg="Colocar año del vehiculo"
                                        icon_right={"fa-solid fa-triangle-exclamation"}
                                        name="year"
                                        input_ref={year}
                                        handle_onchange={onchange_year}
                                        handle_on_input_blur={validate_input_on_blur.clone()}
                                        errors={&*updated_vehicule_validation.clone()}
                                    />
                                </FormField>

                                <FormField label="Placa">
                                    <InputFieldValidated 
                                        msg="Colocar placas del vehiculo"
                                        icon_right={"fa-solid fa-triangle-exclamation"}
                                        name="number_plate"
                                        input_ref={number_plate}
                                        handle_onchange={onchange_number_plate}
                                        handle_on_input_blur={validate_input_on_blur.clone()}
                                        errors={&*updated_vehicule_validation.clone()}
                                    />
                                </FormField>

                                <FormField label="Numero de tarjeta">
                                    <InputFieldValidated 
                                        msg="Colocar numero de tarjeta del vehiculo"
                                        icon_right={"fa-solid fa-triangle-exclamation"}
                                        name="number_card"
                                        input_ref={number_card}
                                        handle_onchange={onchange_number_card}
                                        handle_on_input_blur={validate_input_on_blur.clone()}
                                        errors={&*updated_vehicule_validation.clone()}
                                    />
                                </FormField>

                                <FormField label="Nombre economico">
                                    <InputFieldValidated 
                                        msg="Colocar nombre economico del vehiculo"
                                        icon_right={"fa-solid fa-triangle-exclamation"}
                                        name="short_name"
                                        input_ref={short_name}
                                        handle_onchange={onchange_short_name}
                                        handle_on_input_blur={validate_input_on_blur.clone()}
                                        errors={&*updated_vehicule_validation.clone()}
                                    />
                                </FormField>

                                <hr/>

                                <FormField>
                                    <div class="field is-grouped">
                                      <div class="control">
                                        <button type="submit" 
                                            onclick={onsubmit}
                                            class={classes!["button", if (*updated_vehicule_validation).borrow().errors().is_empty() { "is-primary"} else { "is-danger" }]}
                                        >
                                          <span>{ "Actualizar" }</span>
                                        </button>
                                      </div>
                                      <div class="control">
                                        <button type="button" class="button is-primary is-outlined" onclick={onreset}>
                                          <span>{ "Reiniciar campos" }</span>
                                        </button>
                                      </div>
                                    </div>
                                    if !(*updated_vehicule_validation).borrow().errors().is_empty() {
                                        <p class="help is-danger">{ "Rellenar o corregir los campos" }</p>
                                    }
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

                            { vehicule_to_readonly_form( &(*vehicule) ) }

                        </CardContent>
                    </Card>


                </div>
            </div>

        </MainSection>
    }
    }
}


fn vehicule_to_readonly_form(vehicule: &Vehicule) -> Html {
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
                    <input type="text" readonly={true} value={ vehicule.branch.clone() } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Modelo" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.model.clone() } class="input is-static"/>
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
                    <input type="text" readonly={true} value={ vehicule.number_plate.clone()  } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Nombre economico" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.short_name.clone()  } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Numero de tarjeta" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.number_card.clone()  } class="input is-static"/>
                </div>
            </div>

            <div class="field">
                <label class="label">{ "Estado" }</label>
                <div class="control is-clearfix">
                    <input type="text" readonly={true} value={ vehicule.status.clone()  } class="input is-static"/>
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

