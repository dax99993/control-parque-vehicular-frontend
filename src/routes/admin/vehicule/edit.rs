use uuid::Uuid;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use validator::{Validate, ValidationErrors};

use crate::hooks::user_context::use_user_context;
use crate::services::vehicule::{request_admin_get_vehicule_with_id, request_admin_update_vehicule, request_admin_update_vehicule_picture};
use crate::types::vehicule::{Vehicule, UpdateVehicule};

use crate::shadow_clone;
use crate::utils::forms::{validate_form_field, set_input_value};

//use crate::components::main_section::MainSection;
use crate::components::form::form::{Form, FormField, InputFieldValidated, SelectFieldValidated};
use crate::components::card::{Card, CardContent};

use crate::components::upload::pictures::Pictures;
use crate::types::multipart::{MultipartForm, MultipartPart};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditVehiculeProps {
    pub id: Uuid,
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
    let id = use_state(|| props.id.to_string());  


    let onchange_branch = get_input_callback("branch", &updated_vehicule);
    let onchange_model = get_input_callback("model", &updated_vehicule);
    let onchange_year = get_input_callback("year", &updated_vehicule);
    let onchange_number_plate= get_input_callback("number_plate", &updated_vehicule);
    let onchange_short_name = get_input_callback("short_name", &updated_vehicule);
    let onchange_number_card = get_input_callback("number_card", &updated_vehicule);
    let onchange_status = get_input_callback("status", &updated_vehicule);
    let onchange_active = get_input_callback("active", &updated_vehicule);

    let branch = NodeRef::default();
    let model = NodeRef::default();
    let year = NodeRef::default();
    let number_plate = NodeRef::default();
    let short_name = NodeRef::default();
    let number_card = NodeRef::default();
    let status = NodeRef::default();
    let active = NodeRef::default();

    let validate_input_on_blur = {
        shadow_clone![updated_vehicule, updated_vehicule_validation];
        Callback::from(move |(name, value): (String, String)| {
            set_form_field(name.as_str(), value, &updated_vehicule);
            validate_form_field(name.as_str(), &updated_vehicule, &updated_vehicule_validation);
        })
    };

    // ------- request vehicule information ------
    let request_vehicule_with_id = {
        shadow_clone!(id);
        use_async(async move {
            request_admin_get_vehicule_with_id((*id).clone()).await
        })
    };


    // Request vehicule information on rendering
    {
        shadow_clone!(request_vehicule_with_id);
        use_effect_with_deps(move |_| {
            request_vehicule_with_id.run();
        }, ())
    }

    // ------- request vehicule update information ------
    // create request to update information
    let request_update_vehicule = {
        shadow_clone![id, updated_vehicule];
        use_async(async move {
            request_admin_update_vehicule((*id).clone(), (*updated_vehicule).clone()).await
        })
    };

    // create request to update picture 
    let picture_url = format!("http://127.0.0.1:8000/api/images?filename=vehicules/default-picture.jpeg");


    // Update the form fields when request_vehicule_with_id is done
    {
        shadow_clone!(request_vehicule_with_id);
        shadow_clone!(vehicule);
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        use_effect_with_deps(move |request_vehicule_with_id| {
            if let Some(response) = &request_vehicule_with_id.data {
                log::debug!("{:?}", response);
                if let Some(veh) = &response.data {
                    vehicule.set(veh.clone()); 
                    set_input_value(&veh.branch, &branch);
                    set_input_value(&veh.model, &model);
                    set_input_value(&veh.year.to_string(), &year);
                    set_input_value(&veh.number_plate, &number_plate);
                    set_input_value(&veh.short_name, &short_name);
                    set_input_value(&veh.number_card, &number_card);
                }
            }
        },
        request_vehicule_with_id.clone())
    }

    // Re-render when request_update_vehicule is done and successful
    {
        shadow_clone!(request_update_vehicule);
        shadow_clone!(request_vehicule_with_id);
        shadow_clone![vehicule, updated_vehicule, updated_vehicule_validation];
        use_effect_with_deps(move |request_update_vehicule| {
            if let Some(response) = &request_update_vehicule.data {
                log::debug!("{:?}", response);
                /*
                if let Some(veh) = &response.data {
                    let mut v = (*vehicule).clone();
                    //v.update(updated_vehicule);
                    v.update(veh);
                    updated_vehicule.set(UpdateVehicule::default());
                    vehicule.set(v);
                }
                */
                updated_vehicule.set(UpdateVehicule::default());
                updated_vehicule_validation.set(Rc::new(RefCell::new(ValidationErrors::new())));
                request_vehicule_with_id.run();
            }
        },
        request_update_vehicule.clone())
    }


    // reset all form fields
    let onreset = {
        shadow_clone![updated_vehicule, updated_vehicule_validation];
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        shadow_clone![vehicule];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            updated_vehicule.set(UpdateVehicule::default());
            updated_vehicule_validation.set(Rc::new(RefCell::new(ValidationErrors::new())));

            let veh = (*vehicule).clone();
            set_input_value(&veh.branch, &branch);
            set_input_value(&veh.model, &model);
            set_input_value(&veh.year.to_string(), &year);
            set_input_value(&veh.number_plate, &number_plate);
            set_input_value(&veh.short_name, &short_name);
            set_input_value(&veh.number_card, &number_card);
            //TODO This does not reset the select fields!
        })
    };

    // Submit valid form
    let onsubmit = {
        shadow_clone![updated_vehicule, updated_vehicule_validation];
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        shadow_clone![vehicule];
        shadow_clone![request_update_vehicule];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            match updated_vehicule.validate() {
                Ok(_) => {
                    // make request to database
                    request_update_vehicule.run();
                }
                Err(e) => {
                    let veh = (*vehicule).clone();
                    set_input_value(&veh.branch, &branch);
                    set_input_value(&veh.model, &model);
                    set_input_value(&veh.year.to_string(), &year);
                    set_input_value(&veh.number_plate, &number_plate);
                    set_input_value(&veh.short_name, &short_name);
                    set_input_value(&veh.number_card, &number_card);
                    //TODO This does not reset the select fields!
                    updated_vehicule_validation.set(Rc::new(RefCell::new(e)));
                }
            }
        })
    };

    // Picture upload
    let upload_form = use_state(|| None::<MultipartForm>);
    {
        shadow_clone![id, upload_form];
        use_effect_with_deps(|upload_form| {

            if let Some(form) = (**upload_form).clone() {
                log::debug!("form {:?}", form);
                let multipart = form.into_reqwest_multipart();
                let upload_form = upload_form.clone();
                spawn_local(async move {
                    let response = request_admin_update_vehicule_picture((*id).clone(), multipart).await;
                    match response {
                        Ok(api_response) => {
                            upload_form.set(None);
                            log::debug!("successful vehicule picture update {:?}", api_response);
                            // should update vehicule picture to updated one
                        }
                        Err(api_error) => {
                            log::error!("upload vehicule picture failed {:?}", api_error);
                        }
                    }
                });
                
            } else {
                log::warn!("No multipart for uploading");
            }
        },
        upload_form.clone()
        );
    }


    // HTML
    {
    shadow_clone![vehicule];
    html!{
    <div class="tile is-ancestor">
        <div class="tile is-parent">

            <Card classes={classes!["tile", "is-child"]}
                header_icon_left={"fa-solid fa-car-side"} header_title={"Editar vehiculo"}
            >
                <CardContent>

                    <Form method="get">

                        <div class="is-user-avatar image has-max-width is-aligned-center">
                            <img src={picture_url} alt="John Doe" />
                        </div>

                        <hr/>

                        <FormField label="Imagen">
                            <Pictures upload_form={upload_form.clone()} />
                        </FormField> 

                        <hr/>

                        <FormField label="Id">
                            <div class="control is-clearfix">
                                <input type="text" readonly={true} value={ vehicule.vehicule_id.to_string() } class="input is-static"/>
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

                        <FormField label="Estado">
                            <SelectFieldValidated 
                                options={vec![("available".into(), "Disponible".into()),
                                    ("occupied".into(), "Ocupado".into()),
                                    ("maintenance".into(), "Mantenimiento".into())]}
                                name="status"
                                input_ref={status}
                                selected={Some(vehicule.status.clone())}
                                handle_onchange={onchange_status}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*updated_vehicule_validation.clone()}
                            />
                        </FormField>

                        <FormField label="Activo">
                            <SelectFieldValidated 
                                options={vec![("true".into(), "Si".into()),
                                    ("false".into(), "No".into())]}
                                name="active"
                                input_ref={active}
                                selected={Some(vehicule.active.to_string())}
                                handle_onchange={onchange_active}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*updated_vehicule_validation.clone()}
                            />
                        </FormField>

                        
                        <FormField label="Ultima modificacion">
                            <div class="control is-clearfix">
                                <input type="text" readonly={true} value={ vehicule.updated_at.to_string() } class="input is-static"/>
                            </div>
                        </FormField>

                        <FormField label="Fecha de creacion">
                            <div class="control is-clearfix">
                                <input type="text" readonly={true} value={ vehicule.created_at.to_string() } class="input is-static"/>
                            </div>
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

    </div>
    }
    }
}

fn get_input_callback(
    name: &'static str,
    form: &UseStateHandle<UpdateVehicule>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}

fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<UpdateVehicule>,)
{
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
        "active" => data.active = if let Ok(b) = value.parse::<bool>() {Some(b)} else { None },
        _ => (),
    }
    log::debug!("Onblur update data {:?}", &data); 
    form.set(data);
}
