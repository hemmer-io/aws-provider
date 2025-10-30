//! Codebuild_2016_10_06 Service
//!
//! Auto-generated service module for codebuild_2016_10_06

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codebuild_2016_10_06
pub struct Codebuild_2016_10_06Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Codebuild_2016_10_06Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get build_batch resource handler
    pub fn build_batch(&self) -> resources::Build_batch<'_> {
        resources::Build_batch::new(self.provider)
    }
    /// Get report resource handler
    pub fn report(&self) -> resources::Report<'_> {
        resources::Report::new(self.provider)
    }
    /// Get source_credentials resource handler
    pub fn source_credentials(&self) -> resources::Source_credentials<'_> {
        resources::Source_credentials::new(self.provider)
    }
    /// Get webhook resource handler
    pub fn webhook(&self) -> resources::Webhook<'_> {
        resources::Webhook::new(self.provider)
    }
    /// Get test_cases resource handler
    pub fn test_cases(&self) -> resources::Test_cases<'_> {
        resources::Test_cases::new(self.provider)
    }
    /// Get fleet resource handler
    pub fn fleet(&self) -> resources::Fleet<'_> {
        resources::Fleet::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get report_group_trend resource handler
    pub fn report_group_trend(&self) -> resources::Report_group_trend<'_> {
        resources::Report_group_trend::new(self.provider)
    }
    /// Get code_coverages resource handler
    pub fn code_coverages(&self) -> resources::Code_coverages<'_> {
        resources::Code_coverages::new(self.provider)
    }
    /// Get report_group resource handler
    pub fn report_group(&self) -> resources::Report_group<'_> {
        resources::Report_group::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get project_visibility resource handler
    pub fn project_visibility(&self) -> resources::Project_visibility<'_> {
        resources::Project_visibility::new(self.provider)
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
