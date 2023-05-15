use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::use_navigator;
use yew::platform::spawn_local;
use std::ops::Deref;

use common::models::user::Usuario;

use super::{UsersTable, UsersTableRow};
use crate::features::users::reducer::{UsersAction, UsersReducer};
use crate::features::users::services::{request_admin_get_users, request_admin_delete_user};

use crate::utils::close_modal;

use crate::shadow_clone;
use crate::components::card::{Card, CardContent};
use crate::components::modal::Modal;
use crate::components::pagination::Pagination;
use crate::hooks::user_context::use_user_context;
use crate::layout::main_section::MainSection;


#[function_component]
pub fn AdminUsersView() -> Html {
    // Context 
    let user_ctx = use_user_context();

    if !user_ctx.is_authenticated() {
        user_ctx.redirect_home();
    }

    // Hooks
    let reducer = use_reducer(UsersReducer::default);
    let users = use_state(|| vec![]);
    let current_page = use_state(|| 1);
    let total_pages = use_state(|| 1);
    let navigator = use_navigator();


    // Add navigator
    {
        shadow_clone![reducer, navigator];
        use_effect_with_deps(move |nav| {
            reducer.dispatch(UsersAction::AddNavigator(nav.clone()));
        },
        navigator);
    }


    // Api fetch request
    let request_get_users = {
        use_async(async {
            request_admin_get_users().await
        })
    };

    // Fetch api when rendered
    {
        shadow_clone!(request_get_users);
        use_effect_with_deps(move |_| {
            request_get_users.run()
        },
        ());
    }

    // Update users vector when fetching from api
    {
        shadow_clone![users];
        use_effect_with_deps(
            move |request| {
                if let Some(api_response) = &request.data {
                    log::debug!("users successful api response\n {:?}", &api_response);
                    if let Some(vec_users) = api_response.data.clone() {
                        users.set(vec_users);
                    }
                }
                if let Some(api_response) = &request.error {
                    log::warn!("users failed api response\n {:?}", &api_response);
                }
            },
            request_get_users.clone() 
        );
    }
    

    // Re-fetch api when clicking on button
    let onclick_reload_table = {
        shadow_clone!(request_get_users);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); 
            request_get_users.run();
        })
    };



    // Modal 
    let user_picture = {
        match reducer.selected_user_to_show_id {
            Some(id) => {
                if let Some(user) = users.deref().iter().filter(|u| u.usuario_id.eq(&id)).map(|u| u).next() {
                    log::debug!("usuario seleccionado {:?}", &user);
                    let picture_url = user.imagen_url("http://127.0.0.1:8000/");
                    html!{
                        <img src={picture_url} />
                    }
                } else {
                    html!{}
                }
            },
            None => html!{},
        }
    };

    let onclick_delete = {
        //shadow_clone![reload_table, vehiculos];
        shadow_clone![users];
        let selected_user_to_delete_id = reducer.selected_user_to_delete_id.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // Execute api
            //shadow_clone![reload_table, vehiculos];
            shadow_clone![users];
            if let Some(id) = selected_user_to_delete_id {
                spawn_local(async move {
                    log::debug!("se borrara el usuario con id {}", id.to_string());
                    let response = request_admin_delete_user(id.to_string()).await;
                    match response {
                        Ok(_) => {
                            close_modal("user-delete-modal".to_string());
                            // Delete row from table
                            let u: Vec<Usuario> = users.deref().clone()
                                .into_iter()
                                .filter(|u| u.usuario_id.ne(&id))
                                .collect();
                            users.set(u);
                            // Or we can reload current page
                            //reload_table.set(!(*reload_table));
                        }
                        Err(_) => {
                            log::error!("Peticion de borrar usuario fallo");
                        }
                    }
                });
            }
        })
    };


    //HTML
    html! {
        <MainSection route="Admin" subroute="Usuarios" title="Usuarios">
            <Card classes={classes!["has-table"]}
                header_icon_left={ "fa-solid fa-user" } header_title={ "Usuarios" } 
                header_icon_right={ "fa-solid fa-rotate-right" } header_icon_right_label={ "Recargar tabla" }
                header_icon_right_onclick={ onclick_reload_table } 
            >
                <CardContent>
                    <UsersTable>
                        {
                            users_to_user_table_rows(users.deref().clone(), reducer.dispatcher())
                        }
                    </UsersTable>
                </CardContent>

            </Card>

            <Pagination 
                total_pages = { *total_pages }
                current_page_state = { current_page.clone() }
            />

            <Modal 
                id={"user-picture-modal"}
                title={""}
                body={ user_picture }
                onclose={
                    shadow_clone![reducer];
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        reducer.dispatch(UsersAction::ResetShowPictureSelectedUser);
                    })
                }
            >
           </Modal>

            <Modal 
                id={"user-delete-modal"}
                title={"Borrar Usuario"}
                body={ html!{<p><b>{ "Realmente desea borrar el vehiculo" }</b></p>} }
                footer={
                        html!{
                            <>
                            <button class="button jb-modal-close" onclick={
                                Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    close_modal("user-delete-modal".to_string());
                                })
                            }>
                            { "Cancelar" }
                            </button>

                            <button class="button is-danger jb-modal-close" onclick={onclick_delete}>
                            { "Borrar" }
                            </button>
                            </>
                        }
                }
                onclose={
                    shadow_clone![reducer];
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        reducer.dispatch(UsersAction::ResetDeleteSelectedUser);
                    })
                }
            >
           </Modal>

        </MainSection>
    }
}


fn users_to_user_table_rows(users: Vec<Usuario>, dispatcher: UseReducerDispatcher<UsersReducer>) -> Vec<Html> {
    users.into_iter().map(|u| {
        html!{
            <UsersTableRow
                user={u}
                dispatcher={dispatcher.clone()}
            >
            </UsersTableRow>
        }
    })
    .collect()
}
