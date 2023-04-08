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
