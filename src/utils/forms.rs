use std::rc::Rc;
use std::cell::RefCell;
use validator::{Validate, ValidationErrors};
use yew::{UseStateHandle, NodeRef};
use web_sys::HtmlInputElement;

/// Validate form and update validation errors state
pub fn validate_form_field<'a, T: Validate>(
    name: &'a str,
    form: &UseStateHandle<T>,
    validation_errors: &UseStateHandle<Rc<RefCell<ValidationErrors>>>)
where 
    T: Validate + Clone
{
    let mut has_error = false;
    match form.validate() {
        Ok(_) => {
            //validation_errors 
            //    .borrow_mut()
            //    .errors_mut()
            //    .retain(|key, _| key != &name);
            validation_errors.set(Rc::new(RefCell::new(ValidationErrors::new())));
            log::debug!("Onblur validation Ok {:?}", &validation_errors); 
        }
        Err(errors) => {
            /*
            for(field_name, error) in errors.errors() {
                if field_name == &name {
                    validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .insert(field_name.clone(), error.clone());
                }
            }
            */
            /*
            let mut my_errors = 
                validation_errors
                    .borrow_mut();
            if let Some((field_name, error)) = errors.errors().get_key_value(&name) {
                my_errors.errors_mut().insert(field_name.clone(), error.clone());
            } else {
                my_errors.errors_mut().retain(|key, _| key != &name);
            }
            */
            for(field_name, error) in errors.errors() {
                if field_name == &name {
                    has_error = true;
                    validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .insert(field_name.clone(), error.clone());
                }
            }
            if  !has_error {
                validation_errors 
                    .borrow_mut()
                    .errors_mut()
                    .retain(|key, _| key != &name);

            }
            log::debug!("Onblur validation Err {:?}", &validation_errors); 
        }
    }
}

/// Reset value of node referencing a html input element
pub fn reset_input(node_ref: &NodeRef) {
    if let Some(element) = node_ref.cast::<HtmlInputElement>() {
        element.set_value("")
    }
}

pub fn set_input_value<'a>(value: &'a str, node_ref: &NodeRef) {
    if let Some(element) = node_ref.cast::<HtmlInputElement>() {
        element.set_value(value);
    }
}
