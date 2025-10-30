//! Bcm_dashboards_2025_08_18 Service
//!
//! Auto-generated service module for bcm_dashboards_2025_08_18

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bcm_dashboards_2025_08_18
pub struct Bcm_dashboards_2025_08_18Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Bcm_dashboards_2025_08_18Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get dashboard resource handler
    pub fn dashboard(&self) -> resources::Dashboard<'_> {
        resources::Dashboard::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
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
