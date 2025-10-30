//! Resource modules

pub mod app_instance_bot;
pub use app_instance_bot::App_instance_bot;
pub mod app_instance_user_endpoint;
pub use app_instance_user_endpoint::App_instance_user_endpoint;
pub mod app_instance_user;
pub use app_instance_user::App_instance_user;
pub mod app_instance_admin;
pub use app_instance_admin::App_instance_admin;
pub mod app_instance_user_expiration_settings;
pub use app_instance_user_expiration_settings::App_instance_user_expiration_settings;
pub mod app_instance_retention_settings;
pub use app_instance_retention_settings::App_instance_retention_settings;
pub mod app_instance;
pub use app_instance::App_instance;

