use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilteredUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub employee_number: Option<i16>,
    pub department: Option<i32>,
    pub picture: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

impl LoginUser {
    pub fn is_filled(&self) -> bool {
         !self.email.is_empty() &&
         !self.password.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize, Validate, Default)]
pub struct SignupUser {
    #[validate(length(min = 1, max = 255))]
    pub first_name: String,
    #[validate(length(min = 1, max = 255))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    //#[validate(length(min = 10, max = 255))]
    //pub password: Secret<String>,
    pub password: String,
    //#[validate(length(min = 10, max = 255))]
    //pub re_password: Secret<String>,
    pub re_password: String,
}

impl SignupUser {
    pub fn is_filled(&self) -> bool {
         !self.first_name.is_empty() &&
         !self.last_name.is_empty() &&
         !self.email.is_empty() &&
         !self.password.is_empty() &&
         !self.re_password.is_empty()
    }
}
