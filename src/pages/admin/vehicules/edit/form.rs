use yew::prelude::*;
use yew::platform::spawn_local;
use yew_hooks::prelude::*;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use validator::{Validate, ValidationErrors};

use common::models::vehicule::{Vehiculo, ActualizaVehiculo, EstadoVehiculo};

use crate::services::admin::{request_admin_update_vehicule, request_admin_update_vehicule_picture};

use crate::shadow_clone;
use crate::components::card::{Card, CardContent};
use crate::components::form::{Form, FormField, InputFieldValidated, SelectFieldValidated};
use crate::components::upload::pictures::{Pictures, Reducer, FileActions};
use crate::components::toast::{use_toaster, Toast, ToastType, ToastPosition};

use crate::types::multipart::MultipartForm;
use crate::utils::forms::{validate_form_field, set_input_value};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditVehiculeFormProps {
    pub estado_vehiculo: UseStateHandle<Vehiculo>,
}

#[function_component]
pub fn EditVehiculeForm(props: &EditVehiculeFormProps) -> Html {
    //Context
    let toaster = use_toaster().expect("No ToastViewer");

    // States
    let EditVehiculeFormProps { estado_vehiculo } = props;
    let vehiculo = estado_vehiculo.clone();

    let upload_form = use_state(|| None::<MultipartForm>);
    let upload_form_reducer = use_reducer(Reducer::default);

    let vehiculo_actualiza = use_state(|| ActualizaVehiculo::default());  
    let vehiculo_actualiza_validacion = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    // Form Field Noderef
    let marca = NodeRef::default();
    let modelo = NodeRef::default();
    let año = NodeRef::default();
    let numero_placa = NodeRef::default();
    let nombre_economico = NodeRef::default();
    let numero_tarjeta = NodeRef::default();
    let estado = NodeRef::default();
    let activo = NodeRef::default();


    // Hooks
    // create request to update information
    let request_update_vehicule = {
        shadow_clone![vehiculo, vehiculo_actualiza];
        use_async(async move {
            request_admin_update_vehicule((*vehiculo).vehiculo_id.to_string(), (*vehiculo_actualiza).clone()).await
        })
    };

    // Update the form fields when vehicule state is done
    {
        shadow_clone![vehiculo];
        shadow_clone![marca, modelo, numero_placa, año, numero_tarjeta, nombre_economico, estado];
        use_effect_with_deps(move |vehiculo| {
            let veh = (*vehiculo).clone();
            set_input_value(&veh.marca, &marca);
            set_input_value(&veh.modelo, &modelo);
            set_input_value(&veh.año.to_string(), &año);
            set_input_value(&veh.numero_placa, &numero_placa);
            set_input_value(&veh.nombre_economico, &nombre_economico);
            set_input_value(&veh.numero_tarjeta, &numero_tarjeta);
        },
        vehiculo.clone())
    }

    // Re-render when request_update_vehicule is done and successful
    {
        shadow_clone!(request_update_vehicule);
        shadow_clone![vehiculo, vehiculo_actualiza, vehiculo_actualiza_validacion];
        shadow_clone!(toaster);
        use_effect_with_deps(move |request_update_vehicule| {
            if let Some(response) = &request_update_vehicule.data {
                log::debug!("Peticion actualizar vehiculo exitosa\n{:?}", response);
                if let Some(veh) = &response.data {
                    //let mut v = (*vehicule).clone();
                    //v.update(veh);
                    vehiculo.set(veh.clone());
                    vehiculo_actualiza.set(ActualizaVehiculo::default());
                    vehiculo_actualiza_validacion.set(Rc::new(RefCell::new(ValidationErrors::new())));
                    let toast = Toast {
                        position: ToastPosition::TopCenter,
                        body: "Actualizacion del vehiculo exitosa".into(),
                        r#type: ToastType::Success,
                        timeout: Some(chrono::Duration::milliseconds(3000)),
                    };
                    toaster.toast(toast);
                }
            }
            if let Some(response) = &request_update_vehicule.error {
                log::error!("Peticion actualizar vehiculo fallo\n{:?}", response);
            }
        },
        request_update_vehicule.clone())
    }

    // Upload imagen vehiculo as soon as image bytes are stored
    {
        shadow_clone![toaster];
        shadow_clone![vehiculo];
        shadow_clone![upload_form_reducer];
        use_effect_with_deps(move |upload_form| {
            // make request to update vehicule picture
            if let Some(form) = (**upload_form).clone() {
                let id = (*vehiculo).vehiculo_id.to_string();
                log::debug!("form {:?}", form);
                let multipart = form.into_reqwest_multipart();
                let upload_form = upload_form.clone();
                let toaster = toaster.clone();
                spawn_local(async move {
                    let response = request_admin_update_vehicule_picture(id, multipart).await;
                    match response {
                        Ok(api_response) => {
                            upload_form.set(None);
                            log::debug!("Peticion actulizar imagen vehiculo exitosa {:?}", api_response);
                            // should update vehicule picture to updated one
                            vehiculo.set(api_response.data.clone().unwrap());
                            // should reset the picture component
                            //upload_form_reducer.dispatch(FileActions::Uploaded(())
                            upload_form_reducer.dispatch(FileActions::Reset);
                            let toast = Toast {
                                position: ToastPosition::TopCenter,
                                body: "Actualizacion del imagen del vehiculo exitosa".into(),
                                r#type: ToastType::Success,
                                timeout: Some(chrono::Duration::milliseconds(3000)),
                            };
                            toaster.toast(toast);
                        }
                        Err(api_error) => {
                            log::error!("Peticion actulizar imagen vehiculo exitosa fallo {:?}", api_error);
                            let toast = Toast {
                                position: ToastPosition::TopCenter,
                                body: "Actualizacion del imagen del vehiculo fallo".into(),
                                r#type: ToastType::Danger,
                                timeout: Some(chrono::Duration::milliseconds(3000)),
                            };
                            toaster.toast(toast);
                        }
                    }
                });
            }
        },
        upload_form.clone()
        );
    }


    // Form Input Callbacks
    let onchange_marca = get_input_callback("marca", &vehiculo_actualiza);
    let onchange_modelo = get_input_callback("modelo", &vehiculo_actualiza);
    let onchange_año = get_input_callback("año", &vehiculo_actualiza);
    let onchange_numero_placa= get_input_callback("numero_placa", &vehiculo_actualiza);
    let onchange_nombre_economico = get_input_callback("nombre_economico", &vehiculo_actualiza);
    let onchange_numero_tarjeta = get_input_callback("numero_tarjeta", &vehiculo_actualiza);
    let onchange_estado = get_input_callback("estado", &vehiculo_actualiza);
    let onchange_activo = get_input_callback("activo", &vehiculo_actualiza);



    //Callbacks
    // Validate form inputs
    let validate_input_on_blur = {
        shadow_clone![vehiculo_actualiza, vehiculo_actualiza_validacion];
        Callback::from(move |(name, value): (String, String)| {
            set_form_field(name.as_str(), value, &vehiculo_actualiza);
            validate_form_field(name.as_str(), &vehiculo_actualiza, &vehiculo_actualiza_validacion);
        })
    };
    

    // reset all form fields
    let onreset = {
        shadow_clone![vehiculo_actualiza, vehiculo_actualiza_validacion];
        shadow_clone![marca, modelo, numero_placa, año, numero_tarjeta, nombre_economico];
        shadow_clone![vehiculo];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            vehiculo_actualiza.set(ActualizaVehiculo::default());
            vehiculo_actualiza_validacion.set(Rc::new(RefCell::new(ValidationErrors::new())));

            let veh = (*vehiculo).clone();
            set_input_value(&veh.marca, &marca);
            set_input_value(&veh.modelo, &modelo);
            set_input_value(&veh.año.to_string(), &año);
            set_input_value(&veh.numero_placa, &numero_placa);
            set_input_value(&veh.nombre_economico, &nombre_economico);
            set_input_value(&veh.numero_tarjeta, &numero_tarjeta);

            //TODO This does not reset the select fields!
        })
    };

    // Submit valid form
    let onsubmit = {
        shadow_clone![vehiculo_actualiza, vehiculo_actualiza_validacion];
        shadow_clone![marca, modelo, numero_placa, año, numero_tarjeta, nombre_economico];
        shadow_clone![request_update_vehicule];
        shadow_clone![vehiculo];
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // make request to update vehicule fields 
            match vehiculo_actualiza.validate() {
                Ok(_) => {
                    request_update_vehicule.run();
                }
                Err(e) => {
                    let veh = (*vehiculo).clone();
                    set_input_value(&veh.marca, &marca);
                    set_input_value(&veh.modelo, &modelo);
                    set_input_value(&veh.año.to_string(), &año);
                    set_input_value(&veh.numero_placa, &numero_placa);
                    set_input_value(&veh.nombre_economico, &nombre_economico);
                    set_input_value(&veh.numero_tarjeta, &numero_tarjeta);
                    //TODO This does not reset the select fields!
                    vehiculo_actualiza_validacion.set(Rc::new(RefCell::new(e)));
                }
            }

        })
    };


    // Variables
    let has_errors = !(*vehiculo_actualiza_validacion).borrow().errors().is_empty();


    // HTML
    {
    shadow_clone![vehiculo];
    html!{
    <div class="tile is-ancestor">
        <div class="tile is-parent">

            <Card classes={classes!["tile", "is-child"]}
                header_icon_left={"fa-solid fa-car-side"} header_title={"Editar vehiculo"}
            >
                <CardContent>

                    <Form method="get" onsubmit={onsubmit}>

                        <FormField label="Imagen">
                            <Pictures upload_form={upload_form.clone()} upload_reducer={upload_form_reducer.clone()}/>
                        </FormField> 

                        <hr/>

                        <FormField label="Marca">
                            <InputFieldValidated 
                                msg="Colocar nombre economico del vehiculo"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="marca"
                                input_ref={marca}
                                handle_onchange={onchange_marca}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*vehiculo_actualiza_validacion.clone()}
                            />
                        </FormField> 

                        <FormField label="Modelo">
                            <InputFieldValidated 
                                msg="Colocar Modelo del vehiculo"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="modelo"
                                input_ref={modelo}
                                handle_onchange={onchange_modelo}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*vehiculo_actualiza_validacion.clone()}
                            />
                        </FormField>

                        <FormField label="Año">
                            <InputFieldValidated 
                                msg="Colocar año del vehiculo"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="año"
                                input_ref={año}
                                handle_onchange={onchange_año}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*vehiculo_actualiza_validacion.clone()}
                            />
                        </FormField>

                        <FormField label="Placa">
                            <InputFieldValidated 
                                msg="Colocar placas del vehiculo"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="numero_placa"
                                input_ref={numero_placa}
                                handle_onchange={onchange_numero_placa}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*vehiculo_actualiza_validacion.clone()}
                            />
                        </FormField>

                        <FormField label="Numero de tarjeta">
                            <InputFieldValidated 
                                msg="Colocar numero de tarjeta del vehiculo"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="numero_tarjeta"
                                input_ref={numero_tarjeta}
                                handle_onchange={onchange_numero_tarjeta}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*vehiculo_actualiza_validacion.clone()}
                            />
                        </FormField>

                        <FormField label="Nombre economico">
                            <InputFieldValidated 
                                msg="Colocar nombre economico del vehiculo"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="nombre_economico"
                                input_ref={nombre_economico}
                                handle_onchange={onchange_nombre_economico}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*vehiculo_actualiza_validacion.clone()}
                            />
                        </FormField>

                        <hr/>

                        <FormField label="Estado">
                            <SelectFieldValidated 
                                options={vec![("Disponible".into(), "Disponible".into()),
                                    ("Ocupado".into(), "Ocupado".into()),
                                    ("Mantenimiento".into(), "Mantenimiento".into())]}
                                name="estado"
                                input_ref={estado}
                                selected={Some(vehiculo.estado.to_string())}
                                handle_onchange={onchange_estado}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*vehiculo_actualiza_validacion.clone()}
                            />
                        </FormField>

                        <FormField label="Activo">
                            <SelectFieldValidated 
                                options={vec![("true".into(), "Si".into()),
                                    ("false".into(), "No".into())]}
                                name="activo"
                                input_ref={activo}
                                selected={Some(vehiculo.activo.to_string())}
                                handle_onchange={onchange_activo}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*vehiculo_actualiza_validacion.clone()}
                            />
                        </FormField>

                        <hr/>
                            
                        <FormField>
                            <div class="field is-grouped">
                              <div class="control">
                                <button type="submit" 
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
    form: &UseStateHandle<ActualizaVehiculo>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}


use std::str::FromStr;

fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<ActualizaVehiculo>,)
{
    let mut data = form.deref().clone();
    match name {
        "marca" => data.marca= Some(value),
        "modelo" => data.modelo = Some(value),
        // Maybe need parsing
        "año" => data.año = if let Ok(number) = value.parse::<i16>() {Some(number)} else { Some(-1) },
        "numero_placa" => data.numero_placa= Some(value),
        "nombre_economico" => data.nombre_economico= Some(value),
        "numero_tarjeta" => data.numero_tarjeta= Some(value),
        // Maybe need to create other function for dropdown list
        "estado" => data.estado = if let Ok(estado) = EstadoVehiculo::from_str(value.as_str()) { Some(estado) } else { None },
        // Maybe need to create other function for boolean checkbox
        "activo" => data.activo = if let Ok(b) = value.parse::<bool>() {Some(b)} else { None },
        _ => (),
    }
    log::debug!("Onblur update data {:?}", &data); 
    form.set(data);
}
