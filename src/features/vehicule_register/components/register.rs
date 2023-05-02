use yew::prelude::*;
use yew_hooks::prelude::*;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use validator::{Validate, ValidationErrors};
use common::models::vehicule::NewVehicule;

use crate::shadow_clone;
use crate::hooks::user_context::use_user_context;
use super::super::services::request_admin_create_vehicule;
use crate::routes::AppRoute;
use crate::utils::forms::{validate_form_field, reset_input};



#[function_component]
pub fn VehiculeRegisterForm() -> Html {
    let user_ctx = use_user_context();

    let new_vehicule = use_state(|| NewVehicule::default());
    let new_vehicule_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let onchange_branch = get_input_callback("branch", &new_vehicule);
    let onchange_model = get_input_callback("model", &new_vehicule);
    let onchange_year = get_input_callback("year", &new_vehicule);
    let onchange_number_plate= get_input_callback("number_plate", &new_vehicule);
    let onchange_short_name = get_input_callback("short_name", &new_vehicule);
    let onchange_number_card = get_input_callback("number_card", &new_vehicule);

    let branch = NodeRef::default();
    let model = NodeRef::default();
    let year = NodeRef::default();
    let number_plate = NodeRef::default();
    let short_name = NodeRef::default();
    let number_card = NodeRef::default();
    
    let validate_input_on_blur = {
        shadow_clone![new_vehicule, new_vehicule_validation];
        Callback::from(move |(name, value): (String, String)| {
            set_form_field(name.as_str(), value, &new_vehicule);
            validate_form_field(name.as_str(), &new_vehicule, &new_vehicule_validation);
        })
    };

    
    let request_create_vehicule_admin = {
        shadow_clone!(new_vehicule);
        use_async(async move {
            request_admin_create_vehicule((*new_vehicule).clone()).await
        })
    };


    // Submit valid form
    let onsubmit = {
        shadow_clone![new_vehicule, new_vehicule_validation];
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        shadow_clone![request_create_vehicule_admin];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            match new_vehicule.validate() {
                Ok(_) => {
                    reset_input(&branch);
                    reset_input(&model);
                    reset_input(&year);
                    reset_input(&number_plate);
                    reset_input(&short_name);
                    reset_input(&number_card);
                    // make request to database
                    request_create_vehicule_admin.run();
                }
                Err(e) => {
                    new_vehicule_validation.set(Rc::new(RefCell::new(e)));
                }
            }
        })
    };

    {
        shadow_clone![request_create_vehicule_admin];
        use_effect_with_deps(move |request_create_vehicule_admin| {
            if let Some(response) = &request_create_vehicule_admin.data {
                log::debug!("api response\n{:?}", &response);
                if let Some(vehicule) = &response.data {
                    log::debug!("successful vehicule creation\n{:?}", vehicule);
                    user_ctx.redirect_to(AppRoute::VehiculeEdit { id: vehicule.vehicule_id.clone() });
                }
            }
            if let Some(api_error) = &request_create_vehicule_admin.error {
                log::error!("api error\n{:?}", api_error);
            }
        },
        request_create_vehicule_admin.clone())
    }

    // reset all form fields
    let onreset = {
        shadow_clone![new_vehicule, new_vehicule_validation];
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            new_vehicule.set(NewVehicule::default());
            new_vehicule_validation.set(Rc::new(RefCell::new(ValidationErrors::new())));

            reset_input(&branch);
            reset_input(&model);
            reset_input(&year);
            reset_input(&number_plate);
            reset_input(&short_name);
            reset_input(&number_card);
        })
    };

    html!{
            <VehiculeRegisterFormFields
                {onchange_branch}
                {onchange_model}
                {onchange_year}
                {onchange_short_name}
                {onchange_number_plate}
                {onchange_number_card}

                branch={branch}
                model={model}
                year={year}
                short_name={short_name}
                number_plate={number_plate}
                number_card={number_card}

                handle_on_input_blur={validate_input_on_blur}
                validation_errors={&*new_vehicule_validation}
                
                {onsubmit}
                {onreset}
            >
            </VehiculeRegisterFormFields>
    }
}


fn get_input_callback(
    name: &'static str,
    form: &UseStateHandle<NewVehicule>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}


fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<NewVehicule>,)
{
        let mut data = form.deref().clone();
        match name {
            "branch" => data.branch = value,
            "model" => data.model = value,
            "year" => data.year = if let Ok(number) = value.parse::<i16>() {number} else { -1 },
            "number_plate" => data.number_plate = value,
            "short_name" => data.short_name= value,
            "number_card" => data.number_card= value,
            _ => (),
        }
        form.set(data);
}


use crate::components::form::{Form, FormField, InputFieldValidated};
use crate::components::button::{Button, ButtonType};


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange_branch: Callback<String>, 
    pub onchange_model: Callback<String>, 
    pub onchange_year: Callback<String>, 
    pub onchange_short_name: Callback<String>, 
    pub onchange_number_plate: Callback<String>, 
    pub onchange_number_card: Callback<String>, 

    pub handle_on_input_blur: Callback<(String, String)>,
    pub validation_errors: Rc<RefCell<ValidationErrors>>,

    pub branch: NodeRef,
    pub model: NodeRef,
    pub year: NodeRef,
    pub short_name: NodeRef,
    pub number_plate: NodeRef,
    pub number_card: NodeRef,

    pub onsubmit: Callback<MouseEvent>,
    pub onreset: Callback<MouseEvent>,
}

#[function_component]
pub fn VehiculeRegisterFormFields(props: &Props) -> Html {
    shadow_clone!(props);

    let has_errors = !props.validation_errors.borrow().errors().is_empty();

    html!{
        <Form method="get">
            <FormField label="Marca">
                <InputFieldValidated 
                    placeholder="e.g. Nissan"
                    msg="Colocar nombre marca del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="branch"
                    input_ref={props.branch}
                    handle_onchange={props.onchange_branch}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField> 

            <FormField label="Modelo">
                <InputFieldValidated 
                    placeholder="e.g. Leaf"
                    msg="Colocar Modelo del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="model"
                    input_ref={props.model}
                    handle_onchange={props.onchange_model}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Año">
                <InputFieldValidated 
                    placeholder="e.g. 2016"
                    msg="Colocar año del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="year"
                    input_ref={props.year}
                    handle_onchange={props.onchange_year}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Placa">
                <InputFieldValidated 
                    placeholder="e.g. ABCD XYZ 123"
                    msg="Colocar placas del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="number_plate"
                    input_ref={props.number_plate}
                    handle_onchange={props.onchange_number_plate}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Numero de tarjeta">
                <InputFieldValidated 
                    placeholder="e.g. 12345678asd"
                    msg="Colocar numero de tarjeta del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="number_card"
                    input_ref={props.number_card}
                    handle_onchange={props.onchange_number_card}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Nombre economico">
                <InputFieldValidated 
                    placeholder="e.g. Leaf 202"
                    msg="Colocar nombre economico del vehiculo"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="short_name"
                    input_ref={props.short_name}
                    handle_onchange={props.onchange_short_name}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <hr/>

            <FormField>
                <div class="field is-grouped">
                    <div class="control">
                        <Button r#type={ButtonType::Submit}
                            onclick={props.onsubmit}
                            classes={classes![if has_errors { "is-danger"} else { "is-primary" }]}
                        >
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
