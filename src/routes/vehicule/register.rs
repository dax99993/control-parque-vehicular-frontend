use yew::prelude::*;
use yew_hooks::prelude::*;

use validator::{Validate, ValidationErrors};

use web_sys::HtmlInputElement;

use crate::hooks::user_context::use_user_context;
use crate::services::vehicule::request_admin_create_vehicule;
use crate::types::vehicule::NewVehicule;
use crate::routes::AppRoute;
use crate::components::main_section::MainSection;
use crate::components::card::{Card, CardContent};
use crate::components::vehicule::form::VehiculeCreateForm;

use crate::shadow_clone;



#[function_component]
pub fn RegisterVehiculeView() -> Html {
    let user_ctx = use_user_context();
    if !user_ctx.is_authenticated() || !user_ctx.is_admin() {
        user_ctx.redirect_home();
    }

    let new_vehicule = use_state(|| NewVehicule::default());
    let new_vehicule_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let onchange_branch = get_input_callback("branch", new_vehicule.clone());
    let onchange_model = get_input_callback("model", new_vehicule.clone());
    let onchange_year = get_input_callback("year", new_vehicule.clone());
    let onchange_number_plate= get_input_callback("number_plate", new_vehicule.clone());
    let onchange_short_name = get_input_callback("short_name", new_vehicule.clone());
    let onchange_number_card = get_input_callback("number_card", new_vehicule.clone());

    let branch = NodeRef::default();
    let model = NodeRef::default();
    let year = NodeRef::default();
    let number_plate = NodeRef::default();
    let short_name = NodeRef::default();
    let number_card = NodeRef::default();
    
    let validate_input_on_blur = {
        shadow_clone![new_vehicule, new_vehicule_validation];
        Callback::from(move |(name, value): (String, String)| {
            let mut data = new_vehicule.deref().clone();
            match name.as_str() {
                "branch" => data.branch = value,
                "model" => data.model= value,
                // Maybe need parsing
                "year" => data.year = if let Ok(number) = value.parse::<i16>() {number} else { -1 },
                "number_plate" => data.number_plate= value,
                "short_name" => data.short_name= value,
                "number_card" => data.number_card= value,
                _ => (),
            }
            log::debug!("Onblur login data {:?}", &data); 
            new_vehicule.set(data);

            match new_vehicule.validate() {
                Ok(_) => {
                    new_vehicule_validation 
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    log::debug!("Onblur login user validation ok {:?}", &new_vehicule_validation); 
                }
                Err(errors) => {
                    for(field_name, error) in errors.errors() {
                        if field_name == &name {
                            new_vehicule_validation 
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                            log::debug!("Onblur login user validation errors {:?}", &new_vehicule_validation); 
                        }
                    }

                }
            }
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
        shadow_clone![new_vehicule, new_vehicule_validation];
        shadow_clone![branch, model, number_plate, year, number_card, short_name];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            log::debug!("should reset");
            new_vehicule.set(NewVehicule::default());
            new_vehicule_validation.set(Rc::new(RefCell::new(ValidationErrors::new())));

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

    html!{
        <MainSection route="Admin" subroute="Vehiculos" title="Registrar Vehiculo">

            <Card header_icon_left={ "fa-solid fa-ballot" } header_title={ "Registro de Vehiculo" }>
                <CardContent>
                    <VehiculeCreateForm
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
                    </VehiculeCreateForm>
                </CardContent>
            </Card>

        </MainSection>
    }
}


use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;


fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<NewVehicule>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "branch" => data.branch = value,
            "model" => data.model= value,
            // Maybe need parsing
            "year" => data.year = if let Ok(number) = value.parse::<i16>() {number} else { -1 },
            "number_plate" => data.number_plate = value,
            "short_name" => data.short_name= value,
            "number_card" => data.number_card= value,
            _ => (),
        }
        cloned_form.set(data);
    })
}


