//! Resource modules

pub mod auto_management;
pub use auto_management::Auto_management;
pub mod auto_management_configuration;
pub use auto_management_configuration::Auto_management_configuration;
pub mod aws_default_service_quota;
pub use aws_default_service_quota::Aws_default_service_quota;
pub mod association_for_service_quota_template;
pub use association_for_service_quota_template::Association_for_service_quota_template;
pub mod support_case;
pub use support_case::Support_case;
pub mod requested_service_quota_change;
pub use requested_service_quota_change::Requested_service_quota_change;
pub mod service_quota;
pub use service_quota::Service_quota;
pub mod service_quota_increase_request_from_template;
pub use service_quota_increase_request_from_template::Service_quota_increase_request_from_template;
pub mod service_quota_increase_request_into_template;
pub use service_quota_increase_request_into_template::Service_quota_increase_request_into_template;

