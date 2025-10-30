//! Resource modules

pub mod account_settings;
pub use account_settings::Account_settings;
pub mod configuration;
pub use configuration::Configuration;
pub mod hosted_configuration_version;
pub use hosted_configuration_version::Hosted_configuration_version;
pub mod deployment;
pub use deployment::Deployment;
pub mod extension;
pub use extension::Extension;
pub mod configuration_profile;
pub use configuration_profile::Configuration_profile;
pub mod environment;
pub use environment::Environment;
pub mod extension_association;
pub use extension_association::Extension_association;
pub mod deployment_strategy;
pub use deployment_strategy::Deployment_strategy;
pub mod application;
pub use application::Application;

