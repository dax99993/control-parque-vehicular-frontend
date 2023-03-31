use web_sys::Element;


pub fn toggle_class(e: &Element, class: &str) {
    let classes = e.class_name();
    let mut classes: Vec<&str> = classes
        .split_whitespace()
        .collect();
    if classes.contains(&class) {
        classes.retain(|&c| c != class);
    } else {
        classes.push(class);
    };
    e.set_class_name(&classes.join(" "));
}

pub fn add_class(e: &Element, class: &str) {
    let classes = e.class_name();
    let mut classes: Vec<&str> = classes
        .split_whitespace()
        .collect();
    if !classes.contains(&class) {
        classes.push(class);
    }
    e.set_class_name(&classes.join(" "));
}

pub fn remove_class(e: &Element, class: &str) {
    let classes = e.class_name();
    let mut classes: Vec<&str> = classes
        .split_whitespace()
        .collect();
    if classes.contains(&class) {
        classes.retain(|&c| c != class);
    }
    e.set_class_name(&classes.join(" "));
}

pub fn has_class(e: &Element, class: &str) -> bool {
    let classes = e.class_name();
    let classes: Vec<&str> = classes
        .split_whitespace()
        .collect();
    return classes.contains(&class);
}

#[macro_export]
macro_rules! oninput_macro {
    ( $field_state:tt, $validator:expr) => {
        {
            let $field_state = $field_state.clone(); 
            Callback::from(move |e: InputEvent| {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = format!("{}", &input.value().trim());
                let valid = $validator(value.clone());
                let formfield = FormFieldState { value, valid };
                $field_state.set(formfield);
            })
        }
    };
}

#[macro_export]
macro_rules! shadow_clone {
    // single state shadow_clone!(model);
    ( $state:tt ) => {
        let $state = $state.clone(); 
    };
    // multiple state shadow_clone!(model, year);
    ( $($state:tt), * ) => {
        $(
            let $state = $state.clone(); 
        )*
    };

}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FormFieldState {
    pub value: String,
    pub valid: bool,
}

impl FormFieldState {
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}
