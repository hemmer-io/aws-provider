//! Lakeformation Service
//!
//! Auto-generated service module for lakeformation

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for lakeformation
pub struct LakeformationService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> LakeformationService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get objects_on_cancel resource handler
    pub fn objects_on_cancel(&self) -> resources::Objects_on_cancel<'_> {
        resources::Objects_on_cancel::new(self.provider)
    }
    /// Get lake_formation_identity_center_configuration resource handler
    pub fn lake_formation_identity_center_configuration(&self) -> resources::Lake_formation_identity_center_configuration<'_> {
        resources::Lake_formation_identity_center_configuration::new(self.provider)
    }
    /// Get resource resource handler
    pub fn resource(&self) -> resources::Resource<'_> {
        resources::Resource::new(self.provider)
    }
    /// Get lftag_expression resource handler
    pub fn lftag_expression(&self) -> resources::Lftag_expression<'_> {
        resources::Lftag_expression::new(self.provider)
    }
    /// Get work_unit_results resource handler
    pub fn work_unit_results(&self) -> resources::Work_unit_results<'_> {
        resources::Work_unit_results::new(self.provider)
    }
    /// Get lake_formation_opt_in resource handler
    pub fn lake_formation_opt_in(&self) -> resources::Lake_formation_opt_in<'_> {
        resources::Lake_formation_opt_in::new(self.provider)
    }
    /// Get effective_permissions_for_path resource handler
    pub fn effective_permissions_for_path(&self) -> resources::Effective_permissions_for_path<'_> {
        resources::Effective_permissions_for_path::new(self.provider)
    }
    /// Get table_objects resource handler
    pub fn table_objects(&self) -> resources::Table_objects<'_> {
        resources::Table_objects::new(self.provider)
    }
    /// Get table_storage_optimizer resource handler
    pub fn table_storage_optimizer(&self) -> resources::Table_storage_optimizer<'_> {
        resources::Table_storage_optimizer::new(self.provider)
    }
    /// Get temporary_glue_table_credentials resource handler
    pub fn temporary_glue_table_credentials(&self) -> resources::Temporary_glue_table_credentials<'_> {
        resources::Temporary_glue_table_credentials::new(self.provider)
    }
    /// Get lftag resource handler
    pub fn lftag(&self) -> resources::Lftag<'_> {
        resources::Lftag::new(self.provider)
    }
    /// Get temporary_glue_partition_credentials resource handler
    pub fn temporary_glue_partition_credentials(&self) -> resources::Temporary_glue_partition_credentials<'_> {
        resources::Temporary_glue_partition_credentials::new(self.provider)
    }
    /// Get query_statistics resource handler
    pub fn query_statistics(&self) -> resources::Query_statistics<'_> {
        resources::Query_statistics::new(self.provider)
    }
    /// Get work_units resource handler
    pub fn work_units(&self) -> resources::Work_units<'_> {
        resources::Work_units::new(self.provider)
    }
    /// Get data_lake_principal resource handler
    pub fn data_lake_principal(&self) -> resources::Data_lake_principal<'_> {
        resources::Data_lake_principal::new(self.provider)
    }
    /// Get query_state resource handler
    pub fn query_state(&self) -> resources::Query_state<'_> {
        resources::Query_state::new(self.provider)
    }
    /// Get resource_lftags resource handler
    pub fn resource_lftags(&self) -> resources::Resource_lftags<'_> {
        resources::Resource_lftags::new(self.provider)
    }
    /// Get data_cells_filter resource handler
    pub fn data_cells_filter(&self) -> resources::Data_cells_filter<'_> {
        resources::Data_cells_filter::new(self.provider)
    }
    /// Get transaction resource handler
    pub fn transaction(&self) -> resources::Transaction<'_> {
        resources::Transaction::new(self.provider)
    }
    /// Get data_lake_settings resource handler
    pub fn data_lake_settings(&self) -> resources::Data_lake_settings<'_> {
        resources::Data_lake_settings::new(self.provider)
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
