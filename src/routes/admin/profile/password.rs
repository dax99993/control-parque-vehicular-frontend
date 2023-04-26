use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::card::{Card, CardContent};
use crate::components::profile::password::ProfileChangePassword;

use crate::services::profile::request_change_password;


#[function_component]
fn ChangePassword() -> Html {
    html!{
        <div class="card">
            <header class="card-header">
               <p class="card-header-title">
                    <span class="icon">
                        <i class="fa-solid fa-lock"></i>
                    </span>
                    {"Cambiar contraseña"}
               </p> 
            </header>
            <div class="card-content">
                <ChangePasswordForm/>
            </div>
        </div>
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use validator::{ValidationErrors, Validate};

use common::models::user::ChangePasswordMe;

use crate::shadow_clone;
use crate::utils::forms::{validate_form_field, reset_input};

#[function_component]
pub fn ChangePasswordForm() -> Html {
    let password = use_state(|| ChangePasswordMe::default());
    let password_validation = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));

    let onchange_current_password= get_input_callback("current_password", &password);
    let onchange_new_password = get_input_callback("new_password", &password);
    let onchange_re_new_password = get_input_callback("re_new_password", &password);

    
    let current_password = NodeRef::default();
    let new_password= NodeRef::default();
    let re_new_password= NodeRef::default();

    
    let validate_input_on_blur = {
        shadow_clone![password, password_validation];
        Callback::from(move |(name, value): (String, String)| {
            set_form_field(name.as_str(), value, &password);
            validate_form_field(name.as_str(), &password, &password_validation);
        })
    };

    
    let request_change_password_me = {
        shadow_clone![password];
        use_async(async move {
            request_change_password((*password).clone()).await
        })
    };

    // Submit valid form
    let onsubmit = {
        shadow_clone![password, password_validation];
        shadow_clone![current_password, new_password, re_new_password];
        shadow_clone![request_change_password_me];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            match password.validate() {
                Ok(_) => {
                    reset_input(&current_password);
                    reset_input(&new_password);
                    reset_input(&re_new_password);
                    // make request to database
                    request_change_password_me.run();
                }
                Err(e) => {
                    password_validation.set(Rc::new(RefCell::new(e)));
                }
            }
        })
    };


    {
        //shadow_clone![request_change_password];
        use_effect_with_deps(move |request| {
            if let Some(response) = &request.data {
                log::debug!("api response\n{:?}", &response);
                /*
                if let Some(vehicule) = &response.data {
                    log::debug!("successful vehicule creation\n{:?}", vehicule);
                    user_ctx.redirect_to(AppRoute::VehiculeEdit { id: vehicule.vehicule_id.clone() });
                }
                */
            }
            if let Some(api_error) = &request.error {
                log::error!("change password request error\n{:?}", api_error);
            }
        },
        request_change_password_me.clone())
    }

    // reset all form fields
    let onreset = {
        shadow_clone![password, password_validation];
        shadow_clone![current_password, new_password, re_new_password];
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            password.set(ChangePasswordMe::default());
            password_validation.set(Rc::new(RefCell::new(ValidationErrors::new())));

            reset_input(&current_password);
            reset_input(&new_password);
            reset_input(&re_new_password);
        })
    };

    html!{
        <Card header_icon_left={"fa-solid fa-lock"} header_title={"Cambiar Contraseña"}>
            <CardContent>
                <ProfileChangePassword
                    {onchange_current_password}
                    {onchange_new_password}
                    {onchange_re_new_password}

                    {current_password}
                    {new_password}
                    {re_new_password}

                    handle_on_input_blur={validate_input_on_blur}
                    validation_errors={&*password_validation}

                    {onsubmit}
                    {onreset}
                />
            </CardContent>
        </Card>
    }
}


fn get_input_callback(
    name: &'static str,
    form: &UseStateHandle<ChangePasswordMe>,
) -> Callback<String> {
    let cloned_form = form.clone();
    Callback::from(move |value| {
        set_form_field(name, value, &cloned_form);
    })
}

fn set_form_field<'a>(
    name: &'a str,
    value: String,
    form: &UseStateHandle<ChangePasswordMe>,)
{
    let mut data = form.deref().clone();
    match name {
        "current_password" => data.current_password = value,
        "new_password" => data.new_password = value,
        "re_new_password" => data.re_new_password = value,
        _ => (),
    }
    log::debug!("Onblur update data {:?}", &data); 
    form.set(data);
}
