//! Resource modules

pub mod custom_domains;
pub use custom_domains::Custom_domains;
pub mod vpc_ingress_connection;
pub use vpc_ingress_connection::Vpc_ingress_connection;
pub mod connection;
pub use connection::Connection;
pub mod default_auto_scaling_configuration;
pub use default_auto_scaling_configuration::Default_auto_scaling_configuration;
pub mod auto_scaling_configuration;
pub use auto_scaling_configuration::Auto_scaling_configuration;
pub mod service;
pub use service::Service;
pub mod observability_configuration;
pub use observability_configuration::Observability_configuration;
pub mod vpc_connector;
pub use vpc_connector::Vpc_connector;

