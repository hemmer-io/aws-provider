//! Ivs service for Aws provider
//!
//! This module handles all ivs resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Ivs service handler
pub struct IvsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> IvsService<'a> {
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
            "metadata" => {
                self.plan_metadata(current_state, desired_input).await
            }
            "recording_configuration" => {
                self.plan_recording_configuration(current_state, desired_input).await
            }
            "playback_key_pair" => {
                self.plan_playback_key_pair(current_state, desired_input).await
            }
            "stream" => {
                self.plan_stream(current_state, desired_input).await
            }
            "stream_session" => {
                self.plan_stream_session(current_state, desired_input).await
            }
            "playback_restriction_policy" => {
                self.plan_playback_restriction_policy(current_state, desired_input).await
            }
            "channel" => {
                self.plan_channel(current_state, desired_input).await
            }
            "stream_key" => {
                self.plan_stream_key(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ivs",
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
            "metadata" => {
                self.create_metadata(input).await
            }
            "recording_configuration" => {
                self.create_recording_configuration(input).await
            }
            "playback_key_pair" => {
                self.create_playback_key_pair(input).await
            }
            "stream" => {
                self.create_stream(input).await
            }
            "stream_session" => {
                self.create_stream_session(input).await
            }
            "playback_restriction_policy" => {
                self.create_playback_restriction_policy(input).await
            }
            "channel" => {
                self.create_channel(input).await
            }
            "stream_key" => {
                self.create_stream_key(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ivs",
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
            "metadata" => {
                self.read_metadata(id).await
            }
            "recording_configuration" => {
                self.read_recording_configuration(id).await
            }
            "playback_key_pair" => {
                self.read_playback_key_pair(id).await
            }
            "stream" => {
                self.read_stream(id).await
            }
            "stream_session" => {
                self.read_stream_session(id).await
            }
            "playback_restriction_policy" => {
                self.read_playback_restriction_policy(id).await
            }
            "channel" => {
                self.read_channel(id).await
            }
            "stream_key" => {
                self.read_stream_key(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ivs",
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
            "metadata" => {
                self.update_metadata(id, input).await
            }
            "recording_configuration" => {
                self.update_recording_configuration(id, input).await
            }
            "playback_key_pair" => {
                self.update_playback_key_pair(id, input).await
            }
            "stream" => {
                self.update_stream(id, input).await
            }
            "stream_session" => {
                self.update_stream_session(id, input).await
            }
            "playback_restriction_policy" => {
                self.update_playback_restriction_policy(id, input).await
            }
            "channel" => {
                self.update_channel(id, input).await
            }
            "stream_key" => {
                self.update_stream_key(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ivs",
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
            "metadata" => {
                self.delete_metadata(id).await
            }
            "recording_configuration" => {
                self.delete_recording_configuration(id).await
            }
            "playback_key_pair" => {
                self.delete_playback_key_pair(id).await
            }
            "stream" => {
                self.delete_stream(id).await
            }
            "stream_session" => {
                self.delete_stream_session(id).await
            }
            "playback_restriction_policy" => {
                self.delete_playback_restriction_policy(id).await
            }
            "channel" => {
                self.delete_channel(id).await
            }
            "stream_key" => {
                self.delete_stream_key(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ivs",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Metadata resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a metadata resource
    async fn plan_metadata(
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

    /// Create a new metadata resource
    async fn create_metadata(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let channel_arn = input.get_string("channel_arn")?;
            let metadata = input.get_string("metadata")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .create_metadata()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("channel_arn", channel_arn.unwrap_or_default())
                .with_field("metadata", metadata.unwrap_or_default())
            )
        })
    }

    /// Read a metadata resource
    async fn read_metadata(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .describe_metadata()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a metadata resource
    async fn update_metadata(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let channel_arn = input.get_string("channel_arn")?;
            let metadata = input.get_string("metadata")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .update_metadata()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("channel_arn", channel_arn.unwrap_or_default())
                .with_field("metadata", metadata.unwrap_or_default())
            )
        })
    }

    /// Delete a metadata resource
    async fn delete_metadata(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ivs_client
            //     .delete_metadata()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Recording_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a recording_configuration resource
    async fn plan_recording_configuration(
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

    /// Create a new recording_configuration resource
    async fn create_recording_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_optional_string("name")?;
            let thumbnail_configuration = input.get_optional_string("thumbnail_configuration")?;
            let recording_reconnect_window_seconds = input.get_optional_string("recording_reconnect_window_seconds")?;
            let rendition_configuration = input.get_optional_string("rendition_configuration")?;
            let destination_configuration = input.get_string("destination_configuration")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .create_recording_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("thumbnail_configuration", thumbnail_configuration.unwrap_or_default())
                .with_field("recording_reconnect_window_seconds", recording_reconnect_window_seconds.unwrap_or_default())
                .with_field("rendition_configuration", rendition_configuration.unwrap_or_default())
                .with_field("destination_configuration", destination_configuration.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Read a recording_configuration resource
    async fn read_recording_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .describe_recording_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a recording_configuration resource
    async fn update_recording_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_optional_string("name")?;
            let thumbnail_configuration = input.get_optional_string("thumbnail_configuration")?;
            let recording_reconnect_window_seconds = input.get_optional_string("recording_reconnect_window_seconds")?;
            let rendition_configuration = input.get_optional_string("rendition_configuration")?;
            let destination_configuration = input.get_string("destination_configuration")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .update_recording_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("thumbnail_configuration", thumbnail_configuration.unwrap_or_default())
                .with_field("recording_reconnect_window_seconds", recording_reconnect_window_seconds.unwrap_or_default())
                .with_field("rendition_configuration", rendition_configuration.unwrap_or_default())
                .with_field("destination_configuration", destination_configuration.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Delete a recording_configuration resource
    async fn delete_recording_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ivs_client
            //     .delete_recording_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Playback_key_pair resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a playback_key_pair resource
    async fn plan_playback_key_pair(
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

    /// Create a new playback_key_pair resource
    async fn create_playback_key_pair(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .create_playback_key_pair()
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

    /// Read a playback_key_pair resource
    async fn read_playback_key_pair(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .describe_playback_key_pair()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a playback_key_pair resource
    async fn update_playback_key_pair(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .update_playback_key_pair()
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

    /// Delete a playback_key_pair resource
    async fn delete_playback_key_pair(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ivs_client
            //     .delete_playback_key_pair()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Stream resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a stream resource
    async fn plan_stream(
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

    /// Create a new stream resource
    async fn create_stream(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .create_stream()
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

    /// Read a stream resource
    async fn read_stream(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .describe_stream()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a stream resource
    async fn update_stream(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .update_stream()
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

    /// Delete a stream resource
    async fn delete_stream(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ivs_client
            //     .delete_stream()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Stream_session resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a stream_session resource
    async fn plan_stream_session(
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

    /// Create a new stream_session resource
    async fn create_stream_session(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .create_stream_session()
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

    /// Read a stream_session resource
    async fn read_stream_session(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .describe_stream_session()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a stream_session resource
    async fn update_stream_session(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .update_stream_session()
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

    /// Delete a stream_session resource
    async fn delete_stream_session(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ivs_client
            //     .delete_stream_session()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Playback_restriction_policy resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a playback_restriction_policy resource
    async fn plan_playback_restriction_policy(
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

    /// Create a new playback_restriction_policy resource
    async fn create_playback_restriction_policy(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let allowed_origins = input.get_optional_string("allowed_origins")?;
            let enable_strict_origin_enforcement = input.get_optional_string("enable_strict_origin_enforcement")?;
            let name = input.get_optional_string("name")?;
            let allowed_countries = input.get_optional_string("allowed_countries")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .create_playback_restriction_policy()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("tags", tags.unwrap_or_default())
                .with_field("allowed_origins", allowed_origins.unwrap_or_default())
                .with_field("enable_strict_origin_enforcement", enable_strict_origin_enforcement.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("allowed_countries", allowed_countries.unwrap_or_default())
            )
        })
    }

    /// Read a playback_restriction_policy resource
    async fn read_playback_restriction_policy(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .describe_playback_restriction_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a playback_restriction_policy resource
    async fn update_playback_restriction_policy(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let allowed_origins = input.get_optional_string("allowed_origins")?;
            let enable_strict_origin_enforcement = input.get_optional_string("enable_strict_origin_enforcement")?;
            let name = input.get_optional_string("name")?;
            let allowed_countries = input.get_optional_string("allowed_countries")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .update_playback_restriction_policy()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("tags", tags.unwrap_or_default())
                .with_field("allowed_origins", allowed_origins.unwrap_or_default())
                .with_field("enable_strict_origin_enforcement", enable_strict_origin_enforcement.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("allowed_countries", allowed_countries.unwrap_or_default())
            )
        })
    }

    /// Delete a playback_restriction_policy resource
    async fn delete_playback_restriction_policy(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ivs_client
            //     .delete_playback_restriction_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Channel resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a channel resource
    async fn plan_channel(
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

    /// Create a new channel resource
    async fn create_channel(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let container_format = input.get_optional_string("container_format")?;
            let name = input.get_optional_string("name")?;
            let latency_mode = input.get_optional_string("latency_mode")?;
            let recording_configuration_arn = input.get_optional_string("recording_configuration_arn")?;
            let r#type = input.get_optional_string("type")?;
            let authorized = input.get_optional_string("authorized")?;
            let tags = input.get_optional_string("tags")?;
            let preset = input.get_optional_string("preset")?;
            let playback_restriction_policy_arn = input.get_optional_string("playback_restriction_policy_arn")?;
            let insecure_ingest = input.get_optional_string("insecure_ingest")?;
            let multitrack_input_configuration = input.get_optional_string("multitrack_input_configuration")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .create_channel()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("container_format", container_format.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("latency_mode", latency_mode.unwrap_or_default())
                .with_field("recording_configuration_arn", recording_configuration_arn.unwrap_or_default())
                .with_field("type", r#type.unwrap_or_default())
                .with_field("authorized", authorized.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("preset", preset.unwrap_or_default())
                .with_field("playback_restriction_policy_arn", playback_restriction_policy_arn.unwrap_or_default())
                .with_field("insecure_ingest", insecure_ingest.unwrap_or_default())
                .with_field("multitrack_input_configuration", multitrack_input_configuration.unwrap_or_default())
            )
        })
    }

    /// Read a channel resource
    async fn read_channel(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .describe_channel()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a channel resource
    async fn update_channel(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let container_format = input.get_optional_string("container_format")?;
            let name = input.get_optional_string("name")?;
            let latency_mode = input.get_optional_string("latency_mode")?;
            let recording_configuration_arn = input.get_optional_string("recording_configuration_arn")?;
            let r#type = input.get_optional_string("type")?;
            let authorized = input.get_optional_string("authorized")?;
            let tags = input.get_optional_string("tags")?;
            let preset = input.get_optional_string("preset")?;
            let playback_restriction_policy_arn = input.get_optional_string("playback_restriction_policy_arn")?;
            let insecure_ingest = input.get_optional_string("insecure_ingest")?;
            let multitrack_input_configuration = input.get_optional_string("multitrack_input_configuration")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .update_channel()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("container_format", container_format.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("latency_mode", latency_mode.unwrap_or_default())
                .with_field("recording_configuration_arn", recording_configuration_arn.unwrap_or_default())
                .with_field("type", r#type.unwrap_or_default())
                .with_field("authorized", authorized.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("preset", preset.unwrap_or_default())
                .with_field("playback_restriction_policy_arn", playback_restriction_policy_arn.unwrap_or_default())
                .with_field("insecure_ingest", insecure_ingest.unwrap_or_default())
                .with_field("multitrack_input_configuration", multitrack_input_configuration.unwrap_or_default())
            )
        })
    }

    /// Delete a channel resource
    async fn delete_channel(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ivs_client
            //     .delete_channel()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Stream_key resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a stream_key resource
    async fn plan_stream_key(
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

    /// Create a new stream_key resource
    async fn create_stream_key(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let channel_arn = input.get_string("channel_arn")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .create_stream_key()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("channel_arn", channel_arn.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Read a stream_key resource
    async fn read_stream_key(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .describe_stream_key()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a stream_key resource
    async fn update_stream_key(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let channel_arn = input.get_string("channel_arn")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ivs_client
            //     .update_stream_key()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("channel_arn", channel_arn.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Delete a stream_key resource
    async fn delete_stream_key(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ivs_client
            //     .delete_stream_key()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
