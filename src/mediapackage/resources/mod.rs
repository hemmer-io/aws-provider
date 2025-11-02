//! Resource modules

pub mod origin_endpoint;
pub use origin_endpoint::Origin_endpoint;
pub mod channel;
pub use channel::Channel;
pub mod harvest_job;
pub use harvest_job::Harvest_job;
pub mod asset;
pub use asset::Asset;
pub mod packaging_configuration;
pub use packaging_configuration::Packaging_configuration;
pub mod packaging_group;
pub use packaging_group::Packaging_group;

