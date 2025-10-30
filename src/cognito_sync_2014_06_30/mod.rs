//! Cognito_sync_2014_06_30 Service
//!
//! Auto-generated service module for cognito_sync_2014_06_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cognito_sync_2014_06_30
pub struct Cognito_sync_2014_06_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cognito_sync_2014_06_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get bulk_publish_details resource handler
    pub fn bulk_publish_details(&self) -> resources::Bulk_publish_details<'_> {
        resources::Bulk_publish_details::new(self.provider)
    }
    /// Get identity_pool_configuration resource handler
    pub fn identity_pool_configuration(&self) -> resources::Identity_pool_configuration<'_> {
        resources::Identity_pool_configuration::new(self.provider)
    }
    /// Get identity_pool_usage resource handler
    pub fn identity_pool_usage(&self) -> resources::Identity_pool_usage<'_> {
        resources::Identity_pool_usage::new(self.provider)
    }
    /// Get cognito_events resource handler
    pub fn cognito_events(&self) -> resources::Cognito_events<'_> {
        resources::Cognito_events::new(self.provider)
    }
    /// Get identity_usage resource handler
    pub fn identity_usage(&self) -> resources::Identity_usage<'_> {
        resources::Identity_usage::new(self.provider)
    }
    /// Get records resource handler
    pub fn records(&self) -> resources::Records<'_> {
        resources::Records::new(self.provider)
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
