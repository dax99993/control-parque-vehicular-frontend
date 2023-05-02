use yew::prelude::*;
use yew_hooks::prelude::*;

use super::super::services::request_change_password;

use crate::components::card::{Card, CardContent};
use crate::components::form::{Form, FormField, InputFieldValidated};


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

use common::models::user::ChangePasswordMe;

use crate::shadow_clone;
use crate::utils::forms::{validate_form_field, reset_input};

#[function_component]
pub fn ChangePasswordForm() -> Html {
    let password = use_state(|| ChangePasswordMe::default());
    let password_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let onchange_current_password= get_input_callback("current_password", &password);
    let onchange_new_password = get_input_callback("new_password", &password);
    let onchange_re_new_password = get_input_callback("re_new_password", &password);

    
    let current_password = NodeRef::default();
    let new_password= NodeRef::default();
    let re_new_password= NodeRef::default();

    
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
            request_change_password((*password).clone()).await
        })
    };

    // Submit valid form
    let onsubmit = {
        shadow_clone![password, password_validation];
        shadow_clone![current_password, new_password, re_new_password];
        shadow_clone![request_change_password_me];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            match password.validate() {
                Ok(_) => {
                    reset_input(&current_password);
                    reset_input(&new_password);
                    reset_input(&re_new_password);
                    // Add modal to notify a relogin must be done
                    // make request to database
                    request_change_password_me.run();
                }
                Err(e) => {
                    password_validation.set(Rc::new(RefCell::new(e)));
                }
            }
        })
    };


    {
        //shadow_clone![request_change_password];
        use_effect_with_deps(move |request| {
            if let Some(response) = &request.data {
                log::debug!("api response\n{:?}", &response);
                /*
                if let Some(vehicule) = &response.data {
                    log::debug!("successful vehicule creation\n{:?}", vehicule);
                    user_ctx.redirect_to(AppRoute::VehiculeEdit { id: vehicule.vehicule_id.clone() });
                }
                */
            }
            if let Some(api_error) = &request.error {
                log::error!("change password request error\n{:?}", api_error);
            }
        },
        request_change_password_me.clone())
    }

    // reset all form fields
    let onreset = {
        shadow_clone![password, password_validation];
        shadow_clone![current_password, new_password, re_new_password];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            password.set(ChangePasswordMe::default());
            password_validation.set(Rc::new(RefCell::new(ValidationErrors::new())));

            reset_input(&current_password);
            reset_input(&new_password);
            reset_input(&re_new_password);
        })
    };

    let has_errors = !password_validation.deref().borrow().errors().is_empty();

    html!{
        <Form method="get">
            <FormField label="Contraseña actual">
                <InputFieldValidated 
                    input_type="password"
                    msg="Colocar contraseña actual"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="current_password"
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
                    name="new_password"
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
                    name="re_new_password"
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
                        onclick={onsubmit}
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
    form: &UseStateHandle<ChangePasswordMe>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}

fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<ChangePasswordMe>,)
{
    let mut data = form.deref().clone();
    match name {
        "current_password" => data.current_password = value,
        "new_password" => data.new_password = value,
        "re_new_password" => data.re_new_password = value,
        _ => (),
    }
    log::debug!("Onblur update data {:?}", &data); 
    form.set(data);
}