//! Entityresolution_2018_05_10 Service
//!
//! Auto-generated service module for entityresolution_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for entityresolution_2018_05_10
pub struct Entityresolution_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Entityresolution_2018_05_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get id_mapping_job resource handler
    pub fn id_mapping_job(&self) -> resources::Id_mapping_job<'_> {
        resources::Id_mapping_job::new(self.provider)
    }
    /// Get schema_mapping resource handler
    pub fn schema_mapping(&self) -> resources::Schema_mapping<'_> {
        resources::Schema_mapping::new(self.provider)
    }
    /// Get id_namespace resource handler
    pub fn id_namespace(&self) -> resources::Id_namespace<'_> {
        resources::Id_namespace::new(self.provider)
    }
    /// Get id_mapping_workflow resource handler
    pub fn id_mapping_workflow(&self) -> resources::Id_mapping_workflow<'_> {
        resources::Id_mapping_workflow::new(self.provider)
    }
    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
    }
    /// Get provider_service resource handler
    pub fn provider_service(&self) -> resources::Provider_service<'_> {
        resources::Provider_service::new(self.provider)
    }
    /// Get matching_job resource handler
    pub fn matching_job(&self) -> resources::Matching_job<'_> {
        resources::Matching_job::new(self.provider)
    }
    /// Get policy_statement resource handler
    pub fn policy_statement(&self) -> resources::Policy_statement<'_> {
        resources::Policy_statement::new(self.provider)
    }
    /// Get match_id resource handler
    pub fn match_id(&self) -> resources::Match_id<'_> {
        resources::Match_id::new(self.provider)
    }
    /// Get matching_workflow resource handler
    pub fn matching_workflow(&self) -> resources::Matching_workflow<'_> {
        resources::Matching_workflow::new(self.provider)
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
