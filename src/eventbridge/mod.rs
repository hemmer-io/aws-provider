//! Eventbridge service for Aws provider
//!
//! This module handles all eventbridge resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Eventbridge service handler
pub struct EventbridgeService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> EventbridgeService<'a> {
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
            "replay" => {
                self.plan_replay(current_state, desired_input).await
            }
            "connection" => {
                self.plan_connection(current_state, desired_input).await
            }
            "events" => {
                self.plan_events(current_state, desired_input).await
            }
            "partner_events" => {
                self.plan_partner_events(current_state, desired_input).await
            }
            "rule" => {
                self.plan_rule(current_state, desired_input).await
            }
            "permission" => {
                self.plan_permission(current_state, desired_input).await
            }
            "archive" => {
                self.plan_archive(current_state, desired_input).await
            }
            "api_destination" => {
                self.plan_api_destination(current_state, desired_input).await
            }
            "event_bus" => {
                self.plan_event_bus(current_state, desired_input).await
            }
            "partner_event_source" => {
                self.plan_partner_event_source(current_state, desired_input).await
            }
            "endpoint" => {
                self.plan_endpoint(current_state, desired_input).await
            }
            "targets" => {
                self.plan_targets(current_state, desired_input).await
            }
            "event_source" => {
                self.plan_event_source(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "eventbridge",
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
            "replay" => {
                self.create_replay(input).await
            }
            "connection" => {
                self.create_connection(input).await
            }
            "events" => {
                self.create_events(input).await
            }
            "partner_events" => {
                self.create_partner_events(input).await
            }
            "rule" => {
                self.create_rule(input).await
            }
            "permission" => {
                self.create_permission(input).await
            }
            "archive" => {
                self.create_archive(input).await
            }
            "api_destination" => {
                self.create_api_destination(input).await
            }
            "event_bus" => {
                self.create_event_bus(input).await
            }
            "partner_event_source" => {
                self.create_partner_event_source(input).await
            }
            "endpoint" => {
                self.create_endpoint(input).await
            }
            "targets" => {
                self.create_targets(input).await
            }
            "event_source" => {
                self.create_event_source(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "eventbridge",
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
            "replay" => {
                self.read_replay(id).await
            }
            "connection" => {
                self.read_connection(id).await
            }
            "events" => {
                self.read_events(id).await
            }
            "partner_events" => {
                self.read_partner_events(id).await
            }
            "rule" => {
                self.read_rule(id).await
            }
            "permission" => {
                self.read_permission(id).await
            }
            "archive" => {
                self.read_archive(id).await
            }
            "api_destination" => {
                self.read_api_destination(id).await
            }
            "event_bus" => {
                self.read_event_bus(id).await
            }
            "partner_event_source" => {
                self.read_partner_event_source(id).await
            }
            "endpoint" => {
                self.read_endpoint(id).await
            }
            "targets" => {
                self.read_targets(id).await
            }
            "event_source" => {
                self.read_event_source(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "eventbridge",
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
            "replay" => {
                self.update_replay(id, input).await
            }
            "connection" => {
                self.update_connection(id, input).await
            }
            "events" => {
                self.update_events(id, input).await
            }
            "partner_events" => {
                self.update_partner_events(id, input).await
            }
            "rule" => {
                self.update_rule(id, input).await
            }
            "permission" => {
                self.update_permission(id, input).await
            }
            "archive" => {
                self.update_archive(id, input).await
            }
            "api_destination" => {
                self.update_api_destination(id, input).await
            }
            "event_bus" => {
                self.update_event_bus(id, input).await
            }
            "partner_event_source" => {
                self.update_partner_event_source(id, input).await
            }
            "endpoint" => {
                self.update_endpoint(id, input).await
            }
            "targets" => {
                self.update_targets(id, input).await
            }
            "event_source" => {
                self.update_event_source(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "eventbridge",
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
            "replay" => {
                self.delete_replay(id).await
            }
            "connection" => {
                self.delete_connection(id).await
            }
            "events" => {
                self.delete_events(id).await
            }
            "partner_events" => {
                self.delete_partner_events(id).await
            }
            "rule" => {
                self.delete_rule(id).await
            }
            "permission" => {
                self.delete_permission(id).await
            }
            "archive" => {
                self.delete_archive(id).await
            }
            "api_destination" => {
                self.delete_api_destination(id).await
            }
            "event_bus" => {
                self.delete_event_bus(id).await
            }
            "partner_event_source" => {
                self.delete_partner_event_source(id).await
            }
            "endpoint" => {
                self.delete_endpoint(id).await
            }
            "targets" => {
                self.delete_targets(id).await
            }
            "event_source" => {
                self.delete_event_source(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "eventbridge",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Replay resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a replay resource
    async fn plan_replay(
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

    /// Create a new replay resource
    async fn create_replay(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_replay()
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

    /// Read a replay resource
    async fn read_replay(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_replay()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a replay resource
    async fn update_replay(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_replay()
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

    /// Delete a replay resource
    async fn delete_replay(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_replay()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connection resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connection resource
    async fn plan_connection(
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

    /// Create a new connection resource
    async fn create_connection(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let name = input.get_string("name")?;
            let kms_key_identifier = input.get_optional_string("kms_key_identifier")?;
            let auth_parameters = input.get_string("auth_parameters")?;
            let authorization_type = input.get_string("authorization_type")?;
            let invocation_connectivity_parameters = input.get_optional_string("invocation_connectivity_parameters")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_connection()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("description", description.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("kms_key_identifier", kms_key_identifier.unwrap_or_default())
                .with_field("auth_parameters", auth_parameters.unwrap_or_default())
                .with_field("authorization_type", authorization_type.unwrap_or_default())
                .with_field("invocation_connectivity_parameters", invocation_connectivity_parameters.unwrap_or_default())
            )
        })
    }

    /// Read a connection resource
    async fn read_connection(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_connection()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connection resource
    async fn update_connection(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let name = input.get_string("name")?;
            let kms_key_identifier = input.get_optional_string("kms_key_identifier")?;
            let auth_parameters = input.get_string("auth_parameters")?;
            let authorization_type = input.get_string("authorization_type")?;
            let invocation_connectivity_parameters = input.get_optional_string("invocation_connectivity_parameters")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_connection()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("description", description.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("kms_key_identifier", kms_key_identifier.unwrap_or_default())
                .with_field("auth_parameters", auth_parameters.unwrap_or_default())
                .with_field("authorization_type", authorization_type.unwrap_or_default())
                .with_field("invocation_connectivity_parameters", invocation_connectivity_parameters.unwrap_or_default())
            )
        })
    }

    /// Delete a connection resource
    async fn delete_connection(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_connection()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Events resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a events resource
    async fn plan_events(
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

    /// Create a new events resource
    async fn create_events(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let entries = input.get_string("entries")?;
            let endpoint_id = input.get_optional_string("endpoint_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_events()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("entries", entries.unwrap_or_default())
                .with_field("endpoint_id", endpoint_id.unwrap_or_default())
            )
        })
    }

    /// Read a events resource
    async fn read_events(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_events()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a events resource
    async fn update_events(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let entries = input.get_string("entries")?;
            let endpoint_id = input.get_optional_string("endpoint_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_events()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("entries", entries.unwrap_or_default())
                .with_field("endpoint_id", endpoint_id.unwrap_or_default())
            )
        })
    }

    /// Delete a events resource
    async fn delete_events(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_events()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Partner_events resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a partner_events resource
    async fn plan_partner_events(
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

    /// Create a new partner_events resource
    async fn create_partner_events(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let entries = input.get_string("entries")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_partner_events()
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

    /// Read a partner_events resource
    async fn read_partner_events(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_partner_events()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a partner_events resource
    async fn update_partner_events(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let entries = input.get_string("entries")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_partner_events()
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

    /// Delete a partner_events resource
    async fn delete_partner_events(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_partner_events()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Rule resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rule resource
    async fn plan_rule(
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

    /// Create a new rule resource
    async fn create_rule(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let description = input.get_optional_string("description")?;
            let tags = input.get_optional_string("tags")?;
            let role_arn = input.get_optional_string("role_arn")?;
            let event_pattern = input.get_optional_string("event_pattern")?;
            let schedule_expression = input.get_optional_string("schedule_expression")?;
            let state = input.get_optional_string("state")?;
            let event_bus_name = input.get_optional_string("event_bus_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_rule()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("role_arn", role_arn.unwrap_or_default())
                .with_field("event_pattern", event_pattern.unwrap_or_default())
                .with_field("schedule_expression", schedule_expression.unwrap_or_default())
                .with_field("state", state.unwrap_or_default())
                .with_field("event_bus_name", event_bus_name.unwrap_or_default())
            )
        })
    }

    /// Read a rule resource
    async fn read_rule(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_rule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a rule resource
    async fn update_rule(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let description = input.get_optional_string("description")?;
            let tags = input.get_optional_string("tags")?;
            let role_arn = input.get_optional_string("role_arn")?;
            let event_pattern = input.get_optional_string("event_pattern")?;
            let schedule_expression = input.get_optional_string("schedule_expression")?;
            let state = input.get_optional_string("state")?;
            let event_bus_name = input.get_optional_string("event_bus_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_rule()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("role_arn", role_arn.unwrap_or_default())
                .with_field("event_pattern", event_pattern.unwrap_or_default())
                .with_field("schedule_expression", schedule_expression.unwrap_or_default())
                .with_field("state", state.unwrap_or_default())
                .with_field("event_bus_name", event_bus_name.unwrap_or_default())
            )
        })
    }

    /// Delete a rule resource
    async fn delete_rule(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_rule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Permission resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a permission resource
    async fn plan_permission(
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

    /// Create a new permission resource
    async fn create_permission(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let action = input.get_optional_string("action")?;
            let policy = input.get_optional_string("policy")?;
            let event_bus_name = input.get_optional_string("event_bus_name")?;
            let principal = input.get_optional_string("principal")?;
            let statement_id = input.get_optional_string("statement_id")?;
            let condition = input.get_optional_string("condition")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_permission()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("action", action.unwrap_or_default())
                .with_field("policy", policy.unwrap_or_default())
                .with_field("event_bus_name", event_bus_name.unwrap_or_default())
                .with_field("principal", principal.unwrap_or_default())
                .with_field("statement_id", statement_id.unwrap_or_default())
                .with_field("condition", condition.unwrap_or_default())
            )
        })
    }

    /// Read a permission resource
    async fn read_permission(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_permission()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a permission resource
    async fn update_permission(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let action = input.get_optional_string("action")?;
            let policy = input.get_optional_string("policy")?;
            let event_bus_name = input.get_optional_string("event_bus_name")?;
            let principal = input.get_optional_string("principal")?;
            let statement_id = input.get_optional_string("statement_id")?;
            let condition = input.get_optional_string("condition")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_permission()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("action", action.unwrap_or_default())
                .with_field("policy", policy.unwrap_or_default())
                .with_field("event_bus_name", event_bus_name.unwrap_or_default())
                .with_field("principal", principal.unwrap_or_default())
                .with_field("statement_id", statement_id.unwrap_or_default())
                .with_field("condition", condition.unwrap_or_default())
            )
        })
    }

    /// Delete a permission resource
    async fn delete_permission(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_permission()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Archive resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a archive resource
    async fn plan_archive(
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

    /// Create a new archive resource
    async fn create_archive(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let retention_days = input.get_optional_string("retention_days")?;
            let kms_key_identifier = input.get_optional_string("kms_key_identifier")?;
            let event_pattern = input.get_optional_string("event_pattern")?;
            let archive_name = input.get_string("archive_name")?;
            let event_source_arn = input.get_string("event_source_arn")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_archive()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("description", description.unwrap_or_default())
                .with_field("retention_days", retention_days.unwrap_or_default())
                .with_field("kms_key_identifier", kms_key_identifier.unwrap_or_default())
                .with_field("event_pattern", event_pattern.unwrap_or_default())
                .with_field("archive_name", archive_name.unwrap_or_default())
                .with_field("event_source_arn", event_source_arn.unwrap_or_default())
            )
        })
    }

    /// Read a archive resource
    async fn read_archive(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_archive()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a archive resource
    async fn update_archive(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let retention_days = input.get_optional_string("retention_days")?;
            let kms_key_identifier = input.get_optional_string("kms_key_identifier")?;
            let event_pattern = input.get_optional_string("event_pattern")?;
            let archive_name = input.get_string("archive_name")?;
            let event_source_arn = input.get_string("event_source_arn")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_archive()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("description", description.unwrap_or_default())
                .with_field("retention_days", retention_days.unwrap_or_default())
                .with_field("kms_key_identifier", kms_key_identifier.unwrap_or_default())
                .with_field("event_pattern", event_pattern.unwrap_or_default())
                .with_field("archive_name", archive_name.unwrap_or_default())
                .with_field("event_source_arn", event_source_arn.unwrap_or_default())
            )
        })
    }

    /// Delete a archive resource
    async fn delete_archive(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_archive()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Api_destination resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a api_destination resource
    async fn plan_api_destination(
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

    /// Create a new api_destination resource
    async fn create_api_destination(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let invocation_endpoint = input.get_string("invocation_endpoint")?;
            let http_method = input.get_string("http_method")?;
            let invocation_rate_limit_per_second = input.get_optional_string("invocation_rate_limit_per_second")?;
            let connection_arn = input.get_string("connection_arn")?;
            let description = input.get_optional_string("description")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_api_destination()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("invocation_endpoint", invocation_endpoint.unwrap_or_default())
                .with_field("http_method", http_method.unwrap_or_default())
                .with_field("invocation_rate_limit_per_second", invocation_rate_limit_per_second.unwrap_or_default())
                .with_field("connection_arn", connection_arn.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
            )
        })
    }

    /// Read a api_destination resource
    async fn read_api_destination(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_api_destination()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a api_destination resource
    async fn update_api_destination(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let invocation_endpoint = input.get_string("invocation_endpoint")?;
            let http_method = input.get_string("http_method")?;
            let invocation_rate_limit_per_second = input.get_optional_string("invocation_rate_limit_per_second")?;
            let connection_arn = input.get_string("connection_arn")?;
            let description = input.get_optional_string("description")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_api_destination()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("invocation_endpoint", invocation_endpoint.unwrap_or_default())
                .with_field("http_method", http_method.unwrap_or_default())
                .with_field("invocation_rate_limit_per_second", invocation_rate_limit_per_second.unwrap_or_default())
                .with_field("connection_arn", connection_arn.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
            )
        })
    }

    /// Delete a api_destination resource
    async fn delete_api_destination(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_api_destination()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Event_bus resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a event_bus resource
    async fn plan_event_bus(
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

    /// Create a new event_bus resource
    async fn create_event_bus(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let kms_key_identifier = input.get_optional_string("kms_key_identifier")?;
            let dead_letter_config = input.get_optional_string("dead_letter_config")?;
            let log_config = input.get_optional_string("log_config")?;
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;
            let description = input.get_optional_string("description")?;
            let event_source_name = input.get_optional_string("event_source_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_event_bus()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("kms_key_identifier", kms_key_identifier.unwrap_or_default())
                .with_field("dead_letter_config", dead_letter_config.unwrap_or_default())
                .with_field("log_config", log_config.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("event_source_name", event_source_name.unwrap_or_default())
            )
        })
    }

    /// Read a event_bus resource
    async fn read_event_bus(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_event_bus()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a event_bus resource
    async fn update_event_bus(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let kms_key_identifier = input.get_optional_string("kms_key_identifier")?;
            let dead_letter_config = input.get_optional_string("dead_letter_config")?;
            let log_config = input.get_optional_string("log_config")?;
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;
            let description = input.get_optional_string("description")?;
            let event_source_name = input.get_optional_string("event_source_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_event_bus()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("kms_key_identifier", kms_key_identifier.unwrap_or_default())
                .with_field("dead_letter_config", dead_letter_config.unwrap_or_default())
                .with_field("log_config", log_config.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("event_source_name", event_source_name.unwrap_or_default())
            )
        })
    }

    /// Delete a event_bus resource
    async fn delete_event_bus(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_event_bus()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Partner_event_source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a partner_event_source resource
    async fn plan_partner_event_source(
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

    /// Create a new partner_event_source resource
    async fn create_partner_event_source(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let account = input.get_string("account")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_partner_event_source()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("account", account.unwrap_or_default())
            )
        })
    }

    /// Read a partner_event_source resource
    async fn read_partner_event_source(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_partner_event_source()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a partner_event_source resource
    async fn update_partner_event_source(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let account = input.get_string("account")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_partner_event_source()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("account", account.unwrap_or_default())
            )
        })
    }

    /// Delete a partner_event_source resource
    async fn delete_partner_event_source(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_partner_event_source()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Endpoint resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a endpoint resource
    async fn plan_endpoint(
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

    /// Create a new endpoint resource
    async fn create_endpoint(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let replication_config = input.get_optional_string("replication_config")?;
            let description = input.get_optional_string("description")?;
            let event_buses = input.get_string("event_buses")?;
            let routing_config = input.get_string("routing_config")?;
            let role_arn = input.get_optional_string("role_arn")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_endpoint()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("replication_config", replication_config.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("event_buses", event_buses.unwrap_or_default())
                .with_field("routing_config", routing_config.unwrap_or_default())
                .with_field("role_arn", role_arn.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a endpoint resource
    async fn read_endpoint(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_endpoint()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a endpoint resource
    async fn update_endpoint(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let replication_config = input.get_optional_string("replication_config")?;
            let description = input.get_optional_string("description")?;
            let event_buses = input.get_string("event_buses")?;
            let routing_config = input.get_string("routing_config")?;
            let role_arn = input.get_optional_string("role_arn")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_endpoint()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("replication_config", replication_config.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("event_buses", event_buses.unwrap_or_default())
                .with_field("routing_config", routing_config.unwrap_or_default())
                .with_field("role_arn", role_arn.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a endpoint resource
    async fn delete_endpoint(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_endpoint()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Targets resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a targets resource
    async fn plan_targets(
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

    /// Create a new targets resource
    async fn create_targets(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let rule = input.get_string("rule")?;
            let targets = input.get_string("targets")?;
            let event_bus_name = input.get_optional_string("event_bus_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_targets()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("rule", rule.unwrap_or_default())
                .with_field("targets", targets.unwrap_or_default())
                .with_field("event_bus_name", event_bus_name.unwrap_or_default())
            )
        })
    }

    /// Read a targets resource
    async fn read_targets(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_targets()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a targets resource
    async fn update_targets(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let rule = input.get_string("rule")?;
            let targets = input.get_string("targets")?;
            let event_bus_name = input.get_optional_string("event_bus_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_targets()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("rule", rule.unwrap_or_default())
                .with_field("targets", targets.unwrap_or_default())
                .with_field("event_bus_name", event_bus_name.unwrap_or_default())
            )
        })
    }

    /// Delete a targets resource
    async fn delete_targets(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_targets()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Event_source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a event_source resource
    async fn plan_event_source(
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

    /// Create a new event_source resource
    async fn create_event_source(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .create_event_source()
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

    /// Read a event_source resource
    async fn read_event_source(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .describe_event_source()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a event_source resource
    async fn update_event_source(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.eventbridge_client
            //     .update_event_source()
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

    /// Delete a event_source resource
    async fn delete_event_source(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.eventbridge_client
            //     .delete_event_source()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
