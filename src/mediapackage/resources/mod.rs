//! Resource modules

pub mod channel;
pub use channel::Channel;
pub mod origin_endpoint;
pub use origin_endpoint::Origin_endpoint;
pub mod harvest_job;
pub use harvest_job::Harvest_job;
pub mod packaging_configuration;
pub use packaging_configuration::Packaging_configuration;
pub mod packaging_group;
pub use packaging_group::Packaging_group;
pub mod asset;
pub use asset::Asset;

