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
