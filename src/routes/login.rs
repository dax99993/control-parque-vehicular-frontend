use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::services::auth::{request_login, request_me};
use crate::services::request::store_token;
//use crate::types::user::FilteredUser;
use crate::hooks::user_context::use_user_context;
use crate::types::user::LoginUser;

use crate::components::form::item::FormItem;

use yew_hooks::prelude::*;
use validator::{validate_email, validate_length};


#[function_component]
pub fn Login() -> Html {

    let user_ctx = use_user_context();
    let login_user = use_state(|| LoginUser::default());
    let email_valid = use_state(|| true);
    let password_valid= use_state(|| true);
    let form_valid = use_state(|| true);
    //let token = use_state(|| None::<String>);

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
    

    {
    let get_me_request = get_me_request.clone();
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
        },
        login_request.clone() 
    );
    }

    
    use_effect_with_deps(
        move |get_me_request| {
            if let Some(response) = &get_me_request.data {
                log::debug!("Get me {}", &response);
                user_ctx.login(&response.data.clone().unwrap());
            }
        },
        get_me_request.clone() 
    );
    

    let onsubmit = {
        let login_user = login_user.clone();
        let email_valid = email_valid.clone();
        let password_valid = password_valid.clone();
        let form_valid = form_valid.clone();
        Callback::from(move |e: MouseEvent| {
            if *email_valid && *password_valid
               && (*login_user).is_filled()
            {
                form_valid.set(true);
                log::debug!("Valid login info {:?}", *login_user); 
                e.prevent_default(); /* Prevent event propagation */
                login_request.run();


            } else {
                form_valid.set(false);
                log::debug!("Invalid login info {:?}", *login_user); 
            }
        })
    };

    let oninput_email = {
        let login_user = login_user.clone();
        let email_valid = email_valid.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            //let mut info = login_user.clone();
            let mut info = (*login_user).clone();
            info.email = input.value();
            email_valid.set(validate_email(input.value()));
            //log::debug!("input email {}", &info.email);
            login_user.set(info);
        })
    };
    let oninput_password = {
        let login_user = login_user.clone();
        let password_valid = password_valid.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_user).clone();
            info.password = input.value();
            password_valid.set(validate_length(input.value(), Some(6), Some(128), None));
            login_user.set(info);
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
