//! Resource modules

pub mod job_for_devices;
pub use job_for_devices::Job_for_devices;
pub mod application_instance;
pub use application_instance::Application_instance;
pub mod package_import_job;
pub use package_import_job::Package_import_job;
pub mod device_job;
pub use device_job::Device_job;
pub mod device;
pub use device::Device;
pub mod node;
pub use node::Node;
pub mod device_metadata;
pub use device_metadata::Device_metadata;
pub mod package;
pub use package::Package;
pub mod package_version;
pub use package_version::Package_version;
pub mod application_instance_details;
pub use application_instance_details::Application_instance_details;
pub mod node_from_template_job;
pub use node_from_template_job::Node_from_template_job;

