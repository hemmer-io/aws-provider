//! Backup Service
//!
//! Auto-generated service module for backup

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for backup
pub struct BackupService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> BackupService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get restore_access_backup_vault resource handler
    pub fn restore_access_backup_vault(&self) -> resources::Restore_access_backup_vault<'_> {
        resources::Restore_access_backup_vault::new(self.provider)
    }
    /// Get backup_plan_from_template resource handler
    pub fn backup_plan_from_template(&self) -> resources::Backup_plan_from_template<'_> {
        resources::Backup_plan_from_template::new(self.provider)
    }
    /// Get backup_vault_lock_configuration resource handler
    pub fn backup_vault_lock_configuration(&self) -> resources::Backup_vault_lock_configuration<'_> {
        resources::Backup_vault_lock_configuration::new(self.provider)
    }
    /// Get recovery_point_index_settings resource handler
    pub fn recovery_point_index_settings(&self) -> resources::Recovery_point_index_settings<'_> {
        resources::Recovery_point_index_settings::new(self.provider)
    }
    /// Get copy_job resource handler
    pub fn copy_job(&self) -> resources::Copy_job<'_> {
        resources::Copy_job::new(self.provider)
    }
    /// Get report_plan resource handler
    pub fn report_plan(&self) -> resources::Report_plan<'_> {
        resources::Report_plan::new(self.provider)
    }
    /// Get backup_plan_from_json resource handler
    pub fn backup_plan_from_json(&self) -> resources::Backup_plan_from_json<'_> {
        resources::Backup_plan_from_json::new(self.provider)
    }
    /// Get supported_resource_types resource handler
    pub fn supported_resource_types(&self) -> resources::Supported_resource_types<'_> {
        resources::Supported_resource_types::new(self.provider)
    }
    /// Get backup_selection resource handler
    pub fn backup_selection(&self) -> resources::Backup_selection<'_> {
        resources::Backup_selection::new(self.provider)
    }
    /// Get recovery_point_restore_metadata resource handler
    pub fn recovery_point_restore_metadata(&self) -> resources::Recovery_point_restore_metadata<'_> {
        resources::Recovery_point_restore_metadata::new(self.provider)
    }
    /// Get restore_job_metadata resource handler
    pub fn restore_job_metadata(&self) -> resources::Restore_job_metadata<'_> {
        resources::Restore_job_metadata::new(self.provider)
    }
    /// Get legal_hold resource handler
    pub fn legal_hold(&self) -> resources::Legal_hold<'_> {
        resources::Legal_hold::new(self.provider)
    }
    /// Get backup_job resource handler
    pub fn backup_job(&self) -> resources::Backup_job<'_> {
        resources::Backup_job::new(self.provider)
    }
    /// Get protected_resource resource handler
    pub fn protected_resource(&self) -> resources::Protected_resource<'_> {
        resources::Protected_resource::new(self.provider)
    }
    /// Get restore_testing_selection resource handler
    pub fn restore_testing_selection(&self) -> resources::Restore_testing_selection<'_> {
        resources::Restore_testing_selection::new(self.provider)
    }
    /// Get restore_validation_result resource handler
    pub fn restore_validation_result(&self) -> resources::Restore_validation_result<'_> {
        resources::Restore_validation_result::new(self.provider)
    }
    /// Get framework resource handler
    pub fn framework(&self) -> resources::Framework<'_> {
        resources::Framework::new(self.provider)
    }
    /// Get backup_vault resource handler
    pub fn backup_vault(&self) -> resources::Backup_vault<'_> {
        resources::Backup_vault::new(self.provider)
    }
    /// Get global_settings resource handler
    pub fn global_settings(&self) -> resources::Global_settings<'_> {
        resources::Global_settings::new(self.provider)
    }
    /// Get restore_testing_plan resource handler
    pub fn restore_testing_plan(&self) -> resources::Restore_testing_plan<'_> {
        resources::Restore_testing_plan::new(self.provider)
    }
    /// Get backup_vault_notifications resource handler
    pub fn backup_vault_notifications(&self) -> resources::Backup_vault_notifications<'_> {
        resources::Backup_vault_notifications::new(self.provider)
    }
    /// Get report_job resource handler
    pub fn report_job(&self) -> resources::Report_job<'_> {
        resources::Report_job::new(self.provider)
    }
    /// Get recovery_point_lifecycle resource handler
    pub fn recovery_point_lifecycle(&self) -> resources::Recovery_point_lifecycle<'_> {
        resources::Recovery_point_lifecycle::new(self.provider)
    }
    /// Get region_settings resource handler
    pub fn region_settings(&self) -> resources::Region_settings<'_> {
        resources::Region_settings::new(self.provider)
    }
    /// Get backup_plan resource handler
    pub fn backup_plan(&self) -> resources::Backup_plan<'_> {
        resources::Backup_plan::new(self.provider)
    }
    /// Get restore_job resource handler
    pub fn restore_job(&self) -> resources::Restore_job<'_> {
        resources::Restore_job::new(self.provider)
    }
    /// Get logically_air_gapped_backup_vault resource handler
    pub fn logically_air_gapped_backup_vault(&self) -> resources::Logically_air_gapped_backup_vault<'_> {
        resources::Logically_air_gapped_backup_vault::new(self.provider)
    }
    /// Get recovery_point resource handler
    pub fn recovery_point(&self) -> resources::Recovery_point<'_> {
        resources::Recovery_point::new(self.provider)
    }
    /// Get backup_vault_access_policy resource handler
    pub fn backup_vault_access_policy(&self) -> resources::Backup_vault_access_policy<'_> {
        resources::Backup_vault_access_policy::new(self.provider)
    }
    /// Get recovery_point_index_details resource handler
    pub fn recovery_point_index_details(&self) -> resources::Recovery_point_index_details<'_> {
        resources::Recovery_point_index_details::new(self.provider)
    }
    /// Get restore_testing_inferred_metadata resource handler
    pub fn restore_testing_inferred_metadata(&self) -> resources::Restore_testing_inferred_metadata<'_> {
        resources::Restore_testing_inferred_metadata::new(self.provider)
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
