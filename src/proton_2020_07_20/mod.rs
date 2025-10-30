//! Proton_2020_07_20 Service
//!
//! Auto-generated service module for proton_2020_07_20

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for proton_2020_07_20
pub struct Proton_2020_07_20Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Proton_2020_07_20Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get template_sync_status resource handler
    pub fn template_sync_status(&self) -> resources::Template_sync_status<'_> {
        resources::Template_sync_status::new(self.provider)
    }
    /// Get resources_summary resource handler
    pub fn resources_summary(&self) -> resources::Resources_summary<'_> {
        resources::Resources_summary::new(self.provider)
    }
    /// Get service_instance_sync_status resource handler
    pub fn service_instance_sync_status(&self) -> resources::Service_instance_sync_status<'_> {
        resources::Service_instance_sync_status::new(self.provider)
    }
    /// Get repository_sync_status resource handler
    pub fn repository_sync_status(&self) -> resources::Repository_sync_status<'_> {
        resources::Repository_sync_status::new(self.provider)
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
