use poem_openapi::Tags;

mod auth_controller;
mod user_controller;

pub use auth_controller::AuthController;
pub use user_controller::UserController;

#[derive(Tags)]
pub enum Tag {
    /// Authorization methods.
    Auth,
    /// User methods.
    User,
}
