//! Accessanalyzer_2019_11_01 Service
//!
//! Auto-generated service module for accessanalyzer_2019_11_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for accessanalyzer_2019_11_01
pub struct Accessanalyzer_2019_11_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Accessanalyzer_2019_11_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get finding resource handler
    pub fn finding(&self) -> resources::Finding<'_> {
        resources::Finding::new(self.provider)
    }
    /// Get finding_recommendation resource handler
    pub fn finding_recommendation(&self) -> resources::Finding_recommendation<'_> {
        resources::Finding_recommendation::new(self.provider)
    }
    /// Get finding_v2 resource handler
    pub fn finding_v2(&self) -> resources::Finding_v2<'_> {
        resources::Finding_v2::new(self.provider)
    }
    /// Get access_preview resource handler
    pub fn access_preview(&self) -> resources::Access_preview<'_> {
        resources::Access_preview::new(self.provider)
    }
    /// Get analyzed_resource resource handler
    pub fn analyzed_resource(&self) -> resources::Analyzed_resource<'_> {
        resources::Analyzed_resource::new(self.provider)
    }
    /// Get findings resource handler
    pub fn findings(&self) -> resources::Findings<'_> {
        resources::Findings::new(self.provider)
    }
    /// Get findings_statistics resource handler
    pub fn findings_statistics(&self) -> resources::Findings_statistics<'_> {
        resources::Findings_statistics::new(self.provider)
    }
    /// Get generated_policy resource handler
    pub fn generated_policy(&self) -> resources::Generated_policy<'_> {
        resources::Generated_policy::new(self.provider)
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
