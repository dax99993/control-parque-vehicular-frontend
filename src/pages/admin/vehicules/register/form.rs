use yew::prelude::*;
use yew_hooks::prelude::*;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use validator::{Validate, ValidationErrors};
use common::models::vehicule::NuevoVehiculo;

use crate::shadow_clone;
use crate::hooks::user_context::use_user_context;
use crate::components::toast::{Toast, ToastType, ToastPosition, use_toaster};
use crate::services::admin::request_admin_create_vehicule;
use crate::routes::AppRoute;
use crate::utils::forms::{validate_form_field, reset_input};



#[function_component]
pub fn VehiculeRegisterForm() -> Html {
    //Context
    let user_ctx = use_user_context();
    let toaster = use_toaster().expect("No ToastViewer");
    log::debug!("toaster = {:?}", toaster);

    //States
    let vehiculo_nuevo = use_state(|| NuevoVehiculo::default());
    let vehiculo_nuevo_validacion = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    // Noderef
    let marca = NodeRef::default();
    let modelo = NodeRef::default();
    let año = NodeRef::default();
    let numero_placa = NodeRef::default();
    let nombre_economico = NodeRef::default();
    let numero_tarjeta = NodeRef::default();

    //Callbacks
    let onchange_marca= get_input_callback("marca", &vehiculo_nuevo);
    let onchange_modelo= get_input_callback("modelo", &vehiculo_nuevo);
    let onchange_año= get_input_callback("año", &vehiculo_nuevo);
    let onchange_numero_placa= get_input_callback("numero_placa", &vehiculo_nuevo);
    let onchange_nombre_economico= get_input_callback("nombre_economico", &vehiculo_nuevo);
    let onchange_numero_tarjeta= get_input_callback("numero_tarjeta", &vehiculo_nuevo);

    
    let validate_input_on_blur = {
        shadow_clone![vehiculo_nuevo, vehiculo_nuevo_validacion];
        Callback::from(move |(name, value): (String, String)| {
            set_form_field(name.as_str(), value, &vehiculo_nuevo);
            validate_form_field(name.as_str(), &vehiculo_nuevo, &vehiculo_nuevo_validacion);
        })
    };

    
    let request_create_vehicule_admin = {
        shadow_clone!(vehiculo_nuevo);
        use_async(async move {
            request_admin_create_vehicule((*vehiculo_nuevo).clone()).await
        })
    };


    // Submit valid form
    let onsubmit = {
        shadow_clone![vehiculo_nuevo, vehiculo_nuevo_validacion];
        shadow_clone![request_create_vehicule_admin];
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            match vehiculo_nuevo.validate() {
                Ok(_) => {
                    // make request to database
                    request_create_vehicule_admin.run();
                }
                Err(e) => {
                    vehiculo_nuevo_validacion.set(Rc::new(RefCell::new(e)));
                }
            }
        })
    };

    {
        shadow_clone![request_create_vehicule_admin];
        shadow_clone![toaster];
        use_effect_with_deps(move |request_create_vehicule_admin| {
            if let Some(response) = &request_create_vehicule_admin.data {
                log::debug!("Respuesta api\n{:?}", &response);
                if let Some(vehiculo) = &response.data {
                    log::debug!("Creacion de vehiculo exitosa\n{:?}", vehiculo);
                    user_ctx.redirect_to(AppRoute::VehiculeEdit { id: vehiculo.vehiculo_id.clone() });
                    let toast = Toast {
                        position: ToastPosition::TopCenter,
                        r#type: ToastType::Success,
                        body: "Vehiculo registrado!".into(),
                        timeout: Some(chrono::Duration::milliseconds(2000)),
                    };
                    toaster.toast(toast);
                }
            }
            if let Some(api_error) = &request_create_vehicule_admin.error {
                log::error!("Peticion para crear vehiculo fallo\n{:?}", api_error);
                let toast = Toast {
                    position: ToastPosition::TopCenter,
                    r#type: ToastType::Danger,
                    body: "Error al registrar vehiculo".into(),
                    timeout: Some(chrono::Duration::milliseconds(2000)),
                };
                toaster.toast(toast);
            }
        },
        request_create_vehicule_admin.clone())
    }

    // reset all form fields
    let onreset = {
        shadow_clone![vehiculo_nuevo, vehiculo_nuevo_validacion];
        shadow_clone![marca, modelo, año, numero_placa, numero_tarjeta, nombre_economico];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            vehiculo_nuevo.set(NuevoVehiculo::default());
            vehiculo_nuevo_validacion.set(Rc::new(RefCell::new(ValidationErrors::new())));

            reset_input(&marca);
            reset_input(&modelo);
            reset_input(&año);
            reset_input(&numero_placa);
            reset_input(&nombre_economico);
            reset_input(&numero_tarjeta);
        })
    };

    html!{
            <VehiculeRegisterFormFields
                {onchange_marca}
                {onchange_modelo}
                {onchange_año}
                {onchange_nombre_economico}
                {onchange_numero_placa}
                {onchange_numero_tarjeta}

                marca={marca}
                modelo={modelo}
                año={año}
                nombre_economico={nombre_economico}
                numero_placa={numero_placa}
                numero_tarjeta={numero_tarjeta}

                handle_on_input_blur={validate_input_on_blur}
                validation_errors={&*vehiculo_nuevo_validacion}
                
                {onsubmit}
                {onreset}
            >
            </VehiculeRegisterFormFields>
    }
}


