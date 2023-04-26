use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use common::models::user::LoginUser;

use crate::services::auth::{request_login, request_me};
use crate::services::request::store_token;
use crate::hooks::user_context::use_user_context;
use crate::routes::AppRoute;
use crate::utils::forms::{validate_form_field, reset_input};

use crate::components::form::{Form, FormField, InputFieldValidated };

use validator::{ValidationErrors, Validate};

use crate::shadow_clone;
//use gloo::utils;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;




#[function_component]
pub fn LoginView() -> Html {

    // Context
    let user_ctx = use_user_context();
    if user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }
    // States
    let login_user = use_state(|| LoginUser::default());
    let login_user_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();


    let validate_input_on_blur = {
        shadow_clone![login_user, login_user_validation];
        Callback::from(move |(name, value): (String, String)| {
            set_form_field(name.as_str(), value, &login_user);
            validate_form_field(name.as_str(), &login_user, &login_user_validation);
        })
    };

    let handle_email_input = get_input_callback("email", &login_user);
    let handle_password_input = get_input_callback("password", &login_user);

    

    // Async api request states
    let login_request = {
        let login_user = login_user.clone();
        // Make request
        use_async(async move {
            request_login((*login_user).clone()).await
        })
    };

    let get_me_request = {
        use_async(async move {
            request_me().await
        })
    };
    

    // Execute request get me if login was successfull
    {
        shadow_clone!(get_me_request);
        use_effect_with_deps(
            move |login_request| {
                if let Some(response) = &login_request.data {
                    log::debug!("Login response {}", &response);
                    let token = response.data.clone();
                    // Store token to be able to make requests
                    store_token(token.clone());
                    // execute get me request
                    get_me_request.run();
                }
                if let Some(error) = &login_request.error {
                    log::debug!("Login failed {:?}", &error);
                    //TODO: show message to user depending on error variant 
                }
            },
            login_request.clone() 
        );
    }

    
    // Store context after successful login and get me requests
    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |get_me_request| {
                if let Some(response) = &get_me_request.data {
                    log::debug!("Get me {}", &response);
                    user_ctx.login(&response.data.clone().unwrap());
                }
            },
            get_me_request.clone() 
        );
    }
    


    // Perform all the requests and update states for logging user
    let onsubmit = {
        shadow_clone![login_user, login_user_validation];
        shadow_clone![email_input_ref, password_input_ref];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); /* Prevent event propagation */

            match login_user.validate() {
                Ok(_) => {
                    reset_input(&email_input_ref);
                    reset_input(&password_input_ref);

                    log::debug!("Valid login info {:?}", *login_user); 
                    login_request.run();
                }
                Err(e) => {
                    login_user_validation.set(Rc::new(RefCell::new(e)));
                    log::debug!("Submit Validation errors {:?}", &login_user_validation); 
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
                        <FormField label="Email" is_horizontal={false}>
                            <InputFieldValidated
                                placeholder="e.g. alex@example.com"
                                msg="Colocar Correo Electronico"
                                icon_left={"fa-solid fa-envelope"}
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="email"
                                input_ref={email_input_ref}
                                handle_onchange={handle_email_input}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*login_user_validation}
                            />
                        </FormField>

                        <FormField label="Contraseña" is_horizontal={false}>
                            <InputFieldValidated
                                input_type="password"
                                msg="Contraseña debe tener un minimo de 6 caracteres a-zA-z"
                                icon_left={"fa-solid fa-lock"}
                                icon_right={"fa-solid fa-triangle-exclamation"}
                                name="password"
                                input_ref={password_input_ref}
                                handle_onchange={handle_password_input}
                                handle_on_input_blur={validate_input_on_blur.clone()}
                                errors={&*login_user_validation}
                            />
                        </FormField>

                        <hr/>

                        <FormField>
                            <div class="control">
                                <button type="button" onclick={ onsubmit }
                                    class={classes!["button", if login_user_validation.borrow().errors().is_empty() { "is-primary"} else { "is-danger" }]}
                                >
                                    <span>{ "Iniciar sesion" }</span>
                                </button>
                            </div>
                            if !login_user_validation.borrow().errors().is_empty() {
                                <p class="help is-danger"> {"Formulario invalido o incompleto,"} </p>
                                <p class="help is-danger"> {"por favor corrige los datos"} </p>
                            }
                        </FormField>

                        <hr/>

                        <FormField>
                            <div class="container has-text-centered">
                                {" ¿No tienes cuenta? "}
                                <Link<AppRoute> to={AppRoute::Register} classes="has-text-link">
                                    {"Registrate"}
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
    form: &UseStateHandle<LoginUser>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}


fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<LoginUser>,)
{
    let mut data = form.deref().clone();
    match name {
        "email" => data.email = value,
        "password" => data.password = value,
        _ => (),
    }
    log::debug!("Onblur signup data {:?}", &data); 
    form.set(data);
}
