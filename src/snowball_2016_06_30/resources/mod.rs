//! Resource modules

pub mod snowball_usage;
pub use snowball_usage::Snowball_usage;
pub mod cluster;
pub use cluster::Cluster;
pub mod long_term_pricing;
pub use long_term_pricing::Long_term_pricing;
pub mod addresses;
pub use addresses::Addresses;
pub mod job_shipment_state;
pub use job_shipment_state::Job_shipment_state;
pub mod address;
pub use address::Address;
pub mod job;
pub use job::Job;
pub mod job_manifest;
pub use job_manifest::Job_manifest;
pub mod return_shipping_label;
pub use return_shipping_label::Return_shipping_label;
pub mod job_unlock_code;
pub use job_unlock_code::Job_unlock_code;
pub mod software_updates;
pub use software_updates::Software_updates;

