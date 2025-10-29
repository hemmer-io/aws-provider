//! Migrationhubstrategy Service
//!
//! Auto-generated service module for migrationhubstrategy

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for migrationhubstrategy
pub struct MigrationhubstrategyService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MigrationhubstrategyService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get server_strategies resource handler
    pub fn server_strategies(&self) -> resources::Server_strategies<'_> {
        resources::Server_strategies::new(self.provider)
    }
    /// Get application_component_details resource handler
    pub fn application_component_details(&self) -> resources::Application_component_details<'_> {
        resources::Application_component_details::new(self.provider)
    }
    /// Get application_component_config resource handler
    pub fn application_component_config(&self) -> resources::Application_component_config<'_> {
        resources::Application_component_config::new(self.provider)
    }
    /// Get assessment resource handler
    pub fn assessment(&self) -> resources::Assessment<'_> {
        resources::Assessment::new(self.provider)
    }
    /// Get application_component_strategies resource handler
    pub fn application_component_strategies(&self) -> resources::Application_component_strategies<'_> {
        resources::Application_component_strategies::new(self.provider)
    }
    /// Get portfolio_summary resource handler
    pub fn portfolio_summary(&self) -> resources::Portfolio_summary<'_> {
        resources::Portfolio_summary::new(self.provider)
    }
    /// Get import_file_task resource handler
    pub fn import_file_task(&self) -> resources::Import_file_task<'_> {
        resources::Import_file_task::new(self.provider)
    }
    /// Get server_details resource handler
    pub fn server_details(&self) -> resources::Server_details<'_> {
        resources::Server_details::new(self.provider)
    }
    /// Get latest_assessment_id resource handler
    pub fn latest_assessment_id(&self) -> resources::Latest_assessment_id<'_> {
        resources::Latest_assessment_id::new(self.provider)
    }
    /// Get server_config resource handler
    pub fn server_config(&self) -> resources::Server_config<'_> {
        resources::Server_config::new(self.provider)
    }
    /// Get recommendation_report_details resource handler
    pub fn recommendation_report_details(&self) -> resources::Recommendation_report_details<'_> {
        resources::Recommendation_report_details::new(self.provider)
    }
    /// Get portfolio_preferences resource handler
    pub fn portfolio_preferences(&self) -> resources::Portfolio_preferences<'_> {
        resources::Portfolio_preferences::new(self.provider)
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
