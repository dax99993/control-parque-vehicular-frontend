mod not_found;
mod home;
// Auth
mod login;
mod logout;
mod signup;
// Vehicules
mod vehicules;
mod vehicule_register;
mod vehicule_edit;
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
pub use vehicule_register::RegisterVehiculeView;
pub use vehicule_edit::EditVehiculeView;

pub use profile::ProfileView;

pub use users::UsersView;
