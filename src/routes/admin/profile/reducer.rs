use yew::prelude::*;



#[derive(Debug, Clone, PartialEq)]
pub enum ProfileActions {
    ChangeInfo,
    ChangePassword,
    ChangePicture,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProfileReducer {
    
}


impl Reducible for ProfileReducer {
    type Action = ProfileActions;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        
        Self {
            
        }.into()
    }
}

