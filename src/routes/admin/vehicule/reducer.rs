use uuid::Uuid;

use yew::prelude::*;
use yew::platform::spawn_local;
use yew_router::prelude::Navigator;

use common::models::vehicule::Vehicule;

use crate::routes::AppRoute;
use crate::utils::modal::{open_modal, close_modal};


use crate::services::vehicule::{
    request_admin_delete_vehicule,
    request_admin_update_vehicule_picture
};

pub enum VehiculeAction {
    AddNavigator(Option<Navigator>),
    AddVehicule,
    DeleteVehicule(Uuid),
    GetVehicules(Vec<Vehicule>),
    UploadPicture(Uuid, reqwest::multipart::Form),
    UpdateInfo(Uuid),
    ShowPicture(Uuid),
    SetVehiculePerPage(usize),
    GoToPage(usize),
    ResetModal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VehiculeReducer {
    pub vehicules: Vec<Vehicule>, 
    pub selected_vehicule_id: Option<String>,
    pub modal_title: Option<String>,
    pub modal_body: Option<Html>,
    pub modal_footer: Option<Html>,
    pub modal_onclick: Option<Callback<MouseEvent>>,
    pub vehicules_per_page: usize,
    pub current_page: usize,
    pub total_pages: usize,
    pub current_page_vehicules: Vec<Vehicule>, 
    pub navigator: Option<Navigator>,
}

impl Default for VehiculeReducer {
    fn default() -> Self {
        Self {
            vehicules: vec![],
            selected_vehicule_id: None,
            modal_title: None,
            modal_body: None,
            modal_footer: None,
            modal_onclick: None,
            vehicules_per_page: 4,
            current_page: 1,
            total_pages: 0,
            current_page_vehicules: vec![],
            navigator: None, 
        }
    }
}

impl Reducible for VehiculeReducer {
    type Action = VehiculeAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut vehicules = self.vehicules.clone();
        let mut modal_title = self.modal_title.clone();
        let mut modal_body = self.modal_body.clone();
        let mut modal_footer = self.modal_footer.clone();
        let mut modal_onclick = self.modal_onclick.clone();
        let mut vehicules_per_page = self.vehicules_per_page.clone();
        let mut current_page = self.current_page.clone();
        let mut total_pages = self.total_pages.clone();
        let mut current_page_vehicules = self.current_page_vehicules.clone();
        let mut navigator = self.navigator.clone();

        let id = self.selected_vehicule_id.clone();
        
        match action {
            VehiculeAction::AddNavigator(nav) => {
                navigator = nav;
            }
            VehiculeAction::AddVehicule => {
                if let Some(nav) = navigator.clone() {
                    nav.push(&AppRoute::VehiculeAdd);
                } else {
                    log::error!("navigator is None!");
                }
            }
            VehiculeAction::UpdateInfo(id) => {
                if let Some(nav) = navigator.clone() {
                    nav.push(&AppRoute::VehiculeEdit { id });
                } else {
                    log::error!("navigator is None!");
                }
            }
            VehiculeAction::DeleteVehicule(id) => {
                open_modal("vehicule-modal".to_string());
                if let Some(v) = vehicules.iter().filter(|v| v.vehicule_id.eq(&id)).map(|v| v).next() {
                    log::debug!("should delete vehicule with id = {id}");
                    let msg = format!("Realmente desea borrar el vehiculo {}", &v.branch);

                    modal_body = Some(
                        html!{<p><b>{ msg }</b></p>}
                    );
                    modal_footer = Some(
                        html!{
                            <>
                            <button class="button jb-modal-close" onclick={
                                Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    close_modal("vehicule-modal".to_string());
                                })
                            }>
                            { "Cancelar" }
                            </button>
                            <button class="button is-danger jb-modal-close" onclick={
                                let id = v.vehicule_id.clone();
                                Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    // Execute api
                                    spawn_local(async move {
                                        log::debug!("will delete vehicule with id {}", id.to_string());
                                        let response = request_admin_delete_vehicule(id.to_string()).await;
                                        match response {
                                            Ok(_) => {
                                                close_modal("vehicule-modal".to_string());
                                            }
                                            Err(_) => {
                                                log::error!("delete vehicule request failed");
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
                            log::debug!("DeleteVehicule onclose cb");
                        })
                    )
                };
            }
            VehiculeAction::GetVehicules(new_vehicules) => {
                vehicules = new_vehicules;
            }
            VehiculeAction::UploadPicture(vehicule_id, multipart) => {
                spawn_local(async move {
                    let response = request_admin_update_vehicule_picture(vehicule_id.to_string(), multipart).await;
                    match response {
                        Ok(r) => {
                            log::debug!("image uploaded successfully\n {:?}", r);
                        }
                        Err(e) => {
                            log::error!("image upload request failed\n {:?}", e);
                        }
                    }
                });
            }
            VehiculeAction::ShowPicture(id) => {
                open_modal("vehicule-modal".to_string());
                if let Some(vehicule) = vehicules.iter().filter(|v| v.vehicule_id.eq(&id)).map(|v| v).next() {
                    log::debug!("Vehicule selected {:?}", &vehicule);
                    let picture_url = vehicule.get_picture_url("http://127.0.0.1:8000/");
                    modal_body = Some(
                        html!{
                            <img src={picture_url} />
                        }
                    );
                    modal_footer = None;
                }
            }
            VehiculeAction::SetVehiculePerPage(i) => {
                vehicules_per_page = i; 
            }
            VehiculeAction::GoToPage(page) => {
                log::debug!("go to page {:?}", page);
                current_page = page;
                let pages: f64 = vehicules.len() as f64 / vehicules_per_page as f64;
                total_pages = pages.ceil() as usize;
                log::debug!("vehicules set\n {:?}", &vehicules);
                log::debug!("total pages set\n {:?}", &vehicules);
                // Split vec into pages and get current
                let split: Vec<&[Vehicule]> = vehicules.chunks(vehicules_per_page)
                    .into_iter()
                    .collect();
                // Remember 0 indexing!
                if let Some(vehicules_page) = split.get(current_page - 1) {
                    let vp: Vec<Vehicule> = vehicules_page.into_iter()
                        .map(|v| v.clone())
                        .collect();
                    current_page_vehicules = vp;
                    log::debug!("vehicules current page\n {:?}", current_page_vehicules);
                }
            }
            VehiculeAction::ResetModal => {
                //close_modal("vehicule-modal".to_string()).emit(MouseEvent::new("click").unwrap());
                close_modal("vehicule-modal".to_string());
                modal_title = None;
                modal_body = None;
                modal_footer = None;
                //modal_onclick = None;
            }
        }

        Self {
            //vehicules: vehicules,
            vehicules,
            selected_vehicule_id: id,
            modal_title,
            modal_body,
            modal_footer,
            modal_onclick,
            vehicules_per_page,
            current_page,
            total_pages,
            current_page_vehicules,
            navigator,
        }.into()    
    }
}


/*
fn update_current_page(Vec<Vehicule>) -> Vec<Vehicule> {


}
*/
