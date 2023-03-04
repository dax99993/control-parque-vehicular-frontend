use yew::prelude::*;

use crate::components::form::item::FormItem;
use crate::types::user::SignupUser;
use web_sys::HtmlInputElement;
use validator::{validate_email, validate_length};

#[function_component]
pub fn Register() -> Html {

    // Signup information wrapper
    let signup_user = use_state(|| SignupUser::default());

    // states for formitems validity
    let form_valid = use_state(|| true);
    let firstname_valid= use_state(|| true);
    let lastname_valid= use_state(|| true);
    let email_valid = use_state(|| true);
    let password_valid= use_state(|| true);
    let repassword_valid= use_state(|| true);

    let onsubmit = {
        let signup_user = signup_user.clone();
        let firstname_valid = firstname_valid.clone();
        let lastname_valid = lastname_valid.clone();
        let email_valid = email_valid.clone();
        let password_valid = password_valid.clone();
        let repassword_valid = repassword_valid.clone();
        let form_valid = form_valid.clone();
        Callback::from(move |_: MouseEvent| {
            if (*firstname_valid && *lastname_valid 
                && *email_valid && *password_valid
                && *repassword_valid )
                && (*signup_user).is_filled()
            {
                form_valid.set(true);
                log::debug!("Valid signup info {:?}", *signup_user); 

            } else {
                form_valid.set(false);
                log::debug!("Invalid signup info {:?}", *signup_user); 
            }
            //use_async(move ||{
                //login(*login_info).await;
            //})
        })
    };

    let oninput_firstname = {
        let signup_user = signup_user.clone();
        let firstname_valid = firstname_valid.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut user = (*signup_user).clone();
            user.first_name= format!("{}", &input.value().trim());
            firstname_valid.set(validate_name(user.first_name.clone()));
            signup_user.set(user);
        })
    };
    let oninput_lastname = {
        let signup_user = signup_user.clone();
        let lastname_valid = lastname_valid.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut user = (*signup_user).clone();
            user.last_name= format!("{}", &input.value().trim());
            lastname_valid.set(validate_name(user.last_name.clone()));
            signup_user.set(user);
        })
    };
    let oninput_email = {
        let signup_user = signup_user.clone();
        let email_valid = email_valid.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            //let mut info = login_info.clone();
            let mut user = (*signup_user).clone();
            user.email = input.value();
            email_valid.set(validate_email(input.value()));
            //log::debug!("input email {}", &info.email);
            signup_user.set(user);
        })
    };
    let oninput_password = {
        let signup_user = signup_user.clone();
        let password_valid = password_valid.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut user = (*signup_user).clone();
            user.password = input.value();
            password_valid.set(validate_length(input.value(), Some(6), Some(128), None));
            signup_user.set(user);
        })
    };
    let oninput_repassword = {
        let signup_user = signup_user.clone();
        let repassword_valid = repassword_valid.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut user = (*signup_user).clone();
            user.re_password = input.value();
            repassword_valid.set( user.password == user.re_password );
            signup_user.set(user);
        })
    };

    html! {
        <form class="box" id="register-form">
            if !*(form_valid) {
                <p class="help is-danger"> {"Formulario invalido o incompleto,"} </p>
                <p class="help is-danger"> {"por favor corrige los datos"} </p>
            }
            <FormItem label="Nombre"
                input_type="text"
                placeholder="e.g. Manuel"
                icon_right={ (!(*firstname_valid))
                    .then(|| "fa-solid fa-triangle-exclamation")
                }
                error={ (!(*firstname_valid))
                    .then(|| "Nombre solo debe contener caracteres a-z A-Z")
                }
                oninput={oninput_firstname}
            />

            <FormItem label="Apellidos"
                input_type="text"
                placeholder="e.g. Sanchez Perez"
                icon_right={ (!(*lastname_valid))
                    .then(|| "fa-solid fa-triangle-exclamation")
                }
                error={ (!(*lastname_valid))
                    .then(|| "Apellidos solo deben contener caracteres a-z A-Z")
                }
                oninput={oninput_lastname}
            />

            <FormItem label="Correo electrónico"
                input_type="email"
                placeholder="e.g. alex@ejemplo.com"
                icon_right={ (!(*email_valid))
                    .then(|| "fa-solid fa-triangle-exclamation")
                }
                error={ (!(*email_valid))
                    .then(|| "Correo electronico invalido")
                }
                oninput={oninput_email}
            />


            <FormItem label="Contraseña"
                input_type="password"
                placeholder="********"
                icon_left={Some("fa-solid fa-lock")}
                icon_right={ (!(*password_valid))
                    .then(|| "fa-solid fa-triangle-exclamation")
                }
                error={ (!(*password_valid))
                    .then(|| "La contraseña debe tener entre 6 y 128 caracteres")
                }
                oninput={oninput_password}
            />

            <FormItem label="Repetir contraseña"
                input_type="password"
                placeholder="********"
                icon_left={Some("fa-solid fa-lock")}
                icon_right={ (!(*repassword_valid))
                    .then(|| "fa-solid fa-triangle-exclamation")
                }
                error={ (!(*repassword_valid))
                    .then(|| "Las contraseñas no coinciden")
                }
                oninput={oninput_repassword}
            />


            <button id="signup-submit-button" class="button is-primary" onclick={onsubmit}>{ "Crear cuenta" }</button>
        </form>
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
