pub mod admin;
pub mod normal;

mod home;
pub use home::HomeView;
//Auth
mod login;
pub use login::LoginView;
mod logout;
pub use logout::LogoutView;
mod signup;
pub use signup::SignupView;
//Vehicule
mod vehicules;
pub use vehicules::VehiculesView;
//Users
mod users;
pub use users::UsersView;
mod user_edit;
pub use user_edit::EditUserView;
//Profile
mod profile;
pub use profile::ProfileView;
//Notfound
mod not_found;
pub use not_found::NotFoundView;
//Request
mod request_vehicule;
pub use request_vehicule::RequestsView;
