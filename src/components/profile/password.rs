use yew::prelude::*;

use crate::components::form::{Form, FormField, InputFieldValidated};

use crate::shadow_clone;

use std::rc::Rc;
use std::cell::RefCell;
use validator::ValidationErrors;


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ChangePasswordProps {
    pub onchange_current_password : Callback<String>, 
    pub onchange_new_password: Callback<String>, 
    pub onchange_re_new_password: Callback<String>, 

    pub handle_on_input_blur: Callback<(String, String)>,
    pub validation_errors: Rc<RefCell<ValidationErrors>>,

    pub current_password: NodeRef,
    pub new_password: NodeRef,
    pub re_new_password: NodeRef,

    pub onsubmit: Callback<MouseEvent>,
    pub onreset: Callback<MouseEvent>,
}

#[function_component]
pub fn ProfileChangePassword(props: &ChangePasswordProps) -> Html {
    shadow_clone!(props);

    html!{
        <Form method="get">
            <FormField label="Contraseña actual">
                <InputFieldValidated 
                    input_type="password"
                    msg="Colocar contraseña actual"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="current_password"
                    input_ref={props.current_password}
                    handle_onchange={props.onchange_current_password}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField> 

            <FormField label="Nueva Contraseña">
                <InputFieldValidated 
                    input_type="password"
                    msg="La contraseña debe contener minimo 6 caracteres"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="new_password"
                    input_ref={props.new_password}
                    handle_onchange={props.onchange_new_password}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Repetir Nueva Contraseña">
                <InputFieldValidated 
                    input_type="password"
                    msg="vuelva a escribir la contraseña"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="re_new_password"
                    input_ref={props.re_new_password}
                    handle_onchange={props.onchange_re_new_password}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField>
                <div class="field is-grouped">
                  <div class="control">
                    <button type="submit" 
                        onclick={props.onsubmit}
                        class={classes!["button", if props.validation_errors.borrow().errors().is_empty() { "is-primary"} else { "is-danger" }]}
                    >
                      <span>{ "Cambiar" }</span>
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
