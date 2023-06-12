use yew::prelude::*;
use yew_router::prelude::Navigator;

use uuid::Uuid;

//use common::models::vehicule::Vehicule;

use crate::routes::AppRoute;
use crate::utils::modal::{open_modal, close_modal};


pub enum VehiculeTableAction {
    //SelectVehicule(Uuid),
    //RedirectToAddVehicule,
    AddNavigator(Navigator),
    UpdateVehicule(Uuid),
    ShowVehiculePicture(Uuid),
    DeleteVehicule(Uuid),
    ResetSelectedDelete,
    ResetSelectedShow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VehiculeTableReducer {
    //pub selected_vehicules: Vec<Uuid>, 
    pub selected_vehicule_to_delete_id: Option<Uuid>,
    pub selected_vehicule_to_show_id: Option<Uuid>,

    //pub selected_vehicule: Option<Vehicule>,
    pub navigator: Option<Navigator>,
}

impl Default for VehiculeTableReducer {
    fn default() -> Self {
        Self {
            //selected_vehicules: vec![],
            //selected_vehicule: None,
            selected_vehicule_to_delete_id: None,
            selected_vehicule_to_show_id: None,
            navigator: None, 
        }
    }
}

impl Reducible for VehiculeTableReducer {
    type Action = VehiculeTableAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        //let mut selected_vehicules = self.selected_vehicules.clone();
        //let mut selected_vehicule_id = self.selected_vehicule_id.clone();
        //let mut selected_vehicule = self.selected_vehicule.clone();
        let mut selected_vehicule_to_delete_id = self.selected_vehicule_to_delete_id.clone();
        let mut selected_vehicule_to_show_id = self.selected_vehicule_to_show_id.clone();
        let mut navigator = self.navigator.clone();

        
        match action {
            VehiculeTableAction::AddNavigator(nav) => {
                navigator = Some(nav);
            }
            VehiculeTableAction::UpdateVehicule(id) => {
                if let Some(nav) = navigator.clone() {
                    nav.push(&AppRoute::VehiculeEdit { id });
                } else {
                    log::error!("navigator is None!");
                }
            }
            VehiculeTableAction::DeleteVehicule(id) => {
                open_modal("vehicule-delete-modal".to_string());
                selected_vehicule_to_delete_id = Some(id);
            }
            VehiculeTableAction::ShowVehiculePicture(id) => {
                open_modal("vehicule-picture-modal".to_string());
                selected_vehicule_to_show_id = Some(id);
            }
            VehiculeTableAction::ResetSelectedDelete => {
                //close_modal("vehicule-delete-modal".to_string());
                selected_vehicule_to_delete_id = None;
            }
            VehiculeTableAction::ResetSelectedShow => {
                //close_modal("vehicule-picture-modal".to_string());
                selected_vehicule_to_show_id = None;
            }
        }

        Self {
            selected_vehicule_to_show_id,
            selected_vehicule_to_delete_id,
            navigator,
        }.into()    
    }
}

