//! Workspaces service for Aws provider
//!
//! This module handles all workspaces resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Workspaces service handler
pub struct WorkspacesService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> WorkspacesService<'a> {
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
            "workspaces_pools" => {
                self.plan_workspaces_pools(current_state, desired_input).await
            }
            "workspace_bundle" => {
                self.plan_workspace_bundle(current_state, desired_input).await
            }
            "workspace_directories" => {
                self.plan_workspace_directories(current_state, desired_input).await
            }
            "standby_workspaces" => {
                self.plan_standby_workspaces(current_state, desired_input).await
            }
            "updated_workspace_image" => {
                self.plan_updated_workspace_image(current_state, desired_input).await
            }
            "account" => {
                self.plan_account(current_state, desired_input).await
            }
            "connection_aliases" => {
                self.plan_connection_aliases(current_state, desired_input).await
            }
            "ip_group" => {
                self.plan_ip_group(current_state, desired_input).await
            }
            "workspace_image" => {
                self.plan_workspace_image(current_state, desired_input).await
            }
            "connect_client_add_in" => {
                self.plan_connect_client_add_in(current_state, desired_input).await
            }
            "connection_alias" => {
                self.plan_connection_alias(current_state, desired_input).await
            }
            "applications" => {
                self.plan_applications(current_state, desired_input).await
            }
            "connection_alias_permissions" => {
                self.plan_connection_alias_permissions(current_state, desired_input).await
            }
            "workspace_image_permission" => {
                self.plan_workspace_image_permission(current_state, desired_input).await
            }
            "workspace_images" => {
                self.plan_workspace_images(current_state, desired_input).await
            }
            "connect_client_add_ins" => {
                self.plan_connect_client_add_ins(current_state, desired_input).await
            }
            "workspace_bundles" => {
                self.plan_workspace_bundles(current_state, desired_input).await
            }
            "workspaces" => {
                self.plan_workspaces(current_state, desired_input).await
            }
            "rules_of_ip_group" => {
                self.plan_rules_of_ip_group(current_state, desired_input).await
            }
            "connection_alias_permission" => {
                self.plan_connection_alias_permission(current_state, desired_input).await
            }
            "application_associations" => {
                self.plan_application_associations(current_state, desired_input).await
            }
            "workspace_associations" => {
                self.plan_workspace_associations(current_state, desired_input).await
            }
            "workspaces_pool_sessions" => {
                self.plan_workspaces_pool_sessions(current_state, desired_input).await
            }
            "client_branding" => {
                self.plan_client_branding(current_state, desired_input).await
            }
            "image_associations" => {
                self.plan_image_associations(current_state, desired_input).await
            }
            "workspaces_connection_status" => {
                self.plan_workspaces_connection_status(current_state, desired_input).await
            }
            "account_link_invitation" => {
                self.plan_account_link_invitation(current_state, desired_input).await
            }
            "workspace_snapshots" => {
                self.plan_workspace_snapshots(current_state, desired_input).await
            }
            "ip_groups" => {
                self.plan_ip_groups(current_state, desired_input).await
            }
            "workspaces_pool" => {
                self.plan_workspaces_pool(current_state, desired_input).await
            }
            "account_modifications" => {
                self.plan_account_modifications(current_state, desired_input).await
            }
            "bundle_associations" => {
                self.plan_bundle_associations(current_state, desired_input).await
            }
            "custom_workspace_image_import" => {
                self.plan_custom_workspace_image_import(current_state, desired_input).await
            }
            "workspace_image_permissions" => {
                self.plan_workspace_image_permissions(current_state, desired_input).await
            }
            "tags" => {
                self.plan_tags(current_state, desired_input).await
            }
            "client_properties" => {
                self.plan_client_properties(current_state, desired_input).await
            }
            "account_link" => {
                self.plan_account_link(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "workspaces",
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
            "workspaces_pools" => {
                self.create_workspaces_pools(input).await
            }
            "workspace_bundle" => {
                self.create_workspace_bundle(input).await
            }
            "workspace_directories" => {
                self.create_workspace_directories(input).await
            }
            "standby_workspaces" => {
                self.create_standby_workspaces(input).await
            }
            "updated_workspace_image" => {
                self.create_updated_workspace_image(input).await
            }
            "account" => {
                self.create_account(input).await
            }
            "connection_aliases" => {
                self.create_connection_aliases(input).await
            }
            "ip_group" => {
                self.create_ip_group(input).await
            }
            "workspace_image" => {
                self.create_workspace_image(input).await
            }
            "connect_client_add_in" => {
                self.create_connect_client_add_in(input).await
            }
            "connection_alias" => {
                self.create_connection_alias(input).await
            }
            "applications" => {
                self.create_applications(input).await
            }
            "connection_alias_permissions" => {
                self.create_connection_alias_permissions(input).await
            }
            "workspace_image_permission" => {
                self.create_workspace_image_permission(input).await
            }
            "workspace_images" => {
                self.create_workspace_images(input).await
            }
            "connect_client_add_ins" => {
                self.create_connect_client_add_ins(input).await
            }
            "workspace_bundles" => {
                self.create_workspace_bundles(input).await
            }
            "workspaces" => {
                self.create_workspaces(input).await
            }
            "rules_of_ip_group" => {
                self.create_rules_of_ip_group(input).await
            }
            "connection_alias_permission" => {
                self.create_connection_alias_permission(input).await
            }
            "application_associations" => {
                self.create_application_associations(input).await
            }
            "workspace_associations" => {
                self.create_workspace_associations(input).await
            }
            "workspaces_pool_sessions" => {
                self.create_workspaces_pool_sessions(input).await
            }
            "client_branding" => {
                self.create_client_branding(input).await
            }
            "image_associations" => {
                self.create_image_associations(input).await
            }
            "workspaces_connection_status" => {
                self.create_workspaces_connection_status(input).await
            }
            "account_link_invitation" => {
                self.create_account_link_invitation(input).await
            }
            "workspace_snapshots" => {
                self.create_workspace_snapshots(input).await
            }
            "ip_groups" => {
                self.create_ip_groups(input).await
            }
            "workspaces_pool" => {
                self.create_workspaces_pool(input).await
            }
            "account_modifications" => {
                self.create_account_modifications(input).await
            }
            "bundle_associations" => {
                self.create_bundle_associations(input).await
            }
            "custom_workspace_image_import" => {
                self.create_custom_workspace_image_import(input).await
            }
            "workspace_image_permissions" => {
                self.create_workspace_image_permissions(input).await
            }
            "tags" => {
                self.create_tags(input).await
            }
            "client_properties" => {
                self.create_client_properties(input).await
            }
            "account_link" => {
                self.create_account_link(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "workspaces",
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
            "workspaces_pools" => {
                self.read_workspaces_pools(id).await
            }
            "workspace_bundle" => {
                self.read_workspace_bundle(id).await
            }
            "workspace_directories" => {
                self.read_workspace_directories(id).await
            }
            "standby_workspaces" => {
                self.read_standby_workspaces(id).await
            }
            "updated_workspace_image" => {
                self.read_updated_workspace_image(id).await
            }
            "account" => {
                self.read_account(id).await
            }
            "connection_aliases" => {
                self.read_connection_aliases(id).await
            }
            "ip_group" => {
                self.read_ip_group(id).await
            }
            "workspace_image" => {
                self.read_workspace_image(id).await
            }
            "connect_client_add_in" => {
                self.read_connect_client_add_in(id).await
            }
            "connection_alias" => {
                self.read_connection_alias(id).await
            }
            "applications" => {
                self.read_applications(id).await
            }
            "connection_alias_permissions" => {
                self.read_connection_alias_permissions(id).await
            }
            "workspace_image_permission" => {
                self.read_workspace_image_permission(id).await
            }
            "workspace_images" => {
                self.read_workspace_images(id).await
            }
            "connect_client_add_ins" => {
                self.read_connect_client_add_ins(id).await
            }
            "workspace_bundles" => {
                self.read_workspace_bundles(id).await
            }
            "workspaces" => {
                self.read_workspaces(id).await
            }
            "rules_of_ip_group" => {
                self.read_rules_of_ip_group(id).await
            }
            "connection_alias_permission" => {
                self.read_connection_alias_permission(id).await
            }
            "application_associations" => {
                self.read_application_associations(id).await
            }
            "workspace_associations" => {
                self.read_workspace_associations(id).await
            }
            "workspaces_pool_sessions" => {
                self.read_workspaces_pool_sessions(id).await
            }
            "client_branding" => {
                self.read_client_branding(id).await
            }
            "image_associations" => {
                self.read_image_associations(id).await
            }
            "workspaces_connection_status" => {
                self.read_workspaces_connection_status(id).await
            }
            "account_link_invitation" => {
                self.read_account_link_invitation(id).await
            }
            "workspace_snapshots" => {
                self.read_workspace_snapshots(id).await
            }
            "ip_groups" => {
                self.read_ip_groups(id).await
            }
            "workspaces_pool" => {
                self.read_workspaces_pool(id).await
            }
            "account_modifications" => {
                self.read_account_modifications(id).await
            }
            "bundle_associations" => {
                self.read_bundle_associations(id).await
            }
            "custom_workspace_image_import" => {
                self.read_custom_workspace_image_import(id).await
            }
            "workspace_image_permissions" => {
                self.read_workspace_image_permissions(id).await
            }
            "tags" => {
                self.read_tags(id).await
            }
            "client_properties" => {
                self.read_client_properties(id).await
            }
            "account_link" => {
                self.read_account_link(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "workspaces",
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
            "workspaces_pools" => {
                self.update_workspaces_pools(id, input).await
            }
            "workspace_bundle" => {
                self.update_workspace_bundle(id, input).await
            }
            "workspace_directories" => {
                self.update_workspace_directories(id, input).await
            }
            "standby_workspaces" => {
                self.update_standby_workspaces(id, input).await
            }
            "updated_workspace_image" => {
                self.update_updated_workspace_image(id, input).await
            }
            "account" => {
                self.update_account(id, input).await
            }
            "connection_aliases" => {
                self.update_connection_aliases(id, input).await
            }
            "ip_group" => {
                self.update_ip_group(id, input).await
            }
            "workspace_image" => {
                self.update_workspace_image(id, input).await
            }
            "connect_client_add_in" => {
                self.update_connect_client_add_in(id, input).await
            }
            "connection_alias" => {
                self.update_connection_alias(id, input).await
            }
            "applications" => {
                self.update_applications(id, input).await
            }
            "connection_alias_permissions" => {
                self.update_connection_alias_permissions(id, input).await
            }
            "workspace_image_permission" => {
                self.update_workspace_image_permission(id, input).await
            }
            "workspace_images" => {
                self.update_workspace_images(id, input).await
            }
            "connect_client_add_ins" => {
                self.update_connect_client_add_ins(id, input).await
            }
            "workspace_bundles" => {
                self.update_workspace_bundles(id, input).await
            }
            "workspaces" => {
                self.update_workspaces(id, input).await
            }
            "rules_of_ip_group" => {
                self.update_rules_of_ip_group(id, input).await
            }
            "connection_alias_permission" => {
                self.update_connection_alias_permission(id, input).await
            }
            "application_associations" => {
                self.update_application_associations(id, input).await
            }
            "workspace_associations" => {
                self.update_workspace_associations(id, input).await
            }
            "workspaces_pool_sessions" => {
                self.update_workspaces_pool_sessions(id, input).await
            }
            "client_branding" => {
                self.update_client_branding(id, input).await
            }
            "image_associations" => {
                self.update_image_associations(id, input).await
            }
            "workspaces_connection_status" => {
                self.update_workspaces_connection_status(id, input).await
            }
            "account_link_invitation" => {
                self.update_account_link_invitation(id, input).await
            }
            "workspace_snapshots" => {
                self.update_workspace_snapshots(id, input).await
            }
            "ip_groups" => {
                self.update_ip_groups(id, input).await
            }
            "workspaces_pool" => {
                self.update_workspaces_pool(id, input).await
            }
            "account_modifications" => {
                self.update_account_modifications(id, input).await
            }
            "bundle_associations" => {
                self.update_bundle_associations(id, input).await
            }
            "custom_workspace_image_import" => {
                self.update_custom_workspace_image_import(id, input).await
            }
            "workspace_image_permissions" => {
                self.update_workspace_image_permissions(id, input).await
            }
            "tags" => {
                self.update_tags(id, input).await
            }
            "client_properties" => {
                self.update_client_properties(id, input).await
            }
            "account_link" => {
                self.update_account_link(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "workspaces",
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
            "workspaces_pools" => {
                self.delete_workspaces_pools(id).await
            }
            "workspace_bundle" => {
                self.delete_workspace_bundle(id).await
            }
            "workspace_directories" => {
                self.delete_workspace_directories(id).await
            }
            "standby_workspaces" => {
                self.delete_standby_workspaces(id).await
            }
            "updated_workspace_image" => {
                self.delete_updated_workspace_image(id).await
            }
            "account" => {
                self.delete_account(id).await
            }
            "connection_aliases" => {
                self.delete_connection_aliases(id).await
            }
            "ip_group" => {
                self.delete_ip_group(id).await
            }
            "workspace_image" => {
                self.delete_workspace_image(id).await
            }
            "connect_client_add_in" => {
                self.delete_connect_client_add_in(id).await
            }
            "connection_alias" => {
                self.delete_connection_alias(id).await
            }
            "applications" => {
                self.delete_applications(id).await
            }
            "connection_alias_permissions" => {
                self.delete_connection_alias_permissions(id).await
            }
            "workspace_image_permission" => {
                self.delete_workspace_image_permission(id).await
            }
            "workspace_images" => {
                self.delete_workspace_images(id).await
            }
            "connect_client_add_ins" => {
                self.delete_connect_client_add_ins(id).await
            }
            "workspace_bundles" => {
                self.delete_workspace_bundles(id).await
            }
            "workspaces" => {
                self.delete_workspaces(id).await
            }
            "rules_of_ip_group" => {
                self.delete_rules_of_ip_group(id).await
            }
            "connection_alias_permission" => {
                self.delete_connection_alias_permission(id).await
            }
            "application_associations" => {
                self.delete_application_associations(id).await
            }
            "workspace_associations" => {
                self.delete_workspace_associations(id).await
            }
            "workspaces_pool_sessions" => {
                self.delete_workspaces_pool_sessions(id).await
            }
            "client_branding" => {
                self.delete_client_branding(id).await
            }
            "image_associations" => {
                self.delete_image_associations(id).await
            }
            "workspaces_connection_status" => {
                self.delete_workspaces_connection_status(id).await
            }
            "account_link_invitation" => {
                self.delete_account_link_invitation(id).await
            }
            "workspace_snapshots" => {
                self.delete_workspace_snapshots(id).await
            }
            "ip_groups" => {
                self.delete_ip_groups(id).await
            }
            "workspaces_pool" => {
                self.delete_workspaces_pool(id).await
            }
            "account_modifications" => {
                self.delete_account_modifications(id).await
            }
            "bundle_associations" => {
                self.delete_bundle_associations(id).await
            }
            "custom_workspace_image_import" => {
                self.delete_custom_workspace_image_import(id).await
            }
            "workspace_image_permissions" => {
                self.delete_workspace_image_permissions(id).await
            }
            "tags" => {
                self.delete_tags(id).await
            }
            "client_properties" => {
                self.delete_client_properties(id).await
            }
            "account_link" => {
                self.delete_account_link(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "workspaces",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Workspaces_pools resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspaces_pools resource
    async fn plan_workspaces_pools(
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

    /// Create a new workspaces_pools resource
    async fn create_workspaces_pools(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspaces_pools()
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

    /// Read a workspaces_pools resource
    async fn read_workspaces_pools(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspaces_pools()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspaces_pools resource
    async fn update_workspaces_pools(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspaces_pools()
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

    /// Delete a workspaces_pools resource
    async fn delete_workspaces_pools(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspaces_pools()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspace_bundle resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspace_bundle resource
    async fn plan_workspace_bundle(
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

    /// Create a new workspace_bundle resource
    async fn create_workspace_bundle(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let image_id = input.get_string("image_id")?;
            let bundle_description = input.get_string("bundle_description")?;
            let compute_type = input.get_string("compute_type")?;
            let user_storage = input.get_string("user_storage")?;
            let root_storage = input.get_optional_string("root_storage")?;
            let tags = input.get_optional_string("tags")?;
            let bundle_name = input.get_string("bundle_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspace_bundle()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("image_id", image_id.unwrap_or_default())
                .with_field("bundle_description", bundle_description.unwrap_or_default())
                .with_field("compute_type", compute_type.unwrap_or_default())
                .with_field("user_storage", user_storage.unwrap_or_default())
                .with_field("root_storage", root_storage.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("bundle_name", bundle_name.unwrap_or_default())
            )
        })
    }

    /// Read a workspace_bundle resource
    async fn read_workspace_bundle(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspace_bundle()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspace_bundle resource
    async fn update_workspace_bundle(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let image_id = input.get_string("image_id")?;
            let bundle_description = input.get_string("bundle_description")?;
            let compute_type = input.get_string("compute_type")?;
            let user_storage = input.get_string("user_storage")?;
            let root_storage = input.get_optional_string("root_storage")?;
            let tags = input.get_optional_string("tags")?;
            let bundle_name = input.get_string("bundle_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspace_bundle()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("image_id", image_id.unwrap_or_default())
                .with_field("bundle_description", bundle_description.unwrap_or_default())
                .with_field("compute_type", compute_type.unwrap_or_default())
                .with_field("user_storage", user_storage.unwrap_or_default())
                .with_field("root_storage", root_storage.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("bundle_name", bundle_name.unwrap_or_default())
            )
        })
    }

    /// Delete a workspace_bundle resource
    async fn delete_workspace_bundle(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspace_bundle()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspace_directories resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspace_directories resource
    async fn plan_workspace_directories(
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

    /// Create a new workspace_directories resource
    async fn create_workspace_directories(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspace_directories()
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

    /// Read a workspace_directories resource
    async fn read_workspace_directories(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspace_directories()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspace_directories resource
    async fn update_workspace_directories(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspace_directories()
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

    /// Delete a workspace_directories resource
    async fn delete_workspace_directories(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspace_directories()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Standby_workspaces resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a standby_workspaces resource
    async fn plan_standby_workspaces(
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

    /// Create a new standby_workspaces resource
    async fn create_standby_workspaces(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let primary_region = input.get_string("primary_region")?;
            let standby_workspaces = input.get_string("standby_workspaces")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_standby_workspaces()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("primary_region", primary_region.unwrap_or_default())
                .with_field("standby_workspaces", standby_workspaces.unwrap_or_default())
            )
        })
    }

    /// Read a standby_workspaces resource
    async fn read_standby_workspaces(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_standby_workspaces()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a standby_workspaces resource
    async fn update_standby_workspaces(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let primary_region = input.get_string("primary_region")?;
            let standby_workspaces = input.get_string("standby_workspaces")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_standby_workspaces()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("primary_region", primary_region.unwrap_or_default())
                .with_field("standby_workspaces", standby_workspaces.unwrap_or_default())
            )
        })
    }

    /// Delete a standby_workspaces resource
    async fn delete_standby_workspaces(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_standby_workspaces()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Updated_workspace_image resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a updated_workspace_image resource
    async fn plan_updated_workspace_image(
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

    /// Create a new updated_workspace_image resource
    async fn create_updated_workspace_image(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;
            let description = input.get_string("description")?;
            let source_image_id = input.get_string("source_image_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_updated_workspace_image()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("source_image_id", source_image_id.unwrap_or_default())
            )
        })
    }

    /// Read a updated_workspace_image resource
    async fn read_updated_workspace_image(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_updated_workspace_image()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a updated_workspace_image resource
    async fn update_updated_workspace_image(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;
            let description = input.get_string("description")?;
            let source_image_id = input.get_string("source_image_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_updated_workspace_image()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("source_image_id", source_image_id.unwrap_or_default())
            )
        })
    }

    /// Delete a updated_workspace_image resource
    async fn delete_updated_workspace_image(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_updated_workspace_image()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Account resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a account resource
    async fn plan_account(
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

    /// Create a new account resource
    async fn create_account(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_account()
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

    /// Read a account resource
    async fn read_account(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_account()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a account resource
    async fn update_account(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_account()
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

    /// Delete a account resource
    async fn delete_account(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_account()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connection_aliases resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connection_aliases resource
    async fn plan_connection_aliases(
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

    /// Create a new connection_aliases resource
    async fn create_connection_aliases(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_connection_aliases()
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

    /// Read a connection_aliases resource
    async fn read_connection_aliases(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_connection_aliases()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connection_aliases resource
    async fn update_connection_aliases(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_connection_aliases()
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

    /// Delete a connection_aliases resource
    async fn delete_connection_aliases(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_connection_aliases()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Ip_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ip_group resource
    async fn plan_ip_group(
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

    /// Create a new ip_group resource
    async fn create_ip_group(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let group_desc = input.get_optional_string("group_desc")?;
            let group_name = input.get_string("group_name")?;
            let tags = input.get_optional_string("tags")?;
            let user_rules = input.get_optional_string("user_rules")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_ip_group()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("group_desc", group_desc.unwrap_or_default())
                .with_field("group_name", group_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("user_rules", user_rules.unwrap_or_default())
            )
        })
    }

    /// Read a ip_group resource
    async fn read_ip_group(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_ip_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a ip_group resource
    async fn update_ip_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let group_desc = input.get_optional_string("group_desc")?;
            let group_name = input.get_string("group_name")?;
            let tags = input.get_optional_string("tags")?;
            let user_rules = input.get_optional_string("user_rules")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_ip_group()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("group_desc", group_desc.unwrap_or_default())
                .with_field("group_name", group_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("user_rules", user_rules.unwrap_or_default())
            )
        })
    }

    /// Delete a ip_group resource
    async fn delete_ip_group(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_ip_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspace_image resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspace_image resource
    async fn plan_workspace_image(
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

    /// Create a new workspace_image resource
    async fn create_workspace_image(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let tags = input.get_optional_string("tags")?;
            let description = input.get_string("description")?;
            let workspace_id = input.get_string("workspace_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspace_image()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("workspace_id", workspace_id.unwrap_or_default())
            )
        })
    }

    /// Read a workspace_image resource
    async fn read_workspace_image(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspace_image()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspace_image resource
    async fn update_workspace_image(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let tags = input.get_optional_string("tags")?;
            let description = input.get_string("description")?;
            let workspace_id = input.get_string("workspace_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspace_image()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("workspace_id", workspace_id.unwrap_or_default())
            )
        })
    }

    /// Delete a workspace_image resource
    async fn delete_workspace_image(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspace_image()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connect_client_add_in resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connect_client_add_in resource
    async fn plan_connect_client_add_in(
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

    /// Create a new connect_client_add_in resource
    async fn create_connect_client_add_in(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_id = input.get_string("resource_id")?;
            let name = input.get_string("name")?;
            let url = input.get_string("url")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_connect_client_add_in()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("resource_id", resource_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("url", url.unwrap_or_default())
            )
        })
    }

    /// Read a connect_client_add_in resource
    async fn read_connect_client_add_in(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_connect_client_add_in()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connect_client_add_in resource
    async fn update_connect_client_add_in(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_id = input.get_string("resource_id")?;
            let name = input.get_string("name")?;
            let url = input.get_string("url")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_connect_client_add_in()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("resource_id", resource_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("url", url.unwrap_or_default())
            )
        })
    }

    /// Delete a connect_client_add_in resource
    async fn delete_connect_client_add_in(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_connect_client_add_in()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connection_alias resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connection_alias resource
    async fn plan_connection_alias(
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

    /// Create a new connection_alias resource
    async fn create_connection_alias(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let connection_string = input.get_string("connection_string")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_connection_alias()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("connection_string", connection_string.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Read a connection_alias resource
    async fn read_connection_alias(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_connection_alias()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connection_alias resource
    async fn update_connection_alias(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let connection_string = input.get_string("connection_string")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_connection_alias()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("connection_string", connection_string.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Delete a connection_alias resource
    async fn delete_connection_alias(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_connection_alias()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Applications resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a applications resource
    async fn plan_applications(
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

    /// Create a new applications resource
    async fn create_applications(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_applications()
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

    /// Read a applications resource
    async fn read_applications(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_applications()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a applications resource
    async fn update_applications(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_applications()
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

    /// Delete a applications resource
    async fn delete_applications(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_applications()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connection_alias_permissions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connection_alias_permissions resource
    async fn plan_connection_alias_permissions(
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

    /// Create a new connection_alias_permissions resource
    async fn create_connection_alias_permissions(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_connection_alias_permissions()
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

    /// Read a connection_alias_permissions resource
    async fn read_connection_alias_permissions(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_connection_alias_permissions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connection_alias_permissions resource
    async fn update_connection_alias_permissions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_connection_alias_permissions()
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

    /// Delete a connection_alias_permissions resource
    async fn delete_connection_alias_permissions(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_connection_alias_permissions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspace_image_permission resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspace_image_permission resource
    async fn plan_workspace_image_permission(
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

    /// Create a new workspace_image_permission resource
    async fn create_workspace_image_permission(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let shared_account_id = input.get_string("shared_account_id")?;
            let image_id = input.get_string("image_id")?;
            let allow_copy_image = input.get_string("allow_copy_image")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspace_image_permission()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("shared_account_id", shared_account_id.unwrap_or_default())
                .with_field("image_id", image_id.unwrap_or_default())
                .with_field("allow_copy_image", allow_copy_image.unwrap_or_default())
            )
        })
    }

    /// Read a workspace_image_permission resource
    async fn read_workspace_image_permission(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspace_image_permission()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspace_image_permission resource
    async fn update_workspace_image_permission(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let shared_account_id = input.get_string("shared_account_id")?;
            let image_id = input.get_string("image_id")?;
            let allow_copy_image = input.get_string("allow_copy_image")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspace_image_permission()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("shared_account_id", shared_account_id.unwrap_or_default())
                .with_field("image_id", image_id.unwrap_or_default())
                .with_field("allow_copy_image", allow_copy_image.unwrap_or_default())
            )
        })
    }

    /// Delete a workspace_image_permission resource
    async fn delete_workspace_image_permission(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspace_image_permission()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspace_images resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspace_images resource
    async fn plan_workspace_images(
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

    /// Create a new workspace_images resource
    async fn create_workspace_images(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspace_images()
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

    /// Read a workspace_images resource
    async fn read_workspace_images(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspace_images()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspace_images resource
    async fn update_workspace_images(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspace_images()
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

    /// Delete a workspace_images resource
    async fn delete_workspace_images(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspace_images()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connect_client_add_ins resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connect_client_add_ins resource
    async fn plan_connect_client_add_ins(
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

    /// Create a new connect_client_add_ins resource
    async fn create_connect_client_add_ins(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_connect_client_add_ins()
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

    /// Read a connect_client_add_ins resource
    async fn read_connect_client_add_ins(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_connect_client_add_ins()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connect_client_add_ins resource
    async fn update_connect_client_add_ins(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_connect_client_add_ins()
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

    /// Delete a connect_client_add_ins resource
    async fn delete_connect_client_add_ins(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_connect_client_add_ins()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspace_bundles resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspace_bundles resource
    async fn plan_workspace_bundles(
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

    /// Create a new workspace_bundles resource
    async fn create_workspace_bundles(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspace_bundles()
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

    /// Read a workspace_bundles resource
    async fn read_workspace_bundles(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspace_bundles()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspace_bundles resource
    async fn update_workspace_bundles(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspace_bundles()
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

    /// Delete a workspace_bundles resource
    async fn delete_workspace_bundles(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspace_bundles()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspaces resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspaces resource
    async fn plan_workspaces(
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

    /// Create a new workspaces resource
    async fn create_workspaces(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let workspaces = input.get_string("workspaces")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspaces()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("workspaces", workspaces.unwrap_or_default())
            )
        })
    }

    /// Read a workspaces resource
    async fn read_workspaces(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspaces()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspaces resource
    async fn update_workspaces(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let workspaces = input.get_string("workspaces")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspaces()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("workspaces", workspaces.unwrap_or_default())
            )
        })
    }

    /// Delete a workspaces resource
    async fn delete_workspaces(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspaces()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Rules_of_ip_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rules_of_ip_group resource
    async fn plan_rules_of_ip_group(
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

    /// Create a new rules_of_ip_group resource
    async fn create_rules_of_ip_group(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let group_id = input.get_string("group_id")?;
            let user_rules = input.get_string("user_rules")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_rules_of_ip_group()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("group_id", group_id.unwrap_or_default())
                .with_field("user_rules", user_rules.unwrap_or_default())
            )
        })
    }

    /// Read a rules_of_ip_group resource
    async fn read_rules_of_ip_group(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_rules_of_ip_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a rules_of_ip_group resource
    async fn update_rules_of_ip_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let group_id = input.get_string("group_id")?;
            let user_rules = input.get_string("user_rules")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_rules_of_ip_group()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("group_id", group_id.unwrap_or_default())
                .with_field("user_rules", user_rules.unwrap_or_default())
            )
        })
    }

    /// Delete a rules_of_ip_group resource
    async fn delete_rules_of_ip_group(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_rules_of_ip_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connection_alias_permission resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connection_alias_permission resource
    async fn plan_connection_alias_permission(
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

    /// Create a new connection_alias_permission resource
    async fn create_connection_alias_permission(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let alias_id = input.get_string("alias_id")?;
            let connection_alias_permission = input.get_string("connection_alias_permission")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_connection_alias_permission()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("alias_id", alias_id.unwrap_or_default())
                .with_field("connection_alias_permission", connection_alias_permission.unwrap_or_default())
            )
        })
    }

    /// Read a connection_alias_permission resource
    async fn read_connection_alias_permission(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_connection_alias_permission()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connection_alias_permission resource
    async fn update_connection_alias_permission(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let alias_id = input.get_string("alias_id")?;
            let connection_alias_permission = input.get_string("connection_alias_permission")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_connection_alias_permission()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("alias_id", alias_id.unwrap_or_default())
                .with_field("connection_alias_permission", connection_alias_permission.unwrap_or_default())
            )
        })
    }

    /// Delete a connection_alias_permission resource
    async fn delete_connection_alias_permission(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_connection_alias_permission()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Application_associations resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a application_associations resource
    async fn plan_application_associations(
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

    /// Create a new application_associations resource
    async fn create_application_associations(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_application_associations()
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

    /// Read a application_associations resource
    async fn read_application_associations(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_application_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a application_associations resource
    async fn update_application_associations(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_application_associations()
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

    /// Delete a application_associations resource
    async fn delete_application_associations(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_application_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspace_associations resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspace_associations resource
    async fn plan_workspace_associations(
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

    /// Create a new workspace_associations resource
    async fn create_workspace_associations(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspace_associations()
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

    /// Read a workspace_associations resource
    async fn read_workspace_associations(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspace_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspace_associations resource
    async fn update_workspace_associations(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspace_associations()
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

    /// Delete a workspace_associations resource
    async fn delete_workspace_associations(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspace_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspaces_pool_sessions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspaces_pool_sessions resource
    async fn plan_workspaces_pool_sessions(
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

    /// Create a new workspaces_pool_sessions resource
    async fn create_workspaces_pool_sessions(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspaces_pool_sessions()
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

    /// Read a workspaces_pool_sessions resource
    async fn read_workspaces_pool_sessions(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspaces_pool_sessions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspaces_pool_sessions resource
    async fn update_workspaces_pool_sessions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspaces_pool_sessions()
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

    /// Delete a workspaces_pool_sessions resource
    async fn delete_workspaces_pool_sessions(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspaces_pool_sessions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Client_branding resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a client_branding resource
    async fn plan_client_branding(
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

    /// Create a new client_branding resource
    async fn create_client_branding(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_client_branding()
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

    /// Read a client_branding resource
    async fn read_client_branding(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_client_branding()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a client_branding resource
    async fn update_client_branding(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_client_branding()
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

    /// Delete a client_branding resource
    async fn delete_client_branding(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_client_branding()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Image_associations resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a image_associations resource
    async fn plan_image_associations(
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

    /// Create a new image_associations resource
    async fn create_image_associations(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_image_associations()
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

    /// Read a image_associations resource
    async fn read_image_associations(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_image_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a image_associations resource
    async fn update_image_associations(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_image_associations()
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

    /// Delete a image_associations resource
    async fn delete_image_associations(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_image_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspaces_connection_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspaces_connection_status resource
    async fn plan_workspaces_connection_status(
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

    /// Create a new workspaces_connection_status resource
    async fn create_workspaces_connection_status(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspaces_connection_status()
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

    /// Read a workspaces_connection_status resource
    async fn read_workspaces_connection_status(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspaces_connection_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspaces_connection_status resource
    async fn update_workspaces_connection_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspaces_connection_status()
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

    /// Delete a workspaces_connection_status resource
    async fn delete_workspaces_connection_status(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspaces_connection_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Account_link_invitation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a account_link_invitation resource
    async fn plan_account_link_invitation(
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

    /// Create a new account_link_invitation resource
    async fn create_account_link_invitation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let target_account_id = input.get_string("target_account_id")?;
            let client_token = input.get_optional_string("client_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_account_link_invitation()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("target_account_id", target_account_id.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
            )
        })
    }

    /// Read a account_link_invitation resource
    async fn read_account_link_invitation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_account_link_invitation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a account_link_invitation resource
    async fn update_account_link_invitation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let target_account_id = input.get_string("target_account_id")?;
            let client_token = input.get_optional_string("client_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_account_link_invitation()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("target_account_id", target_account_id.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
            )
        })
    }

    /// Delete a account_link_invitation resource
    async fn delete_account_link_invitation(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_account_link_invitation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspace_snapshots resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspace_snapshots resource
    async fn plan_workspace_snapshots(
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

    /// Create a new workspace_snapshots resource
    async fn create_workspace_snapshots(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspace_snapshots()
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

    /// Read a workspace_snapshots resource
    async fn read_workspace_snapshots(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspace_snapshots()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspace_snapshots resource
    async fn update_workspace_snapshots(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspace_snapshots()
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

    /// Delete a workspace_snapshots resource
    async fn delete_workspace_snapshots(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspace_snapshots()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Ip_groups resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ip_groups resource
    async fn plan_ip_groups(
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

    /// Create a new ip_groups resource
    async fn create_ip_groups(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_ip_groups()
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

    /// Read a ip_groups resource
    async fn read_ip_groups(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_ip_groups()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a ip_groups resource
    async fn update_ip_groups(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_ip_groups()
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

    /// Delete a ip_groups resource
    async fn delete_ip_groups(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_ip_groups()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspaces_pool resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspaces_pool resource
    async fn plan_workspaces_pool(
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

    /// Create a new workspaces_pool resource
    async fn create_workspaces_pool(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let pool_name = input.get_string("pool_name")?;
            let tags = input.get_optional_string("tags")?;
            let bundle_id = input.get_string("bundle_id")?;
            let application_settings = input.get_optional_string("application_settings")?;
            let directory_id = input.get_string("directory_id")?;
            let timeout_settings = input.get_optional_string("timeout_settings")?;
            let capacity = input.get_string("capacity")?;
            let description = input.get_string("description")?;
            let running_mode = input.get_optional_string("running_mode")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspaces_pool()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("pool_name", pool_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("bundle_id", bundle_id.unwrap_or_default())
                .with_field("application_settings", application_settings.unwrap_or_default())
                .with_field("directory_id", directory_id.unwrap_or_default())
                .with_field("timeout_settings", timeout_settings.unwrap_or_default())
                .with_field("capacity", capacity.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("running_mode", running_mode.unwrap_or_default())
            )
        })
    }

    /// Read a workspaces_pool resource
    async fn read_workspaces_pool(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspaces_pool()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspaces_pool resource
    async fn update_workspaces_pool(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let pool_name = input.get_string("pool_name")?;
            let tags = input.get_optional_string("tags")?;
            let bundle_id = input.get_string("bundle_id")?;
            let application_settings = input.get_optional_string("application_settings")?;
            let directory_id = input.get_string("directory_id")?;
            let timeout_settings = input.get_optional_string("timeout_settings")?;
            let capacity = input.get_string("capacity")?;
            let description = input.get_string("description")?;
            let running_mode = input.get_optional_string("running_mode")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspaces_pool()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("pool_name", pool_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("bundle_id", bundle_id.unwrap_or_default())
                .with_field("application_settings", application_settings.unwrap_or_default())
                .with_field("directory_id", directory_id.unwrap_or_default())
                .with_field("timeout_settings", timeout_settings.unwrap_or_default())
                .with_field("capacity", capacity.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("running_mode", running_mode.unwrap_or_default())
            )
        })
    }

    /// Delete a workspaces_pool resource
    async fn delete_workspaces_pool(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspaces_pool()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Account_modifications resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a account_modifications resource
    async fn plan_account_modifications(
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

    /// Create a new account_modifications resource
    async fn create_account_modifications(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_account_modifications()
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

    /// Read a account_modifications resource
    async fn read_account_modifications(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_account_modifications()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a account_modifications resource
    async fn update_account_modifications(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_account_modifications()
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

    /// Delete a account_modifications resource
    async fn delete_account_modifications(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_account_modifications()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Bundle_associations resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a bundle_associations resource
    async fn plan_bundle_associations(
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

    /// Create a new bundle_associations resource
    async fn create_bundle_associations(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_bundle_associations()
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

    /// Read a bundle_associations resource
    async fn read_bundle_associations(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_bundle_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a bundle_associations resource
    async fn update_bundle_associations(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_bundle_associations()
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

    /// Delete a bundle_associations resource
    async fn delete_bundle_associations(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_bundle_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Custom_workspace_image_import resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_workspace_image_import resource
    async fn plan_custom_workspace_image_import(
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

    /// Create a new custom_workspace_image_import resource
    async fn create_custom_workspace_image_import(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_custom_workspace_image_import()
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

    /// Read a custom_workspace_image_import resource
    async fn read_custom_workspace_image_import(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_custom_workspace_image_import()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a custom_workspace_image_import resource
    async fn update_custom_workspace_image_import(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_custom_workspace_image_import()
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

    /// Delete a custom_workspace_image_import resource
    async fn delete_custom_workspace_image_import(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_custom_workspace_image_import()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Workspace_image_permissions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a workspace_image_permissions resource
    async fn plan_workspace_image_permissions(
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

    /// Create a new workspace_image_permissions resource
    async fn create_workspace_image_permissions(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_workspace_image_permissions()
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

    /// Read a workspace_image_permissions resource
    async fn read_workspace_image_permissions(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_workspace_image_permissions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a workspace_image_permissions resource
    async fn update_workspace_image_permissions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_workspace_image_permissions()
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

    /// Delete a workspace_image_permissions resource
    async fn delete_workspace_image_permissions(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_workspace_image_permissions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Tags resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tags resource
    async fn plan_tags(
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

    /// Create a new tags resource
    async fn create_tags(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_string("tags")?;
            let resource_id = input.get_string("resource_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_tags()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("tags", tags.unwrap_or_default())
                .with_field("resource_id", resource_id.unwrap_or_default())
            )
        })
    }

    /// Read a tags resource
    async fn read_tags(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_tags()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a tags resource
    async fn update_tags(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_string("tags")?;
            let resource_id = input.get_string("resource_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_tags()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("tags", tags.unwrap_or_default())
                .with_field("resource_id", resource_id.unwrap_or_default())
            )
        })
    }

    /// Delete a tags resource
    async fn delete_tags(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_tags()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Client_properties resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a client_properties resource
    async fn plan_client_properties(
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

    /// Create a new client_properties resource
    async fn create_client_properties(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_client_properties()
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

    /// Read a client_properties resource
    async fn read_client_properties(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_client_properties()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a client_properties resource
    async fn update_client_properties(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_client_properties()
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

    /// Delete a client_properties resource
    async fn delete_client_properties(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_client_properties()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Account_link resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a account_link resource
    async fn plan_account_link(
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

    /// Create a new account_link resource
    async fn create_account_link(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .create_account_link()
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

    /// Read a account_link resource
    async fn read_account_link(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .describe_account_link()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a account_link resource
    async fn update_account_link(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.workspaces_client
            //     .update_account_link()
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

    /// Delete a account_link resource
    async fn delete_account_link(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.workspaces_client
            //     .delete_account_link()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
