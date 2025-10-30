//! Codeguru_security_2018_05_10 Service
//!
//! Auto-generated service module for codeguru_security_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codeguru_security_2018_05_10
pub struct Codeguru_security_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Codeguru_security_2018_05_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get findings resource handler
    pub fn findings(&self) -> resources::Findings<'_> {
        resources::Findings::new(self.provider)
    }
    /// Get account_configuration resource handler
    pub fn account_configuration(&self) -> resources::Account_configuration<'_> {
        resources::Account_configuration::new(self.provider)
    }
    /// Get metrics_summary resource handler
    pub fn metrics_summary(&self) -> resources::Metrics_summary<'_> {
        resources::Metrics_summary::new(self.provider)
    }
    /// Get scan resource handler
    pub fn scan(&self) -> resources::Scan<'_> {
        resources::Scan::new(self.provider)
    }
    /// Get upload_url resource handler
    pub fn upload_url(&self) -> resources::Upload_url<'_> {
        resources::Upload_url::new(self.provider)
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
