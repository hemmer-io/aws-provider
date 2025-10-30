//! Cloudcontrol_2021_09_30 Service
//!
//! Auto-generated service module for cloudcontrol_2021_09_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudcontrol_2021_09_30
pub struct Cloudcontrol_2021_09_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudcontrol_2021_09_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource resource handler
    pub fn resource(&self) -> resources::Resource<'_> {
        resources::Resource::new(self.provider)
    }
    /// Get resource_request_status resource handler
    pub fn resource_request_status(&self) -> resources::Resource_request_status<'_> {
        resources::Resource_request_status::new(self.provider)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
