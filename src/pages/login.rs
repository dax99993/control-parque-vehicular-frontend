use yew::prelude::*;

use crate::hooks::user_context::use_user_context;


#[function_component]
pub fn LoginView() -> Html {
    // Context
    let user_ctx = use_user_context();

    if user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }


    html! {
    <section class="hero-background is-fullheight">
        <div class="hero-body">
            <div class="container"> 
                <div class="columns is-centered ">
                <div class="column is-5-tablet is-4-desktop is-3-widescreen">
                    <LoginForm/>
                </div>
                </div>
            </div>
        </div>
    </section>
    }
}

use yew::platform::spawn_local;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use validator::{ValidationErrors, Validate};

use common::models::user::LoginUsuario;

use crate::shadow_clone;
use crate::components::toast::{use_toaster, ToastPosition, ToastType, Toast, ToastBuilder};
use crate::components::form::{Form, FormField, InputFieldValidated };

use crate::routes::AppRoute;

use crate::services::auth::{request_login, request_me};
use crate::services::request::store_token;
use crate::utils::forms::{validate_form_field, reset_input};



#[function_component]
pub fn LoginForm() -> Html {
    // Context
    let user_ctx = use_user_context();
    let toaster = use_toaster().expect("");


    // States
    let login_user = use_state(|| LoginUsuario::default());
    let login_user_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    // Field NodeRef
    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();


    // Callbacks
    let validate_input_on_blur = {
        shadow_clone![login_user, login_user_validation];
        Callback::from(move |(name, value): (String, String)| {
            set_form_field(name.as_str(), value, &login_user);
            validate_form_field(name.as_str(), &login_user, &login_user_validation);
        })
    };

    let handle_email_input = get_input_callback("email", &login_user);
    let handle_password_input = get_input_callback("password", &login_user);
    


    let get_me_request = {
        use_async(async move {
            request_me().await
        })
    };
    

    // Store context after successful login and get me requests
    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |get_me_request| {
                if let Some(response) = &get_me_request.data {
                    log::debug!("Get me {:?}", &response.data);
                    if let Some(user) = &response.data.clone() {
                        user_ctx.login(user);
                    }
                }
                if let Some(response) = &get_me_request.error {
                    log::error!("Get me request failed {:?}", &response);
                }
            },
            get_me_request.clone() 
        );
    }
    


    // Perform all the requests and update states for logging user
    let onsubmit = {
        shadow_clone![login_user, login_user_validation];
        shadow_clone![get_me_request];
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */

            match login_user.validate() {
                Ok(_) => {
                    // maybe add an if to check for created token state and just create request me
                    // instead of requesting a new token
                    shadow_clone![login_user, toaster];
                    shadow_clone![get_me_request];
                    spawn_local(async move {
                        let response = request_login((*login_user).clone()).await;
                        match response {
                            Ok(api_response) => {
                                log::debug!("Login response {}", &api_response);
                                let token = api_response.data.clone();
                                // Store token to be able to make requests
                                store_token(token.clone());
                                // execute get me request
                                get_me_request.run();

                                let toast = ToastBuilder::new()
                                    .at_top_center()
                                    .is_success()
                                    .with_timeout(Some(2000))
                                    .with_body(html!{ <p><strong>{"Inicio de sesion exitoso."}</strong></p> })
                                    .build();

                                toaster.toast(toast);
                            }
                            Err(api_error) => {
                                log::error!("Login request failed {:?}", &api_error);
                                let toast = ToastBuilder::new()
                                    .at_top_center()
                                    .is_danger()
                                    .with_timeout(Some(2000))
                                    .with_body(html!{
                                        <>
                                        <p>{"Error al Iniciar sesion."}</p>
                                        <p><strong>{"Intente nuevamente"}</strong></p>
                                        </>
                                    })
                                    .build();

                                toaster.toast(toast);
                            }
                        }
                    });
                }
                Err(e) => {
                    login_user_validation.set(Rc::new(RefCell::new(e)));
                    log::debug!("Submit Validation errors {:?}", &*login_user_validation); 
                }
            }
        })
    };


    let has_errors = !login_user_validation.borrow().errors().is_empty();

    html! {
        <Form method="post" classes={classes!["box"]} onsubmit={onsubmit}>
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
                    <button type="submit"
                        class={classes!["button", if !has_errors { "is-primary"} else { "is-danger" }]}
                    >
                        <span>{ "Iniciar sesion" }</span>
                    </button>
                </div>
                if has_errors {
                    <p class="help is-danger"> {"Formulario invalido o incompleto,"} </p>
                    <p class="help is-danger"> {"por favor corrige los datos"} </p>
                }
            </FormField>

            <hr/>

            <FormField>
                <div class="container has-text-centered">
                    {" ¿No tienes cuenta? "}
                    <Link<AppRoute> to={AppRoute::Signup} classes="has-text-link">
                        {"Registrate"}
                    </Link<AppRoute>>
                </div>
            </FormField>

        </Form>
    }
}


fn get_input_callback(
    name: &'static str,
    form: &UseStateHandle<LoginUsuario>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}


fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<LoginUsuario>,)
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
