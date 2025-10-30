//! Resource modules

pub mod dataset;
pub use dataset::Dataset;
pub mod bulk_publish_details;
pub use bulk_publish_details::Bulk_publish_details;
pub mod identity_pool_configuration;
pub use identity_pool_configuration::Identity_pool_configuration;
pub mod identity_pool_usage;
pub use identity_pool_usage::Identity_pool_usage;
pub mod cognito_events;
pub use cognito_events::Cognito_events;
pub mod identity_usage;
pub use identity_usage::Identity_usage;
pub mod records;
pub use records::Records;

