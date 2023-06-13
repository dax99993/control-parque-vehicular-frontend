use yew::prelude::*;
use yew_router::prelude::Navigator;

use uuid::Uuid;

//use common::models::vehicule::Vehicule;

use crate::routes::AppRoute;
use crate::utils::modal::{open_modal, close_modal};


pub enum VehiculeTableAction {
    AddNavigator(Navigator),
    RequestVehicule(Uuid),
    ShowVehiculePicture(Uuid),
    ResetSelectedShow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VehiculeTableReducer {
    pub selected_vehicule_to_show_id: Option<Uuid>,
    pub navigator: Option<Navigator>,
}

impl Default for VehiculeTableReducer {
    fn default() -> Self {
        Self {
            selected_vehicule_to_show_id: None,
            navigator: None, 
        }
    }
}

impl Reducible for VehiculeTableReducer {
    type Action = VehiculeTableAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut selected_vehicule_to_show_id = self.selected_vehicule_to_show_id.clone();
        let mut navigator = self.navigator.clone();

        
        match action {
            VehiculeTableAction::AddNavigator(nav) => {
                navigator = Some(nav);
            }
            VehiculeTableAction::RequestVehicule(id) => {
                if let Some(nav) = navigator.clone() {
                    //nav.push(&AppRoute::VehiculeRequest { id });
                    log::debug!("redirect to vehicule request with id {id}");
                } else {
                    log::error!("navigator is None!");
                }
            }
            VehiculeTableAction::ShowVehiculePicture(id) => {
                open_modal("vehicule-picture-modal".to_string());
                selected_vehicule_to_show_id = Some(id);
            }
            VehiculeTableAction::ResetSelectedShow => {
                selected_vehicule_to_show_id = None;
            }
        }

        Self {
            selected_vehicule_to_show_id,
            navigator,
        }.into()    
    }
}

