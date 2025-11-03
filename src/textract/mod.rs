//! Textract service for Aws provider
//!
//! This module handles all textract resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Textract service handler
pub struct TextractService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> TextractService<'a> {
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
            "adapter_version" => {
                self.plan_adapter_version(current_state, desired_input).await
            }
            "lending_analysis_summary" => {
                self.plan_lending_analysis_summary(current_state, desired_input).await
            }
            "lending_analysis" => {
                self.plan_lending_analysis(current_state, desired_input).await
            }
            "document_text_detection" => {
                self.plan_document_text_detection(current_state, desired_input).await
            }
            "adapter" => {
                self.plan_adapter(current_state, desired_input).await
            }
            "document_analysis" => {
                self.plan_document_analysis(current_state, desired_input).await
            }
            "expense_analysis" => {
                self.plan_expense_analysis(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "textract",
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
            "adapter_version" => {
                self.create_adapter_version(input).await
            }
            "lending_analysis_summary" => {
                self.create_lending_analysis_summary(input).await
            }
            "lending_analysis" => {
                self.create_lending_analysis(input).await
            }
            "document_text_detection" => {
                self.create_document_text_detection(input).await
            }
            "adapter" => {
                self.create_adapter(input).await
            }
            "document_analysis" => {
                self.create_document_analysis(input).await
            }
            "expense_analysis" => {
                self.create_expense_analysis(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "textract",
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
            "adapter_version" => {
                self.read_adapter_version(id).await
            }
            "lending_analysis_summary" => {
                self.read_lending_analysis_summary(id).await
            }
            "lending_analysis" => {
                self.read_lending_analysis(id).await
            }
            "document_text_detection" => {
                self.read_document_text_detection(id).await
            }
            "adapter" => {
                self.read_adapter(id).await
            }
            "document_analysis" => {
                self.read_document_analysis(id).await
            }
            "expense_analysis" => {
                self.read_expense_analysis(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "textract",
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
            "adapter_version" => {
                self.update_adapter_version(id, input).await
            }
            "lending_analysis_summary" => {
                self.update_lending_analysis_summary(id, input).await
            }
            "lending_analysis" => {
                self.update_lending_analysis(id, input).await
            }
            "document_text_detection" => {
                self.update_document_text_detection(id, input).await
            }
            "adapter" => {
                self.update_adapter(id, input).await
            }
            "document_analysis" => {
                self.update_document_analysis(id, input).await
            }
            "expense_analysis" => {
                self.update_expense_analysis(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "textract",
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
            "adapter_version" => {
                self.delete_adapter_version(id).await
            }
            "lending_analysis_summary" => {
                self.delete_lending_analysis_summary(id).await
            }
            "lending_analysis" => {
                self.delete_lending_analysis(id).await
            }
            "document_text_detection" => {
                self.delete_document_text_detection(id).await
            }
            "adapter" => {
                self.delete_adapter(id).await
            }
            "document_analysis" => {
                self.delete_document_analysis(id).await
            }
            "expense_analysis" => {
                self.delete_expense_analysis(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "textract",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Adapter_version resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a adapter_version resource
    async fn plan_adapter_version(
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

    /// Create a new adapter_version resource
    async fn create_adapter_version(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let client_request_token = input.get_optional_string("client_request_token")?;
            let adapter_id = input.get_string("adapter_id")?;
            let dataset_config = input.get_string("dataset_config")?;
            let kms_key_id = input.get_optional_string("kms_key_id")?;
            let tags = input.get_optional_string("tags")?;
            let output_config = input.get_string("output_config")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.textract_client
            //     .create_adapter_version()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("adapter_id", adapter_id.unwrap_or_default())
                .with_field("dataset_config", dataset_config.unwrap_or_default())
                .with_field("kms_key_id", kms_key_id.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("output_config", output_config.unwrap_or_default())
            )
        })
    }

    /// Read a adapter_version resource
    async fn read_adapter_version(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.textract_client
            //     .describe_adapter_version()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a adapter_version resource
    async fn update_adapter_version(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let client_request_token = input.get_optional_string("client_request_token")?;
            let adapter_id = input.get_string("adapter_id")?;
            let dataset_config = input.get_string("dataset_config")?;
            let kms_key_id = input.get_optional_string("kms_key_id")?;
            let tags = input.get_optional_string("tags")?;
            let output_config = input.get_string("output_config")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.textract_client
            //     .update_adapter_version()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("adapter_id", adapter_id.unwrap_or_default())
                .with_field("dataset_config", dataset_config.unwrap_or_default())
                .with_field("kms_key_id", kms_key_id.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("output_config", output_config.unwrap_or_default())
            )
        })
    }

    /// Delete a adapter_version resource
    async fn delete_adapter_version(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.textract_client
            //     .delete_adapter_version()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Lending_analysis_summary resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a lending_analysis_summary resource
    async fn plan_lending_analysis_summary(
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

    /// Create a new lending_analysis_summary resource
    async fn create_lending_analysis_summary(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.textract_client
            //     .create_lending_analysis_summary()
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

    /// Read a lending_analysis_summary resource
    async fn read_lending_analysis_summary(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.textract_client
            //     .describe_lending_analysis_summary()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a lending_analysis_summary resource
    async fn update_lending_analysis_summary(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.textract_client
            //     .update_lending_analysis_summary()
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

    /// Delete a lending_analysis_summary resource
    async fn delete_lending_analysis_summary(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.textract_client
            //     .delete_lending_analysis_summary()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Lending_analysis resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a lending_analysis resource
    async fn plan_lending_analysis(
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

    /// Create a new lending_analysis resource
    async fn create_lending_analysis(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.textract_client
            //     .create_lending_analysis()
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

    /// Read a lending_analysis resource
    async fn read_lending_analysis(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.textract_client
            //     .describe_lending_analysis()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a lending_analysis resource
    async fn update_lending_analysis(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.textract_client
            //     .update_lending_analysis()
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

    /// Delete a lending_analysis resource
    async fn delete_lending_analysis(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.textract_client
            //     .delete_lending_analysis()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Document_text_detection resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document_text_detection resource
    async fn plan_document_text_detection(
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

    /// Create a new document_text_detection resource
    async fn create_document_text_detection(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.textract_client
            //     .create_document_text_detection()
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

    /// Read a document_text_detection resource
    async fn read_document_text_detection(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.textract_client
            //     .describe_document_text_detection()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a document_text_detection resource
    async fn update_document_text_detection(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.textract_client
            //     .update_document_text_detection()
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

    /// Delete a document_text_detection resource
    async fn delete_document_text_detection(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.textract_client
            //     .delete_document_text_detection()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Adapter resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a adapter resource
    async fn plan_adapter(
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

    /// Create a new adapter resource
    async fn create_adapter(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let client_request_token = input.get_optional_string("client_request_token")?;
            let feature_types = input.get_string("feature_types")?;
            let description = input.get_optional_string("description")?;
            let adapter_name = input.get_string("adapter_name")?;
            let tags = input.get_optional_string("tags")?;
            let auto_update = input.get_optional_string("auto_update")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.textract_client
            //     .create_adapter()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("feature_types", feature_types.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("adapter_name", adapter_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("auto_update", auto_update.unwrap_or_default())
            )
        })
    }

    /// Read a adapter resource
    async fn read_adapter(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.textract_client
            //     .describe_adapter()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a adapter resource
    async fn update_adapter(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let client_request_token = input.get_optional_string("client_request_token")?;
            let feature_types = input.get_string("feature_types")?;
            let description = input.get_optional_string("description")?;
            let adapter_name = input.get_string("adapter_name")?;
            let tags = input.get_optional_string("tags")?;
            let auto_update = input.get_optional_string("auto_update")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.textract_client
            //     .update_adapter()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("feature_types", feature_types.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("adapter_name", adapter_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("auto_update", auto_update.unwrap_or_default())
            )
        })
    }

    /// Delete a adapter resource
    async fn delete_adapter(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.textract_client
            //     .delete_adapter()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Document_analysis resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document_analysis resource
    async fn plan_document_analysis(
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

    /// Create a new document_analysis resource
    async fn create_document_analysis(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.textract_client
            //     .create_document_analysis()
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

    /// Read a document_analysis resource
    async fn read_document_analysis(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.textract_client
            //     .describe_document_analysis()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a document_analysis resource
    async fn update_document_analysis(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.textract_client
            //     .update_document_analysis()
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

    /// Delete a document_analysis resource
    async fn delete_document_analysis(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.textract_client
            //     .delete_document_analysis()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Expense_analysis resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a expense_analysis resource
    async fn plan_expense_analysis(
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

    /// Create a new expense_analysis resource
    async fn create_expense_analysis(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.textract_client
            //     .create_expense_analysis()
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

    /// Read a expense_analysis resource
    async fn read_expense_analysis(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.textract_client
            //     .describe_expense_analysis()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a expense_analysis resource
    async fn update_expense_analysis(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.textract_client
            //     .update_expense_analysis()
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

    /// Delete a expense_analysis resource
    async fn delete_expense_analysis(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.textract_client
            //     .delete_expense_analysis()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
