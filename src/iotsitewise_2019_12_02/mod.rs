//! Iotsitewise_2019_12_02 Service
//!
//! Auto-generated service module for iotsitewise_2019_12_02

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iotsitewise_2019_12_02
pub struct Iotsitewise_2019_12_02Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iotsitewise_2019_12_02Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get execution resource handler
    pub fn execution(&self) -> resources::Execution<'_> {
        resources::Execution::new(self.provider)
    }
    /// Get bulk_import_job resource handler
    pub fn bulk_import_job(&self) -> resources::Bulk_import_job<'_> {
        resources::Bulk_import_job::new(self.provider)
    }
    /// Get asset_property_aggregates resource handler
    pub fn asset_property_aggregates(&self) -> resources::Asset_property_aggregates<'_> {
        resources::Asset_property_aggregates::new(self.provider)
    }
    /// Get asset_property_value resource handler
    pub fn asset_property_value(&self) -> resources::Asset_property_value<'_> {
        resources::Asset_property_value::new(self.provider)
    }
    /// Get asset_model resource handler
    pub fn asset_model(&self) -> resources::Asset_model<'_> {
        resources::Asset_model::new(self.provider)
    }
    /// Get portal resource handler
    pub fn portal(&self) -> resources::Portal<'_> {
        resources::Portal::new(self.provider)
    }
    /// Get time_series resource handler
    pub fn time_series(&self) -> resources::Time_series<'_> {
        resources::Time_series::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get default_encryption_configuration resource handler
    pub fn default_encryption_configuration(&self) -> resources::Default_encryption_configuration<'_> {
        resources::Default_encryption_configuration::new(self.provider)
    }
    /// Get dashboard resource handler
    pub fn dashboard(&self) -> resources::Dashboard<'_> {
        resources::Dashboard::new(self.provider)
    }
    /// Get computation_model_execution_summary resource handler
    pub fn computation_model_execution_summary(&self) -> resources::Computation_model_execution_summary<'_> {
        resources::Computation_model_execution_summary::new(self.provider)
    }
    /// Get storage_configuration resource handler
    pub fn storage_configuration(&self) -> resources::Storage_configuration<'_> {
        resources::Storage_configuration::new(self.provider)
    }
    /// Get computation_model resource handler
    pub fn computation_model(&self) -> resources::Computation_model<'_> {
        resources::Computation_model::new(self.provider)
    }
    /// Get interpolated_asset_property_values resource handler
    pub fn interpolated_asset_property_values(&self) -> resources::Interpolated_asset_property_values<'_> {
        resources::Interpolated_asset_property_values::new(self.provider)
    }
    /// Get asset_composite_model resource handler
    pub fn asset_composite_model(&self) -> resources::Asset_composite_model<'_> {
        resources::Asset_composite_model::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get asset_property resource handler
    pub fn asset_property(&self) -> resources::Asset_property<'_> {
        resources::Asset_property::new(self.provider)
    }
    /// Get asset_model_interface_relationship resource handler
    pub fn asset_model_interface_relationship(&self) -> resources::Asset_model_interface_relationship<'_> {
        resources::Asset_model_interface_relationship::new(self.provider)
    }
    /// Get asset_model_composite_model resource handler
    pub fn asset_model_composite_model(&self) -> resources::Asset_model_composite_model<'_> {
        resources::Asset_model_composite_model::new(self.provider)
    }
    /// Get gateway_capability_configuration resource handler
    pub fn gateway_capability_configuration(&self) -> resources::Gateway_capability_configuration<'_> {
        resources::Gateway_capability_configuration::new(self.provider)
    }
    /// Get logging_options resource handler
    pub fn logging_options(&self) -> resources::Logging_options<'_> {
        resources::Logging_options::new(self.provider)
    }
    /// Get action resource handler
    pub fn action(&self) -> resources::Action<'_> {
        resources::Action::new(self.provider)
    }
    /// Get asset resource handler
    pub fn asset(&self) -> resources::Asset<'_> {
        resources::Asset::new(self.provider)
    }
    /// Get access_policy resource handler
    pub fn access_policy(&self) -> resources::Access_policy<'_> {
        resources::Access_policy::new(self.provider)
    }
    /// Get gateway resource handler
    pub fn gateway(&self) -> resources::Gateway<'_> {
        resources::Gateway::new(self.provider)
    }
    /// Get asset_property_value_history resource handler
    pub fn asset_property_value_history(&self) -> resources::Asset_property_value_history<'_> {
        resources::Asset_property_value_history::new(self.provider)
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
