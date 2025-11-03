//! Sso_admin service for Aws provider
//!
//! This module handles all sso_admin resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Sso_admin service handler
pub struct Sso_adminService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sso_adminService<'a> {
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
            "account_assignment_deletion_status" => {
                self.plan_account_assignment_deletion_status(current_state, desired_input).await
            }
            "application_assignment_configuration" => {
                self.plan_application_assignment_configuration(current_state, desired_input).await
            }
            "inline_policy_from_permission_set" => {
                self.plan_inline_policy_from_permission_set(current_state, desired_input).await
            }
            "permissions_boundary_from_permission_set" => {
                self.plan_permissions_boundary_from_permission_set(current_state, desired_input).await
            }
            "permissions_boundary_for_permission_set" => {
                self.plan_permissions_boundary_for_permission_set(current_state, desired_input).await
            }
            "application" => {
                self.plan_application(current_state, desired_input).await
            }
            "application_assignment" => {
                self.plan_application_assignment(current_state, desired_input).await
            }
            "account_assignment" => {
                self.plan_account_assignment(current_state, desired_input).await
            }
            "account_assignment_creation_status" => {
                self.plan_account_assignment_creation_status(current_state, desired_input).await
            }
            "instance" => {
                self.plan_instance(current_state, desired_input).await
            }
            "permission_set" => {
                self.plan_permission_set(current_state, desired_input).await
            }
            "application_provider" => {
                self.plan_application_provider(current_state, desired_input).await
            }
            "inline_policy_for_permission_set" => {
                self.plan_inline_policy_for_permission_set(current_state, desired_input).await
            }
            "instance_access_control_attribute_configuration" => {
                self.plan_instance_access_control_attribute_configuration(current_state, desired_input).await
            }
            "trusted_token_issuer" => {
                self.plan_trusted_token_issuer(current_state, desired_input).await
            }
            "application_session_configuration" => {
                self.plan_application_session_configuration(current_state, desired_input).await
            }
            "inline_policy_to_permission_set" => {
                self.plan_inline_policy_to_permission_set(current_state, desired_input).await
            }
            "permission_set_provisioning_status" => {
                self.plan_permission_set_provisioning_status(current_state, desired_input).await
            }
            "permissions_boundary_to_permission_set" => {
                self.plan_permissions_boundary_to_permission_set(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "sso_admin",
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
            "account_assignment_deletion_status" => {
                self.create_account_assignment_deletion_status(input).await
            }
            "application_assignment_configuration" => {
                self.create_application_assignment_configuration(input).await
            }
            "inline_policy_from_permission_set" => {
                self.create_inline_policy_from_permission_set(input).await
            }
            "permissions_boundary_from_permission_set" => {
                self.create_permissions_boundary_from_permission_set(input).await
            }
            "permissions_boundary_for_permission_set" => {
                self.create_permissions_boundary_for_permission_set(input).await
            }
            "application" => {
                self.create_application(input).await
            }
            "application_assignment" => {
                self.create_application_assignment(input).await
            }
            "account_assignment" => {
                self.create_account_assignment(input).await
            }
            "account_assignment_creation_status" => {
                self.create_account_assignment_creation_status(input).await
            }
            "instance" => {
                self.create_instance(input).await
            }
            "permission_set" => {
                self.create_permission_set(input).await
            }
            "application_provider" => {
                self.create_application_provider(input).await
            }
            "inline_policy_for_permission_set" => {
                self.create_inline_policy_for_permission_set(input).await
            }
            "instance_access_control_attribute_configuration" => {
                self.create_instance_access_control_attribute_configuration(input).await
            }
            "trusted_token_issuer" => {
                self.create_trusted_token_issuer(input).await
            }
            "application_session_configuration" => {
                self.create_application_session_configuration(input).await
            }
            "inline_policy_to_permission_set" => {
                self.create_inline_policy_to_permission_set(input).await
            }
            "permission_set_provisioning_status" => {
                self.create_permission_set_provisioning_status(input).await
            }
            "permissions_boundary_to_permission_set" => {
                self.create_permissions_boundary_to_permission_set(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "sso_admin",
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
            "account_assignment_deletion_status" => {
                self.read_account_assignment_deletion_status(id).await
            }
            "application_assignment_configuration" => {
                self.read_application_assignment_configuration(id).await
            }
            "inline_policy_from_permission_set" => {
                self.read_inline_policy_from_permission_set(id).await
            }
            "permissions_boundary_from_permission_set" => {
                self.read_permissions_boundary_from_permission_set(id).await
            }
            "permissions_boundary_for_permission_set" => {
                self.read_permissions_boundary_for_permission_set(id).await
            }
            "application" => {
                self.read_application(id).await
            }
            "application_assignment" => {
                self.read_application_assignment(id).await
            }
            "account_assignment" => {
                self.read_account_assignment(id).await
            }
            "account_assignment_creation_status" => {
                self.read_account_assignment_creation_status(id).await
            }
            "instance" => {
                self.read_instance(id).await
            }
            "permission_set" => {
                self.read_permission_set(id).await
            }
            "application_provider" => {
                self.read_application_provider(id).await
            }
            "inline_policy_for_permission_set" => {
                self.read_inline_policy_for_permission_set(id).await
            }
            "instance_access_control_attribute_configuration" => {
                self.read_instance_access_control_attribute_configuration(id).await
            }
            "trusted_token_issuer" => {
                self.read_trusted_token_issuer(id).await
            }
            "application_session_configuration" => {
                self.read_application_session_configuration(id).await
            }
            "inline_policy_to_permission_set" => {
                self.read_inline_policy_to_permission_set(id).await
            }
            "permission_set_provisioning_status" => {
                self.read_permission_set_provisioning_status(id).await
            }
            "permissions_boundary_to_permission_set" => {
                self.read_permissions_boundary_to_permission_set(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "sso_admin",
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
            "account_assignment_deletion_status" => {
                self.update_account_assignment_deletion_status(id, input).await
            }
            "application_assignment_configuration" => {
                self.update_application_assignment_configuration(id, input).await
            }
            "inline_policy_from_permission_set" => {
                self.update_inline_policy_from_permission_set(id, input).await
            }
            "permissions_boundary_from_permission_set" => {
                self.update_permissions_boundary_from_permission_set(id, input).await
            }
            "permissions_boundary_for_permission_set" => {
                self.update_permissions_boundary_for_permission_set(id, input).await
            }
            "application" => {
                self.update_application(id, input).await
            }
            "application_assignment" => {
                self.update_application_assignment(id, input).await
            }
            "account_assignment" => {
                self.update_account_assignment(id, input).await
            }
            "account_assignment_creation_status" => {
                self.update_account_assignment_creation_status(id, input).await
            }
            "instance" => {
                self.update_instance(id, input).await
            }
            "permission_set" => {
                self.update_permission_set(id, input).await
            }
            "application_provider" => {
                self.update_application_provider(id, input).await
            }
            "inline_policy_for_permission_set" => {
                self.update_inline_policy_for_permission_set(id, input).await
            }
            "instance_access_control_attribute_configuration" => {
                self.update_instance_access_control_attribute_configuration(id, input).await
            }
            "trusted_token_issuer" => {
                self.update_trusted_token_issuer(id, input).await
            }
            "application_session_configuration" => {
                self.update_application_session_configuration(id, input).await
            }
            "inline_policy_to_permission_set" => {
                self.update_inline_policy_to_permission_set(id, input).await
            }
            "permission_set_provisioning_status" => {
                self.update_permission_set_provisioning_status(id, input).await
            }
            "permissions_boundary_to_permission_set" => {
                self.update_permissions_boundary_to_permission_set(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "sso_admin",
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
            "account_assignment_deletion_status" => {
                self.delete_account_assignment_deletion_status(id).await
            }
            "application_assignment_configuration" => {
                self.delete_application_assignment_configuration(id).await
            }
            "inline_policy_from_permission_set" => {
                self.delete_inline_policy_from_permission_set(id).await
            }
            "permissions_boundary_from_permission_set" => {
                self.delete_permissions_boundary_from_permission_set(id).await
            }
            "permissions_boundary_for_permission_set" => {
                self.delete_permissions_boundary_for_permission_set(id).await
            }
            "application" => {
                self.delete_application(id).await
            }
            "application_assignment" => {
                self.delete_application_assignment(id).await
            }
            "account_assignment" => {
                self.delete_account_assignment(id).await
            }
            "account_assignment_creation_status" => {
                self.delete_account_assignment_creation_status(id).await
            }
            "instance" => {
                self.delete_instance(id).await
            }
            "permission_set" => {
                self.delete_permission_set(id).await
            }
            "application_provider" => {
                self.delete_application_provider(id).await
            }
            "inline_policy_for_permission_set" => {
                self.delete_inline_policy_for_permission_set(id).await
            }
            "instance_access_control_attribute_configuration" => {
                self.delete_instance_access_control_attribute_configuration(id).await
            }
            "trusted_token_issuer" => {
                self.delete_trusted_token_issuer(id).await
            }
            "application_session_configuration" => {
                self.delete_application_session_configuration(id).await
            }
            "inline_policy_to_permission_set" => {
                self.delete_inline_policy_to_permission_set(id).await
            }
            "permission_set_provisioning_status" => {
                self.delete_permission_set_provisioning_status(id).await
            }
            "permissions_boundary_to_permission_set" => {
                self.delete_permissions_boundary_to_permission_set(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "sso_admin",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Account_assignment_deletion_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a account_assignment_deletion_status resource
    async fn plan_account_assignment_deletion_status(
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

    /// Create a new account_assignment_deletion_status resource
    async fn create_account_assignment_deletion_status(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_account_assignment_deletion_status()
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

    /// Read a account_assignment_deletion_status resource
    async fn read_account_assignment_deletion_status(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_account_assignment_deletion_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a account_assignment_deletion_status resource
    async fn update_account_assignment_deletion_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_account_assignment_deletion_status()
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

    /// Delete a account_assignment_deletion_status resource
    async fn delete_account_assignment_deletion_status(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_account_assignment_deletion_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Application_assignment_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a application_assignment_configuration resource
    async fn plan_application_assignment_configuration(
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

    /// Create a new application_assignment_configuration resource
    async fn create_application_assignment_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let application_arn = input.get_string("application_arn")?;
            let assignment_required = input.get_string("assignment_required")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_application_assignment_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("application_arn", application_arn.unwrap_or_default())
                .with_field("assignment_required", assignment_required.unwrap_or_default())
            )
        })
    }

    /// Read a application_assignment_configuration resource
    async fn read_application_assignment_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_application_assignment_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a application_assignment_configuration resource
    async fn update_application_assignment_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let application_arn = input.get_string("application_arn")?;
            let assignment_required = input.get_string("assignment_required")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_application_assignment_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("application_arn", application_arn.unwrap_or_default())
                .with_field("assignment_required", assignment_required.unwrap_or_default())
            )
        })
    }

    /// Delete a application_assignment_configuration resource
    async fn delete_application_assignment_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_application_assignment_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Inline_policy_from_permission_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inline_policy_from_permission_set resource
    async fn plan_inline_policy_from_permission_set(
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

    /// Create a new inline_policy_from_permission_set resource
    async fn create_inline_policy_from_permission_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_inline_policy_from_permission_set()
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

    /// Read a inline_policy_from_permission_set resource
    async fn read_inline_policy_from_permission_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_inline_policy_from_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a inline_policy_from_permission_set resource
    async fn update_inline_policy_from_permission_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_inline_policy_from_permission_set()
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

    /// Delete a inline_policy_from_permission_set resource
    async fn delete_inline_policy_from_permission_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_inline_policy_from_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Permissions_boundary_from_permission_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a permissions_boundary_from_permission_set resource
    async fn plan_permissions_boundary_from_permission_set(
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

    /// Create a new permissions_boundary_from_permission_set resource
    async fn create_permissions_boundary_from_permission_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_permissions_boundary_from_permission_set()
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

    /// Read a permissions_boundary_from_permission_set resource
    async fn read_permissions_boundary_from_permission_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_permissions_boundary_from_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a permissions_boundary_from_permission_set resource
    async fn update_permissions_boundary_from_permission_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_permissions_boundary_from_permission_set()
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

    /// Delete a permissions_boundary_from_permission_set resource
    async fn delete_permissions_boundary_from_permission_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_permissions_boundary_from_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Permissions_boundary_for_permission_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a permissions_boundary_for_permission_set resource
    async fn plan_permissions_boundary_for_permission_set(
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

    /// Create a new permissions_boundary_for_permission_set resource
    async fn create_permissions_boundary_for_permission_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_permissions_boundary_for_permission_set()
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

    /// Read a permissions_boundary_for_permission_set resource
    async fn read_permissions_boundary_for_permission_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_permissions_boundary_for_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a permissions_boundary_for_permission_set resource
    async fn update_permissions_boundary_for_permission_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_permissions_boundary_for_permission_set()
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

    /// Delete a permissions_boundary_for_permission_set resource
    async fn delete_permissions_boundary_for_permission_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_permissions_boundary_for_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Application resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a application resource
    async fn plan_application(
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

    /// Create a new application resource
    async fn create_application(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let instance_arn = input.get_string("instance_arn")?;
            let tags = input.get_optional_string("tags")?;
            let description = input.get_optional_string("description")?;
            let status = input.get_optional_string("status")?;
            let name = input.get_string("name")?;
            let client_token = input.get_optional_string("client_token")?;
            let portal_options = input.get_optional_string("portal_options")?;
            let application_provider_arn = input.get_string("application_provider_arn")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_application()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("instance_arn", instance_arn.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("status", status.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("portal_options", portal_options.unwrap_or_default())
                .with_field("application_provider_arn", application_provider_arn.unwrap_or_default())
            )
        })
    }

    /// Read a application resource
    async fn read_application(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_application()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a application resource
    async fn update_application(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let instance_arn = input.get_string("instance_arn")?;
            let tags = input.get_optional_string("tags")?;
            let description = input.get_optional_string("description")?;
            let status = input.get_optional_string("status")?;
            let name = input.get_string("name")?;
            let client_token = input.get_optional_string("client_token")?;
            let portal_options = input.get_optional_string("portal_options")?;
            let application_provider_arn = input.get_string("application_provider_arn")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_application()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("instance_arn", instance_arn.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("status", status.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("portal_options", portal_options.unwrap_or_default())
                .with_field("application_provider_arn", application_provider_arn.unwrap_or_default())
            )
        })
    }

    /// Delete a application resource
    async fn delete_application(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_application()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Application_assignment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a application_assignment resource
    async fn plan_application_assignment(
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

    /// Create a new application_assignment resource
    async fn create_application_assignment(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let principal_type = input.get_string("principal_type")?;
            let application_arn = input.get_string("application_arn")?;
            let principal_id = input.get_string("principal_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_application_assignment()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("principal_type", principal_type.unwrap_or_default())
                .with_field("application_arn", application_arn.unwrap_or_default())
                .with_field("principal_id", principal_id.unwrap_or_default())
            )
        })
    }

    /// Read a application_assignment resource
    async fn read_application_assignment(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_application_assignment()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a application_assignment resource
    async fn update_application_assignment(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let principal_type = input.get_string("principal_type")?;
            let application_arn = input.get_string("application_arn")?;
            let principal_id = input.get_string("principal_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_application_assignment()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("principal_type", principal_type.unwrap_or_default())
                .with_field("application_arn", application_arn.unwrap_or_default())
                .with_field("principal_id", principal_id.unwrap_or_default())
            )
        })
    }

    /// Delete a application_assignment resource
    async fn delete_application_assignment(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_application_assignment()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Account_assignment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a account_assignment resource
    async fn plan_account_assignment(
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

    /// Create a new account_assignment resource
    async fn create_account_assignment(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let permission_set_arn = input.get_string("permission_set_arn")?;
            let target_type = input.get_string("target_type")?;
            let principal_id = input.get_string("principal_id")?;
            let principal_type = input.get_string("principal_type")?;
            let target_id = input.get_string("target_id")?;
            let instance_arn = input.get_string("instance_arn")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_account_assignment()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("permission_set_arn", permission_set_arn.unwrap_or_default())
                .with_field("target_type", target_type.unwrap_or_default())
                .with_field("principal_id", principal_id.unwrap_or_default())
                .with_field("principal_type", principal_type.unwrap_or_default())
                .with_field("target_id", target_id.unwrap_or_default())
                .with_field("instance_arn", instance_arn.unwrap_or_default())
            )
        })
    }

    /// Read a account_assignment resource
    async fn read_account_assignment(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_account_assignment()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a account_assignment resource
    async fn update_account_assignment(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let permission_set_arn = input.get_string("permission_set_arn")?;
            let target_type = input.get_string("target_type")?;
            let principal_id = input.get_string("principal_id")?;
            let principal_type = input.get_string("principal_type")?;
            let target_id = input.get_string("target_id")?;
            let instance_arn = input.get_string("instance_arn")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_account_assignment()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("permission_set_arn", permission_set_arn.unwrap_or_default())
                .with_field("target_type", target_type.unwrap_or_default())
                .with_field("principal_id", principal_id.unwrap_or_default())
                .with_field("principal_type", principal_type.unwrap_or_default())
                .with_field("target_id", target_id.unwrap_or_default())
                .with_field("instance_arn", instance_arn.unwrap_or_default())
            )
        })
    }

    /// Delete a account_assignment resource
    async fn delete_account_assignment(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_account_assignment()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Account_assignment_creation_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a account_assignment_creation_status resource
    async fn plan_account_assignment_creation_status(
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

    /// Create a new account_assignment_creation_status resource
    async fn create_account_assignment_creation_status(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_account_assignment_creation_status()
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

    /// Read a account_assignment_creation_status resource
    async fn read_account_assignment_creation_status(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_account_assignment_creation_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a account_assignment_creation_status resource
    async fn update_account_assignment_creation_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_account_assignment_creation_status()
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

    /// Delete a account_assignment_creation_status resource
    async fn delete_account_assignment_creation_status(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_account_assignment_creation_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Instance resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance resource
    async fn plan_instance(
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

    /// Create a new instance resource
    async fn create_instance(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_optional_string("name")?;
            let client_token = input.get_optional_string("client_token")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_instance()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Read a instance resource
    async fn read_instance(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_instance()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a instance resource
    async fn update_instance(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_optional_string("name")?;
            let client_token = input.get_optional_string("client_token")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_instance()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Delete a instance resource
    async fn delete_instance(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_instance()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Permission_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a permission_set resource
    async fn plan_permission_set(
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

    /// Create a new permission_set resource
    async fn create_permission_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let relay_state = input.get_optional_string("relay_state")?;
            let description = input.get_optional_string("description")?;
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;
            let session_duration = input.get_optional_string("session_duration")?;
            let instance_arn = input.get_string("instance_arn")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_permission_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("relay_state", relay_state.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("session_duration", session_duration.unwrap_or_default())
                .with_field("instance_arn", instance_arn.unwrap_or_default())
            )
        })
    }

    /// Read a permission_set resource
    async fn read_permission_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a permission_set resource
    async fn update_permission_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let relay_state = input.get_optional_string("relay_state")?;
            let description = input.get_optional_string("description")?;
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;
            let session_duration = input.get_optional_string("session_duration")?;
            let instance_arn = input.get_string("instance_arn")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_permission_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("relay_state", relay_state.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("session_duration", session_duration.unwrap_or_default())
                .with_field("instance_arn", instance_arn.unwrap_or_default())
            )
        })
    }

    /// Delete a permission_set resource
    async fn delete_permission_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Application_provider resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a application_provider resource
    async fn plan_application_provider(
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

    /// Create a new application_provider resource
    async fn create_application_provider(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_application_provider()
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

    /// Read a application_provider resource
    async fn read_application_provider(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_application_provider()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a application_provider resource
    async fn update_application_provider(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_application_provider()
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

    /// Delete a application_provider resource
    async fn delete_application_provider(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_application_provider()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Inline_policy_for_permission_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inline_policy_for_permission_set resource
    async fn plan_inline_policy_for_permission_set(
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

    /// Create a new inline_policy_for_permission_set resource
    async fn create_inline_policy_for_permission_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_inline_policy_for_permission_set()
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

    /// Read a inline_policy_for_permission_set resource
    async fn read_inline_policy_for_permission_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_inline_policy_for_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a inline_policy_for_permission_set resource
    async fn update_inline_policy_for_permission_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_inline_policy_for_permission_set()
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

    /// Delete a inline_policy_for_permission_set resource
    async fn delete_inline_policy_for_permission_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_inline_policy_for_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Instance_access_control_attribute_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance_access_control_attribute_configuration resource
    async fn plan_instance_access_control_attribute_configuration(
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

    /// Create a new instance_access_control_attribute_configuration resource
    async fn create_instance_access_control_attribute_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let instance_arn = input.get_string("instance_arn")?;
            let instance_access_control_attribute_configuration = input.get_string("instance_access_control_attribute_configuration")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_instance_access_control_attribute_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("instance_arn", instance_arn.unwrap_or_default())
                .with_field("instance_access_control_attribute_configuration", instance_access_control_attribute_configuration.unwrap_or_default())
            )
        })
    }

    /// Read a instance_access_control_attribute_configuration resource
    async fn read_instance_access_control_attribute_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_instance_access_control_attribute_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a instance_access_control_attribute_configuration resource
    async fn update_instance_access_control_attribute_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let instance_arn = input.get_string("instance_arn")?;
            let instance_access_control_attribute_configuration = input.get_string("instance_access_control_attribute_configuration")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_instance_access_control_attribute_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("instance_arn", instance_arn.unwrap_or_default())
                .with_field("instance_access_control_attribute_configuration", instance_access_control_attribute_configuration.unwrap_or_default())
            )
        })
    }

    /// Delete a instance_access_control_attribute_configuration resource
    async fn delete_instance_access_control_attribute_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_instance_access_control_attribute_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Trusted_token_issuer resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a trusted_token_issuer resource
    async fn plan_trusted_token_issuer(
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

    /// Create a new trusted_token_issuer resource
    async fn create_trusted_token_issuer(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let client_token = input.get_optional_string("client_token")?;
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;
            let trusted_token_issuer_type = input.get_string("trusted_token_issuer_type")?;
            let instance_arn = input.get_string("instance_arn")?;
            let trusted_token_issuer_configuration = input.get_string("trusted_token_issuer_configuration")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_trusted_token_issuer()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("trusted_token_issuer_type", trusted_token_issuer_type.unwrap_or_default())
                .with_field("instance_arn", instance_arn.unwrap_or_default())
                .with_field("trusted_token_issuer_configuration", trusted_token_issuer_configuration.unwrap_or_default())
            )
        })
    }

    /// Read a trusted_token_issuer resource
    async fn read_trusted_token_issuer(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_trusted_token_issuer()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a trusted_token_issuer resource
    async fn update_trusted_token_issuer(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let client_token = input.get_optional_string("client_token")?;
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;
            let trusted_token_issuer_type = input.get_string("trusted_token_issuer_type")?;
            let instance_arn = input.get_string("instance_arn")?;
            let trusted_token_issuer_configuration = input.get_string("trusted_token_issuer_configuration")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_trusted_token_issuer()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("trusted_token_issuer_type", trusted_token_issuer_type.unwrap_or_default())
                .with_field("instance_arn", instance_arn.unwrap_or_default())
                .with_field("trusted_token_issuer_configuration", trusted_token_issuer_configuration.unwrap_or_default())
            )
        })
    }

    /// Delete a trusted_token_issuer resource
    async fn delete_trusted_token_issuer(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_trusted_token_issuer()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Application_session_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a application_session_configuration resource
    async fn plan_application_session_configuration(
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

    /// Create a new application_session_configuration resource
    async fn create_application_session_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let application_arn = input.get_string("application_arn")?;
            let user_background_session_application_status = input.get_optional_string("user_background_session_application_status")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_application_session_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("application_arn", application_arn.unwrap_or_default())
                .with_field("user_background_session_application_status", user_background_session_application_status.unwrap_or_default())
            )
        })
    }

    /// Read a application_session_configuration resource
    async fn read_application_session_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_application_session_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a application_session_configuration resource
    async fn update_application_session_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let application_arn = input.get_string("application_arn")?;
            let user_background_session_application_status = input.get_optional_string("user_background_session_application_status")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_application_session_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("application_arn", application_arn.unwrap_or_default())
                .with_field("user_background_session_application_status", user_background_session_application_status.unwrap_or_default())
            )
        })
    }

    /// Delete a application_session_configuration resource
    async fn delete_application_session_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_application_session_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Inline_policy_to_permission_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a inline_policy_to_permission_set resource
    async fn plan_inline_policy_to_permission_set(
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

    /// Create a new inline_policy_to_permission_set resource
    async fn create_inline_policy_to_permission_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let inline_policy = input.get_string("inline_policy")?;
            let permission_set_arn = input.get_string("permission_set_arn")?;
            let instance_arn = input.get_string("instance_arn")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_inline_policy_to_permission_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("inline_policy", inline_policy.unwrap_or_default())
                .with_field("permission_set_arn", permission_set_arn.unwrap_or_default())
                .with_field("instance_arn", instance_arn.unwrap_or_default())
            )
        })
    }

    /// Read a inline_policy_to_permission_set resource
    async fn read_inline_policy_to_permission_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_inline_policy_to_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a inline_policy_to_permission_set resource
    async fn update_inline_policy_to_permission_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let inline_policy = input.get_string("inline_policy")?;
            let permission_set_arn = input.get_string("permission_set_arn")?;
            let instance_arn = input.get_string("instance_arn")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_inline_policy_to_permission_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("inline_policy", inline_policy.unwrap_or_default())
                .with_field("permission_set_arn", permission_set_arn.unwrap_or_default())
                .with_field("instance_arn", instance_arn.unwrap_or_default())
            )
        })
    }

    /// Delete a inline_policy_to_permission_set resource
    async fn delete_inline_policy_to_permission_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_inline_policy_to_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Permission_set_provisioning_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a permission_set_provisioning_status resource
    async fn plan_permission_set_provisioning_status(
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

    /// Create a new permission_set_provisioning_status resource
    async fn create_permission_set_provisioning_status(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_permission_set_provisioning_status()
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

    /// Read a permission_set_provisioning_status resource
    async fn read_permission_set_provisioning_status(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_permission_set_provisioning_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a permission_set_provisioning_status resource
    async fn update_permission_set_provisioning_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_permission_set_provisioning_status()
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

    /// Delete a permission_set_provisioning_status resource
    async fn delete_permission_set_provisioning_status(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_permission_set_provisioning_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Permissions_boundary_to_permission_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a permissions_boundary_to_permission_set resource
    async fn plan_permissions_boundary_to_permission_set(
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

    /// Create a new permissions_boundary_to_permission_set resource
    async fn create_permissions_boundary_to_permission_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let permission_set_arn = input.get_string("permission_set_arn")?;
            let permissions_boundary = input.get_string("permissions_boundary")?;
            let instance_arn = input.get_string("instance_arn")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .create_permissions_boundary_to_permission_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("permission_set_arn", permission_set_arn.unwrap_or_default())
                .with_field("permissions_boundary", permissions_boundary.unwrap_or_default())
                .with_field("instance_arn", instance_arn.unwrap_or_default())
            )
        })
    }

    /// Read a permissions_boundary_to_permission_set resource
    async fn read_permissions_boundary_to_permission_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .describe_permissions_boundary_to_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a permissions_boundary_to_permission_set resource
    async fn update_permissions_boundary_to_permission_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let permission_set_arn = input.get_string("permission_set_arn")?;
            let permissions_boundary = input.get_string("permissions_boundary")?;
            let instance_arn = input.get_string("instance_arn")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.sso_admin_client
            //     .update_permissions_boundary_to_permission_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("permission_set_arn", permission_set_arn.unwrap_or_default())
                .with_field("permissions_boundary", permissions_boundary.unwrap_or_default())
                .with_field("instance_arn", instance_arn.unwrap_or_default())
            )
        })
    }

    /// Delete a permissions_boundary_to_permission_set resource
    async fn delete_permissions_boundary_to_permission_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.sso_admin_client
            //     .delete_permissions_boundary_to_permission_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
