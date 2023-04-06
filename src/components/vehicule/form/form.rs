use yew::prelude::*;

use crate::components::form::{Form, FormField, InputFieldValidated};

use crate::utils::FormFieldState;
use crate::shadow_clone;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use validator::ValidationErrors;

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
pub fn VehiculeCreateForm(props: &Props) -> Html {
    shadow_clone!(props);

    html!{
        <Form method="get">
            <FormField label="Marca">
                <InputFieldValidated 
                    placeholder="e.g. Nissan"
                    msg="Colocar nombre economico del vehiculo"
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
                    <button type="submit" 
                        onclick={props.onsubmit}
                        class={classes!["button", if props.validation_errors.borrow().errors().is_empty() { "is-primary"} else { "is-danger" }]}
                    >
                      <span>{ "Registrar" }</span>
                    </button>
                  </div>
                  <div class="control">
                    <button type="button" class="button is-primary is-outlined" onclick={props.onreset}>
                      <span>{ "Borrar campos" }</span>
                    </button>
                  </div>
                </div>
                if !props.validation_errors.borrow().errors().is_empty() {
                    <p class="help is-danger">{ "Rellenar o corregir los campos" }</p>
                }
            </FormField>

        </Form>

    }
}
