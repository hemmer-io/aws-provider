//! Ssm service for Aws provider
//!
//! This module handles all ssm resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Ssm service handler
pub struct SsmService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> SsmService<'a> {
    /// Create a new service handler
    pub fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Plan changes to a resource
    pub async fn plan_resource(
        &self,
        resource_name: &str,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        match resource_name {
            "document_default_version" => {
                self.plan_document_default_version(current_state, desired_input).await
            }
            "maintenance_window_targets" => {
                self.plan_maintenance_window_targets(current_state, desired_input).await
            }
            "parameter_history" => {
                self.plan_parameter_history(current_state, desired_input).await
            }
            "effective_patches_for_patch_baseline" => {
                self.plan_effective_patches_for_patch_baseline(current_state, desired_input).await
            }
            "maintenance_window_target" => {
                self.plan_maintenance_window_target(current_state, desired_input).await
            }
            "association" => {
                self.plan_association(current_state, desired_input).await
            }
            "association_execution_targets" => {
                self.plan_association_execution_targets(current_state, desired_input).await
            }
            "calendar_state" => {
                self.plan_calendar_state(current_state, desired_input).await
            }
            "ops_item" => {
                self.plan_ops_item(current_state, desired_input).await
            }
            "maintenance_window_executions" => {
                self.plan_maintenance_window_executions(current_state, desired_input).await
            }
            "activation" => {
                self.plan_activation(current_state, desired_input).await
            }
            "effective_instance_associations" => {
                self.plan_effective_instance_associations(current_state, desired_input).await
            }
            "execution_preview" => {
                self.plan_execution_preview(current_state, desired_input).await
            }
            "resource_policies" => {
                self.plan_resource_policies(current_state, desired_input).await
            }
            "association_batch" => {
                self.plan_association_batch(current_state, desired_input).await
            }
            "instance_associations_status" => {
                self.plan_instance_associations_status(current_state, desired_input).await
            }
            "document" => {
                self.plan_document(current_state, desired_input).await
            }
            "ops_summary" => {
                self.plan_ops_summary(current_state, desired_input).await
            }
            "command_invocation" => {
                self.plan_command_invocation(current_state, desired_input).await
            }
            "compliance_items" => {
                self.plan_compliance_items(current_state, desired_input).await
            }
            "patch_baselines" => {
                self.plan_patch_baselines(current_state, desired_input).await
            }
            "access_token" => {
                self.plan_access_token(current_state, desired_input).await
            }
            "maintenance_window_tasks" => {
                self.plan_maintenance_window_tasks(current_state, desired_input).await
            }
            "maintenance_window_execution_tasks" => {
                self.plan_maintenance_window_execution_tasks(current_state, desired_input).await
            }
            "deployable_patch_snapshot_for_instance" => {
                self.plan_deployable_patch_snapshot_for_instance(current_state, desired_input).await
            }
            "maintenance_window_schedule" => {
                self.plan_maintenance_window_schedule(current_state, desired_input).await
            }
            "document_metadata" => {
                self.plan_document_metadata(current_state, desired_input).await
            }
            "association_status" => {
                self.plan_association_status(current_state, desired_input).await
            }
            "maintenance_window_execution_task_invocations" => {
                self.plan_maintenance_window_execution_task_invocations(current_state, desired_input).await
            }
            "instance_patch_states_for_patch_group" => {
                self.plan_instance_patch_states_for_patch_group(current_state, desired_input).await
            }
            "patch_groups" => {
                self.plan_patch_groups(current_state, desired_input).await
            }
            "automation_step_executions" => {
                self.plan_automation_step_executions(current_state, desired_input).await
            }
            "patch_baseline_for_patch_group" => {
                self.plan_patch_baseline_for_patch_group(current_state, desired_input).await
            }
            "ops_metadata" => {
                self.plan_ops_metadata(current_state, desired_input).await
            }
            "inventory" => {
                self.plan_inventory(current_state, desired_input).await
            }
            "patch_properties" => {
                self.plan_patch_properties(current_state, desired_input).await
            }
            "instance_patch_states" => {
                self.plan_instance_patch_states(current_state, desired_input).await
            }
            "instance_information" => {
                self.plan_instance_information(current_state, desired_input).await
            }
            "resource_policy" => {
                self.plan_resource_policy(current_state, desired_input).await
            }
            "activations" => {
                self.plan_activations(current_state, desired_input).await
            }
            "maintenance_window_execution_task_invocation" => {
                self.plan_maintenance_window_execution_task_invocation(current_state, desired_input).await
            }
            "maintenance_windows_for_target" => {
                self.plan_maintenance_windows_for_target(current_state, desired_input).await
            }
            "service_setting" => {
                self.plan_service_setting(current_state, desired_input).await
            }
            "automation_execution" => {
                self.plan_automation_execution(current_state, desired_input).await
            }
            "patch_group_state" => {
                self.plan_patch_group_state(current_state, desired_input).await
            }
            "maintenance_window" => {
                self.plan_maintenance_window(current_state, desired_input).await
            }
            "parameter" => {
                self.plan_parameter(current_state, desired_input).await
            }
            "sessions" => {
                self.plan_sessions(current_state, desired_input).await
            }
            "resource_data_sync" => {
                self.plan_resource_data_sync(current_state, desired_input).await
            }
            "managed_instance_role" => {
                self.plan_managed_instance_role(current_state, desired_input).await
            }
            "default_patch_baseline" => {
                self.plan_default_patch_baseline(current_state, desired_input).await
            }
            "automation_executions" => {
                self.plan_automation_executions(current_state, desired_input).await
            }
            "connection_status" => {
                self.plan_connection_status(current_state, desired_input).await
            }
            "ops_items" => {
                self.plan_ops_items(current_state, desired_input).await
            }
            "maintenance_window_execution" => {
                self.plan_maintenance_window_execution(current_state, desired_input).await
            }
            "maintenance_window_task" => {
                self.plan_maintenance_window_task(current_state, desired_input).await
            }
            "document_permission" => {
                self.plan_document_permission(current_state, desired_input).await
            }
            "instance_properties" => {
                self.plan_instance_properties(current_state, desired_input).await
            }
            "inventory_schema" => {
                self.plan_inventory_schema(current_state, desired_input).await
            }
            "association_executions" => {
                self.plan_association_executions(current_state, desired_input).await
            }
            "parameters_by_path" => {
                self.plan_parameters_by_path(current_state, desired_input).await
            }
            "patch_baseline" => {
                self.plan_patch_baseline(current_state, desired_input).await
            }
            "parameters" => {
                self.plan_parameters(current_state, desired_input).await
            }
            "available_patches" => {
                self.plan_available_patches(current_state, desired_input).await
            }
            "maintenance_window_execution_task" => {
                self.plan_maintenance_window_execution_task(current_state, desired_input).await
            }
            "instance_patches" => {
                self.plan_instance_patches(current_state, desired_input).await
            }
            "inventory_deletions" => {
                self.plan_inventory_deletions(current_state, desired_input).await
            }
            "maintenance_windows" => {
                self.plan_maintenance_windows(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ssm",
                resource_name
            ))),
        }
    }

    /// Create a new resource
    pub async fn create_resource(
        &self,
        resource_name: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        match resource_name {
            "document_default_version" => {
                self.create_document_default_version(input).await
            }
            "maintenance_window_targets" => {
                self.create_maintenance_window_targets(input).await
            }
            "parameter_history" => {
                self.create_parameter_history(input).await
            }
            "effective_patches_for_patch_baseline" => {
                self.create_effective_patches_for_patch_baseline(input).await
            }
            "maintenance_window_target" => {
                self.create_maintenance_window_target(input).await
            }
            "association" => {
                self.create_association(input).await
            }
            "association_execution_targets" => {
                self.create_association_execution_targets(input).await
            }
            "calendar_state" => {
                self.create_calendar_state(input).await
            }
            "ops_item" => {
                self.create_ops_item(input).await
            }
            "maintenance_window_executions" => {
                self.create_maintenance_window_executions(input).await
            }
            "activation" => {
                self.create_activation(input).await
            }
            "effective_instance_associations" => {
                self.create_effective_instance_associations(input).await
            }
            "execution_preview" => {
                self.create_execution_preview(input).await
            }
            "resource_policies" => {
                self.create_resource_policies(input).await
            }
            "association_batch" => {
                self.create_association_batch(input).await
            }
            "instance_associations_status" => {
                self.create_instance_associations_status(input).await
            }
            "document" => {
                self.create_document(input).await
            }
            "ops_summary" => {
                self.create_ops_summary(input).await
            }
            "command_invocation" => {
                self.create_command_invocation(input).await
            }
            "compliance_items" => {
                self.create_compliance_items(input).await
            }
            "patch_baselines" => {
                self.create_patch_baselines(input).await
            }
            "access_token" => {
                self.create_access_token(input).await
            }
            "maintenance_window_tasks" => {
                self.create_maintenance_window_tasks(input).await
            }
            "maintenance_window_execution_tasks" => {
                self.create_maintenance_window_execution_tasks(input).await
            }
            "deployable_patch_snapshot_for_instance" => {
                self.create_deployable_patch_snapshot_for_instance(input).await
            }
            "maintenance_window_schedule" => {
                self.create_maintenance_window_schedule(input).await
            }
            "document_metadata" => {
                self.create_document_metadata(input).await
            }
            "association_status" => {
                self.create_association_status(input).await
            }
            "maintenance_window_execution_task_invocations" => {
                self.create_maintenance_window_execution_task_invocations(input).await
            }
            "instance_patch_states_for_patch_group" => {
                self.create_instance_patch_states_for_patch_group(input).await
            }
            "patch_groups" => {
                self.create_patch_groups(input).await
            }
            "automation_step_executions" => {
                self.create_automation_step_executions(input).await
            }
            "patch_baseline_for_patch_group" => {
                self.create_patch_baseline_for_patch_group(input).await
            }
            "ops_metadata" => {
                self.create_ops_metadata(input).await
            }
            "inventory" => {
                self.create_inventory(input).await
            }
            "patch_properties" => {
                self.create_patch_properties(input).await
            }
            "instance_patch_states" => {
                self.create_instance_patch_states(input).await
            }
            "instance_information" => {
                self.create_instance_information(input).await
            }
            "resource_policy" => {
                self.create_resource_policy(input).await
            }
            "activations" => {
                self.create_activations(input).await
            }
            "maintenance_window_execution_task_invocation" => {
                self.create_maintenance_window_execution_task_invocation(input).await
            }
            "maintenance_windows_for_target" => {
                self.create_maintenance_windows_for_target(input).await
            }
            "service_setting" => {
                self.create_service_setting(input).await
            }
            "automation_execution" => {
                self.create_automation_execution(input).await
            }
            "patch_group_state" => {
                self.create_patch_group_state(input).await
            }
            "maintenance_window" => {
                self.create_maintenance_window(input).await
            }
            "parameter" => {
                self.create_parameter(input).await
            }
            "sessions" => {
                self.create_sessions(input).await
            }
            "resource_data_sync" => {
                self.create_resource_data_sync(input).await
            }
            "managed_instance_role" => {
                self.create_managed_instance_role(input).await
            }
            "default_patch_baseline" => {
                self.create_default_patch_baseline(input).await
            }
            "automation_executions" => {
                self.create_automation_executions(input).await
            }
            "connection_status" => {
                self.create_connection_status(input).await
            }
            "ops_items" => {
                self.create_ops_items(input).await
            }
            "maintenance_window_execution" => {
                self.create_maintenance_window_execution(input).await
            }
            "maintenance_window_task" => {
                self.create_maintenance_window_task(input).await
            }
            "document_permission" => {
                self.create_document_permission(input).await
            }
            "instance_properties" => {
                self.create_instance_properties(input).await
            }
            "inventory_schema" => {
                self.create_inventory_schema(input).await
            }
            "association_executions" => {
                self.create_association_executions(input).await
            }
            "parameters_by_path" => {
                self.create_parameters_by_path(input).await
            }
            "patch_baseline" => {
                self.create_patch_baseline(input).await
            }
            "parameters" => {
                self.create_parameters(input).await
            }
            "available_patches" => {
                self.create_available_patches(input).await
            }
            "maintenance_window_execution_task" => {
                self.create_maintenance_window_execution_task(input).await
            }
            "instance_patches" => {
                self.create_instance_patches(input).await
            }
            "inventory_deletions" => {
                self.create_inventory_deletions(input).await
            }
            "maintenance_windows" => {
                self.create_maintenance_windows(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ssm",
                resource_name
            ))),
        }
    }

    /// Read resource state
    pub async fn read_resource(
        &self,
        resource_name: &str,
        id: &str,
    ) -> Result<ResourceOutput> {
        match resource_name {
            "document_default_version" => {
                self.read_document_default_version(id).await
            }
            "maintenance_window_targets" => {
                self.read_maintenance_window_targets(id).await
            }
            "parameter_history" => {
                self.read_parameter_history(id).await
            }
            "effective_patches_for_patch_baseline" => {
                self.read_effective_patches_for_patch_baseline(id).await
            }
            "maintenance_window_target" => {
                self.read_maintenance_window_target(id).await
            }
            "association" => {
                self.read_association(id).await
            }
            "association_execution_targets" => {
                self.read_association_execution_targets(id).await
            }
            "calendar_state" => {
                self.read_calendar_state(id).await
            }
            "ops_item" => {
                self.read_ops_item(id).await
            }
            "maintenance_window_executions" => {
                self.read_maintenance_window_executions(id).await
            }
            "activation" => {
                self.read_activation(id).await
            }
            "effective_instance_associations" => {
                self.read_effective_instance_associations(id).await
            }
            "execution_preview" => {
                self.read_execution_preview(id).await
            }
            "resource_policies" => {
                self.read_resource_policies(id).await
            }
            "association_batch" => {
                self.read_association_batch(id).await
            }
            "instance_associations_status" => {
                self.read_instance_associations_status(id).await
            }
            "document" => {
                self.read_document(id).await
            }
            "ops_summary" => {
                self.read_ops_summary(id).await
            }
            "command_invocation" => {
                self.read_command_invocation(id).await
            }
            "compliance_items" => {
                self.read_compliance_items(id).await
            }
            "patch_baselines" => {
                self.read_patch_baselines(id).await
            }
            "access_token" => {
                self.read_access_token(id).await
            }
            "maintenance_window_tasks" => {
                self.read_maintenance_window_tasks(id).await
            }
            "maintenance_window_execution_tasks" => {
                self.read_maintenance_window_execution_tasks(id).await
            }
            "deployable_patch_snapshot_for_instance" => {
                self.read_deployable_patch_snapshot_for_instance(id).await
            }
            "maintenance_window_schedule" => {
                self.read_maintenance_window_schedule(id).await
            }
            "document_metadata" => {
                self.read_document_metadata(id).await
            }
            "association_status" => {
                self.read_association_status(id).await
            }
            "maintenance_window_execution_task_invocations" => {
                self.read_maintenance_window_execution_task_invocations(id).await
            }
            "instance_patch_states_for_patch_group" => {
                self.read_instance_patch_states_for_patch_group(id).await
            }
            "patch_groups" => {
                self.read_patch_groups(id).await
            }
            "automation_step_executions" => {
                self.read_automation_step_executions(id).await
            }
            "patch_baseline_for_patch_group" => {
                self.read_patch_baseline_for_patch_group(id).await
            }
            "ops_metadata" => {
                self.read_ops_metadata(id).await
            }
            "inventory" => {
                self.read_inventory(id).await
            }
            "patch_properties" => {
                self.read_patch_properties(id).await
            }
            "instance_patch_states" => {
                self.read_instance_patch_states(id).await
            }
            "instance_information" => {
                self.read_instance_information(id).await
            }
            "resource_policy" => {
                self.read_resource_policy(id).await
            }
            "activations" => {
                self.read_activations(id).await
            }
            "maintenance_window_execution_task_invocation" => {
                self.read_maintenance_window_execution_task_invocation(id).await
            }
            "maintenance_windows_for_target" => {
                self.read_maintenance_windows_for_target(id).await
            }
            "service_setting" => {
                self.read_service_setting(id).await
            }
            "automation_execution" => {
                self.read_automation_execution(id).await
            }
            "patch_group_state" => {
                self.read_patch_group_state(id).await
            }
            "maintenance_window" => {
                self.read_maintenance_window(id).await
            }
            "parameter" => {
                self.read_parameter(id).await
            }
            "sessions" => {
                self.read_sessions(id).await
            }
            "resource_data_sync" => {
                self.read_resource_data_sync(id).await
            }
            "managed_instance_role" => {
                self.read_managed_instance_role(id).await
            }
            "default_patch_baseline" => {
                self.read_default_patch_baseline(id).await
            }
            "automation_executions" => {
                self.read_automation_executions(id).await
            }
            "connection_status" => {
                self.read_connection_status(id).await
            }
            "ops_items" => {
                self.read_ops_items(id).await
            }
            "maintenance_window_execution" => {
                self.read_maintenance_window_execution(id).await
            }
            "maintenance_window_task" => {
                self.read_maintenance_window_task(id).await
            }
            "document_permission" => {
                self.read_document_permission(id).await
            }
            "instance_properties" => {
                self.read_instance_properties(id).await
            }
            "inventory_schema" => {
                self.read_inventory_schema(id).await
            }
            "association_executions" => {
                self.read_association_executions(id).await
            }
            "parameters_by_path" => {
                self.read_parameters_by_path(id).await
            }
            "patch_baseline" => {
                self.read_patch_baseline(id).await
            }
            "parameters" => {
                self.read_parameters(id).await
            }
            "available_patches" => {
                self.read_available_patches(id).await
            }
            "maintenance_window_execution_task" => {
                self.read_maintenance_window_execution_task(id).await
            }
            "instance_patches" => {
                self.read_instance_patches(id).await
            }
            "inventory_deletions" => {
                self.read_inventory_deletions(id).await
            }
            "maintenance_windows" => {
                self.read_maintenance_windows(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ssm",
                resource_name
            ))),
        }
    }

    /// Update an existing resource
    pub async fn update_resource(
        &self,
        resource_name: &str,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        match resource_name {
            "document_default_version" => {
                self.update_document_default_version(id, input).await
            }
            "maintenance_window_targets" => {
                self.update_maintenance_window_targets(id, input).await
            }
            "parameter_history" => {
                self.update_parameter_history(id, input).await
            }
            "effective_patches_for_patch_baseline" => {
                self.update_effective_patches_for_patch_baseline(id, input).await
            }
            "maintenance_window_target" => {
                self.update_maintenance_window_target(id, input).await
            }
            "association" => {
                self.update_association(id, input).await
            }
            "association_execution_targets" => {
                self.update_association_execution_targets(id, input).await
            }
            "calendar_state" => {
                self.update_calendar_state(id, input).await
            }
            "ops_item" => {
                self.update_ops_item(id, input).await
            }
            "maintenance_window_executions" => {
                self.update_maintenance_window_executions(id, input).await
            }
            "activation" => {
                self.update_activation(id, input).await
            }
            "effective_instance_associations" => {
                self.update_effective_instance_associations(id, input).await
            }
            "execution_preview" => {
                self.update_execution_preview(id, input).await
            }
            "resource_policies" => {
                self.update_resource_policies(id, input).await
            }
            "association_batch" => {
                self.update_association_batch(id, input).await
            }
            "instance_associations_status" => {
                self.update_instance_associations_status(id, input).await
            }
            "document" => {
                self.update_document(id, input).await
            }
            "ops_summary" => {
                self.update_ops_summary(id, input).await
            }
            "command_invocation" => {
                self.update_command_invocation(id, input).await
            }
            "compliance_items" => {
                self.update_compliance_items(id, input).await
            }
            "patch_baselines" => {
                self.update_patch_baselines(id, input).await
            }
            "access_token" => {
                self.update_access_token(id, input).await
            }
            "maintenance_window_tasks" => {
                self.update_maintenance_window_tasks(id, input).await
            }
            "maintenance_window_execution_tasks" => {
                self.update_maintenance_window_execution_tasks(id, input).await
            }
            "deployable_patch_snapshot_for_instance" => {
                self.update_deployable_patch_snapshot_for_instance(id, input).await
            }
            "maintenance_window_schedule" => {
                self.update_maintenance_window_schedule(id, input).await
            }
            "document_metadata" => {
                self.update_document_metadata(id, input).await
            }
            "association_status" => {
                self.update_association_status(id, input).await
            }
            "maintenance_window_execution_task_invocations" => {
                self.update_maintenance_window_execution_task_invocations(id, input).await
            }
            "instance_patch_states_for_patch_group" => {
                self.update_instance_patch_states_for_patch_group(id, input).await
            }
            "patch_groups" => {
                self.update_patch_groups(id, input).await
            }
            "automation_step_executions" => {
                self.update_automation_step_executions(id, input).await
            }
            "patch_baseline_for_patch_group" => {
                self.update_patch_baseline_for_patch_group(id, input).await
            }
            "ops_metadata" => {
                self.update_ops_metadata(id, input).await
            }
            "inventory" => {
                self.update_inventory(id, input).await
            }
            "patch_properties" => {
                self.update_patch_properties(id, input).await
            }
            "instance_patch_states" => {
                self.update_instance_patch_states(id, input).await
            }
            "instance_information" => {
                self.update_instance_information(id, input).await
            }
            "resource_policy" => {
                self.update_resource_policy(id, input).await
            }
            "activations" => {
                self.update_activations(id, input).await
            }
            "maintenance_window_execution_task_invocation" => {
                self.update_maintenance_window_execution_task_invocation(id, input).await
            }
            "maintenance_windows_for_target" => {
                self.update_maintenance_windows_for_target(id, input).await
            }
            "service_setting" => {
                self.update_service_setting(id, input).await
            }
            "automation_execution" => {
                self.update_automation_execution(id, input).await
            }
            "patch_group_state" => {
                self.update_patch_group_state(id, input).await
            }
            "maintenance_window" => {
                self.update_maintenance_window(id, input).await
            }
            "parameter" => {
                self.update_parameter(id, input).await
            }
            "sessions" => {
                self.update_sessions(id, input).await
            }
            "resource_data_sync" => {
                self.update_resource_data_sync(id, input).await
            }
            "managed_instance_role" => {
                self.update_managed_instance_role(id, input).await
            }
            "default_patch_baseline" => {
                self.update_default_patch_baseline(id, input).await
            }
            "automation_executions" => {
                self.update_automation_executions(id, input).await
            }
            "connection_status" => {
                self.update_connection_status(id, input).await
            }
            "ops_items" => {
                self.update_ops_items(id, input).await
            }
            "maintenance_window_execution" => {
                self.update_maintenance_window_execution(id, input).await
            }
            "maintenance_window_task" => {
                self.update_maintenance_window_task(id, input).await
            }
            "document_permission" => {
                self.update_document_permission(id, input).await
            }
            "instance_properties" => {
                self.update_instance_properties(id, input).await
            }
            "inventory_schema" => {
                self.update_inventory_schema(id, input).await
            }
            "association_executions" => {
                self.update_association_executions(id, input).await
            }
            "parameters_by_path" => {
                self.update_parameters_by_path(id, input).await
            }
            "patch_baseline" => {
                self.update_patch_baseline(id, input).await
            }
            "parameters" => {
                self.update_parameters(id, input).await
            }
            "available_patches" => {
                self.update_available_patches(id, input).await
            }
            "maintenance_window_execution_task" => {
                self.update_maintenance_window_execution_task(id, input).await
            }
            "instance_patches" => {
                self.update_instance_patches(id, input).await
            }
            "inventory_deletions" => {
                self.update_inventory_deletions(id, input).await
            }
            "maintenance_windows" => {
                self.update_maintenance_windows(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ssm",
                resource_name
            ))),
        }
    }

    /// Delete a resource
    pub async fn delete_resource(
        &self,
        resource_name: &str,
        id: &str,
    ) -> Result<()> {
        match resource_name {
            "document_default_version" => {
                self.delete_document_default_version(id).await
            }
            "maintenance_window_targets" => {
                self.delete_maintenance_window_targets(id).await
            }
            "parameter_history" => {
                self.delete_parameter_history(id).await
            }
            "effective_patches_for_patch_baseline" => {
                self.delete_effective_patches_for_patch_baseline(id).await
            }
            "maintenance_window_target" => {
                self.delete_maintenance_window_target(id).await
            }
            "association" => {
                self.delete_association(id).await
            }
            "association_execution_targets" => {
                self.delete_association_execution_targets(id).await
            }
            "calendar_state" => {
                self.delete_calendar_state(id).await
            }
            "ops_item" => {
                self.delete_ops_item(id).await
            }
            "maintenance_window_executions" => {
                self.delete_maintenance_window_executions(id).await
            }
            "activation" => {
                self.delete_activation(id).await
            }
            "effective_instance_associations" => {
                self.delete_effective_instance_associations(id).await
            }
            "execution_preview" => {
                self.delete_execution_preview(id).await
            }
            "resource_policies" => {
                self.delete_resource_policies(id).await
            }
            "association_batch" => {
                self.delete_association_batch(id).await
            }
            "instance_associations_status" => {
                self.delete_instance_associations_status(id).await
            }
            "document" => {
                self.delete_document(id).await
            }
            "ops_summary" => {
                self.delete_ops_summary(id).await
            }
            "command_invocation" => {
                self.delete_command_invocation(id).await
            }
            "compliance_items" => {
                self.delete_compliance_items(id).await
            }
            "patch_baselines" => {
                self.delete_patch_baselines(id).await
            }
            "access_token" => {
                self.delete_access_token(id).await
            }
            "maintenance_window_tasks" => {
                self.delete_maintenance_window_tasks(id).await
            }
            "maintenance_window_execution_tasks" => {
                self.delete_maintenance_window_execution_tasks(id).await
            }
            "deployable_patch_snapshot_for_instance" => {
                self.delete_deployable_patch_snapshot_for_instance(id).await
            }
            "maintenance_window_schedule" => {
                self.delete_maintenance_window_schedule(id).await
            }
            "document_metadata" => {
                self.delete_document_metadata(id).await
            }
            "association_status" => {
                self.delete_association_status(id).await
            }
            "maintenance_window_execution_task_invocations" => {
                self.delete_maintenance_window_execution_task_invocations(id).await
            }
            "instance_patch_states_for_patch_group" => {
                self.delete_instance_patch_states_for_patch_group(id).await
            }
            "patch_groups" => {
                self.delete_patch_groups(id).await
            }
            "automation_step_executions" => {
                self.delete_automation_step_executions(id).await
            }
            "patch_baseline_for_patch_group" => {
                self.delete_patch_baseline_for_patch_group(id).await
            }
            "ops_metadata" => {
                self.delete_ops_metadata(id).await
            }
            "inventory" => {
                self.delete_inventory(id).await
            }
            "patch_properties" => {
                self.delete_patch_properties(id).await
            }
            "instance_patch_states" => {
                self.delete_instance_patch_states(id).await
            }
            "instance_information" => {
                self.delete_instance_information(id).await
            }
            "resource_policy" => {
                self.delete_resource_policy(id).await
            }
            "activations" => {
                self.delete_activations(id).await
            }
            "maintenance_window_execution_task_invocation" => {
                self.delete_maintenance_window_execution_task_invocation(id).await
            }
            "maintenance_windows_for_target" => {
                self.delete_maintenance_windows_for_target(id).await
            }
            "service_setting" => {
                self.delete_service_setting(id).await
            }
            "automation_execution" => {
                self.delete_automation_execution(id).await
            }
            "patch_group_state" => {
                self.delete_patch_group_state(id).await
            }
            "maintenance_window" => {
                self.delete_maintenance_window(id).await
            }
            "parameter" => {
                self.delete_parameter(id).await
            }
            "sessions" => {
                self.delete_sessions(id).await
            }
            "resource_data_sync" => {
                self.delete_resource_data_sync(id).await
            }
            "managed_instance_role" => {
                self.delete_managed_instance_role(id).await
            }
            "default_patch_baseline" => {
                self.delete_default_patch_baseline(id).await
            }
            "automation_executions" => {
                self.delete_automation_executions(id).await
            }
            "connection_status" => {
                self.delete_connection_status(id).await
            }
            "ops_items" => {
                self.delete_ops_items(id).await
            }
            "maintenance_window_execution" => {
                self.delete_maintenance_window_execution(id).await
            }
            "maintenance_window_task" => {
                self.delete_maintenance_window_task(id).await
            }
            "document_permission" => {
                self.delete_document_permission(id).await
            }
            "instance_properties" => {
                self.delete_instance_properties(id).await
            }
            "inventory_schema" => {
                self.delete_inventory_schema(id).await
            }
            "association_executions" => {
                self.delete_association_executions(id).await
            }
            "parameters_by_path" => {
                self.delete_parameters_by_path(id).await
            }
            "patch_baseline" => {
                self.delete_patch_baseline(id).await
            }
            "parameters" => {
                self.delete_parameters(id).await
            }
            "available_patches" => {
                self.delete_available_patches(id).await
            }
            "maintenance_window_execution_task" => {
                self.delete_maintenance_window_execution_task(id).await
            }
            "instance_patches" => {
                self.delete_instance_patches(id).await
            }
            "inventory_deletions" => {
                self.delete_inventory_deletions(id).await
            }
            "maintenance_windows" => {
                self.delete_maintenance_windows(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ssm",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Document_default_version resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document_default_version resource
    async fn plan_document_default_version(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new document_default_version resource
    async fn create_document_default_version(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let document_version = input.get_string("document_version")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_document_default_version()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("document_version", document_version.unwrap_or_default())
            )
        })
    }

    /// Read a document_default_version resource
    async fn read_document_default_version(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_document_default_version()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a document_default_version resource
    async fn update_document_default_version(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let document_version = input.get_string("document_version")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_document_default_version()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("document_version", document_version.unwrap_or_default())
            )
        })
    }

    /// Delete a document_default_version resource
    async fn delete_document_default_version(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_document_default_version()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_targets resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_targets resource
    async fn plan_maintenance_window_targets(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_targets resource
    async fn create_maintenance_window_targets(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_targets()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_window_targets resource
    async fn read_maintenance_window_targets(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_targets()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_targets resource
    async fn update_maintenance_window_targets(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_targets()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_window_targets resource
    async fn delete_maintenance_window_targets(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_targets()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Parameter_history resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a parameter_history resource
    async fn plan_parameter_history(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new parameter_history resource
    async fn create_parameter_history(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_parameter_history()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a parameter_history resource
    async fn read_parameter_history(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_parameter_history()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a parameter_history resource
    async fn update_parameter_history(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_parameter_history()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a parameter_history resource
    async fn delete_parameter_history(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_parameter_history()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Effective_patches_for_patch_baseline resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a effective_patches_for_patch_baseline resource
    async fn plan_effective_patches_for_patch_baseline(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new effective_patches_for_patch_baseline resource
    async fn create_effective_patches_for_patch_baseline(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_effective_patches_for_patch_baseline()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a effective_patches_for_patch_baseline resource
    async fn read_effective_patches_for_patch_baseline(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_effective_patches_for_patch_baseline()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a effective_patches_for_patch_baseline resource
    async fn update_effective_patches_for_patch_baseline(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_effective_patches_for_patch_baseline()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a effective_patches_for_patch_baseline resource
    async fn delete_effective_patches_for_patch_baseline(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_effective_patches_for_patch_baseline()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_target resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_target resource
    async fn plan_maintenance_window_target(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_target resource
    async fn create_maintenance_window_target(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let window_target_id = input.get_string("window_target_id")?;
            let window_id = input.get_string("window_id")?;
            let name = input.get_optional_string("name")?;
            let owner_information = input.get_optional_string("owner_information")?;
            let targets = input.get_optional_string("targets")?;
            let replace = input.get_optional_string("replace")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_target()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("description", description.unwrap_or_default())
                .with_field("window_target_id", window_target_id.unwrap_or_default())
                .with_field("window_id", window_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("owner_information", owner_information.unwrap_or_default())
                .with_field("targets", targets.unwrap_or_default())
                .with_field("replace", replace.unwrap_or_default())
            )
        })
    }

    /// Read a maintenance_window_target resource
    async fn read_maintenance_window_target(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_target()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_target resource
    async fn update_maintenance_window_target(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let window_target_id = input.get_string("window_target_id")?;
            let window_id = input.get_string("window_id")?;
            let name = input.get_optional_string("name")?;
            let owner_information = input.get_optional_string("owner_information")?;
            let targets = input.get_optional_string("targets")?;
            let replace = input.get_optional_string("replace")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_target()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("description", description.unwrap_or_default())
                .with_field("window_target_id", window_target_id.unwrap_or_default())
                .with_field("window_id", window_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("owner_information", owner_information.unwrap_or_default())
                .with_field("targets", targets.unwrap_or_default())
                .with_field("replace", replace.unwrap_or_default())
            )
        })
    }

    /// Delete a maintenance_window_target resource
    async fn delete_maintenance_window_target(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_target()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Association resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a association resource
    async fn plan_association(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new association resource
    async fn create_association(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let sync_compliance = input.get_optional_string("sync_compliance")?;
            let schedule_expression = input.get_optional_string("schedule_expression")?;
            let alarm_configuration = input.get_optional_string("alarm_configuration")?;
            let duration = input.get_optional_string("duration")?;
            let parameters = input.get_optional_string("parameters")?;
            let instance_id = input.get_optional_string("instance_id")?;
            let output_location = input.get_optional_string("output_location")?;
            let max_errors = input.get_optional_string("max_errors")?;
            let max_concurrency = input.get_optional_string("max_concurrency")?;
            let apply_only_at_cron_interval = input.get_optional_string("apply_only_at_cron_interval")?;
            let tags = input.get_optional_string("tags")?;
            let association_name = input.get_optional_string("association_name")?;
            let schedule_offset = input.get_optional_string("schedule_offset")?;
            let target_locations = input.get_optional_string("target_locations")?;
            let calendar_names = input.get_optional_string("calendar_names")?;
            let target_maps = input.get_optional_string("target_maps")?;
            let document_version = input.get_optional_string("document_version")?;
            let targets = input.get_optional_string("targets")?;
            let automation_target_parameter_name = input.get_optional_string("automation_target_parameter_name")?;
            let compliance_severity = input.get_optional_string("compliance_severity")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_association()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("sync_compliance", sync_compliance.unwrap_or_default())
                .with_field("schedule_expression", schedule_expression.unwrap_or_default())
                .with_field("alarm_configuration", alarm_configuration.unwrap_or_default())
                .with_field("duration", duration.unwrap_or_default())
                .with_field("parameters", parameters.unwrap_or_default())
                .with_field("instance_id", instance_id.unwrap_or_default())
                .with_field("output_location", output_location.unwrap_or_default())
                .with_field("max_errors", max_errors.unwrap_or_default())
                .with_field("max_concurrency", max_concurrency.unwrap_or_default())
                .with_field("apply_only_at_cron_interval", apply_only_at_cron_interval.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("association_name", association_name.unwrap_or_default())
                .with_field("schedule_offset", schedule_offset.unwrap_or_default())
                .with_field("target_locations", target_locations.unwrap_or_default())
                .with_field("calendar_names", calendar_names.unwrap_or_default())
                .with_field("target_maps", target_maps.unwrap_or_default())
                .with_field("document_version", document_version.unwrap_or_default())
                .with_field("targets", targets.unwrap_or_default())
                .with_field("automation_target_parameter_name", automation_target_parameter_name.unwrap_or_default())
                .with_field("compliance_severity", compliance_severity.unwrap_or_default())
            )
        })
    }

    /// Read a association resource
    async fn read_association(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_association()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a association resource
    async fn update_association(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let sync_compliance = input.get_optional_string("sync_compliance")?;
            let schedule_expression = input.get_optional_string("schedule_expression")?;
            let alarm_configuration = input.get_optional_string("alarm_configuration")?;
            let duration = input.get_optional_string("duration")?;
            let parameters = input.get_optional_string("parameters")?;
            let instance_id = input.get_optional_string("instance_id")?;
            let output_location = input.get_optional_string("output_location")?;
            let max_errors = input.get_optional_string("max_errors")?;
            let max_concurrency = input.get_optional_string("max_concurrency")?;
            let apply_only_at_cron_interval = input.get_optional_string("apply_only_at_cron_interval")?;
            let tags = input.get_optional_string("tags")?;
            let association_name = input.get_optional_string("association_name")?;
            let schedule_offset = input.get_optional_string("schedule_offset")?;
            let target_locations = input.get_optional_string("target_locations")?;
            let calendar_names = input.get_optional_string("calendar_names")?;
            let target_maps = input.get_optional_string("target_maps")?;
            let document_version = input.get_optional_string("document_version")?;
            let targets = input.get_optional_string("targets")?;
            let automation_target_parameter_name = input.get_optional_string("automation_target_parameter_name")?;
            let compliance_severity = input.get_optional_string("compliance_severity")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_association()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("sync_compliance", sync_compliance.unwrap_or_default())
                .with_field("schedule_expression", schedule_expression.unwrap_or_default())
                .with_field("alarm_configuration", alarm_configuration.unwrap_or_default())
                .with_field("duration", duration.unwrap_or_default())
                .with_field("parameters", parameters.unwrap_or_default())
                .with_field("instance_id", instance_id.unwrap_or_default())
                .with_field("output_location", output_location.unwrap_or_default())
                .with_field("max_errors", max_errors.unwrap_or_default())
                .with_field("max_concurrency", max_concurrency.unwrap_or_default())
                .with_field("apply_only_at_cron_interval", apply_only_at_cron_interval.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("association_name", association_name.unwrap_or_default())
                .with_field("schedule_offset", schedule_offset.unwrap_or_default())
                .with_field("target_locations", target_locations.unwrap_or_default())
                .with_field("calendar_names", calendar_names.unwrap_or_default())
                .with_field("target_maps", target_maps.unwrap_or_default())
                .with_field("document_version", document_version.unwrap_or_default())
                .with_field("targets", targets.unwrap_or_default())
                .with_field("automation_target_parameter_name", automation_target_parameter_name.unwrap_or_default())
                .with_field("compliance_severity", compliance_severity.unwrap_or_default())
            )
        })
    }

    /// Delete a association resource
    async fn delete_association(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_association()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Association_execution_targets resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a association_execution_targets resource
    async fn plan_association_execution_targets(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new association_execution_targets resource
    async fn create_association_execution_targets(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_association_execution_targets()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a association_execution_targets resource
    async fn read_association_execution_targets(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_association_execution_targets()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a association_execution_targets resource
    async fn update_association_execution_targets(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_association_execution_targets()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a association_execution_targets resource
    async fn delete_association_execution_targets(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_association_execution_targets()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Calendar_state resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a calendar_state resource
    async fn plan_calendar_state(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new calendar_state resource
    async fn create_calendar_state(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_calendar_state()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a calendar_state resource
    async fn read_calendar_state(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_calendar_state()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a calendar_state resource
    async fn update_calendar_state(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_calendar_state()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a calendar_state resource
    async fn delete_calendar_state(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_calendar_state()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Ops_item resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ops_item resource
    async fn plan_ops_item(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ops_item resource
    async fn create_ops_item(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_string("description")?;
            let actual_end_time = input.get_optional_string("actual_end_time")?;
            let priority = input.get_optional_string("priority")?;
            let actual_start_time = input.get_optional_string("actual_start_time")?;
            let operational_data = input.get_optional_string("operational_data")?;
            let source = input.get_string("source")?;
            let tags = input.get_optional_string("tags")?;
            let planned_end_time = input.get_optional_string("planned_end_time")?;
            let ops_item_type = input.get_optional_string("ops_item_type")?;
            let title = input.get_string("title")?;
            let severity = input.get_optional_string("severity")?;
            let notifications = input.get_optional_string("notifications")?;
            let category = input.get_optional_string("category")?;
            let planned_start_time = input.get_optional_string("planned_start_time")?;
            let account_id = input.get_optional_string("account_id")?;
            let related_ops_items = input.get_optional_string("related_ops_items")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_ops_item()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("description", description.unwrap_or_default())
                .with_field("actual_end_time", actual_end_time.unwrap_or_default())
                .with_field("priority", priority.unwrap_or_default())
                .with_field("actual_start_time", actual_start_time.unwrap_or_default())
                .with_field("operational_data", operational_data.unwrap_or_default())
                .with_field("source", source.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("planned_end_time", planned_end_time.unwrap_or_default())
                .with_field("ops_item_type", ops_item_type.unwrap_or_default())
                .with_field("title", title.unwrap_or_default())
                .with_field("severity", severity.unwrap_or_default())
                .with_field("notifications", notifications.unwrap_or_default())
                .with_field("category", category.unwrap_or_default())
                .with_field("planned_start_time", planned_start_time.unwrap_or_default())
                .with_field("account_id", account_id.unwrap_or_default())
                .with_field("related_ops_items", related_ops_items.unwrap_or_default())
            )
        })
    }

    /// Read a ops_item resource
    async fn read_ops_item(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_ops_item()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a ops_item resource
    async fn update_ops_item(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_string("description")?;
            let actual_end_time = input.get_optional_string("actual_end_time")?;
            let priority = input.get_optional_string("priority")?;
            let actual_start_time = input.get_optional_string("actual_start_time")?;
            let operational_data = input.get_optional_string("operational_data")?;
            let source = input.get_string("source")?;
            let tags = input.get_optional_string("tags")?;
            let planned_end_time = input.get_optional_string("planned_end_time")?;
            let ops_item_type = input.get_optional_string("ops_item_type")?;
            let title = input.get_string("title")?;
            let severity = input.get_optional_string("severity")?;
            let notifications = input.get_optional_string("notifications")?;
            let category = input.get_optional_string("category")?;
            let planned_start_time = input.get_optional_string("planned_start_time")?;
            let account_id = input.get_optional_string("account_id")?;
            let related_ops_items = input.get_optional_string("related_ops_items")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_ops_item()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("description", description.unwrap_or_default())
                .with_field("actual_end_time", actual_end_time.unwrap_or_default())
                .with_field("priority", priority.unwrap_or_default())
                .with_field("actual_start_time", actual_start_time.unwrap_or_default())
                .with_field("operational_data", operational_data.unwrap_or_default())
                .with_field("source", source.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("planned_end_time", planned_end_time.unwrap_or_default())
                .with_field("ops_item_type", ops_item_type.unwrap_or_default())
                .with_field("title", title.unwrap_or_default())
                .with_field("severity", severity.unwrap_or_default())
                .with_field("notifications", notifications.unwrap_or_default())
                .with_field("category", category.unwrap_or_default())
                .with_field("planned_start_time", planned_start_time.unwrap_or_default())
                .with_field("account_id", account_id.unwrap_or_default())
                .with_field("related_ops_items", related_ops_items.unwrap_or_default())
            )
        })
    }

    /// Delete a ops_item resource
    async fn delete_ops_item(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_ops_item()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_executions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_executions resource
    async fn plan_maintenance_window_executions(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_executions resource
    async fn create_maintenance_window_executions(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_executions()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_window_executions resource
    async fn read_maintenance_window_executions(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_executions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_executions resource
    async fn update_maintenance_window_executions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_executions()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_window_executions resource
    async fn delete_maintenance_window_executions(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_executions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Activation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a activation resource
    async fn plan_activation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new activation resource
    async fn create_activation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let iam_role = input.get_string("iam_role")?;
            let registration_limit = input.get_optional_string("registration_limit")?;
            let registration_metadata = input.get_optional_string("registration_metadata")?;
            let expiration_date = input.get_optional_string("expiration_date")?;
            let default_instance_name = input.get_optional_string("default_instance_name")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_activation()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("description", description.unwrap_or_default())
                .with_field("iam_role", iam_role.unwrap_or_default())
                .with_field("registration_limit", registration_limit.unwrap_or_default())
                .with_field("registration_metadata", registration_metadata.unwrap_or_default())
                .with_field("expiration_date", expiration_date.unwrap_or_default())
                .with_field("default_instance_name", default_instance_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Read a activation resource
    async fn read_activation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_activation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a activation resource
    async fn update_activation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let iam_role = input.get_string("iam_role")?;
            let registration_limit = input.get_optional_string("registration_limit")?;
            let registration_metadata = input.get_optional_string("registration_metadata")?;
            let expiration_date = input.get_optional_string("expiration_date")?;
            let default_instance_name = input.get_optional_string("default_instance_name")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_activation()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("description", description.unwrap_or_default())
                .with_field("iam_role", iam_role.unwrap_or_default())
                .with_field("registration_limit", registration_limit.unwrap_or_default())
                .with_field("registration_metadata", registration_metadata.unwrap_or_default())
                .with_field("expiration_date", expiration_date.unwrap_or_default())
                .with_field("default_instance_name", default_instance_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Delete a activation resource
    async fn delete_activation(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_activation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Effective_instance_associations resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a effective_instance_associations resource
    async fn plan_effective_instance_associations(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new effective_instance_associations resource
    async fn create_effective_instance_associations(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_effective_instance_associations()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a effective_instance_associations resource
    async fn read_effective_instance_associations(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_effective_instance_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a effective_instance_associations resource
    async fn update_effective_instance_associations(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_effective_instance_associations()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a effective_instance_associations resource
    async fn delete_effective_instance_associations(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_effective_instance_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Execution_preview resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a execution_preview resource
    async fn plan_execution_preview(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new execution_preview resource
    async fn create_execution_preview(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_execution_preview()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a execution_preview resource
    async fn read_execution_preview(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_execution_preview()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a execution_preview resource
    async fn update_execution_preview(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_execution_preview()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a execution_preview resource
    async fn delete_execution_preview(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_execution_preview()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Resource_policies resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a resource_policies resource
    async fn plan_resource_policies(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new resource_policies resource
    async fn create_resource_policies(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_resource_policies()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a resource_policies resource
    async fn read_resource_policies(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_resource_policies()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a resource_policies resource
    async fn update_resource_policies(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_resource_policies()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a resource_policies resource
    async fn delete_resource_policies(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_resource_policies()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Association_batch resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a association_batch resource
    async fn plan_association_batch(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new association_batch resource
    async fn create_association_batch(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let entries = input.get_string("entries")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_association_batch()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("entries", entries.unwrap_or_default())
            )
        })
    }

    /// Read a association_batch resource
    async fn read_association_batch(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_association_batch()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a association_batch resource
    async fn update_association_batch(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let entries = input.get_string("entries")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_association_batch()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("entries", entries.unwrap_or_default())
            )
        })
    }

    /// Delete a association_batch resource
    async fn delete_association_batch(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_association_batch()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Instance_associations_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance_associations_status resource
    async fn plan_instance_associations_status(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new instance_associations_status resource
    async fn create_instance_associations_status(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_instance_associations_status()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a instance_associations_status resource
    async fn read_instance_associations_status(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_instance_associations_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a instance_associations_status resource
    async fn update_instance_associations_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_instance_associations_status()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a instance_associations_status resource
    async fn delete_instance_associations_status(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_instance_associations_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Document resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document resource
    async fn plan_document(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new document resource
    async fn create_document(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let target_type = input.get_optional_string("target_type")?;
            let name = input.get_string("name")?;
            let attachments = input.get_optional_string("attachments")?;
            let version_name = input.get_optional_string("version_name")?;
            let document_format = input.get_optional_string("document_format")?;
            let content = input.get_string("content")?;
            let display_name = input.get_optional_string("display_name")?;
            let tags = input.get_optional_string("tags")?;
            let requires = input.get_optional_string("requires")?;
            let document_type = input.get_optional_string("document_type")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_document()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("target_type", target_type.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("attachments", attachments.unwrap_or_default())
                .with_field("version_name", version_name.unwrap_or_default())
                .with_field("document_format", document_format.unwrap_or_default())
                .with_field("content", content.unwrap_or_default())
                .with_field("display_name", display_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("requires", requires.unwrap_or_default())
                .with_field("document_type", document_type.unwrap_or_default())
            )
        })
    }

    /// Read a document resource
    async fn read_document(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_document()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a document resource
    async fn update_document(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let target_type = input.get_optional_string("target_type")?;
            let name = input.get_string("name")?;
            let attachments = input.get_optional_string("attachments")?;
            let version_name = input.get_optional_string("version_name")?;
            let document_format = input.get_optional_string("document_format")?;
            let content = input.get_string("content")?;
            let display_name = input.get_optional_string("display_name")?;
            let tags = input.get_optional_string("tags")?;
            let requires = input.get_optional_string("requires")?;
            let document_type = input.get_optional_string("document_type")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_document()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("target_type", target_type.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("attachments", attachments.unwrap_or_default())
                .with_field("version_name", version_name.unwrap_or_default())
                .with_field("document_format", document_format.unwrap_or_default())
                .with_field("content", content.unwrap_or_default())
                .with_field("display_name", display_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("requires", requires.unwrap_or_default())
                .with_field("document_type", document_type.unwrap_or_default())
            )
        })
    }

    /// Delete a document resource
    async fn delete_document(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_document()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Ops_summary resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ops_summary resource
    async fn plan_ops_summary(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ops_summary resource
    async fn create_ops_summary(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_ops_summary()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a ops_summary resource
    async fn read_ops_summary(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_ops_summary()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a ops_summary resource
    async fn update_ops_summary(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_ops_summary()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a ops_summary resource
    async fn delete_ops_summary(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_ops_summary()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Command_invocation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a command_invocation resource
    async fn plan_command_invocation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new command_invocation resource
    async fn create_command_invocation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_command_invocation()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a command_invocation resource
    async fn read_command_invocation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_command_invocation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a command_invocation resource
    async fn update_command_invocation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_command_invocation()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a command_invocation resource
    async fn delete_command_invocation(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_command_invocation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Compliance_items resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a compliance_items resource
    async fn plan_compliance_items(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new compliance_items resource
    async fn create_compliance_items(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let compliance_type = input.get_string("compliance_type")?;
            let items = input.get_string("items")?;
            let upload_type = input.get_optional_string("upload_type")?;
            let resource_type = input.get_string("resource_type")?;
            let resource_id = input.get_string("resource_id")?;
            let execution_summary = input.get_string("execution_summary")?;
            let item_content_hash = input.get_optional_string("item_content_hash")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_compliance_items()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("compliance_type", compliance_type.unwrap_or_default())
                .with_field("items", items.unwrap_or_default())
                .with_field("upload_type", upload_type.unwrap_or_default())
                .with_field("resource_type", resource_type.unwrap_or_default())
                .with_field("resource_id", resource_id.unwrap_or_default())
                .with_field("execution_summary", execution_summary.unwrap_or_default())
                .with_field("item_content_hash", item_content_hash.unwrap_or_default())
            )
        })
    }

    /// Read a compliance_items resource
    async fn read_compliance_items(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_compliance_items()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a compliance_items resource
    async fn update_compliance_items(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let compliance_type = input.get_string("compliance_type")?;
            let items = input.get_string("items")?;
            let upload_type = input.get_optional_string("upload_type")?;
            let resource_type = input.get_string("resource_type")?;
            let resource_id = input.get_string("resource_id")?;
            let execution_summary = input.get_string("execution_summary")?;
            let item_content_hash = input.get_optional_string("item_content_hash")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_compliance_items()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("compliance_type", compliance_type.unwrap_or_default())
                .with_field("items", items.unwrap_or_default())
                .with_field("upload_type", upload_type.unwrap_or_default())
                .with_field("resource_type", resource_type.unwrap_or_default())
                .with_field("resource_id", resource_id.unwrap_or_default())
                .with_field("execution_summary", execution_summary.unwrap_or_default())
                .with_field("item_content_hash", item_content_hash.unwrap_or_default())
            )
        })
    }

    /// Delete a compliance_items resource
    async fn delete_compliance_items(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_compliance_items()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Patch_baselines resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a patch_baselines resource
    async fn plan_patch_baselines(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new patch_baselines resource
    async fn create_patch_baselines(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_patch_baselines()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a patch_baselines resource
    async fn read_patch_baselines(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_patch_baselines()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a patch_baselines resource
    async fn update_patch_baselines(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_patch_baselines()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a patch_baselines resource
    async fn delete_patch_baselines(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_patch_baselines()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Access_token resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a access_token resource
    async fn plan_access_token(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new access_token resource
    async fn create_access_token(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_access_token()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a access_token resource
    async fn read_access_token(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_access_token()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a access_token resource
    async fn update_access_token(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_access_token()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a access_token resource
    async fn delete_access_token(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_access_token()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_tasks resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_tasks resource
    async fn plan_maintenance_window_tasks(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_tasks resource
    async fn create_maintenance_window_tasks(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_tasks()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_window_tasks resource
    async fn read_maintenance_window_tasks(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_tasks()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_tasks resource
    async fn update_maintenance_window_tasks(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_tasks()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_window_tasks resource
    async fn delete_maintenance_window_tasks(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_tasks()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_execution_tasks resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_execution_tasks resource
    async fn plan_maintenance_window_execution_tasks(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_execution_tasks resource
    async fn create_maintenance_window_execution_tasks(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_execution_tasks()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_window_execution_tasks resource
    async fn read_maintenance_window_execution_tasks(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_execution_tasks()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_execution_tasks resource
    async fn update_maintenance_window_execution_tasks(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_execution_tasks()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_window_execution_tasks resource
    async fn delete_maintenance_window_execution_tasks(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_execution_tasks()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Deployable_patch_snapshot_for_instance resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a deployable_patch_snapshot_for_instance resource
    async fn plan_deployable_patch_snapshot_for_instance(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new deployable_patch_snapshot_for_instance resource
    async fn create_deployable_patch_snapshot_for_instance(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_deployable_patch_snapshot_for_instance()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a deployable_patch_snapshot_for_instance resource
    async fn read_deployable_patch_snapshot_for_instance(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_deployable_patch_snapshot_for_instance()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a deployable_patch_snapshot_for_instance resource
    async fn update_deployable_patch_snapshot_for_instance(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_deployable_patch_snapshot_for_instance()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a deployable_patch_snapshot_for_instance resource
    async fn delete_deployable_patch_snapshot_for_instance(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_deployable_patch_snapshot_for_instance()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_schedule resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_schedule resource
    async fn plan_maintenance_window_schedule(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_schedule resource
    async fn create_maintenance_window_schedule(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_schedule()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_window_schedule resource
    async fn read_maintenance_window_schedule(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_schedule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_schedule resource
    async fn update_maintenance_window_schedule(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_schedule()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_window_schedule resource
    async fn delete_maintenance_window_schedule(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_schedule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Document_metadata resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document_metadata resource
    async fn plan_document_metadata(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new document_metadata resource
    async fn create_document_metadata(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let document_version = input.get_optional_string("document_version")?;
            let document_reviews = input.get_string("document_reviews")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_document_metadata()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("document_version", document_version.unwrap_or_default())
                .with_field("document_reviews", document_reviews.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a document_metadata resource
    async fn read_document_metadata(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_document_metadata()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a document_metadata resource
    async fn update_document_metadata(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let document_version = input.get_optional_string("document_version")?;
            let document_reviews = input.get_string("document_reviews")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_document_metadata()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("document_version", document_version.unwrap_or_default())
                .with_field("document_reviews", document_reviews.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a document_metadata resource
    async fn delete_document_metadata(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_document_metadata()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Association_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a association_status resource
    async fn plan_association_status(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new association_status resource
    async fn create_association_status(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let association_status = input.get_string("association_status")?;
            let name = input.get_string("name")?;
            let instance_id = input.get_string("instance_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_association_status()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("association_status", association_status.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("instance_id", instance_id.unwrap_or_default())
            )
        })
    }

    /// Read a association_status resource
    async fn read_association_status(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_association_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a association_status resource
    async fn update_association_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let association_status = input.get_string("association_status")?;
            let name = input.get_string("name")?;
            let instance_id = input.get_string("instance_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_association_status()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("association_status", association_status.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("instance_id", instance_id.unwrap_or_default())
            )
        })
    }

    /// Delete a association_status resource
    async fn delete_association_status(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_association_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_execution_task_invocations resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_execution_task_invocations resource
    async fn plan_maintenance_window_execution_task_invocations(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_execution_task_invocations resource
    async fn create_maintenance_window_execution_task_invocations(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_execution_task_invocations()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_window_execution_task_invocations resource
    async fn read_maintenance_window_execution_task_invocations(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_execution_task_invocations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_execution_task_invocations resource
    async fn update_maintenance_window_execution_task_invocations(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_execution_task_invocations()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_window_execution_task_invocations resource
    async fn delete_maintenance_window_execution_task_invocations(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_execution_task_invocations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Instance_patch_states_for_patch_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance_patch_states_for_patch_group resource
    async fn plan_instance_patch_states_for_patch_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new instance_patch_states_for_patch_group resource
    async fn create_instance_patch_states_for_patch_group(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_instance_patch_states_for_patch_group()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a instance_patch_states_for_patch_group resource
    async fn read_instance_patch_states_for_patch_group(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_instance_patch_states_for_patch_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a instance_patch_states_for_patch_group resource
    async fn update_instance_patch_states_for_patch_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_instance_patch_states_for_patch_group()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a instance_patch_states_for_patch_group resource
    async fn delete_instance_patch_states_for_patch_group(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_instance_patch_states_for_patch_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Patch_groups resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a patch_groups resource
    async fn plan_patch_groups(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new patch_groups resource
    async fn create_patch_groups(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_patch_groups()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a patch_groups resource
    async fn read_patch_groups(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_patch_groups()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a patch_groups resource
    async fn update_patch_groups(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_patch_groups()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a patch_groups resource
    async fn delete_patch_groups(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_patch_groups()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Automation_step_executions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a automation_step_executions resource
    async fn plan_automation_step_executions(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new automation_step_executions resource
    async fn create_automation_step_executions(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_automation_step_executions()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a automation_step_executions resource
    async fn read_automation_step_executions(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_automation_step_executions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a automation_step_executions resource
    async fn update_automation_step_executions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_automation_step_executions()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a automation_step_executions resource
    async fn delete_automation_step_executions(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_automation_step_executions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Patch_baseline_for_patch_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a patch_baseline_for_patch_group resource
    async fn plan_patch_baseline_for_patch_group(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new patch_baseline_for_patch_group resource
    async fn create_patch_baseline_for_patch_group(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_patch_baseline_for_patch_group()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a patch_baseline_for_patch_group resource
    async fn read_patch_baseline_for_patch_group(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_patch_baseline_for_patch_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a patch_baseline_for_patch_group resource
    async fn update_patch_baseline_for_patch_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_patch_baseline_for_patch_group()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a patch_baseline_for_patch_group resource
    async fn delete_patch_baseline_for_patch_group(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_patch_baseline_for_patch_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Ops_metadata resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ops_metadata resource
    async fn plan_ops_metadata(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ops_metadata resource
    async fn create_ops_metadata(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_id = input.get_string("resource_id")?;
            let tags = input.get_optional_string("tags")?;
            let metadata = input.get_optional_string("metadata")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_ops_metadata()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("resource_id", resource_id.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("metadata", metadata.unwrap_or_default())
            )
        })
    }

    /// Read a ops_metadata resource
    async fn read_ops_metadata(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_ops_metadata()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a ops_metadata resource
    async fn update_ops_metadata(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_id = input.get_string("resource_id")?;
            let tags = input.get_optional_string("tags")?;
            let metadata = input.get_optional_string("metadata")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_ops_metadata()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("resource_id", resource_id.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("metadata", metadata.unwrap_or_default())
            )
        })
    }

    /// Delete a ops_metadata resource
    async fn delete_ops_metadata(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_ops_metadata()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Inventory resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory resource
    async fn plan_inventory(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory resource
    async fn create_inventory(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let items = input.get_string("items")?;
            let instance_id = input.get_string("instance_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_inventory()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("items", items.unwrap_or_default())
                .with_field("instance_id", instance_id.unwrap_or_default())
            )
        })
    }

    /// Read a inventory resource
    async fn read_inventory(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_inventory()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a inventory resource
    async fn update_inventory(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let items = input.get_string("items")?;
            let instance_id = input.get_string("instance_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_inventory()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("items", items.unwrap_or_default())
                .with_field("instance_id", instance_id.unwrap_or_default())
            )
        })
    }

    /// Delete a inventory resource
    async fn delete_inventory(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_inventory()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Patch_properties resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a patch_properties resource
    async fn plan_patch_properties(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new patch_properties resource
    async fn create_patch_properties(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_patch_properties()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a patch_properties resource
    async fn read_patch_properties(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_patch_properties()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a patch_properties resource
    async fn update_patch_properties(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_patch_properties()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a patch_properties resource
    async fn delete_patch_properties(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_patch_properties()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Instance_patch_states resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance_patch_states resource
    async fn plan_instance_patch_states(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new instance_patch_states resource
    async fn create_instance_patch_states(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_instance_patch_states()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a instance_patch_states resource
    async fn read_instance_patch_states(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_instance_patch_states()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a instance_patch_states resource
    async fn update_instance_patch_states(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_instance_patch_states()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a instance_patch_states resource
    async fn delete_instance_patch_states(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_instance_patch_states()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Instance_information resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance_information resource
    async fn plan_instance_information(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new instance_information resource
    async fn create_instance_information(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_instance_information()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a instance_information resource
    async fn read_instance_information(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_instance_information()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a instance_information resource
    async fn update_instance_information(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_instance_information()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a instance_information resource
    async fn delete_instance_information(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_instance_information()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Resource_policy resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a resource_policy resource
    async fn plan_resource_policy(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new resource_policy resource
    async fn create_resource_policy(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let policy_hash = input.get_optional_string("policy_hash")?;
            let policy_id = input.get_optional_string("policy_id")?;
            let policy = input.get_string("policy")?;
            let resource_arn = input.get_string("resource_arn")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_resource_policy()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("policy_hash", policy_hash.unwrap_or_default())
                .with_field("policy_id", policy_id.unwrap_or_default())
                .with_field("policy", policy.unwrap_or_default())
                .with_field("resource_arn", resource_arn.unwrap_or_default())
            )
        })
    }

    /// Read a resource_policy resource
    async fn read_resource_policy(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_resource_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a resource_policy resource
    async fn update_resource_policy(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let policy_hash = input.get_optional_string("policy_hash")?;
            let policy_id = input.get_optional_string("policy_id")?;
            let policy = input.get_string("policy")?;
            let resource_arn = input.get_string("resource_arn")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_resource_policy()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("policy_hash", policy_hash.unwrap_or_default())
                .with_field("policy_id", policy_id.unwrap_or_default())
                .with_field("policy", policy.unwrap_or_default())
                .with_field("resource_arn", resource_arn.unwrap_or_default())
            )
        })
    }

    /// Delete a resource_policy resource
    async fn delete_resource_policy(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_resource_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Activations resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a activations resource
    async fn plan_activations(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new activations resource
    async fn create_activations(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_activations()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a activations resource
    async fn read_activations(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_activations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a activations resource
    async fn update_activations(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_activations()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a activations resource
    async fn delete_activations(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_activations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_execution_task_invocation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_execution_task_invocation resource
    async fn plan_maintenance_window_execution_task_invocation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_execution_task_invocation resource
    async fn create_maintenance_window_execution_task_invocation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_execution_task_invocation()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_window_execution_task_invocation resource
    async fn read_maintenance_window_execution_task_invocation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_execution_task_invocation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_execution_task_invocation resource
    async fn update_maintenance_window_execution_task_invocation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_execution_task_invocation()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_window_execution_task_invocation resource
    async fn delete_maintenance_window_execution_task_invocation(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_execution_task_invocation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_windows_for_target resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_windows_for_target resource
    async fn plan_maintenance_windows_for_target(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_windows_for_target resource
    async fn create_maintenance_windows_for_target(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_windows_for_target()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_windows_for_target resource
    async fn read_maintenance_windows_for_target(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_windows_for_target()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_windows_for_target resource
    async fn update_maintenance_windows_for_target(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_windows_for_target()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_windows_for_target resource
    async fn delete_maintenance_windows_for_target(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_windows_for_target()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Service_setting resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a service_setting resource
    async fn plan_service_setting(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new service_setting resource
    async fn create_service_setting(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let setting_id = input.get_string("setting_id")?;
            let setting_value = input.get_string("setting_value")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_service_setting()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("setting_id", setting_id.unwrap_or_default())
                .with_field("setting_value", setting_value.unwrap_or_default())
            )
        })
    }

    /// Read a service_setting resource
    async fn read_service_setting(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_service_setting()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a service_setting resource
    async fn update_service_setting(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let setting_id = input.get_string("setting_id")?;
            let setting_value = input.get_string("setting_value")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_service_setting()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("setting_id", setting_id.unwrap_or_default())
                .with_field("setting_value", setting_value.unwrap_or_default())
            )
        })
    }

    /// Delete a service_setting resource
    async fn delete_service_setting(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_service_setting()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Automation_execution resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a automation_execution resource
    async fn plan_automation_execution(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new automation_execution resource
    async fn create_automation_execution(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_automation_execution()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a automation_execution resource
    async fn read_automation_execution(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_automation_execution()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a automation_execution resource
    async fn update_automation_execution(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_automation_execution()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a automation_execution resource
    async fn delete_automation_execution(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_automation_execution()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Patch_group_state resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a patch_group_state resource
    async fn plan_patch_group_state(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new patch_group_state resource
    async fn create_patch_group_state(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_patch_group_state()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a patch_group_state resource
    async fn read_patch_group_state(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_patch_group_state()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a patch_group_state resource
    async fn update_patch_group_state(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_patch_group_state()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a patch_group_state resource
    async fn delete_patch_group_state(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_patch_group_state()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window resource
    async fn plan_maintenance_window(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window resource
    async fn create_maintenance_window(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let end_date = input.get_optional_string("end_date")?;
            let client_token = input.get_optional_string("client_token")?;
            let schedule = input.get_string("schedule")?;
            let name = input.get_string("name")?;
            let start_date = input.get_optional_string("start_date")?;
            let allow_unassociated_targets = input.get_string("allow_unassociated_targets")?;
            let schedule_timezone = input.get_optional_string("schedule_timezone")?;
            let description = input.get_optional_string("description")?;
            let schedule_offset = input.get_optional_string("schedule_offset")?;
            let cutoff = input.get_string("cutoff")?;
            let duration = input.get_string("duration")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("end_date", end_date.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("schedule", schedule.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("start_date", start_date.unwrap_or_default())
                .with_field("allow_unassociated_targets", allow_unassociated_targets.unwrap_or_default())
                .with_field("schedule_timezone", schedule_timezone.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("schedule_offset", schedule_offset.unwrap_or_default())
                .with_field("cutoff", cutoff.unwrap_or_default())
                .with_field("duration", duration.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Read a maintenance_window resource
    async fn read_maintenance_window(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window resource
    async fn update_maintenance_window(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let end_date = input.get_optional_string("end_date")?;
            let client_token = input.get_optional_string("client_token")?;
            let schedule = input.get_string("schedule")?;
            let name = input.get_string("name")?;
            let start_date = input.get_optional_string("start_date")?;
            let allow_unassociated_targets = input.get_string("allow_unassociated_targets")?;
            let schedule_timezone = input.get_optional_string("schedule_timezone")?;
            let description = input.get_optional_string("description")?;
            let schedule_offset = input.get_optional_string("schedule_offset")?;
            let cutoff = input.get_string("cutoff")?;
            let duration = input.get_string("duration")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("end_date", end_date.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("schedule", schedule.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("start_date", start_date.unwrap_or_default())
                .with_field("allow_unassociated_targets", allow_unassociated_targets.unwrap_or_default())
                .with_field("schedule_timezone", schedule_timezone.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("schedule_offset", schedule_offset.unwrap_or_default())
                .with_field("cutoff", cutoff.unwrap_or_default())
                .with_field("duration", duration.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Delete a maintenance_window resource
    async fn delete_maintenance_window(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Parameter resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a parameter resource
    async fn plan_parameter(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new parameter resource
    async fn create_parameter(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let data_type = input.get_optional_string("data_type")?;
            let description = input.get_optional_string("description")?;
            let value = input.get_string("value")?;
            let name = input.get_string("name")?;
            let r#type = input.get_optional_string("type")?;
            let allowed_pattern = input.get_optional_string("allowed_pattern")?;
            let tags = input.get_optional_string("tags")?;
            let key_id = input.get_optional_string("key_id")?;
            let overwrite = input.get_optional_string("overwrite")?;
            let tier = input.get_optional_string("tier")?;
            let policies = input.get_optional_string("policies")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_parameter()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("data_type", data_type.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("value", value.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("type", r#type.unwrap_or_default())
                .with_field("allowed_pattern", allowed_pattern.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("key_id", key_id.unwrap_or_default())
                .with_field("overwrite", overwrite.unwrap_or_default())
                .with_field("tier", tier.unwrap_or_default())
                .with_field("policies", policies.unwrap_or_default())
            )
        })
    }

    /// Read a parameter resource
    async fn read_parameter(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_parameter()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a parameter resource
    async fn update_parameter(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let data_type = input.get_optional_string("data_type")?;
            let description = input.get_optional_string("description")?;
            let value = input.get_string("value")?;
            let name = input.get_string("name")?;
            let r#type = input.get_optional_string("type")?;
            let allowed_pattern = input.get_optional_string("allowed_pattern")?;
            let tags = input.get_optional_string("tags")?;
            let key_id = input.get_optional_string("key_id")?;
            let overwrite = input.get_optional_string("overwrite")?;
            let tier = input.get_optional_string("tier")?;
            let policies = input.get_optional_string("policies")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_parameter()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("data_type", data_type.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("value", value.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("type", r#type.unwrap_or_default())
                .with_field("allowed_pattern", allowed_pattern.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("key_id", key_id.unwrap_or_default())
                .with_field("overwrite", overwrite.unwrap_or_default())
                .with_field("tier", tier.unwrap_or_default())
                .with_field("policies", policies.unwrap_or_default())
            )
        })
    }

    /// Delete a parameter resource
    async fn delete_parameter(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_parameter()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Sessions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sessions resource
    async fn plan_sessions(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sessions resource
    async fn create_sessions(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_sessions()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a sessions resource
    async fn read_sessions(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_sessions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a sessions resource
    async fn update_sessions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_sessions()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a sessions resource
    async fn delete_sessions(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_sessions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Resource_data_sync resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a resource_data_sync resource
    async fn plan_resource_data_sync(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new resource_data_sync resource
    async fn create_resource_data_sync(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let sync_type = input.get_optional_string("sync_type")?;
            let sync_source = input.get_optional_string("sync_source")?;
            let s3_destination = input.get_optional_string("s3_destination")?;
            let sync_name = input.get_string("sync_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_resource_data_sync()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("sync_type", sync_type.unwrap_or_default())
                .with_field("sync_source", sync_source.unwrap_or_default())
                .with_field("s3_destination", s3_destination.unwrap_or_default())
                .with_field("sync_name", sync_name.unwrap_or_default())
            )
        })
    }

    /// Read a resource_data_sync resource
    async fn read_resource_data_sync(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_resource_data_sync()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a resource_data_sync resource
    async fn update_resource_data_sync(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let sync_type = input.get_optional_string("sync_type")?;
            let sync_source = input.get_optional_string("sync_source")?;
            let s3_destination = input.get_optional_string("s3_destination")?;
            let sync_name = input.get_string("sync_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_resource_data_sync()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("sync_type", sync_type.unwrap_or_default())
                .with_field("sync_source", sync_source.unwrap_or_default())
                .with_field("s3_destination", s3_destination.unwrap_or_default())
                .with_field("sync_name", sync_name.unwrap_or_default())
            )
        })
    }

    /// Delete a resource_data_sync resource
    async fn delete_resource_data_sync(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_resource_data_sync()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Managed_instance_role resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a managed_instance_role resource
    async fn plan_managed_instance_role(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new managed_instance_role resource
    async fn create_managed_instance_role(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let instance_id = input.get_string("instance_id")?;
            let iam_role = input.get_string("iam_role")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_managed_instance_role()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("instance_id", instance_id.unwrap_or_default())
                .with_field("iam_role", iam_role.unwrap_or_default())
            )
        })
    }

    /// Read a managed_instance_role resource
    async fn read_managed_instance_role(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_managed_instance_role()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a managed_instance_role resource
    async fn update_managed_instance_role(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let instance_id = input.get_string("instance_id")?;
            let iam_role = input.get_string("iam_role")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_managed_instance_role()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("instance_id", instance_id.unwrap_or_default())
                .with_field("iam_role", iam_role.unwrap_or_default())
            )
        })
    }

    /// Delete a managed_instance_role resource
    async fn delete_managed_instance_role(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_managed_instance_role()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Default_patch_baseline resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a default_patch_baseline resource
    async fn plan_default_patch_baseline(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new default_patch_baseline resource
    async fn create_default_patch_baseline(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_default_patch_baseline()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a default_patch_baseline resource
    async fn read_default_patch_baseline(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_default_patch_baseline()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a default_patch_baseline resource
    async fn update_default_patch_baseline(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_default_patch_baseline()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a default_patch_baseline resource
    async fn delete_default_patch_baseline(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_default_patch_baseline()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Automation_executions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a automation_executions resource
    async fn plan_automation_executions(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new automation_executions resource
    async fn create_automation_executions(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_automation_executions()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a automation_executions resource
    async fn read_automation_executions(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_automation_executions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a automation_executions resource
    async fn update_automation_executions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_automation_executions()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a automation_executions resource
    async fn delete_automation_executions(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_automation_executions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connection_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connection_status resource
    async fn plan_connection_status(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new connection_status resource
    async fn create_connection_status(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_connection_status()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a connection_status resource
    async fn read_connection_status(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_connection_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connection_status resource
    async fn update_connection_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_connection_status()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a connection_status resource
    async fn delete_connection_status(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_connection_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Ops_items resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ops_items resource
    async fn plan_ops_items(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ops_items resource
    async fn create_ops_items(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_ops_items()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a ops_items resource
    async fn read_ops_items(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_ops_items()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a ops_items resource
    async fn update_ops_items(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_ops_items()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a ops_items resource
    async fn delete_ops_items(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_ops_items()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_execution resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_execution resource
    async fn plan_maintenance_window_execution(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_execution resource
    async fn create_maintenance_window_execution(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_execution()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_window_execution resource
    async fn read_maintenance_window_execution(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_execution()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_execution resource
    async fn update_maintenance_window_execution(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_execution()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_window_execution resource
    async fn delete_maintenance_window_execution(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_execution()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_task resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_task resource
    async fn plan_maintenance_window_task(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_task resource
    async fn create_maintenance_window_task(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let targets = input.get_optional_string("targets")?;
            let priority = input.get_optional_string("priority")?;
            let window_id = input.get_string("window_id")?;
            let alarm_configuration = input.get_optional_string("alarm_configuration")?;
            let service_role_arn = input.get_optional_string("service_role_arn")?;
            let replace = input.get_optional_string("replace")?;
            let max_errors = input.get_optional_string("max_errors")?;
            let logging_info = input.get_optional_string("logging_info")?;
            let task_arn = input.get_optional_string("task_arn")?;
            let task_parameters = input.get_optional_string("task_parameters")?;
            let window_task_id = input.get_string("window_task_id")?;
            let task_invocation_parameters = input.get_optional_string("task_invocation_parameters")?;
            let max_concurrency = input.get_optional_string("max_concurrency")?;
            let name = input.get_optional_string("name")?;
            let description = input.get_optional_string("description")?;
            let cutoff_behavior = input.get_optional_string("cutoff_behavior")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_task()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("targets", targets.unwrap_or_default())
                .with_field("priority", priority.unwrap_or_default())
                .with_field("window_id", window_id.unwrap_or_default())
                .with_field("alarm_configuration", alarm_configuration.unwrap_or_default())
                .with_field("service_role_arn", service_role_arn.unwrap_or_default())
                .with_field("replace", replace.unwrap_or_default())
                .with_field("max_errors", max_errors.unwrap_or_default())
                .with_field("logging_info", logging_info.unwrap_or_default())
                .with_field("task_arn", task_arn.unwrap_or_default())
                .with_field("task_parameters", task_parameters.unwrap_or_default())
                .with_field("window_task_id", window_task_id.unwrap_or_default())
                .with_field("task_invocation_parameters", task_invocation_parameters.unwrap_or_default())
                .with_field("max_concurrency", max_concurrency.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("cutoff_behavior", cutoff_behavior.unwrap_or_default())
            )
        })
    }

    /// Read a maintenance_window_task resource
    async fn read_maintenance_window_task(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_task()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_task resource
    async fn update_maintenance_window_task(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let targets = input.get_optional_string("targets")?;
            let priority = input.get_optional_string("priority")?;
            let window_id = input.get_string("window_id")?;
            let alarm_configuration = input.get_optional_string("alarm_configuration")?;
            let service_role_arn = input.get_optional_string("service_role_arn")?;
            let replace = input.get_optional_string("replace")?;
            let max_errors = input.get_optional_string("max_errors")?;
            let logging_info = input.get_optional_string("logging_info")?;
            let task_arn = input.get_optional_string("task_arn")?;
            let task_parameters = input.get_optional_string("task_parameters")?;
            let window_task_id = input.get_string("window_task_id")?;
            let task_invocation_parameters = input.get_optional_string("task_invocation_parameters")?;
            let max_concurrency = input.get_optional_string("max_concurrency")?;
            let name = input.get_optional_string("name")?;
            let description = input.get_optional_string("description")?;
            let cutoff_behavior = input.get_optional_string("cutoff_behavior")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_task()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("targets", targets.unwrap_or_default())
                .with_field("priority", priority.unwrap_or_default())
                .with_field("window_id", window_id.unwrap_or_default())
                .with_field("alarm_configuration", alarm_configuration.unwrap_or_default())
                .with_field("service_role_arn", service_role_arn.unwrap_or_default())
                .with_field("replace", replace.unwrap_or_default())
                .with_field("max_errors", max_errors.unwrap_or_default())
                .with_field("logging_info", logging_info.unwrap_or_default())
                .with_field("task_arn", task_arn.unwrap_or_default())
                .with_field("task_parameters", task_parameters.unwrap_or_default())
                .with_field("window_task_id", window_task_id.unwrap_or_default())
                .with_field("task_invocation_parameters", task_invocation_parameters.unwrap_or_default())
                .with_field("max_concurrency", max_concurrency.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("cutoff_behavior", cutoff_behavior.unwrap_or_default())
            )
        })
    }

    /// Delete a maintenance_window_task resource
    async fn delete_maintenance_window_task(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_task()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Document_permission resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document_permission resource
    async fn plan_document_permission(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new document_permission resource
    async fn create_document_permission(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_document_permission()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a document_permission resource
    async fn read_document_permission(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_document_permission()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a document_permission resource
    async fn update_document_permission(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_document_permission()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a document_permission resource
    async fn delete_document_permission(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_document_permission()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Instance_properties resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance_properties resource
    async fn plan_instance_properties(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new instance_properties resource
    async fn create_instance_properties(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_instance_properties()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a instance_properties resource
    async fn read_instance_properties(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_instance_properties()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a instance_properties resource
    async fn update_instance_properties(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_instance_properties()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a instance_properties resource
    async fn delete_instance_properties(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_instance_properties()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Inventory_schema resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory_schema resource
    async fn plan_inventory_schema(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory_schema resource
    async fn create_inventory_schema(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_inventory_schema()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a inventory_schema resource
    async fn read_inventory_schema(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_inventory_schema()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a inventory_schema resource
    async fn update_inventory_schema(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_inventory_schema()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a inventory_schema resource
    async fn delete_inventory_schema(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_inventory_schema()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Association_executions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a association_executions resource
    async fn plan_association_executions(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new association_executions resource
    async fn create_association_executions(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_association_executions()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a association_executions resource
    async fn read_association_executions(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_association_executions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a association_executions resource
    async fn update_association_executions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_association_executions()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a association_executions resource
    async fn delete_association_executions(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_association_executions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Parameters_by_path resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a parameters_by_path resource
    async fn plan_parameters_by_path(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new parameters_by_path resource
    async fn create_parameters_by_path(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_parameters_by_path()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a parameters_by_path resource
    async fn read_parameters_by_path(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_parameters_by_path()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a parameters_by_path resource
    async fn update_parameters_by_path(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_parameters_by_path()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a parameters_by_path resource
    async fn delete_parameters_by_path(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_parameters_by_path()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Patch_baseline resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a patch_baseline resource
    async fn plan_patch_baseline(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new patch_baseline resource
    async fn create_patch_baseline(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let rejected_patches_action = input.get_optional_string("rejected_patches_action")?;
            let approved_patches = input.get_optional_string("approved_patches")?;
            let name = input.get_string("name")?;
            let approved_patches_compliance_level = input.get_optional_string("approved_patches_compliance_level")?;
            let approved_patches_enable_non_security = input.get_optional_string("approved_patches_enable_non_security")?;
            let available_security_updates_compliance_status = input.get_optional_string("available_security_updates_compliance_status")?;
            let operating_system = input.get_optional_string("operating_system")?;
            let client_token = input.get_optional_string("client_token")?;
            let sources = input.get_optional_string("sources")?;
            let tags = input.get_optional_string("tags")?;
            let description = input.get_optional_string("description")?;
            let rejected_patches = input.get_optional_string("rejected_patches")?;
            let global_filters = input.get_optional_string("global_filters")?;
            let approval_rules = input.get_optional_string("approval_rules")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_patch_baseline()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("rejected_patches_action", rejected_patches_action.unwrap_or_default())
                .with_field("approved_patches", approved_patches.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("approved_patches_compliance_level", approved_patches_compliance_level.unwrap_or_default())
                .with_field("approved_patches_enable_non_security", approved_patches_enable_non_security.unwrap_or_default())
                .with_field("available_security_updates_compliance_status", available_security_updates_compliance_status.unwrap_or_default())
                .with_field("operating_system", operating_system.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("sources", sources.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("rejected_patches", rejected_patches.unwrap_or_default())
                .with_field("global_filters", global_filters.unwrap_or_default())
                .with_field("approval_rules", approval_rules.unwrap_or_default())
            )
        })
    }

    /// Read a patch_baseline resource
    async fn read_patch_baseline(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_patch_baseline()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a patch_baseline resource
    async fn update_patch_baseline(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let rejected_patches_action = input.get_optional_string("rejected_patches_action")?;
            let approved_patches = input.get_optional_string("approved_patches")?;
            let name = input.get_string("name")?;
            let approved_patches_compliance_level = input.get_optional_string("approved_patches_compliance_level")?;
            let approved_patches_enable_non_security = input.get_optional_string("approved_patches_enable_non_security")?;
            let available_security_updates_compliance_status = input.get_optional_string("available_security_updates_compliance_status")?;
            let operating_system = input.get_optional_string("operating_system")?;
            let client_token = input.get_optional_string("client_token")?;
            let sources = input.get_optional_string("sources")?;
            let tags = input.get_optional_string("tags")?;
            let description = input.get_optional_string("description")?;
            let rejected_patches = input.get_optional_string("rejected_patches")?;
            let global_filters = input.get_optional_string("global_filters")?;
            let approval_rules = input.get_optional_string("approval_rules")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_patch_baseline()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("rejected_patches_action", rejected_patches_action.unwrap_or_default())
                .with_field("approved_patches", approved_patches.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("approved_patches_compliance_level", approved_patches_compliance_level.unwrap_or_default())
                .with_field("approved_patches_enable_non_security", approved_patches_enable_non_security.unwrap_or_default())
                .with_field("available_security_updates_compliance_status", available_security_updates_compliance_status.unwrap_or_default())
                .with_field("operating_system", operating_system.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("sources", sources.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("rejected_patches", rejected_patches.unwrap_or_default())
                .with_field("global_filters", global_filters.unwrap_or_default())
                .with_field("approval_rules", approval_rules.unwrap_or_default())
            )
        })
    }

    /// Delete a patch_baseline resource
    async fn delete_patch_baseline(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_patch_baseline()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Parameters resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a parameters resource
    async fn plan_parameters(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new parameters resource
    async fn create_parameters(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_parameters()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a parameters resource
    async fn read_parameters(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_parameters()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a parameters resource
    async fn update_parameters(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_parameters()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a parameters resource
    async fn delete_parameters(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_parameters()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Available_patches resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a available_patches resource
    async fn plan_available_patches(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new available_patches resource
    async fn create_available_patches(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_available_patches()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a available_patches resource
    async fn read_available_patches(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_available_patches()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a available_patches resource
    async fn update_available_patches(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_available_patches()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a available_patches resource
    async fn delete_available_patches(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_available_patches()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_window_execution_task resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_window_execution_task resource
    async fn plan_maintenance_window_execution_task(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_window_execution_task resource
    async fn create_maintenance_window_execution_task(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_window_execution_task()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_window_execution_task resource
    async fn read_maintenance_window_execution_task(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_window_execution_task()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_window_execution_task resource
    async fn update_maintenance_window_execution_task(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_window_execution_task()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_window_execution_task resource
    async fn delete_maintenance_window_execution_task(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_window_execution_task()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Instance_patches resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance_patches resource
    async fn plan_instance_patches(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new instance_patches resource
    async fn create_instance_patches(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_instance_patches()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a instance_patches resource
    async fn read_instance_patches(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_instance_patches()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a instance_patches resource
    async fn update_instance_patches(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_instance_patches()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a instance_patches resource
    async fn delete_instance_patches(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_instance_patches()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Inventory_deletions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inventory_deletions resource
    async fn plan_inventory_deletions(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new inventory_deletions resource
    async fn create_inventory_deletions(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_inventory_deletions()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a inventory_deletions resource
    async fn read_inventory_deletions(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_inventory_deletions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a inventory_deletions resource
    async fn update_inventory_deletions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_inventory_deletions()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a inventory_deletions resource
    async fn delete_inventory_deletions(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_inventory_deletions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Maintenance_windows resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a maintenance_windows resource
    async fn plan_maintenance_windows(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new maintenance_windows resource
    async fn create_maintenance_windows(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .create_maintenance_windows()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
            )
        })
    }

    /// Read a maintenance_windows resource
    async fn read_maintenance_windows(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .describe_maintenance_windows()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a maintenance_windows resource
    async fn update_maintenance_windows(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_client
            //     .update_maintenance_windows()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
            )
        })
    }

    /// Delete a maintenance_windows resource
    async fn delete_maintenance_windows(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_client
            //     .delete_maintenance_windows()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