fn get_input_callback(
    name: &'static str,
    form: &UseStateHandle<NuevoVehiculo>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}


fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<NuevoVehiculo>,)
{
        let mut data = form.deref().clone();
        match name {
            "marca" => data.marca= value,
            "modelo" => data.modelo = value,
            "año" => data.año= if let Ok(number) = value.parse::<i16>() {number} else { -1 },
            "numero_placa" => data.numero_placa= value,
            "nombre_economico" => data.nombre_economico= value,
            "numero_tarjeta" => data.numero_tarjeta= value,
            _ => (),
        }
        form.set(data);
}


use crate::components::form::{Form, FormField, InputFieldValidated};
use crate::components::button::{Button, ButtonType};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange_marca: Callback<String>, 
    pub onchange_modelo: Callback<String>, 
    pub onchange_año: Callback<String>, 
    pub onchange_nombre_economico: Callback<String>, 
    pub onchange_numero_placa: Callback<String>, 
    pub onchange_numero_tarjeta: Callback<String>, 

    pub handle_on_input_blur: Callback<(String, String)>,
    pub validation_errors: Rc<RefCell<ValidationErrors>>,

    pub marca: NodeRef,
    pub modelo: NodeRef,
    pub año: NodeRef,
    pub nombre_economico: NodeRef,
    pub numero_placa: NodeRef,
    pub numero_tarjeta: NodeRef,

    pub onsubmit: Callback<SubmitEvent>,
    pub onreset: Callback<MouseEvent>,
}

#[function_component]
pub fn VehiculeRegisterFormFields(props: &Props) -> Html {
    shadow_clone!(props);

    let has_errors = !props.validation_errors.borrow().errors().is_empty();
    let submit_button_class = classes![if has_errors { "is-danger"} else { "is-primary" }];

    html!{
        <Form method="get" onsubmit={props.onsubmit}>
            <FormField label="Marca">
                <InputFieldValidated 
                    placeholder="e.g. Nissan"
                    msg="Colocar nombre marca del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="marca"
                    input_ref={props.marca}
                    handle_onchange={props.onchange_marca}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField> 

            <FormField label="Modelo">
                <InputFieldValidated 
                    placeholder="e.g. Leaf"
                    msg="Colocar Modelo del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="modelo"
                    input_ref={props.modelo}
                    handle_onchange={props.onchange_modelo}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Año">
                <InputFieldValidated 
                    placeholder="e.g. 2016"
                    msg="Colocar año del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="año"
                    input_ref={props.año}
                    handle_onchange={props.onchange_año}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Placa">
                <InputFieldValidated 
                    placeholder="e.g. ABCD XYZ 123"
                    msg="Colocar placas del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="numero_placa"
                    input_ref={props.numero_placa}
                    handle_onchange={props.onchange_numero_placa}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Numero de tarjeta">
                <InputFieldValidated 
                    placeholder="e.g. 12345678asd"
                    msg="Colocar numero de tarjeta del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="numero_tarjeta"
                    input_ref={props.numero_tarjeta}
                    handle_onchange={props.onchange_numero_tarjeta}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Nombre economico">
                <InputFieldValidated 
                    placeholder="e.g. Leaf 202"
                    msg="Colocar nombre economico del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="nombre_economico"
                    input_ref={props.nombre_economico}
                    handle_onchange={props.onchange_nombre_economico}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <hr/>

            <FormField>
                <div class="field is-grouped">
                    <div class="control">
                        <Button r#type={ButtonType::Submit} classes={submit_button_class} >
                            <span>{ "Registrar" }</span>
                        </Button>
                    </div>
                    <div class="control">
                        <Button r#type={ButtonType::Reset} classes="button is-primary is-outlined" onclick={props.onreset}>
                            <span>{ "Borrar campos" }</span>
                        </Button>
                    </div>
                </div>
                if has_errors {
                    <p class="help is-danger">{ "Rellenar o corregir los campos" }</p>
                }
            </FormField>

        </Form>
    }
}
