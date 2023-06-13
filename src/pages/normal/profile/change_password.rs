use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::card::{Card, CardContent};
use crate::components::form::{Form, FormField, InputFieldValidated};
use crate::components::toast::{use_toaster, ToastBuilder};


#[function_component]
pub fn ChangePassword() -> Html {
    html!{
        <Card header_icon_left={"fa-solid fa-lock"} header_title={"Cambiar Contraseña"}>
            <CardContent>
                <ChangePasswordForm/>
            </CardContent>
        </Card>
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use validator::{ValidationErrors, Validate};

use common::models::user::CambiarMiPassword;

use crate::shadow_clone;
use crate::utils::forms::{validate_form_field, reset_input};

#[function_component]
pub fn ChangePasswordForm() -> Html {
    //Context
    let toaster = use_toaster().expect("No ToastViewer");

    //States
    let password = use_state(|| CambiarMiPassword::default());
    let password_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    //Fields Noderef
    let current_password = NodeRef::default();
    let new_password= NodeRef::default();
    let re_new_password= NodeRef::default();

    //Callbacks
    let onchange_current_password= get_input_callback("password_actual", &password);
    let onchange_new_password = get_input_callback("nuevo_password", &password);
    let onchange_re_new_password = get_input_callback("re_nuevo_password", &password);


    let validate_input_on_blur = {
        shadow_clone![password, password_validation];
        Callback::from(move |(name, value): (String, String)| {
            set_form_field(name.as_str(), value, &password);
            validate_form_field(name.as_str(), &password, &password_validation);
        })
    };

    
    let request_change_password_me = {
        shadow_clone![password];
        use_async(async move {
            crate::services::normal::request_cambiar_password((*password).clone()).await
        })
    };


    // Notify success
    {
        let toaster = toaster.clone();
        use_effect_with_deps(move |request| {
            if let Some(response) = &request.data {
                log::debug!("api response\n{:?}", &response);
                // Add modal to notify a relogin must be done
                let toast = ToastBuilder::new()
                    .is_success()
                    .at_top_center()
                    .with_timeout(Some(3000))
                    .with_body(html!{<p>{"Contraseña actualizada!"}</p>})
                    .build();
                toaster.toast(toast);
            }
            if let Some(api_error) = &request.error {
                let msg = match api_error {
                    crate::error::Error::BadRequestError(m) => m.clone(),
                    _ => String::new(),
                };
                let toast = ToastBuilder::new()
                    .is_danger()
                    .at_top_center()
                    .with_timeout(Some(3000))
                    .with_body(html!{<>
                        <p>{"La contraseña no se pudo actualizar!"}</p>
                        <hr/>
                        <p><strong>{msg}</strong></p>
                        </>})
                    .build();
                toaster.toast(toast);
            }
        },
        request_change_password_me.clone())
    }


    // Submit valid form
    let onsubmit = {
        shadow_clone![password, password_validation];
        shadow_clone![current_password, new_password, re_new_password];
        shadow_clone![request_change_password_me];
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            match password.validate() {
                Ok(_) => {
                    reset_input(&current_password);
                    reset_input(&new_password);
                    reset_input(&re_new_password);
                    request_change_password_me.run();
                }
                Err(e) => {
                    password_validation.set(Rc::new(RefCell::new(e)));
                }
            }
        })
    };

    // reset all form fields
    let onreset = {
        shadow_clone![password, password_validation];
        shadow_clone![current_password, new_password, re_new_password];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            password.set(CambiarMiPassword::default());
            password_validation.set(Rc::new(RefCell::new(ValidationErrors::new())));

            reset_input(&current_password);
            reset_input(&new_password);
            reset_input(&re_new_password);
        })
    };

    let has_errors = !password_validation.deref().borrow().errors().is_empty();

    html!{
        <Form method="get" onsubmit={onsubmit}>
            <FormField label="Contraseña actual">
                <InputFieldValidated 
                    input_type="password"
                    msg="Colocar contraseña actual"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="password_actual"
                    input_ref={current_password}
                    handle_onchange={onchange_current_password}
                    handle_on_input_blur={validate_input_on_blur.clone()}
                    errors={password_validation.deref().clone()}
                />
            </FormField> 

            <FormField label="Nueva Contraseña">
                <InputFieldValidated 
                    input_type="password"
                    msg="La contraseña debe contener minimo 6 caracteres"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="nuevo_password"
                    input_ref={new_password}
                    handle_onchange={onchange_new_password}
                    handle_on_input_blur={validate_input_on_blur.clone()}
                    errors={password_validation.deref().clone()}
                />
            </FormField>

            <FormField label="Repetir Nueva Contraseña">
                <InputFieldValidated 
                    input_type="password"
                    msg="vuelva a escribir la contraseña"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="re_nuevo_password"
                    input_ref={re_new_password}
                    handle_onchange={onchange_re_new_password}
                    handle_on_input_blur={validate_input_on_blur.clone()}
                    errors={password_validation.deref().clone()}
                />
            </FormField>

            <FormField>
                <div class="field is-grouped">
                  <div class="control">
                    <button type="submit" 
                        class={classes!["button", if has_errors { "is-danger" } else { "is-primary" }]}
                    >
                      <span>{ "Cambiar" }</span>
                    </button>
                  </div>
                  <div class="control">
                    <button type="button" class="button is-primary is-outlined" onclick={onreset}>
                      <span>{ "Borrar campos" }</span>
                    </button>
                  </div>
                </div>
                if has_errors {
                    <p class="help is-danger">{ "Rellenar o corregir los campos" }</p>
                }
            </FormField>

        </Form>
    }
}


fn get_input_callback(
    name: &'static str,
    form: &UseStateHandle<CambiarMiPassword>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}

fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<CambiarMiPassword>,)
{
    let mut data = form.deref().clone();
    match name {
        "password_actual" => data.password_actual= value,
        "nuevo_password" => data.nuevo_password = value,
        "re_nuevo_password" => data.re_nuevo_password = value,
        _ => (),
    }
    log::debug!("Onblur update data {:?}", &data); 
    form.set(data);
}
