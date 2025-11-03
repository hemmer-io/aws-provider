//! Ssm_contacts service for Aws provider
//!
//! This module handles all ssm_contacts resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Ssm_contacts service handler
pub struct Ssm_contactsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ssm_contactsService<'a> {
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
            "contact_policy" => {
                self.plan_contact_policy(current_state, desired_input).await
            }
            "contact_channel" => {
                self.plan_contact_channel(current_state, desired_input).await
            }
            "rotation" => {
                self.plan_rotation(current_state, desired_input).await
            }
            "rotation_override" => {
                self.plan_rotation_override(current_state, desired_input).await
            }
            "engagement" => {
                self.plan_engagement(current_state, desired_input).await
            }
            "page" => {
                self.plan_page(current_state, desired_input).await
            }
            "contact" => {
                self.plan_contact(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ssm_contacts",
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
            "contact_policy" => {
                self.create_contact_policy(input).await
            }
            "contact_channel" => {
                self.create_contact_channel(input).await
            }
            "rotation" => {
                self.create_rotation(input).await
            }
            "rotation_override" => {
                self.create_rotation_override(input).await
            }
            "engagement" => {
                self.create_engagement(input).await
            }
            "page" => {
                self.create_page(input).await
            }
            "contact" => {
                self.create_contact(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ssm_contacts",
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
            "contact_policy" => {
                self.read_contact_policy(id).await
            }
            "contact_channel" => {
                self.read_contact_channel(id).await
            }
            "rotation" => {
                self.read_rotation(id).await
            }
            "rotation_override" => {
                self.read_rotation_override(id).await
            }
            "engagement" => {
                self.read_engagement(id).await
            }
            "page" => {
                self.read_page(id).await
            }
            "contact" => {
                self.read_contact(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ssm_contacts",
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
            "contact_policy" => {
                self.update_contact_policy(id, input).await
            }
            "contact_channel" => {
                self.update_contact_channel(id, input).await
            }
            "rotation" => {
                self.update_rotation(id, input).await
            }
            "rotation_override" => {
                self.update_rotation_override(id, input).await
            }
            "engagement" => {
                self.update_engagement(id, input).await
            }
            "page" => {
                self.update_page(id, input).await
            }
            "contact" => {
                self.update_contact(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ssm_contacts",
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
            "contact_policy" => {
                self.delete_contact_policy(id).await
            }
            "contact_channel" => {
                self.delete_contact_channel(id).await
            }
            "rotation" => {
                self.delete_rotation(id).await
            }
            "rotation_override" => {
                self.delete_rotation_override(id).await
            }
            "engagement" => {
                self.delete_engagement(id).await
            }
            "page" => {
                self.delete_page(id).await
            }
            "contact" => {
                self.delete_contact(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ssm_contacts",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Contact_policy resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a contact_policy resource
    async fn plan_contact_policy(
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

    /// Create a new contact_policy resource
    async fn create_contact_policy(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let contact_arn = input.get_string("contact_arn")?;
            let policy = input.get_string("policy")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .create_contact_policy()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("contact_arn", contact_arn.unwrap_or_default())
                .with_field("policy", policy.unwrap_or_default())
            )
        })
    }

    /// Read a contact_policy resource
    async fn read_contact_policy(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .describe_contact_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a contact_policy resource
    async fn update_contact_policy(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let contact_arn = input.get_string("contact_arn")?;
            let policy = input.get_string("policy")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .update_contact_policy()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("contact_arn", contact_arn.unwrap_or_default())
                .with_field("policy", policy.unwrap_or_default())
            )
        })
    }

    /// Delete a contact_policy resource
    async fn delete_contact_policy(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_contacts_client
            //     .delete_contact_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Contact_channel resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a contact_channel resource
    async fn plan_contact_channel(
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

    /// Create a new contact_channel resource
    async fn create_contact_channel(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let defer_activation = input.get_optional_string("defer_activation")?;
            let r#type = input.get_string("type")?;
            let delivery_address = input.get_string("delivery_address")?;
            let idempotency_token = input.get_optional_string("idempotency_token")?;
            let contact_id = input.get_string("contact_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .create_contact_channel()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("defer_activation", defer_activation.unwrap_or_default())
                .with_field("type", r#type.unwrap_or_default())
                .with_field("delivery_address", delivery_address.unwrap_or_default())
                .with_field("idempotency_token", idempotency_token.unwrap_or_default())
                .with_field("contact_id", contact_id.unwrap_or_default())
            )
        })
    }

    /// Read a contact_channel resource
    async fn read_contact_channel(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .describe_contact_channel()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a contact_channel resource
    async fn update_contact_channel(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let defer_activation = input.get_optional_string("defer_activation")?;
            let r#type = input.get_string("type")?;
            let delivery_address = input.get_string("delivery_address")?;
            let idempotency_token = input.get_optional_string("idempotency_token")?;
            let contact_id = input.get_string("contact_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .update_contact_channel()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("defer_activation", defer_activation.unwrap_or_default())
                .with_field("type", r#type.unwrap_or_default())
                .with_field("delivery_address", delivery_address.unwrap_or_default())
                .with_field("idempotency_token", idempotency_token.unwrap_or_default())
                .with_field("contact_id", contact_id.unwrap_or_default())
            )
        })
    }

    /// Delete a contact_channel resource
    async fn delete_contact_channel(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_contacts_client
            //     .delete_contact_channel()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Rotation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rotation resource
    async fn plan_rotation(
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

    /// Create a new rotation resource
    async fn create_rotation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let tags = input.get_optional_string("tags")?;
            let contact_ids = input.get_string("contact_ids")?;
            let start_time = input.get_optional_string("start_time")?;
            let time_zone_id = input.get_string("time_zone_id")?;
            let recurrence = input.get_string("recurrence")?;
            let idempotency_token = input.get_optional_string("idempotency_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .create_rotation()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("contact_ids", contact_ids.unwrap_or_default())
                .with_field("start_time", start_time.unwrap_or_default())
                .with_field("time_zone_id", time_zone_id.unwrap_or_default())
                .with_field("recurrence", recurrence.unwrap_or_default())
                .with_field("idempotency_token", idempotency_token.unwrap_or_default())
            )
        })
    }

    /// Read a rotation resource
    async fn read_rotation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .describe_rotation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a rotation resource
    async fn update_rotation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let tags = input.get_optional_string("tags")?;
            let contact_ids = input.get_string("contact_ids")?;
            let start_time = input.get_optional_string("start_time")?;
            let time_zone_id = input.get_string("time_zone_id")?;
            let recurrence = input.get_string("recurrence")?;
            let idempotency_token = input.get_optional_string("idempotency_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .update_rotation()
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
                .with_field("contact_ids", contact_ids.unwrap_or_default())
                .with_field("start_time", start_time.unwrap_or_default())
                .with_field("time_zone_id", time_zone_id.unwrap_or_default())
                .with_field("recurrence", recurrence.unwrap_or_default())
                .with_field("idempotency_token", idempotency_token.unwrap_or_default())
            )
        })
    }

    /// Delete a rotation resource
    async fn delete_rotation(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_contacts_client
            //     .delete_rotation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Rotation_override resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rotation_override resource
    async fn plan_rotation_override(
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

    /// Create a new rotation_override resource
    async fn create_rotation_override(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let end_time = input.get_string("end_time")?;
            let start_time = input.get_string("start_time")?;
            let new_contact_ids = input.get_string("new_contact_ids")?;
            let idempotency_token = input.get_optional_string("idempotency_token")?;
            let rotation_id = input.get_string("rotation_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .create_rotation_override()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("end_time", end_time.unwrap_or_default())
                .with_field("start_time", start_time.unwrap_or_default())
                .with_field("new_contact_ids", new_contact_ids.unwrap_or_default())
                .with_field("idempotency_token", idempotency_token.unwrap_or_default())
                .with_field("rotation_id", rotation_id.unwrap_or_default())
            )
        })
    }

    /// Read a rotation_override resource
    async fn read_rotation_override(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .describe_rotation_override()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a rotation_override resource
    async fn update_rotation_override(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let end_time = input.get_string("end_time")?;
            let start_time = input.get_string("start_time")?;
            let new_contact_ids = input.get_string("new_contact_ids")?;
            let idempotency_token = input.get_optional_string("idempotency_token")?;
            let rotation_id = input.get_string("rotation_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .update_rotation_override()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("end_time", end_time.unwrap_or_default())
                .with_field("start_time", start_time.unwrap_or_default())
                .with_field("new_contact_ids", new_contact_ids.unwrap_or_default())
                .with_field("idempotency_token", idempotency_token.unwrap_or_default())
                .with_field("rotation_id", rotation_id.unwrap_or_default())
            )
        })
    }

    /// Delete a rotation_override resource
    async fn delete_rotation_override(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_contacts_client
            //     .delete_rotation_override()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Engagement resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a engagement resource
    async fn plan_engagement(
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

    /// Create a new engagement resource
    async fn create_engagement(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .create_engagement()
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

    /// Read a engagement resource
    async fn read_engagement(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .describe_engagement()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a engagement resource
    async fn update_engagement(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .update_engagement()
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

    /// Delete a engagement resource
    async fn delete_engagement(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_contacts_client
            //     .delete_engagement()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Page resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a page resource
    async fn plan_page(
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

    /// Create a new page resource
    async fn create_page(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .create_page()
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

    /// Read a page resource
    async fn read_page(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .describe_page()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a page resource
    async fn update_page(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .update_page()
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

    /// Delete a page resource
    async fn delete_page(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_contacts_client
            //     .delete_page()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Contact resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a contact resource
    async fn plan_contact(
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

    /// Create a new contact resource
    async fn create_contact(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let idempotency_token = input.get_optional_string("idempotency_token")?;
            let plan = input.get_string("plan")?;
            let tags = input.get_optional_string("tags")?;
            let r#type = input.get_string("type")?;
            let alias = input.get_string("alias")?;
            let display_name = input.get_optional_string("display_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .create_contact()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("idempotency_token", idempotency_token.unwrap_or_default())
                .with_field("plan", plan.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("type", r#type.unwrap_or_default())
                .with_field("alias", alias.unwrap_or_default())
                .with_field("display_name", display_name.unwrap_or_default())
            )
        })
    }

    /// Read a contact resource
    async fn read_contact(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .describe_contact()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a contact resource
    async fn update_contact(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let idempotency_token = input.get_optional_string("idempotency_token")?;
            let plan = input.get_string("plan")?;
            let tags = input.get_optional_string("tags")?;
            let r#type = input.get_string("type")?;
            let alias = input.get_string("alias")?;
            let display_name = input.get_optional_string("display_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ssm_contacts_client
            //     .update_contact()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("idempotency_token", idempotency_token.unwrap_or_default())
                .with_field("plan", plan.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("type", r#type.unwrap_or_default())
                .with_field("alias", alias.unwrap_or_default())
                .with_field("display_name", display_name.unwrap_or_default())
            )
        })
    }

    /// Delete a contact resource
    async fn delete_contact(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ssm_contacts_client
            //     .delete_contact()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
