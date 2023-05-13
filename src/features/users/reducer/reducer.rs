use yew::prelude::*;
use yew::platform::spawn_local;
use yew_router::prelude::Navigator;

use uuid::Uuid;

use common::models::user::Usuario;

use crate::routes::AppRoute;
use crate::utils::modal::{open_modal, close_modal};

use super::super::services::request_admin_delete_user;


pub enum UsersAction {
    AddNavigator(Option<Navigator>),
    DeleteUser(Uuid),
    GetUsers(Vec<Usuario>),
    UpdateInfo(Uuid),
    ShowPicture(Uuid),
    SetVehiculePerPage(usize),
    GoToPage(usize),
    ResetModal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UsersReducer {
    pub users: Vec<Usuario>, 
    pub selected_user_id: Option<String>,
    pub modal_title: Option<String>,
    pub modal_body: Option<Html>,
    pub modal_footer: Option<Html>,
    pub modal_onclick: Option<Callback<MouseEvent>>,
    pub users_per_page: usize,
    pub current_page: usize,
    pub total_pages: usize,
    pub current_page_users: Vec<Usuario>, 
    pub navigator: Option<Navigator>,
}

impl Default for UsersReducer {
    fn default() -> Self {
        Self {
            users: vec![],
            selected_user_id: None,
            modal_title: None,
            modal_body: None,
            modal_footer: None,
            modal_onclick: None,
            users_per_page: 4,
            current_page: 1,
            total_pages: 0,
            current_page_users: vec![],
            navigator: None, 
        }
    }
}

impl Reducible for UsersReducer {
    type Action = UsersAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut users = self.users.clone();
        let mut modal_title = self.modal_title.clone();
        let mut modal_body = self.modal_body.clone();
        let mut modal_footer = self.modal_footer.clone();
        let mut modal_onclick = self.modal_onclick.clone();
        let mut users_per_page = self.users_per_page.clone();
        let mut current_page = self.current_page.clone();
        let mut total_pages = self.total_pages.clone();
        let mut current_page_users = self.current_page_users.clone();
        let mut navigator = self.navigator.clone();

        let id = self.selected_user_id.clone();
        
        match action {
            UsersAction::AddNavigator(nav) => {
                navigator = nav;
            }
            UsersAction::UpdateInfo(id) => {
                if let Some(nav) = navigator.clone() {
                    nav.push(&AppRoute::UserEdit{ id });
                } else {
                    log::error!("navigator is None!");
                }
            }
            UsersAction::DeleteUser(id) => {
                open_modal("user-modal".to_string());
                if let Some(u) = users.iter().filter(|u| u.usuario_id.eq(&id)).map(|u| u).next() {
                    log::debug!("should delete vehicule with id = {id}");
                    let msg = format!("Realmente desea borrar el vehiculo {}", &u.nombres);

                    modal_body = Some(
                        html!{<p><b>{ msg }</b></p>}
                    );
                    modal_footer = Some(
                        html!{
                            <>
                            <button class="button jb-modal-close" onclick={
                                Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    close_modal("user-modal".to_string());
                                })
                            }>
                            { "Cancelar" }
                            </button>
                            <button class="button is-danger jb-modal-close" onclick={
                                let id = u.usuario_id.clone();
                                Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    // Execute api
                                    spawn_local(async move {
                                        log::debug!("will delete user with id {}", id.to_string());
                                        let response = request_admin_delete_user(id.to_string()).await;
                                        match response {
                                            Ok(_) => {
                                                close_modal("user-modal".to_string());
                                            }
                                            Err(_) => {
                                                log::error!("delete user request failed");
                                            }
                                        }
                                    });
                                })
                            }>
                            { "Borrar" }
                            </button>
                            </>
                        }
                    );
                }
                modal_onclick = {
                    Some(
                        Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            log::debug!("DeleteUser onclose cb");
                        })
                    )
                };
            }
            UsersAction::GetUsers(new_users) => {
                users = new_users;
            }
            UsersAction::ShowPicture(id) => {
                open_modal("users-modal".to_string());
                if let Some(user) = users.iter().filter(|u| u.usuario_id.eq(&id)).map(|u| u).next() {
                    log::debug!("User selected {:?}", &user);
                    let imagen_url = user.imagen_url("http://127.0.0.1:8000/");
                    modal_body = Some(
                        html!{
                            <img src={imagen_url} />
                        }
                    );
                    modal_footer = None;
                }
            }
            UsersAction::SetVehiculePerPage(i) => {
                users_per_page = i; 
            }
            UsersAction::GoToPage(page) => {
                log::debug!("go to page {:?}", page);
                current_page = page;
                let pages: f64 = users.len() as f64 / users_per_page as f64;
                total_pages = pages.ceil() as usize;
                log::debug!("vehicules set\n {:?}", &users);
                log::debug!("total pages set\n {:?}", &total_pages);
                // Split vec into pages and get current
                let split: Vec<&[Usuario]> = users.chunks(users_per_page)
                    .into_iter()
                    .collect();
                // Remember 0 indexing!
                if let Some(users_page) = split.get(current_page - 1) {
                    let vp: Vec<Usuario> = users_page.into_iter()
                        .map(|v| v.clone())
                        .collect();
                    current_page_users = vp;
                    log::debug!("users current page\n {:?}", current_page_users);
                }
            }
            UsersAction::ResetModal => {
                //close_modal("vehicule-modal".to_string()).emit(MouseEvent::new("click").unwrap());
                close_modal("users-modal".to_string());
                modal_title = None;
                modal_body = None;
                modal_footer = None;
                //modal_onclick = None;
            }
        }

        Self {
            //vehicules: vehicules,
            users,
            selected_user_id: id,
            modal_title,
            modal_body,
            modal_footer,
            modal_onclick,
            users_per_page,
            current_page,
            total_pages,
            current_page_users,
            navigator,
        }.into()    
    }
}

