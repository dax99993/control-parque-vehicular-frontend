use yew::prelude::*;
use yew_hooks::prelude::*;


use web_sys::HtmlInputElement;

use crate::hooks::user_context::use_user_context;
use crate::components::main_section::MainSection;
use crate::components::form::form::{Form, FormField, FormInputField};
use crate::routes::AppRoute;
use crate::services::vehicule::request_admin_create_vehicule;
use crate::types::vehicule::NewVehicule;


#[macro_export]
macro_rules! oninput_macro {
    ( $field_state:tt, $validator:expr) => {
        {
            let $field_state = $field_state.clone(); 
            Callback::from(move |e: InputEvent| {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = format!("{}", &input.value().trim());
                let valid = $validator(value.clone());
                let formfield = FormFieldState { value, valid };
                $field_state.set(formfield);
            })
        }
    };
}

macro_rules! shadow_clone {
    // single state shadow_clone!(model);
    ( $state:tt ) => {
        let $state = $state.clone(); 
    };
    // multiple state shadow_clone!(model, year);
    ( $($state:tt), * ) => {
        $(
            let $state = $state.clone(); 
        )*
    };

}

#[derive(Debug, Clone, PartialEq, Default)]
struct FormFieldState {
    pub value: String,
    pub valid: bool,
}

#[function_component]
pub fn RegisterVehicule() -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }

    let new_vehicule = use_state(|| NewVehicule::default());
    let new_vehicule_valid = use_state(|| bool::default());


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
    
    // check the validity of all form fields and store validity state
    {
    shadow_clone![new_vehicule_valid];
    shadow_clone![branch, model, number_plate, year, number_card, short_name];
    use_effect_with_deps(move |(branch, model, number_plate, year, number_card, short_name)| {
            let valid = (*branch).valid && (*model).valid && (*number_plate).valid &&
                (*year).valid && (*number_card).valid && (*short_name).clone().valid;
            new_vehicule_valid.set(valid);
        },
        (branch.clone(), model.clone(), number_plate.clone(), year.clone(), number_card.clone(), short_name.clone()))
    }

    
    let request_create_vehicule_admin = {
        shadow_clone!(new_vehicule);
        use_async(async move {
            request_admin_create_vehicule((*new_vehicule).clone()).await
        })
    };


    // Submit valid form
    let onsubmit = {
        shadow_clone![new_vehicule, new_vehicule_valid];
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        shadow_clone![request_create_vehicule_admin];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if *new_vehicule_valid {
                let mut vehicule = (*new_vehicule).clone();
                vehicule.branch = (*branch).value.clone();
                vehicule.model = (*model).value.clone();
                vehicule.year =  (*year).value.clone().parse::<i16>().unwrap();
                vehicule.number_plate = (*number_plate).value.clone();
                vehicule.short_name = (*short_name).value.clone();
                vehicule.number_card = (*number_card).value.clone();
                new_vehicule.set(vehicule); 

                // make request to database
                request_create_vehicule_admin.run();
            }
            log::debug!("Vehicule Registration validity {:?}", *new_vehicule_valid);
            log::debug!("Vehicule Registration {:?}", *new_vehicule);
        })
    };

    {
        shadow_clone![request_create_vehicule_admin];
        use_effect_with_deps(move |request_create_vehicule_admin| {
            if let Some(response) = &request_create_vehicule_admin.data {
                log::debug!("response data {:?}", &response);
                if let Some(vehicule) = &response.data {
                    user_ctx.redirect_to(AppRoute::VehiculesEdit { id: vehicule.vehicule_id.to_string() });
                }
            }
        },
        request_create_vehicule_admin.clone())
    }

    // reset all form fields
    let onreset = {
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
                branch.set(FormFieldState::default());
                model.set(FormFieldState::default());
                year.set(FormFieldState::default());
                number_plate.set(FormFieldState::default());
                short_name.set(FormFieldState::default());
                number_card.set(FormFieldState::default());
        })
    };

    html!{
        <MainSection route="Admin" subroute="Vehiculos" title="Registrar Vehiculo">

        <div class="card">
            <header class="card-header">
                <p class="card-header-title">
                    <span class="icon"><i class="fa-solid fa-ballot"></i></span>
                    { "Registro" }
                </p>
            </header>

            <div class="card-content">
            <Form method="get">
                <FormField label="Marca">
                    <FormInputField 
                        input_type="text"
                        placeholder="e.g. Nissan"
                        danger_msg="Campo Obligatorio"
                        oninput={oninput_branch.clone()}
                        valid={(*branch).valid}
                    />
                </FormField> 

                <FormField label="Modelo">
                    <FormInputField 
                        input_type="text"
                        placeholder="e.g. Leaf"
                        danger_msg="Campo Obligatorio"
                        oninput={oninput_model.clone()}
                        valid={(*model).valid}
                    />
                </FormField>

                <FormField label="Año">
                    <FormInputField 
                        input_type="text"
                        placeholder="e.g. 2016"
                        danger_msg="Campo Obligatorio"
                        oninput={oninput_year.clone()}
                        valid={(*year).valid}
                    />
                </FormField>

                <FormField label="Placa">
                    <FormInputField 
                        input_type="text"
                        placeholder="e.g. ABCD XYZ 123"
                        danger_msg="Campo Obligatorio"
                        oninput={oninput_number_plate.clone()}
                        valid={(*number_plate).valid}
                    />
                </FormField>

                <FormField label="Numero de tarjeta">
                    <FormInputField 
                        input_type="text"
                        placeholder="e.g. 12345678asd"
                        danger_msg="Campo Obligatorio"
                        oninput={oninput_number_card.clone()}
                        valid={(*number_card).valid}
                    />
                </FormField>

                <FormField label="Nombre economico">
                    <FormInputField 
                        input_type="text"
                        placeholder="e.g. Leaf 202"
                        danger_msg="Campo Obligatorio"
                        oninput={oninput_short_name.clone()}
                        valid={(*short_name).valid}
                    />
                </FormField>

                <hr/>

                <FormField>
                    <div class="field is-grouped">
                      <div class="control">
                        <button type="submit" 
                            onclick={ onsubmit }
                            class={classes!["button", if (*new_vehicule_valid).clone() { "is-primary"} else { "is-danger" }]}
                        >
                          <span>{ "Registrar" }</span>
                        </button>
                      </div>
                      <div class="control">
                        <button type="button" class="button is-primary is-outlined" onclick={ onreset }>
                          <span>{ "Borrar campos" }</span>
                        </button>
                      </div>
                    </div>
                    if !(*new_vehicule_valid).clone() {
                        <p class="help is-danger">{ "Rellenar o corrigir los campos" }</p>
                    }
                </FormField>

            </Form>

            </div>
        </div>

        </MainSection>
    }
}



fn validate(s: String) -> bool {
    !s.is_empty()
}

fn validate_number(s: String) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}
