use yew::prelude::*;
use yew_router::prelude::Navigator;

use uuid::Uuid;

use crate::routes::AppRoute;
use crate::utils::modal::open_modal;


pub enum UsersAction {
    // Nav 
    AddNavigator(Option<Navigator>),
    // Redirect
    UpdateInfo(Uuid),
    // Modal action
    DeleteUser(Uuid),
    ShowUserPicture(Uuid),
    // Reset 
    ResetDeleteSelectedUser,
    ResetShowPictureSelectedUser,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UsersReducer {
    // Nav
    pub navigator: Option<Navigator>,

    //pub selected_user_id: Option<String>,
    pub selected_user_to_delete_id: Option<Uuid>,
    pub selected_user_to_show_id: Option<Uuid>,
}

impl Default for UsersReducer {
    fn default() -> Self {
        Self {
            navigator: None, 

            //selected_user_id: None,
            selected_user_to_delete_id: None,
            selected_user_to_show_id: None,

        }
    }
}

impl Reducible for UsersReducer {
    type Action = UsersAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut navigator = self.navigator.clone();

        //let id = self.selected_user_id.clone();
        let mut selected_user_to_delete_id = self.selected_user_to_delete_id.clone();
        let mut selected_user_to_show_id = self.selected_user_to_show_id.clone();

        
        match action {
            // Nav
            UsersAction::AddNavigator(nav) => {
                navigator = nav;
            }

            // Redirect
            UsersAction::UpdateInfo(id) => {
                if let Some(nav) = navigator.clone() {
                    nav.push(&AppRoute::UserEdit{ id });
                } else {
                    log::error!("navigator is None!");
                }
            }

            // Modal action
            UsersAction::DeleteUser(id) => {
                open_modal("user-delete-modal".to_string());
                selected_user_to_delete_id = Some(id);
            }
            UsersAction::ShowUserPicture(id) => {
                open_modal("user-picture-modal".to_string());
                selected_user_to_show_id = Some(id);
            }

            // Reset
            UsersAction::ResetDeleteSelectedUser => {
                //close_modal("users-delete-modal".to_string());
                selected_user_to_delete_id = None;
            }
            UsersAction::ResetShowPictureSelectedUser => {
                //close_modal("users-picture-modal".to_string());
                selected_user_to_show_id = None;
            }
        }

        Self {
            navigator,
            //selected_user_id: id,
            selected_user_to_delete_id,
            selected_user_to_show_id,
        }.into()    
    }
}

