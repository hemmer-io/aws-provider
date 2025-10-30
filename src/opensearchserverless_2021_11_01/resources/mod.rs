//! Resource modules

pub mod policies_stats;
pub use policies_stats::Policies_stats;
pub mod security_policy;
pub use security_policy::Security_policy;
pub mod vpc_endpoint;
pub use vpc_endpoint::Vpc_endpoint;
pub mod lifecycle_policy;
pub use lifecycle_policy::Lifecycle_policy;
pub mod account_settings;
pub use account_settings::Account_settings;

