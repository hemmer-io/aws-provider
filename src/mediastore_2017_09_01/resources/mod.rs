//! Resource modules

pub mod metric_policy;
pub use metric_policy::Metric_policy;
pub mod container;
pub use container::Container;
pub mod container_policy;
pub use container_policy::Container_policy;
pub mod cors_policy;
pub use cors_policy::Cors_policy;
pub mod lifecycle_policy;
pub use lifecycle_policy::Lifecycle_policy;

