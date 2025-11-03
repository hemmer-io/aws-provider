//! Comprehend service for Aws provider
//!
//! This module handles all comprehend resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Comprehend service handler
pub struct ComprehendService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ComprehendService<'a> {
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
            "document_classification_job" => {
                self.plan_document_classification_job(current_state, desired_input).await
            }
            "entity_recognizer" => {
                self.plan_entity_recognizer(current_state, desired_input).await
            }
            "events_detection_job" => {
                self.plan_events_detection_job(current_state, desired_input).await
            }
            "dominant_language_detection_job" => {
                self.plan_dominant_language_detection_job(current_state, desired_input).await
            }
            "sentiment_detection_job" => {
                self.plan_sentiment_detection_job(current_state, desired_input).await
            }
            "endpoint" => {
                self.plan_endpoint(current_state, desired_input).await
            }
            "flywheel" => {
                self.plan_flywheel(current_state, desired_input).await
            }
            "topics_detection_job" => {
                self.plan_topics_detection_job(current_state, desired_input).await
            }
            "dataset" => {
                self.plan_dataset(current_state, desired_input).await
            }
            "entities_detection_job" => {
                self.plan_entities_detection_job(current_state, desired_input).await
            }
            "document_classifier" => {
                self.plan_document_classifier(current_state, desired_input).await
            }
            "pii_entities_detection_job" => {
                self.plan_pii_entities_detection_job(current_state, desired_input).await
            }
            "key_phrases_detection_job" => {
                self.plan_key_phrases_detection_job(current_state, desired_input).await
            }
            "resource_policy" => {
                self.plan_resource_policy(current_state, desired_input).await
            }
            "targeted_sentiment_detection_job" => {
                self.plan_targeted_sentiment_detection_job(current_state, desired_input).await
            }
            "flywheel_iteration" => {
                self.plan_flywheel_iteration(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "comprehend",
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
            "document_classification_job" => {
                self.create_document_classification_job(input).await
            }
            "entity_recognizer" => {
                self.create_entity_recognizer(input).await
            }
            "events_detection_job" => {
                self.create_events_detection_job(input).await
            }
            "dominant_language_detection_job" => {
                self.create_dominant_language_detection_job(input).await
            }
            "sentiment_detection_job" => {
                self.create_sentiment_detection_job(input).await
            }
            "endpoint" => {
                self.create_endpoint(input).await
            }
            "flywheel" => {
                self.create_flywheel(input).await
            }
            "topics_detection_job" => {
                self.create_topics_detection_job(input).await
            }
            "dataset" => {
                self.create_dataset(input).await
            }
            "entities_detection_job" => {
                self.create_entities_detection_job(input).await
            }
            "document_classifier" => {
                self.create_document_classifier(input).await
            }
            "pii_entities_detection_job" => {
                self.create_pii_entities_detection_job(input).await
            }
            "key_phrases_detection_job" => {
                self.create_key_phrases_detection_job(input).await
            }
            "resource_policy" => {
                self.create_resource_policy(input).await
            }
            "targeted_sentiment_detection_job" => {
                self.create_targeted_sentiment_detection_job(input).await
            }
            "flywheel_iteration" => {
                self.create_flywheel_iteration(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "comprehend",
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
            "document_classification_job" => {
                self.read_document_classification_job(id).await
            }
            "entity_recognizer" => {
                self.read_entity_recognizer(id).await
            }
            "events_detection_job" => {
                self.read_events_detection_job(id).await
            }
            "dominant_language_detection_job" => {
                self.read_dominant_language_detection_job(id).await
            }
            "sentiment_detection_job" => {
                self.read_sentiment_detection_job(id).await
            }
            "endpoint" => {
                self.read_endpoint(id).await
            }
            "flywheel" => {
                self.read_flywheel(id).await
            }
            "topics_detection_job" => {
                self.read_topics_detection_job(id).await
            }
            "dataset" => {
                self.read_dataset(id).await
            }
            "entities_detection_job" => {
                self.read_entities_detection_job(id).await
            }
            "document_classifier" => {
                self.read_document_classifier(id).await
            }
            "pii_entities_detection_job" => {
                self.read_pii_entities_detection_job(id).await
            }
            "key_phrases_detection_job" => {
                self.read_key_phrases_detection_job(id).await
            }
            "resource_policy" => {
                self.read_resource_policy(id).await
            }
            "targeted_sentiment_detection_job" => {
                self.read_targeted_sentiment_detection_job(id).await
            }
            "flywheel_iteration" => {
                self.read_flywheel_iteration(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "comprehend",
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
            "document_classification_job" => {
                self.update_document_classification_job(id, input).await
            }
            "entity_recognizer" => {
                self.update_entity_recognizer(id, input).await
            }
            "events_detection_job" => {
                self.update_events_detection_job(id, input).await
            }
            "dominant_language_detection_job" => {
                self.update_dominant_language_detection_job(id, input).await
            }
            "sentiment_detection_job" => {
                self.update_sentiment_detection_job(id, input).await
            }
            "endpoint" => {
                self.update_endpoint(id, input).await
            }
            "flywheel" => {
                self.update_flywheel(id, input).await
            }
            "topics_detection_job" => {
                self.update_topics_detection_job(id, input).await
            }
            "dataset" => {
                self.update_dataset(id, input).await
            }
            "entities_detection_job" => {
                self.update_entities_detection_job(id, input).await
            }
            "document_classifier" => {
                self.update_document_classifier(id, input).await
            }
            "pii_entities_detection_job" => {
                self.update_pii_entities_detection_job(id, input).await
            }
            "key_phrases_detection_job" => {
                self.update_key_phrases_detection_job(id, input).await
            }
            "resource_policy" => {
                self.update_resource_policy(id, input).await
            }
            "targeted_sentiment_detection_job" => {
                self.update_targeted_sentiment_detection_job(id, input).await
            }
            "flywheel_iteration" => {
                self.update_flywheel_iteration(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "comprehend",
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
            "document_classification_job" => {
                self.delete_document_classification_job(id).await
            }
            "entity_recognizer" => {
                self.delete_entity_recognizer(id).await
            }
            "events_detection_job" => {
                self.delete_events_detection_job(id).await
            }
            "dominant_language_detection_job" => {
                self.delete_dominant_language_detection_job(id).await
            }
            "sentiment_detection_job" => {
                self.delete_sentiment_detection_job(id).await
            }
            "endpoint" => {
                self.delete_endpoint(id).await
            }
            "flywheel" => {
                self.delete_flywheel(id).await
            }
            "topics_detection_job" => {
                self.delete_topics_detection_job(id).await
            }
            "dataset" => {
                self.delete_dataset(id).await
            }
            "entities_detection_job" => {
                self.delete_entities_detection_job(id).await
            }
            "document_classifier" => {
                self.delete_document_classifier(id).await
            }
            "pii_entities_detection_job" => {
                self.delete_pii_entities_detection_job(id).await
            }
            "key_phrases_detection_job" => {
                self.delete_key_phrases_detection_job(id).await
            }
            "resource_policy" => {
                self.delete_resource_policy(id).await
            }
            "targeted_sentiment_detection_job" => {
                self.delete_targeted_sentiment_detection_job(id).await
            }
            "flywheel_iteration" => {
                self.delete_flywheel_iteration(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "comprehend",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Document_classification_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document_classification_job resource
    async fn plan_document_classification_job(
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

    /// Create a new document_classification_job resource
    async fn create_document_classification_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_document_classification_job()
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

    /// Read a document_classification_job resource
    async fn read_document_classification_job(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_document_classification_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a document_classification_job resource
    async fn update_document_classification_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_document_classification_job()
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

    /// Delete a document_classification_job resource
    async fn delete_document_classification_job(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_document_classification_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Entity_recognizer resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a entity_recognizer resource
    async fn plan_entity_recognizer(
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

    /// Create a new entity_recognizer resource
    async fn create_entity_recognizer(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let data_access_role_arn = input.get_string("data_access_role_arn")?;
            let recognizer_name = input.get_string("recognizer_name")?;
            let model_kms_key_id = input.get_optional_string("model_kms_key_id")?;
            let client_request_token = input.get_optional_string("client_request_token")?;
            let vpc_config = input.get_optional_string("vpc_config")?;
            let volume_kms_key_id = input.get_optional_string("volume_kms_key_id")?;
            let input_data_config = input.get_string("input_data_config")?;
            let model_policy = input.get_optional_string("model_policy")?;
            let version_name = input.get_optional_string("version_name")?;
            let tags = input.get_optional_string("tags")?;
            let language_code = input.get_string("language_code")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_entity_recognizer()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("data_access_role_arn", data_access_role_arn.unwrap_or_default())
                .with_field("recognizer_name", recognizer_name.unwrap_or_default())
                .with_field("model_kms_key_id", model_kms_key_id.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("vpc_config", vpc_config.unwrap_or_default())
                .with_field("volume_kms_key_id", volume_kms_key_id.unwrap_or_default())
                .with_field("input_data_config", input_data_config.unwrap_or_default())
                .with_field("model_policy", model_policy.unwrap_or_default())
                .with_field("version_name", version_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("language_code", language_code.unwrap_or_default())
            )
        })
    }

    /// Read a entity_recognizer resource
    async fn read_entity_recognizer(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_entity_recognizer()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a entity_recognizer resource
    async fn update_entity_recognizer(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let data_access_role_arn = input.get_string("data_access_role_arn")?;
            let recognizer_name = input.get_string("recognizer_name")?;
            let model_kms_key_id = input.get_optional_string("model_kms_key_id")?;
            let client_request_token = input.get_optional_string("client_request_token")?;
            let vpc_config = input.get_optional_string("vpc_config")?;
            let volume_kms_key_id = input.get_optional_string("volume_kms_key_id")?;
            let input_data_config = input.get_string("input_data_config")?;
            let model_policy = input.get_optional_string("model_policy")?;
            let version_name = input.get_optional_string("version_name")?;
            let tags = input.get_optional_string("tags")?;
            let language_code = input.get_string("language_code")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_entity_recognizer()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("data_access_role_arn", data_access_role_arn.unwrap_or_default())
                .with_field("recognizer_name", recognizer_name.unwrap_or_default())
                .with_field("model_kms_key_id", model_kms_key_id.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("vpc_config", vpc_config.unwrap_or_default())
                .with_field("volume_kms_key_id", volume_kms_key_id.unwrap_or_default())
                .with_field("input_data_config", input_data_config.unwrap_or_default())
                .with_field("model_policy", model_policy.unwrap_or_default())
                .with_field("version_name", version_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("language_code", language_code.unwrap_or_default())
            )
        })
    }

    /// Delete a entity_recognizer resource
    async fn delete_entity_recognizer(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_entity_recognizer()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Events_detection_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a events_detection_job resource
    async fn plan_events_detection_job(
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

    /// Create a new events_detection_job resource
    async fn create_events_detection_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_events_detection_job()
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

    /// Read a events_detection_job resource
    async fn read_events_detection_job(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_events_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a events_detection_job resource
    async fn update_events_detection_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_events_detection_job()
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

    /// Delete a events_detection_job resource
    async fn delete_events_detection_job(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_events_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Dominant_language_detection_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dominant_language_detection_job resource
    async fn plan_dominant_language_detection_job(
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

    /// Create a new dominant_language_detection_job resource
    async fn create_dominant_language_detection_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_dominant_language_detection_job()
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

    /// Read a dominant_language_detection_job resource
    async fn read_dominant_language_detection_job(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_dominant_language_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a dominant_language_detection_job resource
    async fn update_dominant_language_detection_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_dominant_language_detection_job()
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

    /// Delete a dominant_language_detection_job resource
    async fn delete_dominant_language_detection_job(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_dominant_language_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Sentiment_detection_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sentiment_detection_job resource
    async fn plan_sentiment_detection_job(
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

    /// Create a new sentiment_detection_job resource
    async fn create_sentiment_detection_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_sentiment_detection_job()
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

    /// Read a sentiment_detection_job resource
    async fn read_sentiment_detection_job(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_sentiment_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a sentiment_detection_job resource
    async fn update_sentiment_detection_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_sentiment_detection_job()
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

    /// Delete a sentiment_detection_job resource
    async fn delete_sentiment_detection_job(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_sentiment_detection_job()
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
            let desired_inference_units = input.get_string("desired_inference_units")?;
            let client_request_token = input.get_optional_string("client_request_token")?;
            let data_access_role_arn = input.get_optional_string("data_access_role_arn")?;
            let flywheel_arn = input.get_optional_string("flywheel_arn")?;
            let model_arn = input.get_optional_string("model_arn")?;
            let endpoint_name = input.get_string("endpoint_name")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_endpoint()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("desired_inference_units", desired_inference_units.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("data_access_role_arn", data_access_role_arn.unwrap_or_default())
                .with_field("flywheel_arn", flywheel_arn.unwrap_or_default())
                .with_field("model_arn", model_arn.unwrap_or_default())
                .with_field("endpoint_name", endpoint_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
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
            // let result = self.provider.comprehend_client
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
            let desired_inference_units = input.get_string("desired_inference_units")?;
            let client_request_token = input.get_optional_string("client_request_token")?;
            let data_access_role_arn = input.get_optional_string("data_access_role_arn")?;
            let flywheel_arn = input.get_optional_string("flywheel_arn")?;
            let model_arn = input.get_optional_string("model_arn")?;
            let endpoint_name = input.get_string("endpoint_name")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_endpoint()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("desired_inference_units", desired_inference_units.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("data_access_role_arn", data_access_role_arn.unwrap_or_default())
                .with_field("flywheel_arn", flywheel_arn.unwrap_or_default())
                .with_field("model_arn", model_arn.unwrap_or_default())
                .with_field("endpoint_name", endpoint_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
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
            // self.provider.comprehend_client
            //     .delete_endpoint()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Flywheel resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a flywheel resource
    async fn plan_flywheel(
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

    /// Create a new flywheel resource
    async fn create_flywheel(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let active_model_arn = input.get_optional_string("active_model_arn")?;
            let client_request_token = input.get_optional_string("client_request_token")?;
            let data_access_role_arn = input.get_string("data_access_role_arn")?;
            let data_lake_s3_uri = input.get_string("data_lake_s3_uri")?;
            let tags = input.get_optional_string("tags")?;
            let flywheel_name = input.get_string("flywheel_name")?;
            let task_config = input.get_optional_string("task_config")?;
            let data_security_config = input.get_optional_string("data_security_config")?;
            let model_type = input.get_optional_string("model_type")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_flywheel()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("active_model_arn", active_model_arn.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("data_access_role_arn", data_access_role_arn.unwrap_or_default())
                .with_field("data_lake_s3_uri", data_lake_s3_uri.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("flywheel_name", flywheel_name.unwrap_or_default())
                .with_field("task_config", task_config.unwrap_or_default())
                .with_field("data_security_config", data_security_config.unwrap_or_default())
                .with_field("model_type", model_type.unwrap_or_default())
            )
        })
    }

    /// Read a flywheel resource
    async fn read_flywheel(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_flywheel()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a flywheel resource
    async fn update_flywheel(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let active_model_arn = input.get_optional_string("active_model_arn")?;
            let client_request_token = input.get_optional_string("client_request_token")?;
            let data_access_role_arn = input.get_string("data_access_role_arn")?;
            let data_lake_s3_uri = input.get_string("data_lake_s3_uri")?;
            let tags = input.get_optional_string("tags")?;
            let flywheel_name = input.get_string("flywheel_name")?;
            let task_config = input.get_optional_string("task_config")?;
            let data_security_config = input.get_optional_string("data_security_config")?;
            let model_type = input.get_optional_string("model_type")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_flywheel()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("active_model_arn", active_model_arn.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("data_access_role_arn", data_access_role_arn.unwrap_or_default())
                .with_field("data_lake_s3_uri", data_lake_s3_uri.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("flywheel_name", flywheel_name.unwrap_or_default())
                .with_field("task_config", task_config.unwrap_or_default())
                .with_field("data_security_config", data_security_config.unwrap_or_default())
                .with_field("model_type", model_type.unwrap_or_default())
            )
        })
    }

    /// Delete a flywheel resource
    async fn delete_flywheel(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_flywheel()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Topics_detection_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a topics_detection_job resource
    async fn plan_topics_detection_job(
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

    /// Create a new topics_detection_job resource
    async fn create_topics_detection_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_topics_detection_job()
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

    /// Read a topics_detection_job resource
    async fn read_topics_detection_job(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_topics_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a topics_detection_job resource
    async fn update_topics_detection_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_topics_detection_job()
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

    /// Delete a topics_detection_job resource
    async fn delete_topics_detection_job(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_topics_detection_job()
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
            let tags = input.get_optional_string("tags")?;
            let dataset_name = input.get_string("dataset_name")?;
            let dataset_type = input.get_optional_string("dataset_type")?;
            let description = input.get_optional_string("description")?;
            let input_data_config = input.get_string("input_data_config")?;
            let flywheel_arn = input.get_string("flywheel_arn")?;
            let client_request_token = input.get_optional_string("client_request_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_dataset()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("tags", tags.unwrap_or_default())
                .with_field("dataset_name", dataset_name.unwrap_or_default())
                .with_field("dataset_type", dataset_type.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("input_data_config", input_data_config.unwrap_or_default())
                .with_field("flywheel_arn", flywheel_arn.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
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
            // let result = self.provider.comprehend_client
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
            let tags = input.get_optional_string("tags")?;
            let dataset_name = input.get_string("dataset_name")?;
            let dataset_type = input.get_optional_string("dataset_type")?;
            let description = input.get_optional_string("description")?;
            let input_data_config = input.get_string("input_data_config")?;
            let flywheel_arn = input.get_string("flywheel_arn")?;
            let client_request_token = input.get_optional_string("client_request_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_dataset()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("tags", tags.unwrap_or_default())
                .with_field("dataset_name", dataset_name.unwrap_or_default())
                .with_field("dataset_type", dataset_type.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("input_data_config", input_data_config.unwrap_or_default())
                .with_field("flywheel_arn", flywheel_arn.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
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
            // self.provider.comprehend_client
            //     .delete_dataset()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Entities_detection_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a entities_detection_job resource
    async fn plan_entities_detection_job(
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

    /// Create a new entities_detection_job resource
    async fn create_entities_detection_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_entities_detection_job()
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

    /// Read a entities_detection_job resource
    async fn read_entities_detection_job(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_entities_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a entities_detection_job resource
    async fn update_entities_detection_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_entities_detection_job()
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

    /// Delete a entities_detection_job resource
    async fn delete_entities_detection_job(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_entities_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Document_classifier resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document_classifier resource
    async fn plan_document_classifier(
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

    /// Create a new document_classifier resource
    async fn create_document_classifier(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let vpc_config = input.get_optional_string("vpc_config")?;
            let mode = input.get_optional_string("mode")?;
            let input_data_config = input.get_string("input_data_config")?;
            let model_kms_key_id = input.get_optional_string("model_kms_key_id")?;
            let data_access_role_arn = input.get_string("data_access_role_arn")?;
            let language_code = input.get_string("language_code")?;
            let model_policy = input.get_optional_string("model_policy")?;
            let output_data_config = input.get_optional_string("output_data_config")?;
            let tags = input.get_optional_string("tags")?;
            let version_name = input.get_optional_string("version_name")?;
            let volume_kms_key_id = input.get_optional_string("volume_kms_key_id")?;
            let document_classifier_name = input.get_string("document_classifier_name")?;
            let client_request_token = input.get_optional_string("client_request_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_document_classifier()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("vpc_config", vpc_config.unwrap_or_default())
                .with_field("mode", mode.unwrap_or_default())
                .with_field("input_data_config", input_data_config.unwrap_or_default())
                .with_field("model_kms_key_id", model_kms_key_id.unwrap_or_default())
                .with_field("data_access_role_arn", data_access_role_arn.unwrap_or_default())
                .with_field("language_code", language_code.unwrap_or_default())
                .with_field("model_policy", model_policy.unwrap_or_default())
                .with_field("output_data_config", output_data_config.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("version_name", version_name.unwrap_or_default())
                .with_field("volume_kms_key_id", volume_kms_key_id.unwrap_or_default())
                .with_field("document_classifier_name", document_classifier_name.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
            )
        })
    }

    /// Read a document_classifier resource
    async fn read_document_classifier(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_document_classifier()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a document_classifier resource
    async fn update_document_classifier(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let vpc_config = input.get_optional_string("vpc_config")?;
            let mode = input.get_optional_string("mode")?;
            let input_data_config = input.get_string("input_data_config")?;
            let model_kms_key_id = input.get_optional_string("model_kms_key_id")?;
            let data_access_role_arn = input.get_string("data_access_role_arn")?;
            let language_code = input.get_string("language_code")?;
            let model_policy = input.get_optional_string("model_policy")?;
            let output_data_config = input.get_optional_string("output_data_config")?;
            let tags = input.get_optional_string("tags")?;
            let version_name = input.get_optional_string("version_name")?;
            let volume_kms_key_id = input.get_optional_string("volume_kms_key_id")?;
            let document_classifier_name = input.get_string("document_classifier_name")?;
            let client_request_token = input.get_optional_string("client_request_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_document_classifier()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("vpc_config", vpc_config.unwrap_or_default())
                .with_field("mode", mode.unwrap_or_default())
                .with_field("input_data_config", input_data_config.unwrap_or_default())
                .with_field("model_kms_key_id", model_kms_key_id.unwrap_or_default())
                .with_field("data_access_role_arn", data_access_role_arn.unwrap_or_default())
                .with_field("language_code", language_code.unwrap_or_default())
                .with_field("model_policy", model_policy.unwrap_or_default())
                .with_field("output_data_config", output_data_config.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("version_name", version_name.unwrap_or_default())
                .with_field("volume_kms_key_id", volume_kms_key_id.unwrap_or_default())
                .with_field("document_classifier_name", document_classifier_name.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
            )
        })
    }

    /// Delete a document_classifier resource
    async fn delete_document_classifier(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_document_classifier()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Pii_entities_detection_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a pii_entities_detection_job resource
    async fn plan_pii_entities_detection_job(
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

    /// Create a new pii_entities_detection_job resource
    async fn create_pii_entities_detection_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_pii_entities_detection_job()
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

    /// Read a pii_entities_detection_job resource
    async fn read_pii_entities_detection_job(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_pii_entities_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a pii_entities_detection_job resource
    async fn update_pii_entities_detection_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_pii_entities_detection_job()
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

    /// Delete a pii_entities_detection_job resource
    async fn delete_pii_entities_detection_job(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_pii_entities_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Key_phrases_detection_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a key_phrases_detection_job resource
    async fn plan_key_phrases_detection_job(
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

    /// Create a new key_phrases_detection_job resource
    async fn create_key_phrases_detection_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_key_phrases_detection_job()
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

    /// Read a key_phrases_detection_job resource
    async fn read_key_phrases_detection_job(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_key_phrases_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a key_phrases_detection_job resource
    async fn update_key_phrases_detection_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_key_phrases_detection_job()
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

    /// Delete a key_phrases_detection_job resource
    async fn delete_key_phrases_detection_job(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_key_phrases_detection_job()
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
            let resource_arn = input.get_string("resource_arn")?;
            let resource_policy = input.get_string("resource_policy")?;
            let policy_revision_id = input.get_optional_string("policy_revision_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_resource_policy()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("resource_arn", resource_arn.unwrap_or_default())
                .with_field("resource_policy", resource_policy.unwrap_or_default())
                .with_field("policy_revision_id", policy_revision_id.unwrap_or_default())
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
            // let result = self.provider.comprehend_client
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
            let resource_arn = input.get_string("resource_arn")?;
            let resource_policy = input.get_string("resource_policy")?;
            let policy_revision_id = input.get_optional_string("policy_revision_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_resource_policy()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("resource_arn", resource_arn.unwrap_or_default())
                .with_field("resource_policy", resource_policy.unwrap_or_default())
                .with_field("policy_revision_id", policy_revision_id.unwrap_or_default())
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
            // self.provider.comprehend_client
            //     .delete_resource_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Targeted_sentiment_detection_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a targeted_sentiment_detection_job resource
    async fn plan_targeted_sentiment_detection_job(
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

    /// Create a new targeted_sentiment_detection_job resource
    async fn create_targeted_sentiment_detection_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_targeted_sentiment_detection_job()
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

    /// Read a targeted_sentiment_detection_job resource
    async fn read_targeted_sentiment_detection_job(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_targeted_sentiment_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a targeted_sentiment_detection_job resource
    async fn update_targeted_sentiment_detection_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_targeted_sentiment_detection_job()
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

    /// Delete a targeted_sentiment_detection_job resource
    async fn delete_targeted_sentiment_detection_job(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_targeted_sentiment_detection_job()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Flywheel_iteration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a flywheel_iteration resource
    async fn plan_flywheel_iteration(
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

    /// Create a new flywheel_iteration resource
    async fn create_flywheel_iteration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .create_flywheel_iteration()
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

    /// Read a flywheel_iteration resource
    async fn read_flywheel_iteration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .describe_flywheel_iteration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a flywheel_iteration resource
    async fn update_flywheel_iteration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.comprehend_client
            //     .update_flywheel_iteration()
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

    /// Delete a flywheel_iteration resource
    async fn delete_flywheel_iteration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.comprehend_client
            //     .delete_flywheel_iteration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
