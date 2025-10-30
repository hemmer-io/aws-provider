//! Resource_groups_tagging_api_2017_01_26 Service
//!
//! Auto-generated service module for resource_groups_tagging_api_2017_01_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for resource_groups_tagging_api_2017_01_26
pub struct Resource_groups_tagging_api_2017_01_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Resource_groups_tagging_api_2017_01_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get report_creation resource handler
    pub fn report_creation(&self) -> resources::Report_creation<'_> {
        resources::Report_creation::new(self.provider)
    }
    /// Get resources resource handler
    pub fn resources(&self) -> resources::Resources<'_> {
        resources::Resources::new(self.provider)
    }
    /// Get tag_values resource handler
    pub fn tag_values(&self) -> resources::Tag_values<'_> {
        resources::Tag_values::new(self.provider)
    }
    /// Get compliance_summary resource handler
    pub fn compliance_summary(&self) -> resources::Compliance_summary<'_> {
        resources::Compliance_summary::new(self.provider)
    }
    /// Get tag_keys resource handler
    pub fn tag_keys(&self) -> resources::Tag_keys<'_> {
        resources::Tag_keys::new(self.provider)
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
