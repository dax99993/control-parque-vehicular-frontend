use yew::prelude::*;
use yew_hooks::prelude::*;

use web_sys::HtmlInputElement;
use validator::{validate_email, validate_length};

use crate::components::form::{Form, FormField, TextInputField};
use crate::services::auth::request_signup;
use crate::types::user::SignupUser;

use crate::routes::AppRoute;
use crate::hooks::user_context::use_user_context;

use crate::utils::FormFieldState;
use crate::{oninput_macro, shadow_clone};

#[function_component]
pub fn Register() -> Html {

    // Context
    let user_ctx = use_user_context();
    if user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }
    // States
    let signup_user = use_state(|| SignupUser::default());
    let signup_user_valid = use_state(|| bool::default());

    let firstname= use_state(|| FormFieldState::default());
    let oninput_firstname = oninput_macro!(firstname, validate_name);

    let lastname= use_state(|| FormFieldState::default());
    let oninput_lastname = oninput_macro!(lastname, validate_name);

    let email = use_state(|| FormFieldState::default());
    let oninput_email = oninput_macro!(email, validate_email);

    let password= use_state(|| FormFieldState::default());
    let oninput_password = oninput_macro!(password, validate_password);

    let repassword= use_state(|| FormFieldState::default());
    //let oninput_repassword = oninput_macro!(repassword, validate_password);
    let oninput_repassword = {
        shadow_clone!(password, repassword);
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = format!("{}", &input.value().trim());
            let valid = (*password).value == value;
            let form_field = FormFieldState { value, valid };
            repassword.set(form_field);
        })
    };


    // Check validity of all form fields to allow signup
    {
        shadow_clone!(signup_user_valid);
        shadow_clone![firstname, lastname, email, password, repassword];
        use_effect_with_deps(move |(firstname, lastname, email, password, repassword)| {
            let valid = (*firstname).valid && (*lastname). valid &&
                (*email).valid && (*password).valid && (*repassword).valid;

            signup_user_valid.set(valid);
        }, 
        (firstname.clone(), lastname.clone(), email.clone(), password.clone(), repassword.clone())
        )
    }

    // async api request 
    let request_signup_user = {
        shadow_clone!(signup_user);
        use_async(async move {
            request_signup((*signup_user).clone()).await
        })
    };

    // Execute request get me if login was successfull
    {
        shadow_clone!(request_signup_user);
        use_effect_with_deps(
            move |request_signup_user| {
                if let Some(response) = &request_signup_user.data {
                    log::debug!("Sign up response {}", &response);
                    user_ctx.redirect_to(AppRoute::Login);
                }
            },
            request_signup_user.clone() 
        );
    }

    // signup user if valid data
    let onsubmit = {
        shadow_clone![signup_user, signup_user_valid];
        shadow_clone![firstname, lastname, email, password, repassword];
        Callback::from(move |_: MouseEvent| {
            if *signup_user_valid {
                let mut signup = (*signup_user).clone();
                signup.first_name= (*firstname).value.clone();
                signup.last_name= (*lastname).value.clone();
                signup.email = (*email).value.clone();
                signup.password = (*password).value.clone();
                signup.re_password = (*repassword).value.clone();
                signup_user.set(signup);

                request_signup_user.run();

            }
            log::debug!("Valid signup info {:?}", *signup_user); 
        })
    };

    html! {
        <div class="container">
        <div class="box">
        <Form method="post">
            <FormField label="Nombre">
                <TextInputField 
                    placeholder="e.g. Manuel"
                    error_msg="Campo Obligatorio"
                    oninput={oninput_firstname.clone()}
                    value={(*firstname).value.clone()}
                    has_error={(*firstname).valid}
                    icon_right={ if !(*firstname).valid { "fa-solid fa-triangle-exclamation" } else { "" } }
                />
            </FormField>

            <FormField label="Apellidos">
                <TextInputField 
                    placeholder="e.g. Sanchez Perez"
                    error_msg="Campo Obligatorio"
                    oninput={oninput_lastname.clone()}
                    value={(*lastname).value.clone()}
                    has_error={(*lastname).valid}
                    icon_right={ if !(*lastname).valid { "fa-solid fa-triangle-exclamation" } else { "" } }
                />
            </FormField>

            <FormField label="Email">
                <TextInputField 
                    placeholder="e.g. alex@example.com"
                    error_msg="Campo Obligatorio"
                    oninput={oninput_email.clone()}
                    value={(*email).value.clone()}
                    has_error={(*email).valid}
                    icon_left={"fa-solid fa-envelope"}
                    icon_right={ if !(*email).valid { "fa-solid fa-triangle-exclamation" } else { "" } }
                />
            </FormField>

            <FormField label="Contraseña">
                <TextInputField 
                    error_msg="Campo Obligatorio"
                    oninput={oninput_password.clone()}
                    value={(*password).value.clone()}
                    has_error={(*password).valid}
                    icon_left={"fa-solid fa-lock"}
                    icon_right={ if !(*password).valid { "fa-solid fa-triangle-exclamation" } else { "" } }
                />
            </FormField>

            <FormField label="Repetir Contraseña">
                <TextInputField 
                    error_msg="Campo Obligatorio"
                    oninput={oninput_repassword.clone()}
                    value={(*repassword).value.clone()}
                    has_error={(*repassword).valid}
                    icon_left={"fa-solid fa-lock"}
                    icon_right={ if !(*repassword).valid { "fa-solid fa-triangle-exclamation" } else { "" } }
                />
            </FormField>

            <hr/>

            <FormField>
                <div class="control">
                    <button type="button" onclick={ onsubmit }
                        class={classes!["button", if (*signup_user_valid).clone() { "is-primary"} else { "is-danger" }]}
                    >
                        <span>{ "Iniciar sesion" }</span>
                    </button>
                </div>
                if !(*signup_user_valid) {
                    <p class="help is-danger"> {"Formulario invalido o incompleto,"} </p>
                    <p class="help is-danger"> {"por favor corrige los datos"} </p>
                }
            </FormField>
        </Form>
        </div>
        </div>
    }
}

use std::borrow::Cow;

fn validate_name<'a, T>(value: T) -> bool
where 
    T: Into<Cow<'a, str>>
{
    let val = value.into();
    if val.is_empty() || val.chars().all(char::is_whitespace) {
        return false;
    }

    for c in val.chars() {
        if c.is_digit(10)  {
            return false;
        }
    }

    return true;
}

fn validate_password(s: String) -> bool {
    validate_length(s, Some(6), Some(128), None)
}
