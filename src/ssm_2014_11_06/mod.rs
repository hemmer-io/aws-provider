//! Ssm_2014_11_06 Service
//!
//! Auto-generated service module for ssm_2014_11_06

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ssm_2014_11_06
pub struct Ssm_2014_11_06Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ssm_2014_11_06Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get ops_metadata resource handler
    pub fn ops_metadata(&self) -> resources::Ops_metadata<'_> {
        resources::Ops_metadata::new(self.provider)
    }
    /// Get default_patch_baseline resource handler
    pub fn default_patch_baseline(&self) -> resources::Default_patch_baseline<'_> {
        resources::Default_patch_baseline::new(self.provider)
    }
    /// Get inventory resource handler
    pub fn inventory(&self) -> resources::Inventory<'_> {
        resources::Inventory::new(self.provider)
    }
    /// Get instance_information resource handler
    pub fn instance_information(&self) -> resources::Instance_information<'_> {
        resources::Instance_information::new(self.provider)
    }
    /// Get patch_baseline_for_patch_group resource handler
    pub fn patch_baseline_for_patch_group(&self) -> resources::Patch_baseline_for_patch_group<'_> {
        resources::Patch_baseline_for_patch_group::new(self.provider)
    }
    /// Get activations resource handler
    pub fn activations(&self) -> resources::Activations<'_> {
        resources::Activations::new(self.provider)
    }
    /// Get instance_patch_states resource handler
    pub fn instance_patch_states(&self) -> resources::Instance_patch_states<'_> {
        resources::Instance_patch_states::new(self.provider)
    }
    /// Get maintenance_windows_for_target resource handler
    pub fn maintenance_windows_for_target(&self) -> resources::Maintenance_windows_for_target<'_> {
        resources::Maintenance_windows_for_target::new(self.provider)
    }
    /// Get compliance_items resource handler
    pub fn compliance_items(&self) -> resources::Compliance_items<'_> {
        resources::Compliance_items::new(self.provider)
    }
    /// Get maintenance_window_target resource handler
    pub fn maintenance_window_target(&self) -> resources::Maintenance_window_target<'_> {
        resources::Maintenance_window_target::new(self.provider)
    }
    /// Get patch_baseline resource handler
    pub fn patch_baseline(&self) -> resources::Patch_baseline<'_> {
        resources::Patch_baseline::new(self.provider)
    }
    /// Get maintenance_window_targets resource handler
    pub fn maintenance_window_targets(&self) -> resources::Maintenance_window_targets<'_> {
        resources::Maintenance_window_targets::new(self.provider)
    }
    /// Get patch_group_state resource handler
    pub fn patch_group_state(&self) -> resources::Patch_group_state<'_> {
        resources::Patch_group_state::new(self.provider)
    }
    /// Get automation_executions resource handler
    pub fn automation_executions(&self) -> resources::Automation_executions<'_> {
        resources::Automation_executions::new(self.provider)
    }
    /// Get maintenance_window_execution_task_invocations resource handler
    pub fn maintenance_window_execution_task_invocations(&self) -> resources::Maintenance_window_execution_task_invocations<'_> {
        resources::Maintenance_window_execution_task_invocations::new(self.provider)
    }
    /// Get instance_associations_status resource handler
    pub fn instance_associations_status(&self) -> resources::Instance_associations_status<'_> {
        resources::Instance_associations_status::new(self.provider)
    }
    /// Get patch_baselines resource handler
    pub fn patch_baselines(&self) -> resources::Patch_baselines<'_> {
        resources::Patch_baselines::new(self.provider)
    }
    /// Get ops_item resource handler
    pub fn ops_item(&self) -> resources::Ops_item<'_> {
        resources::Ops_item::new(self.provider)
    }
    /// Get sessions resource handler
    pub fn sessions(&self) -> resources::Sessions<'_> {
        resources::Sessions::new(self.provider)
    }
    /// Get managed_instance_role resource handler
    pub fn managed_instance_role(&self) -> resources::Managed_instance_role<'_> {
        resources::Managed_instance_role::new(self.provider)
    }
    /// Get association_execution_targets resource handler
    pub fn association_execution_targets(&self) -> resources::Association_execution_targets<'_> {
        resources::Association_execution_targets::new(self.provider)
    }
    /// Get parameters resource handler
    pub fn parameters(&self) -> resources::Parameters<'_> {
        resources::Parameters::new(self.provider)
    }
    /// Get association resource handler
    pub fn association(&self) -> resources::Association<'_> {
        resources::Association::new(self.provider)
    }
    /// Get available_patches resource handler
    pub fn available_patches(&self) -> resources::Available_patches<'_> {
        resources::Available_patches::new(self.provider)
    }
    /// Get ops_items resource handler
    pub fn ops_items(&self) -> resources::Ops_items<'_> {
        resources::Ops_items::new(self.provider)
    }
    /// Get activation resource handler
    pub fn activation(&self) -> resources::Activation<'_> {
        resources::Activation::new(self.provider)
    }
    /// Get automation_execution resource handler
    pub fn automation_execution(&self) -> resources::Automation_execution<'_> {
        resources::Automation_execution::new(self.provider)
    }
    /// Get document_default_version resource handler
    pub fn document_default_version(&self) -> resources::Document_default_version<'_> {
        resources::Document_default_version::new(self.provider)
    }
    /// Get patch_properties resource handler
    pub fn patch_properties(&self) -> resources::Patch_properties<'_> {
        resources::Patch_properties::new(self.provider)
    }
    /// Get maintenance_window_tasks resource handler
    pub fn maintenance_window_tasks(&self) -> resources::Maintenance_window_tasks<'_> {
        resources::Maintenance_window_tasks::new(self.provider)
    }
    /// Get maintenance_window_executions resource handler
    pub fn maintenance_window_executions(&self) -> resources::Maintenance_window_executions<'_> {
        resources::Maintenance_window_executions::new(self.provider)
    }
    /// Get service_setting resource handler
    pub fn service_setting(&self) -> resources::Service_setting<'_> {
        resources::Service_setting::new(self.provider)
    }
    /// Get calendar_state resource handler
    pub fn calendar_state(&self) -> resources::Calendar_state<'_> {
        resources::Calendar_state::new(self.provider)
    }
    /// Get association_executions resource handler
    pub fn association_executions(&self) -> resources::Association_executions<'_> {
        resources::Association_executions::new(self.provider)
    }
    /// Get document resource handler
    pub fn document(&self) -> resources::Document<'_> {
        resources::Document::new(self.provider)
    }
    /// Get effective_instance_associations resource handler
    pub fn effective_instance_associations(&self) -> resources::Effective_instance_associations<'_> {
        resources::Effective_instance_associations::new(self.provider)
    }
    /// Get maintenance_windows resource handler
    pub fn maintenance_windows(&self) -> resources::Maintenance_windows<'_> {
        resources::Maintenance_windows::new(self.provider)
    }
    /// Get maintenance_window_execution_task resource handler
    pub fn maintenance_window_execution_task(&self) -> resources::Maintenance_window_execution_task<'_> {
        resources::Maintenance_window_execution_task::new(self.provider)
    }
    /// Get access_token resource handler
    pub fn access_token(&self) -> resources::Access_token<'_> {
        resources::Access_token::new(self.provider)
    }
    /// Get execution_preview resource handler
    pub fn execution_preview(&self) -> resources::Execution_preview<'_> {
        resources::Execution_preview::new(self.provider)
    }
    /// Get patch_groups resource handler
    pub fn patch_groups(&self) -> resources::Patch_groups<'_> {
        resources::Patch_groups::new(self.provider)
    }
    /// Get resource_data_sync resource handler
    pub fn resource_data_sync(&self) -> resources::Resource_data_sync<'_> {
        resources::Resource_data_sync::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get inventory_schema resource handler
    pub fn inventory_schema(&self) -> resources::Inventory_schema<'_> {
        resources::Inventory_schema::new(self.provider)
    }
    /// Get parameter_history resource handler
    pub fn parameter_history(&self) -> resources::Parameter_history<'_> {
        resources::Parameter_history::new(self.provider)
    }
    /// Get parameter resource handler
    pub fn parameter(&self) -> resources::Parameter<'_> {
        resources::Parameter::new(self.provider)
    }
    /// Get maintenance_window_task resource handler
    pub fn maintenance_window_task(&self) -> resources::Maintenance_window_task<'_> {
        resources::Maintenance_window_task::new(self.provider)
    }
    /// Get parameters_by_path resource handler
    pub fn parameters_by_path(&self) -> resources::Parameters_by_path<'_> {
        resources::Parameters_by_path::new(self.provider)
    }
    /// Get maintenance_window_execution_task_invocation resource handler
    pub fn maintenance_window_execution_task_invocation(&self) -> resources::Maintenance_window_execution_task_invocation<'_> {
        resources::Maintenance_window_execution_task_invocation::new(self.provider)
    }
    /// Get instance_patches resource handler
    pub fn instance_patches(&self) -> resources::Instance_patches<'_> {
        resources::Instance_patches::new(self.provider)
    }
    /// Get connection_status resource handler
    pub fn connection_status(&self) -> resources::Connection_status<'_> {
        resources::Connection_status::new(self.provider)
    }
    /// Get automation_step_executions resource handler
    pub fn automation_step_executions(&self) -> resources::Automation_step_executions<'_> {
        resources::Automation_step_executions::new(self.provider)
    }
    /// Get document_permission resource handler
    pub fn document_permission(&self) -> resources::Document_permission<'_> {
        resources::Document_permission::new(self.provider)
    }
    /// Get maintenance_window_execution_tasks resource handler
    pub fn maintenance_window_execution_tasks(&self) -> resources::Maintenance_window_execution_tasks<'_> {
        resources::Maintenance_window_execution_tasks::new(self.provider)
    }
    /// Get maintenance_window resource handler
    pub fn maintenance_window(&self) -> resources::Maintenance_window<'_> {
        resources::Maintenance_window::new(self.provider)
    }
    /// Get resource_policies resource handler
    pub fn resource_policies(&self) -> resources::Resource_policies<'_> {
        resources::Resource_policies::new(self.provider)
    }
    /// Get document_metadata resource handler
    pub fn document_metadata(&self) -> resources::Document_metadata<'_> {
        resources::Document_metadata::new(self.provider)
    }
    /// Get maintenance_window_schedule resource handler
    pub fn maintenance_window_schedule(&self) -> resources::Maintenance_window_schedule<'_> {
        resources::Maintenance_window_schedule::new(self.provider)
    }
    /// Get association_batch resource handler
    pub fn association_batch(&self) -> resources::Association_batch<'_> {
        resources::Association_batch::new(self.provider)
    }
    /// Get deployable_patch_snapshot_for_instance resource handler
    pub fn deployable_patch_snapshot_for_instance(&self) -> resources::Deployable_patch_snapshot_for_instance<'_> {
        resources::Deployable_patch_snapshot_for_instance::new(self.provider)
    }
    /// Get ops_summary resource handler
    pub fn ops_summary(&self) -> resources::Ops_summary<'_> {
        resources::Ops_summary::new(self.provider)
    }
    /// Get instance_properties resource handler
    pub fn instance_properties(&self) -> resources::Instance_properties<'_> {
        resources::Instance_properties::new(self.provider)
    }
    /// Get command_invocation resource handler
    pub fn command_invocation(&self) -> resources::Command_invocation<'_> {
        resources::Command_invocation::new(self.provider)
    }
    /// Get instance_patch_states_for_patch_group resource handler
    pub fn instance_patch_states_for_patch_group(&self) -> resources::Instance_patch_states_for_patch_group<'_> {
        resources::Instance_patch_states_for_patch_group::new(self.provider)
    }
    /// Get effective_patches_for_patch_baseline resource handler
    pub fn effective_patches_for_patch_baseline(&self) -> resources::Effective_patches_for_patch_baseline<'_> {
        resources::Effective_patches_for_patch_baseline::new(self.provider)
    }
    /// Get inventory_deletions resource handler
    pub fn inventory_deletions(&self) -> resources::Inventory_deletions<'_> {
        resources::Inventory_deletions::new(self.provider)
    }
    /// Get association_status resource handler
    pub fn association_status(&self) -> resources::Association_status<'_> {
        resources::Association_status::new(self.provider)
    }
    /// Get maintenance_window_execution resource handler
    pub fn maintenance_window_execution(&self) -> resources::Maintenance_window_execution<'_> {
        resources::Maintenance_window_execution::new(self.provider)
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
