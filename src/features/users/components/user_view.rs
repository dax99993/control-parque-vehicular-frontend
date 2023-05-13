use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::use_navigator;

use common::models::user::Usuario;

use super::{UsersTable, UsersTableRow};
use super::super::reducer::{UsersAction, UsersReducer};
use super::super::services::request_admin_get_users;

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
    let current_page = use_state(|| reducer.current_page);
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

    // Update vehicule vector when fetching from api
    {
        shadow_clone![reducer];
        use_effect_with_deps(
            move |request| {
                if let Some(api_response) = &request.data {
                    log::debug!("vehicules successful api response\n {:?}", &api_response);
                    if let Some(vec_users) = api_response.data.clone() {
                        reducer.dispatch(UsersAction::GetUsers(vec_users));
                    }
                }
                if let Some(api_response) = &request.error {
                    log::warn!("vehicules failed api response\n {:?}", &api_response);
                }
            },
            request_get_users.clone() 
        );
    }
    

    // Re-fetch api when clicking on button
    let onclick_add_vehicule = {
        //shadow_clone!(reducer);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); 
            //reducer.dispatch(VehiculeAction::AddVehicule);
        })
    };

    // Effect for keeping in sync Pagination state with reducer
    {
        shadow_clone![reducer, current_page];
        use_effect_with_deps(move |(current_page, _)| {
            log::debug!("effect pagination page = {:?}", current_page);
            reducer.dispatch(UsersAction::GoToPage(**current_page));
        },
        (current_page.clone(), request_get_users.clone())
        //current_page.clone()
        );
    }


    html! {
        <MainSection route="Admin" subroute="Usuarios" title="Usuarios">
            <Card classes={classes!["has-table"]}
                header_icon_left={ "fa-solid fa-user" } header_title={ "Usuarios" } 
                header_icon_right={ "fa-solid fa-plus" } header_icon_right_label={ "Agregar vehiculo" }
                header_icon_right_onclick={ onclick_add_vehicule } 
            >
                <CardContent>
                    <UsersTable>
                        {
                            users_to_user_table_rows(reducer.current_page_users.clone(), reducer.dispatcher())
                        }
                    </UsersTable>
                </CardContent>

            </Card>

            <Pagination 
                total_pages = { reducer.total_pages }
                current_page_state = { current_page.clone() }
            />

            <Modal 
                id={"user-modal"}
                title={reducer.modal_title.clone()}
                body={if reducer.modal_body.is_some() { reducer.modal_body.as_ref().unwrap().clone() } else {html!{}}}
                footer={reducer.modal_footer.clone()}
                onclose={
                    //if reducer.modal_onclick.is_some() 
                    //    { reducer.modal_onclick.as_ref().unwrap().clone() }
                    //else {
                        shadow_clone![reducer];
                        Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            reducer.dispatch(UsersAction::ResetModal);
                        })
                    //}
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
