use std::rc::Rc;

use yew::prelude::*;

pub enum VehiculeItemAction {
    Delete(String),
    Edit(String),
    Get(String),
    SetNone,
    //BulkDelete(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VehiculeItemState {
    pub vehicule_id: Option<String>,
    //pub selected_vehicules: Vec<String>,
}


impl Reducible for VehiculeItemState {
    type Action = VehiculeItemAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let id = match action {
            Self::Action::Delete(id) => Some(id),
            Self::Action::Edit(id) => Some(id),
            Self::Action::Get(id) => Some(id),
            Self::Action::SetNone => None,
        };

        Self { vehicule_id: id }.into()
    }
}

pub type VehiculeItemContext = UseReducerHandle<VehiculeItemState>;

#[derive(Debug, Properties, PartialEq)]
pub struct VehiculeItemStateProviderProps {
    #[prop_or_default]
    pub children: Children
}


#[function_component]
pub fn VehiculeItemProvider(props: &VehiculeItemStateProviderProps) -> Html {
    let ctx = use_reducer(|| VehiculeItemState::default());

    html!{
        <ContextProvider<VehiculeItemContext> context={ctx}>
            {props.children.clone()}
        </ContextProvider<VehiculeItemContext>>
    }
}









