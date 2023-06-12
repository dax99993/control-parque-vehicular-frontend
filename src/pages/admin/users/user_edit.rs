use yew::prelude::*;
use yew::platform::spawn_local;

use validator::{Validate, ValidationErrors};

use common::models::user::{Usuario, ActualizaUsuario, UsuarioRol};
use common::models::department::Departamento;

use crate::shadow_clone;
use crate::components::card::{Card, CardContent};
use crate::components::form::{Form, FormField, InputFieldValidated, SelectFieldValidated};
use crate::components::upload::pictures::{Pictures, Reducer, FileActions};
use crate::services::admin::{request_admin_update_user_picture, request_admin_update_user};

use crate::types::multipart::MultipartForm;
use crate::utils::forms::{validate_form_field, set_input_value};

use std::str::FromStr;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub usuario: UseStateHandle<Usuario>,
}


#[function_component]
pub fn AdminUserEditForm(props: &Props) -> Html {
    // Props
    let Props { usuario } = props;
    let usuario = usuario.clone();


    //States
    let usuario_form = use_state(|| ActualizaUsuario::default());
    let validacion = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let reset = use_state(|| false);
    let upload_form = use_state(|| None::<MultipartForm>);
    let upload_form_reducer = use_reducer(Reducer::default);
    

    //Node refs
    let nombres_ref = NodeRef::default();
    let apellidos_ref = NodeRef::default();
    let email_ref = NodeRef::default();
    let numero_empleado_ref = NodeRef::default();
    let activo_ref = NodeRef::default();
    let verificado_ref = NodeRef::default();
    //pub departamento: Option<i32>,
    let rol_ref = NodeRef::default();


    // Input callbacks (obtener datos del input field)
    let onchange_nombres = get_input_callback("nombres", &usuario_form);
    let onchange_apellidos = get_input_callback("apellidos", &usuario_form);
    let onchange_email = get_input_callback("email", &usuario_form);
    let onchange_numero_empleado = get_input_callback("numero_empleado", &usuario_form);
    let onchange_activo = get_input_callback("activo", &usuario_form);
    let onchange_verificado = get_input_callback("verificado", &usuario_form);
    let onchange_rol= get_input_callback("rol", &usuario_form);


    // Validation callback on input blur
    let validate_input_on_blur = {
        shadow_clone![usuario_form, validacion];
        Callback::from(move |(name, value): (String, String)| {
            set_form_field(name.as_str(), value, &usuario_form);
            validate_form_field(name.as_str(), &usuario_form, &validacion);
        })
    };


    // Hooks
    
    // Asignar valores al form del estado del prop
    {
        shadow_clone![usuario_form, validacion];
        shadow_clone![nombres_ref, apellidos_ref, email_ref, numero_empleado_ref];
        use_effect_with_deps(move |(usuario, _)| {

            // Reiniciar estados
            usuario_form.set(ActualizaUsuario::default());
            validacion.set(Rc::new(RefCell::new(ValidationErrors::new())));

            // Asignar input fields a los valores actuales del usuario
            let u = (*usuario).clone();
            set_input_value(&u.nombres, &nombres_ref);
            set_input_value(&u.apellidos, &apellidos_ref);
            set_input_value(&u.email, &email_ref);
            let numero_empleado = match u.numero_empleado {
                Some(n) => n.to_string(),
                None => String::default(),
            };
            set_input_value(&numero_empleado, &numero_empleado_ref);
        },
        (usuario.clone(), reset.clone()))
    }

    // create request to update information
    /*
    let request_actualizar_usuario = {
        shadow_clone![usuario, usuario_form];
        use_async(async move {
            log::warn!("Ejecutando peticion asincrona:\nid\n{:?}\nactualizado\n{:?}", vehiculo, vehiculo_actualiza);
            request_admin_update_vehicule((*vehiculo).vehiculo_id.to_string(), (*vehiculo_actualiza).clone()).await
        })
    };
    
    // Re-render cuando request para actualizar es ejecutada
    {
        shadow_clone!(request_update_vehicule);
        shadow_clone![vehiculo, vehiculo_actualiza, vehiculo_actualiza_validacion];
        use_effect_with_deps(move |request_update_vehicule| {
            if let Some(response) = &request_update_vehicule.data {
                log::debug!("Peticion actualizar vehiculo exitosa\n{:?}", response);
                if let Some(veh) = &response.data {
                    //let mut v = (*vehicule).clone();
                    //v.update(veh);
                    vehiculo.set(veh.clone());
                    vehiculo_actualiza.set(ActualizaVehiculo::default());
                    vehiculo_actualiza_validacion.set(Rc::new(RefCell::new(ValidationErrors::new())));
                }
            }
            if let Some(response) = &request_update_vehicule.error {
                log::error!("Peticion actualizar vehiculo fallo\n{:?}", response);
            }
        },
        request_update_vehicule.clone())
    }
    */

    // Reset todos los input field
    let onreset = {
        shadow_clone![reset];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            reset.set(!reset.deref());
        })
    };

    // Submit form validada
    let onsubmit = {
        //shadow_clone![request_actualizar_usuario];
        shadow_clone![usuario, usuario_form];
        shadow_clone![reset];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // Realizar request para actualizar vehiculo
            match usuario_form.validate() {
                Ok(_) => {
                    //request_update_vehicule.run();
                    shadow_clone![usuario, usuario_form];
                    spawn_local(async move {
                        let response = request_admin_update_user(usuario.deref().usuario_id.to_string(), usuario_form.deref().clone()).await;
                        match response {
                            Ok(api_response) => {
                                log::debug!("Peticion actulizar imagen vehiculo exitosa {:?}", &api_response);
                                //usuario.set(api_response.data.clone().unwrap());
                                if let Some(u) = api_response.data {
                                    usuario.set(u.clone());
                                }
                            }
                            Err(api_error) => {
                                log::error!("Peticion actulizar imagen vehiculo exitosa fallo {:?}", api_error);
                            }
                        }
                    });
                }
                Err(_) => {
                    reset.set(!reset.deref());
                }
            }

        })
    };


    // Subir imagen del usuario
    {
        shadow_clone![usuario];
        shadow_clone![upload_form_reducer];
        use_effect_with_deps(move |upload_form| {
            // make request to update vehicule picture
            if let Some(form) = (**upload_form).clone() {
                let id = (*usuario).usuario_id.to_string();
                log::debug!("form {:?}", form);
                let multipart = form.into_reqwest_multipart();
                let upload_form = upload_form.clone();
                spawn_local(async move {
                    let response = request_admin_update_user_picture(id, multipart).await;
                    match response {
                        Ok(api_response) => {
                            upload_form.set(None);
                            log::debug!("Peticion actulizar imagen vehiculo exitosa {:?}", api_response);
                            // should update vehicule picture to updated one
                            usuario.set(api_response.data.clone().unwrap());
                            // should reset the picture component
                            //upload_form_reducer.dispatch(FileActions::Uploaded(())
                            upload_form_reducer.dispatch(FileActions::Reset);
                        }
                        Err(api_error) => {
                            log::error!("Peticion actulizar imagen vehiculo exitosa fallo {:?}", api_error);
                        }
                    }
                });
            }
        },
        upload_form.clone()
        );
    }


    let has_errors = !(*validacion).borrow().errors().is_empty();


    //HTML
    
    {
    shadow_clone![usuario];
    html!{
    <div class="tile is-ancestor">
        <div class="tile is-parent">

            <Card classes={classes!["tile", "is-child"]}
                header_icon_left={"fa-solid fa-user"} header_title={"Editar Usuario"}
            >
                <CardContent>

                    <Form method="get">

                        <FormField label="Imagen">
                            <Pictures upload_form={upload_form.clone()} upload_reducer={upload_form_reducer.clone()}/>
                        </FormField> 

                        <hr/>

                        <FormField label="Nombres">
                            <InputFieldValidated 
                                msg="Colocar nombre economico del vehiculo"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="nombres"
                                input_ref={nombres_ref}
                                handle_onchange={onchange_nombres}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*validacion.clone()}
                            />
                        </FormField> 

                        <FormField label="Apellidos">
                            <InputFieldValidated 
                                msg="Colocar Modelo del vehiculo"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="apellidos"
                                input_ref={apellidos_ref}
                                handle_onchange={onchange_apellidos}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*validacion.clone()}
                            />
                        </FormField>

                        <FormField label="email">
                            <InputFieldValidated 
                                msg="Colocar año del vehiculo"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="email"
                                input_ref={email_ref}
                                handle_onchange={onchange_email}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*validacion.clone()}
                            />
                        </FormField>

                        <FormField label="Numero de empleado">
                            <InputFieldValidated 
                                msg="Colocar numero de empleado"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="numero_empleado"
                                input_ref={numero_empleado_ref}
                                handle_onchange={onchange_numero_empleado}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*validacion.clone()}
                            />
                        </FormField>

                        <hr/>

                        <FormField label="Rol">
                            <SelectFieldValidated 
                                options={vec![(UsuarioRol::Normal.to_string(), UsuarioRol::Normal.to_string()),
                                    (UsuarioRol::Admin.to_string(), UsuarioRol::Admin.to_string())]}
                                name="rol"
                                input_ref={rol_ref}
                                selected={Some(usuario.rol.to_string())}
                                handle_onchange={onchange_rol}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*validacion.clone()}
                            />
                        </FormField>

                        <FormField label="Activo">
                            <SelectFieldValidated 
                                options={vec![("true".into(), "Si".into()),
                                    ("false".into(), "No".into())]}
                                name="activo"
                                input_ref={activo_ref}
                                selected={Some(usuario.activo.to_string())}
                                handle_onchange={onchange_activo}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*validacion.clone()}
                            />
                        </FormField>

                        <FormField label="Verificado">
                            <SelectFieldValidated 
                                options={vec![("true".into(), "Si".into()),
                                    ("false".into(), "No".into())]}
                                name="verificado"
                                input_ref={verificado_ref}
                                selected={Some(usuario.verificado.to_string())}
                                handle_onchange={onchange_verificado}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*validacion.clone()}
                            />
                        </FormField>
                        
                        <FormField label="Ultima modificacion">
                            <div class="control is-clearfix">
                                <input type="text" readonly={true} value={ usuario.modificado_en.to_string() } class="input is-static"/>
                            </div>
                        </FormField>

                        <FormField label="Fecha de creacion">
                            <div class="control is-clearfix">
                                <input type="text" readonly={true} value={ usuario.creado_en.to_string() } class="input is-static"/>
                            </div>
                        </FormField>

                        <hr/>
                            
                        <FormField>
                            <div class="field is-grouped">
                              <div class="control">
                                <button type="submit" 
                                    onclick={onsubmit}
                                    class={classes!["button", if has_errors { "is-danger" } else { "is-primary" }]}
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
                            if has_errors {
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
    form: &UseStateHandle<ActualizaUsuario>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}




fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<ActualizaUsuario>,)
{
    let mut data = form.deref().clone();
    match name {
        "nombres" => data.nombres= Some(value),
        "apellidos" => data.apellidos= Some(value),
        "email" => data.email= Some(value),
        // Maybe need parsing
        "numero_empleado" => {data.numero_empleado =
            if let Ok(number) = value.parse::<i16>() {
                Some(number)
            } else { Some(-1) }
        }
        // Maybe need to create other function for boolean checkbox
        "activo" => {data.activo =
            if let Ok(b) = value.parse::<bool>() {
                Some(b)
            } else { None }
        }
        "verificado" => {data.verificado =
            if let Ok(b) = value.parse::<bool>() {
                Some(b)
            } else {
                None
            }
        }
        // Maybe need to create other function for dropdown list
        "rol" => {
            data.rol =
            if let Ok(rol) = UsuarioRol::from_str(value.as_str()) {
                Some(rol)
            } else {
                None
            }
        }
        _ => (),
    }
    form.set(data);
}
