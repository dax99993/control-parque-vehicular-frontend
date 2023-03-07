use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::services::auth::{request_login, request_me};
use crate::services::request::store_token;
//use crate::types::user::FilteredUser;
use crate::hooks::user_context::use_user_context;
use crate::types::user::LoginUser;

//use crate::components::form::item::FormItem;

use yew_hooks::prelude::*;
use validator::{validate_email, validate_length};

//use gloo::utils;

use wasm_bindgen::{JsCast, JsValue};


#[function_component]
pub fn Login() -> Html {

    let user_ctx = use_user_context();
    let login_user = use_state(|| LoginUser::default());

    let email_value = use_state(|| String::default());
    let password_value = use_state(|| String::default());

    let email_valid = use_state(|| true);
    let password_valid= use_state(|| true);

    let form_valid = use_state(|| true);

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
        let email_value = email_value.clone();
        let password_value = password_value.clone();
        let form_valid = form_valid.clone();
        Callback::from(move |e: MouseEvent| {
            if *email_valid && *password_valid {
                form_valid.set(true);
                let mut login = (*login_user).clone();
                login.email = format!("{}", *email_value);
                login.password = format!("{}", *password_value);
                log::debug!("Valid login info {:?}", login); 
                login_user.set(login);
                e.prevent_default(); /* Prevent event propagation */
                login_request.run();
            } else {
                form_valid.set(false);
                /*
                if let Some(email_input) = utils::document_element()
                    .get_elements_by_class_name("input")
                    .get_with_name("email") 
                {
                    log::debug!("email input {:?}", email_input.to_string()); 
                    let input: HtmlInputElement= email_input.unchecked_into();
                    log::debug!("input {:?}", input.value()); 

                }
                */
            }
        })
    };


    
    let update_email_value = {
        let email_valid = email_valid.clone();
        let email_value = email_value.clone();
        Callback::from(move |s: String| {
            let check_s = format!("{s}");
            email_valid.set(validate_email(check_s));
            email_value.set(s)
        })
    };

    let update_password_value= {
        let password_valid = password_valid.clone();
        let password_value = password_value.clone();
        Callback::from(move |s: String| {
            let check_s = format!("{s}");
            password_valid.set(validate_length(check_s, Some(6), Some(128), None));
            password_value.set(s);
        })
    };

    {
        let email_value = email_value.clone();
        let password_value = password_value.clone();
    html! {
        <ybc::Box>
            if !*(form_valid) {
                <p class="help is-danger"> {"Formulario invalido o incompleto,"} </p>
                <p class="help is-danger"> {"por favor corrige los datos"} </p>
            }
            <ybc::Field 
                label={Some("Email")}
                icons_right={!(*email_valid)}
                help_has_error={!(*email_valid)}
                help={(!(*email_valid)).then(|| "Correo electronico invalido".to_string())}
            >
                <ybc::Control
                    classes={classes!(
                            "has-icons-right"
                            )}
                    expanded=true
                >
                    <ybc::Input
                        name="email"
                        placeholder="e.g. alex@example.com"
                        value={(*email_value).to_owned()}
                        update={update_email_value}
                        classes={classes!(
                                (!(*email_valid))
                                .then(|| "is-danger")
                                )}
                    >
                    </ybc::Input>
                    if !(*email_valid) {
                        <span class="icon is-small is-right">
                            <i class="fa-solid fa-triangle-exclamation"></i>
                        </span>
                    }
                </ybc::Control>
            </ybc::Field>


            <ybc::Field 
                label={Some("Contraseña")}
                icons_left=true
                icons_right={!(*password_valid)}
                help_has_error={!(*password_valid)}
                help={(!(*password_valid)).then(|| "Password debe consistir de entre 6 y 128 caracteres".to_string())}
            >
                <ybc::Control
                    classes={classes!(
                            "has-icons-right",
                            "has-icons-left"
                            )}
                    expanded=true
                >
                    <span class="icon is-small is-left">
                        <i class="fa-solid fa-lock"></i>
                    </span>
                    <ybc::Input
                        name="password"
                        placeholder="********"
                        r#type={ybc::InputType::Password}
                        value={(*password_value).to_owned()}
                        update={update_password_value}
                        classes={classes!(
                                (!(*password_valid))
                                .then(|| "is-danger")
                                )}
                    >
                    </ybc::Input>
                    if !(*password_valid) {
                        <span class="icon is-small is-right">
                            <i class="fa-solid fa-triangle-exclamation"></i>
                        </span>
                    }
                </ybc::Control>
            </ybc::Field>

            <ybc::Button
                classes={classes!(
                    "is-primary"
                    )}
                onclick={onsubmit}
            >
                {"Iniciar sesion"}
            </ybc::Button>
        </ybc::Box>
    }
    }
}
