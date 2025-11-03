//! Finspace_data service for Aws provider
//!
//! This module handles all finspace_data resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Finspace_data service handler
pub struct Finspace_dataService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Finspace_dataService<'a> {
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
            "external_data_view_access_details" => {
                self.plan_external_data_view_access_details(current_state, desired_input).await
            }
            "dataset" => {
                self.plan_dataset(current_state, desired_input).await
            }
            "changeset" => {
                self.plan_changeset(current_state, desired_input).await
            }
            "user" => {
                self.plan_user(current_state, desired_input).await
            }
            "data_view" => {
                self.plan_data_view(current_state, desired_input).await
            }
            "working_location" => {
                self.plan_working_location(current_state, desired_input).await
            }
            "programmatic_access_credentials" => {
                self.plan_programmatic_access_credentials(current_state, desired_input).await
            }
            "permission_group" => {
                self.plan_permission_group(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "finspace_data",
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
            "external_data_view_access_details" => {
                self.create_external_data_view_access_details(input).await
            }
            "dataset" => {
                self.create_dataset(input).await
            }
            "changeset" => {
                self.create_changeset(input).await
            }
            "user" => {
                self.create_user(input).await
            }
            "data_view" => {
                self.create_data_view(input).await
            }
            "working_location" => {
                self.create_working_location(input).await
            }
            "programmatic_access_credentials" => {
                self.create_programmatic_access_credentials(input).await
            }
            "permission_group" => {
                self.create_permission_group(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "finspace_data",
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
            "external_data_view_access_details" => {
                self.read_external_data_view_access_details(id).await
            }
            "dataset" => {
                self.read_dataset(id).await
            }
            "changeset" => {
                self.read_changeset(id).await
            }
            "user" => {
                self.read_user(id).await
            }
            "data_view" => {
                self.read_data_view(id).await
            }
            "working_location" => {
                self.read_working_location(id).await
            }
            "programmatic_access_credentials" => {
                self.read_programmatic_access_credentials(id).await
            }
            "permission_group" => {
                self.read_permission_group(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "finspace_data",
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
            "external_data_view_access_details" => {
                self.update_external_data_view_access_details(id, input).await
            }
            "dataset" => {
                self.update_dataset(id, input).await
            }
            "changeset" => {
                self.update_changeset(id, input).await
            }
            "user" => {
                self.update_user(id, input).await
            }
            "data_view" => {
                self.update_data_view(id, input).await
            }
            "working_location" => {
                self.update_working_location(id, input).await
            }
            "programmatic_access_credentials" => {
                self.update_programmatic_access_credentials(id, input).await
            }
            "permission_group" => {
                self.update_permission_group(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "finspace_data",
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
            "external_data_view_access_details" => {
                self.delete_external_data_view_access_details(id).await
            }
            "dataset" => {
                self.delete_dataset(id).await
            }
            "changeset" => {
                self.delete_changeset(id).await
            }
            "user" => {
                self.delete_user(id).await
            }
            "data_view" => {
                self.delete_data_view(id).await
            }
            "working_location" => {
                self.delete_working_location(id).await
            }
            "programmatic_access_credentials" => {
                self.delete_programmatic_access_credentials(id).await
            }
            "permission_group" => {
                self.delete_permission_group(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "finspace_data",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // External_data_view_access_details resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a external_data_view_access_details resource
    async fn plan_external_data_view_access_details(
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

    /// Create a new external_data_view_access_details resource
    async fn create_external_data_view_access_details(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .create_external_data_view_access_details()
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

    /// Read a external_data_view_access_details resource
    async fn read_external_data_view_access_details(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .describe_external_data_view_access_details()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a external_data_view_access_details resource
    async fn update_external_data_view_access_details(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .update_external_data_view_access_details()
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

    /// Delete a external_data_view_access_details resource
    async fn delete_external_data_view_access_details(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.finspace_data_client
            //     .delete_external_data_view_access_details()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Dataset resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dataset resource
    async fn plan_dataset(
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

    /// Create a new dataset resource
    async fn create_dataset(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let kind = input.get_string("kind")?;
            let permission_group_params = input.get_string("permission_group_params")?;
            let schema_definition = input.get_optional_string("schema_definition")?;
            let dataset_title = input.get_string("dataset_title")?;
            let owner_info = input.get_optional_string("owner_info")?;
            let dataset_description = input.get_optional_string("dataset_description")?;
            let client_token = input.get_optional_string("client_token")?;
            let alias = input.get_optional_string("alias")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .create_dataset()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("kind", kind.unwrap_or_default())
                .with_field("permission_group_params", permission_group_params.unwrap_or_default())
                .with_field("schema_definition", schema_definition.unwrap_or_default())
                .with_field("dataset_title", dataset_title.unwrap_or_default())
                .with_field("owner_info", owner_info.unwrap_or_default())
                .with_field("dataset_description", dataset_description.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("alias", alias.unwrap_or_default())
            )
        })
    }

    /// Read a dataset resource
    async fn read_dataset(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .describe_dataset()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a dataset resource
    async fn update_dataset(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let kind = input.get_string("kind")?;
            let permission_group_params = input.get_string("permission_group_params")?;
            let schema_definition = input.get_optional_string("schema_definition")?;
            let dataset_title = input.get_string("dataset_title")?;
            let owner_info = input.get_optional_string("owner_info")?;
            let dataset_description = input.get_optional_string("dataset_description")?;
            let client_token = input.get_optional_string("client_token")?;
            let alias = input.get_optional_string("alias")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .update_dataset()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("kind", kind.unwrap_or_default())
                .with_field("permission_group_params", permission_group_params.unwrap_or_default())
                .with_field("schema_definition", schema_definition.unwrap_or_default())
                .with_field("dataset_title", dataset_title.unwrap_or_default())
                .with_field("owner_info", owner_info.unwrap_or_default())
                .with_field("dataset_description", dataset_description.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("alias", alias.unwrap_or_default())
            )
        })
    }

    /// Delete a dataset resource
    async fn delete_dataset(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.finspace_data_client
            //     .delete_dataset()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Changeset resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a changeset resource
    async fn plan_changeset(
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

    /// Create a new changeset resource
    async fn create_changeset(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let dataset_id = input.get_string("dataset_id")?;
            let format_params = input.get_string("format_params")?;
            let source_params = input.get_string("source_params")?;
            let client_token = input.get_optional_string("client_token")?;
            let change_type = input.get_string("change_type")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .create_changeset()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("dataset_id", dataset_id.unwrap_or_default())
                .with_field("format_params", format_params.unwrap_or_default())
                .with_field("source_params", source_params.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("change_type", change_type.unwrap_or_default())
            )
        })
    }

    /// Read a changeset resource
    async fn read_changeset(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .describe_changeset()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a changeset resource
    async fn update_changeset(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let dataset_id = input.get_string("dataset_id")?;
            let format_params = input.get_string("format_params")?;
            let source_params = input.get_string("source_params")?;
            let client_token = input.get_optional_string("client_token")?;
            let change_type = input.get_string("change_type")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .update_changeset()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("dataset_id", dataset_id.unwrap_or_default())
                .with_field("format_params", format_params.unwrap_or_default())
                .with_field("source_params", source_params.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("change_type", change_type.unwrap_or_default())
            )
        })
    }

    /// Delete a changeset resource
    async fn delete_changeset(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.finspace_data_client
            //     .delete_changeset()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // User resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user resource
    async fn plan_user(
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

    /// Create a new user resource
    async fn create_user(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let first_name = input.get_optional_string("first_name")?;
            let api_access = input.get_optional_string("api_access")?;
            let r#type = input.get_string("type")?;
            let last_name = input.get_optional_string("last_name")?;
            let client_token = input.get_optional_string("client_token")?;
            let api_access_principal_arn = input.get_optional_string("api_access_principal_arn")?;
            let email_address = input.get_string("email_address")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .create_user()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("first_name", first_name.unwrap_or_default())
                .with_field("api_access", api_access.unwrap_or_default())
                .with_field("type", r#type.unwrap_or_default())
                .with_field("last_name", last_name.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("api_access_principal_arn", api_access_principal_arn.unwrap_or_default())
                .with_field("email_address", email_address.unwrap_or_default())
            )
        })
    }

    /// Read a user resource
    async fn read_user(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .describe_user()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a user resource
    async fn update_user(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let first_name = input.get_optional_string("first_name")?;
            let api_access = input.get_optional_string("api_access")?;
            let r#type = input.get_string("type")?;
            let last_name = input.get_optional_string("last_name")?;
            let client_token = input.get_optional_string("client_token")?;
            let api_access_principal_arn = input.get_optional_string("api_access_principal_arn")?;
            let email_address = input.get_string("email_address")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .update_user()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("first_name", first_name.unwrap_or_default())
                .with_field("api_access", api_access.unwrap_or_default())
                .with_field("type", r#type.unwrap_or_default())
                .with_field("last_name", last_name.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("api_access_principal_arn", api_access_principal_arn.unwrap_or_default())
                .with_field("email_address", email_address.unwrap_or_default())
            )
        })
    }

    /// Delete a user resource
    async fn delete_user(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.finspace_data_client
            //     .delete_user()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Data_view resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_view resource
    async fn plan_data_view(
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

    /// Create a new data_view resource
    async fn create_data_view(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let partition_columns = input.get_optional_string("partition_columns")?;
            let destination_type_params = input.get_string("destination_type_params")?;
            let as_of_timestamp = input.get_optional_string("as_of_timestamp")?;
            let dataset_id = input.get_string("dataset_id")?;
            let client_token = input.get_optional_string("client_token")?;
            let auto_update = input.get_optional_string("auto_update")?;
            let sort_columns = input.get_optional_string("sort_columns")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .create_data_view()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("partition_columns", partition_columns.unwrap_or_default())
                .with_field("destination_type_params", destination_type_params.unwrap_or_default())
                .with_field("as_of_timestamp", as_of_timestamp.unwrap_or_default())
                .with_field("dataset_id", dataset_id.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("auto_update", auto_update.unwrap_or_default())
                .with_field("sort_columns", sort_columns.unwrap_or_default())
            )
        })
    }

    /// Read a data_view resource
    async fn read_data_view(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .describe_data_view()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a data_view resource
    async fn update_data_view(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let partition_columns = input.get_optional_string("partition_columns")?;
            let destination_type_params = input.get_string("destination_type_params")?;
            let as_of_timestamp = input.get_optional_string("as_of_timestamp")?;
            let dataset_id = input.get_string("dataset_id")?;
            let client_token = input.get_optional_string("client_token")?;
            let auto_update = input.get_optional_string("auto_update")?;
            let sort_columns = input.get_optional_string("sort_columns")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .update_data_view()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("partition_columns", partition_columns.unwrap_or_default())
                .with_field("destination_type_params", destination_type_params.unwrap_or_default())
                .with_field("as_of_timestamp", as_of_timestamp.unwrap_or_default())
                .with_field("dataset_id", dataset_id.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("auto_update", auto_update.unwrap_or_default())
                .with_field("sort_columns", sort_columns.unwrap_or_default())
            )
        })
    }

    /// Delete a data_view resource
    async fn delete_data_view(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.finspace_data_client
            //     .delete_data_view()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Working_location resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a working_location resource
    async fn plan_working_location(
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

    /// Create a new working_location resource
    async fn create_working_location(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .create_working_location()
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

    /// Read a working_location resource
    async fn read_working_location(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .describe_working_location()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a working_location resource
    async fn update_working_location(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .update_working_location()
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

    /// Delete a working_location resource
    async fn delete_working_location(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.finspace_data_client
            //     .delete_working_location()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Programmatic_access_credentials resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a programmatic_access_credentials resource
    async fn plan_programmatic_access_credentials(
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

    /// Create a new programmatic_access_credentials resource
    async fn create_programmatic_access_credentials(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .create_programmatic_access_credentials()
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

    /// Read a programmatic_access_credentials resource
    async fn read_programmatic_access_credentials(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .describe_programmatic_access_credentials()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a programmatic_access_credentials resource
    async fn update_programmatic_access_credentials(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .update_programmatic_access_credentials()
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

    /// Delete a programmatic_access_credentials resource
    async fn delete_programmatic_access_credentials(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.finspace_data_client
            //     .delete_programmatic_access_credentials()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Permission_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a permission_group resource
    async fn plan_permission_group(
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

    /// Create a new permission_group resource
    async fn create_permission_group(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let application_permissions = input.get_string("application_permissions")?;
            let name = input.get_string("name")?;
            let client_token = input.get_optional_string("client_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .create_permission_group()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("description", description.unwrap_or_default())
                .with_field("application_permissions", application_permissions.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
            )
        })
    }

    /// Read a permission_group resource
    async fn read_permission_group(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .describe_permission_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a permission_group resource
    async fn update_permission_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let application_permissions = input.get_string("application_permissions")?;
            let name = input.get_string("name")?;
            let client_token = input.get_optional_string("client_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.finspace_data_client
            //     .update_permission_group()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("description", description.unwrap_or_default())
                .with_field("application_permissions", application_permissions.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
            )
        })
    }

    /// Delete a permission_group resource
    async fn delete_permission_group(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.finspace_data_client
            //     .delete_permission_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
