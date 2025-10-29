//! Mediastore Service
//!
//! Auto-generated service module for mediastore

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mediastore
pub struct MediastoreService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MediastoreService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get container_policy resource handler
    pub fn container_policy(&self) -> resources::Container_policy<'_> {
        resources::Container_policy::new(self.provider)
    }
    /// Get container resource handler
    pub fn container(&self) -> resources::Container<'_> {
        resources::Container::new(self.provider)
    }
    /// Get lifecycle_policy resource handler
    pub fn lifecycle_policy(&self) -> resources::Lifecycle_policy<'_> {
        resources::Lifecycle_policy::new(self.provider)
    }
    /// Get cors_policy resource handler
    pub fn cors_policy(&self) -> resources::Cors_policy<'_> {
        resources::Cors_policy::new(self.provider)
    }
    /// Get metric_policy resource handler
    pub fn metric_policy(&self) -> resources::Metric_policy<'_> {
        resources::Metric_policy::new(self.provider)
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
