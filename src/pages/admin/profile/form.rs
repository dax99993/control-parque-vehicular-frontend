/*
use yew::prelude::*;
use yew::platform::spawn_local;
use yew_hooks::prelude::*;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use validator::{Validate, ValidationErrors};

use common::models::user::{User, UpdateUser};

//use crate::services::vehicule::{request_admin_update_vehicule, request_admin_update_vehicule_picture};
//use super::super::services::{request_admin_update_vehicule, request_admin_update_vehicule_picture};

use crate::shadow_clone;
use crate::components::card::{Card, CardContent};
use crate::components::form::{Form, FormField, InputFieldValidated, SelectFieldValidated};
use crate::components::upload::pictures::{Pictures, Reducer, FileActions};

use crate::types::multipart::MultipartForm;
use crate::utils::forms::{validate_form_field, set_input_value};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditAdminProfileFormProps {
    pub user: UseStateHandle<User>,
}

#[function_component]
pub fn EditAdminProfileForm(props: &EditAdminProfileFormProps) -> Html {
    // States
    let EditAdminProfileFormProps { user } = props;

    let upload_form = use_state(|| None::<MultipartForm>);
    let upload_form_reducer = use_reducer(Reducer::default);

    let updated_user = use_state(|| UpdateUser::default());  
    let updated_user_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let onchange_first_name = get_input_callback("first_name", &updated_user);
    let onchange_last_name = get_input_callback("last_name", &updated_user);
    let onchange_email = get_input_callback("email", &updated_user);
    let onchange_employee_number = get_input_callback("employee_number", &updated_user);
    let onchange_active = get_input_callback("active", &updated_user);
    let onchange_verified = get_input_callback("verified", &updated_user);
    let onchange_department = get_input_callback("department", &updated_user);
    let onchange_role = get_input_callback("role", &updated_user);

    let first_name = NodeRef::default();
    let last_name = NodeRef::default();
    let email = NodeRef::default();
    let employee_number = NodeRef::default();
    let active = NodeRef::default();
    let verified = NodeRef::default();
    let department = NodeRef::default();
    let role = NodeRef::default();


    let validate_input_on_blur = {
        shadow_clone![updated_user, updated_user_validation];
        Callback::from(move |(name, value): (String, String)| {
            set_form_field(name.as_str(), value, &updated_user);
            validate_form_field(name.as_str(), &updated_user, &updated_user_validation);
        })
    };


    // ------- request vehicule update information ------
    // create request to update information
    let request_update_user = {
        shadow_clone![user , updated_user];
        use_async(async move {
            log::warn!("executing async request:\nid\n{:?}\nupdated user\n{:?}", user, updated_user);
            request_admin_update_user((*user).user_id.to_string(), (*updated_user).clone()).await
        })
    };


    // Update the form fields when vehicule state is done
    {
        shadow_clone![first_name, last_name, email, employee_number, active, verified, department, role];
        use_effect_with_deps(move |user| {
            let user = (*user).clone();
            set_input_value(&user.first_name, &first_name);
            set_input_value(&user.last_name, &last_name);
            set_input_value(&user.email, &email);
            set_input_value(&user.employee_number, &employee_number);
            set_input_value(&user.active.to_string(), &active);
            set_input_value(&user.verified.to_string(), &verified);
            set_input_value(&user.department, &department);
            set_input_value(&user.role, &role);
        },
        user.clone())
    }

    // Re-render when request_update_vehicule is done and successful
    {
        shadow_clone!(request_update_vehicule);
        shadow_clone![vehicule, updated_user, updated_vehicule_validation];
        use_effect_with_deps(move |request_update_vehicule| {
            if let Some(response) = &request_update_vehicule.data {
                log::debug!("update vehicule request successful\n{:?}", response);
                if let Some(veh) = &response.data {
                    //let mut v = (*vehicule).clone();
                    //v.update(veh);
                    vehicule.set(veh.clone());
                    updated_user.set(UpdateVehicule::default());
                    updated_user_validation.set(Rc::new(RefCell::new(ValidationErrors::new())));
                }
            }
            if let Some(response) = &request_update_vehicule.error {
                log::error!("update vehicule request failed\n{:?}", response);
            }
        },
        request_update_vehicule.clone())
    }


    // Picture upload
    /*
    {
        //shadow_clone![id, upload_form];
        let id = (*vehicule).vehicule_id.to_string();
        shadow_clone![vehicule];
        use_effect_with_deps(move |upload_form| {
            if let Some(form) = (**upload_form).clone() {
                log::debug!("form {:?}", form);
                let multipart = form.into_reqwest_multipart();
                let upload_form = upload_form.clone();
                spawn_local(async move {
                    let response = request_admin_update_vehicule_picture(id, multipart).await;
                    match response {
                        Ok(api_response) => {
                            upload_form.set(None);
                            log::debug!("successful vehicule picture update {:?}", api_response);
                            // should update vehicule picture to updated one
                            vehicule.set(api_response.data.clone().unwrap());
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
    */


    // reset all form fields
    let onreset = {
        shadow_clone![updated_user, updated_vehicule_validation];
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        shadow_clone![vehicule];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            updated_user.set(UpdateVehicule::default());
            updated_user_validation.set(Rc::new(RefCell::new(ValidationErrors::new())));

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
        shadow_clone![updated_user, updated_vehicule_validation];
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        shadow_clone![request_update_vehicule];
        shadow_clone![vehicule];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            // make request to update vehicule fields 
            match updated_user.validate() {
                Ok(_) => {
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
                    updated_user_validation.set(Rc::new(RefCell::new(e)));
                }
            }

        })
    };

    {
        shadow_clone![vehicule];
        shadow_clone![upload_form_reducer];
        use_effect_with_deps(move |upload_form| {
            // make request to update vehicule picture
            if let Some(form) = (**upload_form).clone() {
                let id = (*vehicule).vehicule_id.to_string();
                log::debug!("form {:?}", form);
                let multipart = form.into_reqwest_multipart();
                let upload_form = upload_form.clone();
                spawn_local(async move {
                    let response = request_admin_update_vehicule_picture(id, multipart).await;
                    match response {
                        Ok(api_response) => {
                            upload_form.set(None);
                            log::debug!("successful vehicule picture update {:?}", api_response);
                            // should update vehicule picture to updated one
                            vehicule.set(api_response.data.clone().unwrap());
                            // should reset the picture component
                            //upload_form_reducer.dispatch(FileActions::Uploaded(())
                            upload_form_reducer.dispatch(FileActions::Reset);
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

                        <FormField label="Imagen">
                            <Pictures upload_form={upload_form.clone()} upload_reducer={upload_form_reducer.clone()}/>
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
                                errors={&*updated_user_validation.clone()}
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
                                errors={&*updated_user_validation.clone()}
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
                                errors={&*updated_user_validation.clone()}
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
                                errors={&*updated_user_validation.clone()}
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
                                errors={&*updated_user_validation.clone()}
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
                                errors={&*updated_user_validation.clone()}
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
                                errors={&*updated_user_validation.clone()}
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
                                errors={&*updated_user_validation.clone()}
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
                                    class={classes!["button", if (*updated_user_validation).borrow().errors().is_empty() { "is-primary"} else { "is-danger" }]}
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
                            if !(*updated_user_validation).borrow().errors().is_empty() {
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
    form: &UseStateHandle<UpdateUser>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}

fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<UpdateUser>,)
{
    let mut data = form.deref().clone();
    match name {
        "first_name" => data.first_name = Some(value),
        "last_name" => data.last_name = Some(value),
        //"employee_number" => data.employee_number = if let Ok(number) = value.parse::<i16>() {Some(number)} else { Some(-1) },
        "employee_number" => data.employee_number = value.parse::<i16>().ok(),
        "active" => data.active = value.parse::<bool>().ok(),
        "verified" => data.verified = value.parse::<bool>().ok(),
        "email" => data.email = Some(value),
        // Maybe need to create other function for dropdown list
        "department" => data.department = value.parse::<i32>().ok(),
        // Maybe need to create other function for boolean checkbox
        "role" => data.role = Some(value),
        _ => (),
    }
    log::debug!("Onblur update data {:?}", &data); 
    form.set(data);
}
*/
