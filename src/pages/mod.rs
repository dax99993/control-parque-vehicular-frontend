pub mod admin;


mod not_found;
mod home;
// Auth
mod login;
mod logout;
mod signup;
// Vehicules
mod vehicules;
// Profile
mod profile;
// Users
mod users;
mod user_edit;
// Reports
// Requests


pub use home::HomeView;
pub use not_found::NotFoundView;

pub use login::LoginView;
pub use logout::LogoutView;
pub use signup::SignupView;

pub use vehicules::VehiculesView;

pub use profile::ProfileView;

pub use users::UsersView;
pub use user_edit::EditUserView;
