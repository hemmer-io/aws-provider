//! Inspector_2016_02_16 Service
//!
//! Auto-generated service module for inspector_2016_02_16

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for inspector_2016_02_16
pub struct Inspector_2016_02_16Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Inspector_2016_02_16Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_groups resource handler
    pub fn resource_groups(&self) -> resources::Resource_groups<'_> {
        resources::Resource_groups::new(self.provider)
    }
    /// Get cross_account_access_role resource handler
    pub fn cross_account_access_role(&self) -> resources::Cross_account_access_role<'_> {
        resources::Cross_account_access_role::new(self.provider)
    }
    /// Get resource_group resource handler
    pub fn resource_group(&self) -> resources::Resource_group<'_> {
        resources::Resource_group::new(self.provider)
    }
    /// Get assessment_targets resource handler
    pub fn assessment_targets(&self) -> resources::Assessment_targets<'_> {
        resources::Assessment_targets::new(self.provider)
    }
    /// Get exclusions_preview resource handler
    pub fn exclusions_preview(&self) -> resources::Exclusions_preview<'_> {
        resources::Exclusions_preview::new(self.provider)
    }
    /// Get findings resource handler
    pub fn findings(&self) -> resources::Findings<'_> {
        resources::Findings::new(self.provider)
    }
    /// Get telemetry_metadata resource handler
    pub fn telemetry_metadata(&self) -> resources::Telemetry_metadata<'_> {
        resources::Telemetry_metadata::new(self.provider)
    }
    /// Get assessment_templates resource handler
    pub fn assessment_templates(&self) -> resources::Assessment_templates<'_> {
        resources::Assessment_templates::new(self.provider)
    }
    /// Get rules_packages resource handler
    pub fn rules_packages(&self) -> resources::Rules_packages<'_> {
        resources::Rules_packages::new(self.provider)
    }
    /// Get assessment_target resource handler
    pub fn assessment_target(&self) -> resources::Assessment_target<'_> {
        resources::Assessment_target::new(self.provider)
    }
    /// Get assessment_runs resource handler
    pub fn assessment_runs(&self) -> resources::Assessment_runs<'_> {
        resources::Assessment_runs::new(self.provider)
    }
    /// Get assessment_run resource handler
    pub fn assessment_run(&self) -> resources::Assessment_run<'_> {
        resources::Assessment_run::new(self.provider)
    }
    /// Get assessment_template resource handler
    pub fn assessment_template(&self) -> resources::Assessment_template<'_> {
        resources::Assessment_template::new(self.provider)
    }
    /// Get assessment_report resource handler
    pub fn assessment_report(&self) -> resources::Assessment_report<'_> {
        resources::Assessment_report::new(self.provider)
    }
    /// Get exclusions resource handler
    pub fn exclusions(&self) -> resources::Exclusions<'_> {
        resources::Exclusions::new(self.provider)
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
