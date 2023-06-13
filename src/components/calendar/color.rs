

#[derive(Clone, PartialEq, Default)]
pub enum CalendarColor {
    #[default]
    Primary,
    Info,
    Success,
    Warning,
    Danger,
    Grey,
    Dark,
    Black,
}

impl CalendarColor {
    pub fn as_classname(&self) -> Vec<String> {
        match self {
           Self::Primary => vec!["is-primary".to_string()],
           Self::Info => vec!["is-info".to_string()],
           Self::Success => vec!["is-success".to_string()],
           Self::Warning => vec!["is-warning".to_string()],
           Self::Danger => vec!["is-danger".to_string()],
           Self::Grey => vec!["is-grey".to_string()],
           Self::Dark => vec!["is-dark".to_string()],
           Self::Black => vec!["is-black".to_string()],
        }
    }
}
