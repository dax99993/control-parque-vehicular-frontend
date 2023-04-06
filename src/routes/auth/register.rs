use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use validator::{ValidationErrors, Validate};
use web_sys::HtmlInputElement;

use crate::shadow_clone;
use crate::services::auth::request_signup;
use crate::types::user::SignupUser;
use crate::routes::AppRoute;
use crate::hooks::user_context::use_user_context;

use crate::components::form::{Form, FormField, InputFieldValidated};


#[function_component]
pub fn RegisterView() -> Html {

    // Context
    let user_ctx = use_user_context();
    if user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }
    // States
    let signup_user = use_state(|| SignupUser::default());
    let signup_user_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    // Form field nodes
    let firstname_ref = NodeRef::default();
    let lastname_ref = NodeRef::default();
    let email_ref = NodeRef::default();
    let password_ref = NodeRef::default();
    let repassword_ref = NodeRef::default();


    let validate_input_on_blur = {
        shadow_clone![signup_user, signup_user_validation];
        Callback::from(move |(name, value): (String, String)| {
            let mut data = signup_user.deref().clone();
            match name.as_str() {
                "first_name" => data.first_name= value,
                "last_name" => data.last_name = value,
                "email" => data.email = value,
                "password" => data.password = value,
                "re_password" => data.re_password = value,
                _ => (),
            }
            log::debug!("Onblur signup data {:?}", &data); 
            signup_user.set(data);

            match signup_user.validate() {
                Ok(_) => {
                    signup_user_validation
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    log::debug!("Onblur signup user validation ok {:?}", &signup_user_validation); 
                }
                Err(errors) => {
                    for(field_name, error) in errors.errors() {
                        if field_name == &name {
                            signup_user_validation
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                            log::debug!("Onblur signup user validation errors {:?}", &signup_user_validation); 
                        }
                    }

                }
            }
        })
    };


    let handle_firstname_input = get_input_callback("first_name", signup_user.clone());
    let handle_lastname_input = get_input_callback("last_name", signup_user.clone());
    let handle_email_input = get_input_callback("email", signup_user.clone());
    let handle_password_input = get_input_callback("password", signup_user.clone());
    let handle_repassword_input = get_input_callback("password", signup_user.clone());


    // Async api request states
    let request_signup_user = {
        shadow_clone!(signup_user);
        use_async(async move {
            request_signup((*signup_user).clone()).await
        })
    };

    // Redirect to login if signup was successfull
    // TODO: maybe redirect to home page or page with message to confirm account by email
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
        shadow_clone![signup_user, signup_user_validation];
        shadow_clone![firstname_ref, lastname_ref, email_ref, password_ref, repassword_ref];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); /* Prevent event propagation */

            match signup_user.validate() {
                Ok(_) => {
                    let firstname_input = if let Some(element) = firstname_ref.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };
                    let lastname_input = if let Some(element) = lastname_ref.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };
                    let email_input = if let Some(element) = email_ref.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };
                    let password_input = if let Some(element) = password_ref.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };
                    let repassword_input = if let Some(element) = repassword_ref.cast::<HtmlInputElement>() { element }
                    else {
                        return;
                    };

                    firstname_input.set_value("");
                    lastname_input.set_value("");
                    email_input.set_value("");
                    password_input.set_value("");
                    repassword_input.set_value("");
                    log::debug!("Valid signup info {:?}", *signup_user); 
                    request_signup_user.run();
                }
                Err(e) => {
                    signup_user_validation.set(Rc::new(RefCell::new(e)));
                    log::debug!("Submit Validation errors {:?}", &signup_user_validation); 
                }
            }
        })
    };


    html! {
    <section class="hero is-fullheight is-primary">
        <div class="hero-body">
            <div class="container"> 
                <div class="columns is-centered ">
                <div class="column is-5-tablet is-4-desktop is-3-widescreen">
                    <Form method="post" classes={classes!["box"]}>

                        <FormField label="Nombre" is_horizontal={false}>
                            <InputFieldValidated
                                placeholder="e.g. Manuel"
                                msg="Escribir Nombres"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="first_name"
                                input_ref={firstname_ref}
                                handle_onchange={handle_firstname_input}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*signup_user_validation}
                            />
                        </FormField>

                        <FormField label="Apellidos" is_horizontal={false}>
                            <InputFieldValidated
                                placeholder="e.g. Sanchez Perez"
                                msg="Escribir primero apellido materno"
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="last_name"
                                input_ref={lastname_ref}
                                handle_onchange={handle_lastname_input}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*signup_user_validation}
                            />
                        </FormField>

                        <FormField label="Email" is_horizontal={false}>
                            <InputFieldValidated
                                placeholder="e.g. alex@example.com"
                                msg="Colocar Correo Electronico"
                                icon_left={"fa-solid fa-envelope"}
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="email"
                                input_ref={email_ref}
                                handle_onchange={handle_email_input}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*signup_user_validation}
                            />
                        </FormField>

                        <FormField label="Contraseña" is_horizontal={false}>
                            <InputFieldValidated
                                msg="Colocar Correo Electronico"
                                icon_left={"fa-solid fa-lock"}
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="password"
                                input_ref={password_ref}
                                handle_onchange={handle_password_input}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*signup_user_validation}
                            />
                        </FormField>

                        <FormField label="Repetir Contraseña" is_horizontal={false}>
                            <InputFieldValidated
                                msg="Repetir Contraseña"
                                icon_left={"fa-solid fa-lock"}
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="re_password"
                                input_ref={repassword_ref}
                                handle_onchange={handle_repassword_input}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*signup_user_validation}
                            />
                        </FormField>

                        <hr/>

                        <FormField>
                            <div class="control">
                                <button type="button" onclick={ onsubmit }
                                    class={classes!["button", if signup_user_validation.borrow().errors().is_empty() { "is-primary"} else { "is-danger" }]}
                                >
                                    <span>{ "Crear cuenta" }</span>
                                </button>
                            </div>
                            if !signup_user_validation.borrow().errors().is_empty() {
                                <p class="help is-danger"> {"Formulario invalido o incompleto,"} </p>
                                <p class="help is-danger"> {"por favor corrige los datos"} </p>
                            }
                        </FormField>

                        <hr/>

                        <FormField>
                            <div class="container has-text-centered">
                                {" ¿Ya tienes cuenta? "}
                                <Link<AppRoute> to={AppRoute::Login} classes="has-text-link">
                                    {"Inicia sesion"}
                                </Link<AppRoute>>
                            </div>
                        </FormField>
                </Form>
                </div>
                </div>
            </div>
        </div>
    </section>
    }
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<SignupUser>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "first_name" => data.first_name= value,
            "last_name" => data.last_name = value,
            "email" => data.email = value,
            "password" => data.password = value,
            "re_password" => data.re_password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
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
