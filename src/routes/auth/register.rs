use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use validator::{ValidationErrors, Validate};

use crate::shadow_clone;
use crate::services::auth::request_signup;
use crate::types::user::SignupUser;
use crate::routes::AppRoute;
use crate::hooks::user_context::use_user_context;
use crate::utils::forms::{validate_form_field, reset_input};

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
            set_form_field(name.as_str(), value, &signup_user);
            validate_form_field(name.as_str(), &signup_user, &signup_user_validation);
        })
    };


    let handle_firstname_input = get_input_callback("first_name", &signup_user);
    let handle_lastname_input = get_input_callback("last_name", &signup_user);
    let handle_email_input = get_input_callback("email", &signup_user);
    let handle_password_input = get_input_callback("password", &signup_user);
    let handle_repassword_input = get_input_callback("password", &signup_user);


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
                    //user_ctx.redirect_to(AppRoute::Login);
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
                    reset_input(&firstname_ref);
                    reset_input(&lastname_ref);
                    reset_input(&email_ref);
                    reset_input(&password_ref);
                    reset_input(&repassword_ref);

                    request_signup_user.run();
                }
                Err(e) => {
                    signup_user_validation.set(Rc::new(RefCell::new(e)));
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
                                input_type="password"
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
                                input_type="password"
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
    form: &UseStateHandle<SignupUser>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}

fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<SignupUser>,)
{
    let mut data = form.deref().clone();
    match name {
        "first_name" => data.first_name= value,
        "last_name" => data.last_name = value,
        "email" => data.email = value,
        "password" => data.password = value,
        "re_password" => data.re_password = value,
        _ => (),
    }
    log::debug!("Onblur signup data {:?}", &data); 
    form.set(data);
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
