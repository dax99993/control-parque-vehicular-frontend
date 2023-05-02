use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use validator::{Validate, ValidationErrors};

use common::models::user::SignupUser;

use crate::shadow_clone;
use crate::components::form::{Form, FormField, InputFieldValidated};
use crate::components::button::{Button, ButtonType};
use crate::routes::AppRoute;
use crate::services::auth::request_signup;
use crate::utils::forms::{validate_form_field, reset_input};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange_first_name: Callback<String>, 
    pub onchange_last_name: Callback<String>, 
    pub onchange_email: Callback<String>, 
    pub onchange_password: Callback<String>, 
    pub onchange_re_password: Callback<String>, 

    pub handle_on_input_blur: Callback<(String, String)>,
    pub validation_errors: Rc<RefCell<ValidationErrors>>,

    pub first_name: NodeRef,
    pub last_name: NodeRef,
    pub email: NodeRef,
    pub password: NodeRef,
    pub re_password: NodeRef,

    pub onsubmit: Callback<MouseEvent>,
}


#[function_component]
pub fn SignupFormFields(props: &Props) -> Html {
    shadow_clone!(props);

    let has_errors = !props.validation_errors.borrow().errors().is_empty();

    html!{
        <Form method="post" classes={classes!["box"]}>

            <FormField label="Nombre" is_horizontal={false}>
                <InputFieldValidated
                    placeholder="e.g. Manuel"
                    msg="Escribir Nombres"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="first_name"
                    input_ref={props.first_name}
                    handle_onchange={props.onchange_first_name}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Apellidos" is_horizontal={false}>
                <InputFieldValidated
                    placeholder="e.g. Sanchez Perez"
                    msg="Escribir primero apellido paterno"
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="last_name"
                    input_ref={props.last_name}
                    handle_onchange={props.onchange_last_name}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Email" is_horizontal={false}>
                <InputFieldValidated
                    placeholder="e.g. alex@example.com"
                    msg="Colocar Correo Electronico"
                    icon_left={"fa-solid fa-envelope"}
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="email"
                    input_ref={props.email}
                    handle_onchange={props.onchange_email}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Contraseña" is_horizontal={false}>
                <InputFieldValidated
                    input_type="password"
                    msg="Contraseña debe tener un minimo de 6 caracteres a-zA-z"
                    icon_left={"fa-solid fa-lock"}
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="password"
                    input_ref={props.password}
                    handle_onchange={props.onchange_password}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <FormField label="Repetir Contraseña" is_horizontal={false}>
                <InputFieldValidated
                    input_type="password"
                    msg="Repetir Contraseña"
                    icon_left={"fa-solid fa-lock"}
                    icon_right={"fa-solid fa-triangle-exclamation"}
                    name="re_password"
                    input_ref={props.re_password}
                    handle_onchange={props.onchange_re_password}
                    handle_on_input_blur={props.handle_on_input_blur.clone()}
                    errors={&props.validation_errors.clone()}
                />
            </FormField>

            <hr/>

            <FormField>
                <div class="control">
                    <Button r#type={ButtonType::Submit} onclick={props.onsubmit}
                        classes={classes![if has_errors { "is-danger" } else { "is-primary" }]}
                    >
                        <span>{ "Crear cuenta" }</span>
                    </Button>
                </div>
                if has_errors {
                    <p class="help is-danger">{"Formulario invalido o incompleto."}</p>
                    <p class="help is-danger">{"por favor corrige los datos"}</p>
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
    }
}

#[function_component]
pub fn SignupForm() -> Html {
    // States
    let signup_user = use_state(|| SignupUser::default());
    let signup_user_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator();

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
        shadow_clone!(navigator);
        use_effect_with_deps(
            move |request_signup_user| {
                if let Some(response) = &request_signup_user.data {
                    log::debug!("Sign up response {}", &response);
                    //user_ctx.redirect_to(AppRoute::Login);
                    if let Some(nav) = navigator {
                        nav.push(&AppRoute::Login);
                    }
                }
                if let Some(response) = &request_signup_user.error {
                    log::error!("Sign up request failed {:?}", &response);
                    //TODO: notify user about failure so that a retry can be done
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
        <SignupFormFields
            onchange_first_name={handle_firstname_input}
            onchange_last_name={handle_lastname_input}
            onchange_email={handle_email_input}
            onchange_password={handle_password_input}
            onchange_re_password={handle_repassword_input}

            handle_on_input_blur={validate_input_on_blur}
            validation_errors={signup_user_validation.deref().clone()}

            first_name={firstname_ref}
            last_name={lastname_ref}
            email={email_ref}
            password={password_ref}
            re_password={repassword_ref}

            onsubmit={onsubmit}
        />
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
