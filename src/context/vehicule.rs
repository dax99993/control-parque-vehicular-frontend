use std::rc::Rc;

use yew::prelude::*;

pub enum VehiculeAction {
    Delete,
    Edit,
    BulkDelete,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VehiculeDeleteId {
    pub vehicule_id: String,
    pub selected_vehicules: Vec<String>,
}










