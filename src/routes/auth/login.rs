use yew::prelude::*;
use web_sys::HtmlInputElement;

use crate::services::auth::{request_login, request_me};
use crate::services::request::store_token;
use crate::hooks::user_context::use_user_context;
use crate::types::user::LoginUser;


use crate::components::form::{Form, FormField, FormInputField };

use yew_hooks::prelude::*;
use validator::{validate_email, validate_length};

use crate::utils::FormFieldState;
use crate::{oninput_macro, shadow_clone};
//use gloo::utils;


#[function_component]
pub fn Login() -> Html {

    // Context
    let user_ctx = use_user_context();
    if user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }
    // States
    let login_user = use_state(|| LoginUser::default());
    let login_user_valid = use_state(|| bool::default());

    let email = use_state(|| FormFieldState::default());
    let oninput_email = oninput_macro!(email, validate_email);

    let password = use_state(|| FormFieldState::default());
    let oninput_password = oninput_macro!(password, validate_password);


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
        shadow_clone![login_user, login_user_valid];
        shadow_clone![email, password];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); /* Prevent event propagation */

            if *login_user_valid {
                let mut login = (*login_user).clone();
                login.email = (*email).value.clone();
                login.password = (*password).value.clone();
                login_user.set(login);
                login_request.run();
            }
            log::debug!("Valid login info {:?}", *login_user); 
        })
    };

    {
        shadow_clone!(login_user_valid);
        shadow_clone![email, password];
        use_effect_with_deps(move |(email, password)| {
            let valid = (*email).valid && (*password).valid;
            login_user_valid.set(valid);
        }, 
        (email.clone(), password.clone()) )
    }

    html! {
    <section class="hero is-fullheight is-primary">
        <div class="hero-body">
            <div class="container"> 
                <div class="box is-centered">
                    <Form method="post">
                        <FormField label="Email">
                            <FormInputField 
                                input_type="text"
                                placeholder="e.g. alex@example.com"
                                danger_msg="Campo Obligatorio"
                                oninput={oninput_email.clone()}
                                value={(*email).value.clone()}
                                valid={(*email).valid}
                                icon_left={"fa-solid fa-envelope"}
                                icon_right={ if !(*email).valid { "fa-solid fa-triangle-exclamation" } else { "" } }
                            />
                        </FormField>

                        <FormField label="Contraseña">
                            <FormInputField 
                                input_type="password"
                                danger_msg="Campo Obligatorio"
                                oninput={oninput_password.clone()}
                                value={(*password).value.clone()}
                                valid={(*password).valid}
                                icon_left={"fa-solid fa-lock"}
                                icon_right={ if !(*password).valid { "fa-solid fa-triangle-exclamation" } else { "" } }
                            />
                        </FormField>

                        <hr/>

                        <FormField>
                            <div class="control">
                                <button type="button" onclick={ onsubmit }
                                    class={classes!["button", if (*login_user_valid).clone() { "is-primary"} else { "is-danger" }]}
                                >
                                    <span>{ "Iniciar sesion" }</span>
                                </button>
                            </div>
                            if !(*login_user_valid) {
                                <p class="help is-danger"> {"Formulario invalido o incompleto,"} </p>
                                <p class="help is-danger"> {"por favor corrige los datos"} </p>
                            }
                        </FormField>

                    </Form>
                </div>
            </div>
        </div>
    </section>
    }
}

fn validate_password(s: String) -> bool {
    validate_length(s, Some(6), Some(128), None)
}
