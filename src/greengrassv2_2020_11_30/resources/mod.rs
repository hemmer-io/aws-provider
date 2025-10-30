//! Resource modules

pub mod core_device;
pub use core_device::Core_device;
pub mod deployment;
pub use deployment::Deployment;
pub mod service_role_for_account;
pub use service_role_for_account::Service_role_for_account;
pub mod component_version;
pub use component_version::Component_version;
pub mod component_version_artifact;
pub use component_version_artifact::Component_version_artifact;
pub mod component;
pub use component::Component;
pub mod connectivity_info;
pub use connectivity_info::Connectivity_info;

