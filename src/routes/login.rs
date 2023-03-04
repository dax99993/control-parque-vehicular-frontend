use yew::prelude::*;
use reqwasm::http::Request;
use web_sys::{console, window, HtmlInputElement};
use wasm_bindgen::UnwrapThrowExt;
use js_sys::JsString;

use crate::{types::user::LoginUser, services::auth::login};

use crate::components::form::item::FormItem;


/*
#[derive(Clone, PartialEq, Properties)]
pub struct LoginFormProps {
    #[prop_or(String::from(""))]
    pub email: String,
    #[prop_or(String::from(""))]
    pub password: String,
    //#[prop_or_else(submit_login_form)]
    pub submit: Callback<MouseEvent>
}
*/

use yew_hooks::prelude::*;
use validator::{validate_email, validate_length};

#[function_component]
pub fn LoginForm() -> Html {

    let login_info = use_state(|| LoginUser::default());
    let email_valid = use_state(|| true);
    let password_valid= use_state(|| true);
    let form_valid = use_state(|| true);

    let onsubmit = {
        let login_info = login_info.clone();
        let email_valid = email_valid.clone();
        let password_valid = password_valid.clone();
        let form_valid = form_valid.clone();
        Callback::from(move |_: MouseEvent| {
            if *email_valid && *password_valid
               && (*login_info).is_filled()
            {
                form_valid.set(true);
                log::debug!("Valid login info {:?}", *login_info); 

            } else {
                form_valid.set(false);
                log::debug!("Invalid login info {:?}", *login_info); 
            }
            //use_async(move ||{
                //login(*login_info).await;
            //})
        })
    };

    let oninput_email = {
        let login_info = login_info.clone();
        let email_valid = email_valid.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            //let mut info = login_info.clone();
            let mut info = (*login_info).clone();
            info.email = input.value();
            email_valid.set(validate_email(input.value()));
            //log::debug!("input email {}", &info.email);
            login_info.set(info);
        })
    };
    let oninput_password = {
        let login_info = login_info.clone();
        let password_valid = password_valid.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            password_valid.set(validate_length(input.value(), Some(6), Some(128), None));
            login_info.set(info);
        })
    };


    
    html! {
        <form class="box" id="login-form">
            if !*(form_valid) {
                <p class="help is-danger"> {"Formulario invalido o incompleto,"} </p>
                <p class="help is-danger"> {"por favor corrige los datos"} </p>
            }
            <FormItem label="Email"
                input_type="email"
                placeholder="e.g. alex@example.com"
                icon_right={ (!(*email_valid)).then(|| "fa-solid fa-triangle-exclamation") }
                error={ (!(*email_valid)).then(|| "Correo electronico invalido") }
                oninput={oninput_email}
            />


            <FormItem label="Password"
                input_type="password"
                placeholder="********"
                icon_left={Some("fa-solid fa-lock")}
                icon_right={ (!(*password_valid)).then(|| "fa-solid fa-triangle-exclamation") }
                error={ (!(*password_valid)).then(|| "Password debe consistir de entre 6 y 128 caracteres") }
                oninput={oninput_password}
            />


            <button id="login-submit-button" class="button is-primary" onclick={onsubmit}>{ "Iniciar sesion" }</button>
        </form>
    }
}
