//! Resource modules

pub mod secret;
pub use secret::Secret;
pub mod resource_policy;
pub use resource_policy::Resource_policy;
pub mod random_password;
pub use random_password::Random_password;
pub mod secret_version_stage;
pub use secret_version_stage::Secret_version_stage;
pub mod secret_value;
pub use secret_value::Secret_value;

