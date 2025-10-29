//! Cloudcontrol Service
//!
//! Auto-generated service module for cloudcontrol

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudcontrol
pub struct CloudcontrolService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CloudcontrolService<'a> {
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
