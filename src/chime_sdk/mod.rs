//! Chime_sdk service for Aws provider
//!
//! This module handles all chime_sdk resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Chime_sdk service handler
pub struct Chime_sdkService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Chime_sdkService<'a> {
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
            "sip_media_application_logging_configuration" => {
                self.plan_sip_media_application_logging_configuration(current_state, desired_input).await
            }
            "voice_connector_group" => {
                self.plan_voice_connector_group(current_state, desired_input).await
            }
            "voice_connector_external_systems_configuration" => {
                self.plan_voice_connector_external_systems_configuration(current_state, desired_input).await
            }
            "speaker_search_task" => {
                self.plan_speaker_search_task(current_state, desired_input).await
            }
            "voice_connector_emergency_calling_configuration" => {
                self.plan_voice_connector_emergency_calling_configuration(current_state, desired_input).await
            }
            "voice_profile_domain" => {
                self.plan_voice_profile_domain(current_state, desired_input).await
            }
            "phone_number_settings" => {
                self.plan_phone_number_settings(current_state, desired_input).await
            }
            "global_settings" => {
                self.plan_global_settings(current_state, desired_input).await
            }
            "voice_connector_logging_configuration" => {
                self.plan_voice_connector_logging_configuration(current_state, desired_input).await
            }
            "sip_media_application_call" => {
                self.plan_sip_media_application_call(current_state, desired_input).await
            }
            "voice_connector" => {
                self.plan_voice_connector(current_state, desired_input).await
            }
            "proxy_session" => {
                self.plan_proxy_session(current_state, desired_input).await
            }
            "phone_number_order" => {
                self.plan_phone_number_order(current_state, desired_input).await
            }
            "voice_connector_proxy" => {
                self.plan_voice_connector_proxy(current_state, desired_input).await
            }
            "voice_profile" => {
                self.plan_voice_profile(current_state, desired_input).await
            }
            "voice_connector_origination" => {
                self.plan_voice_connector_origination(current_state, desired_input).await
            }
            "sip_rule" => {
                self.plan_sip_rule(current_state, desired_input).await
            }
            "voice_tone_analysis_task" => {
                self.plan_voice_tone_analysis_task(current_state, desired_input).await
            }
            "voice_connector_streaming_configuration" => {
                self.plan_voice_connector_streaming_configuration(current_state, desired_input).await
            }
            "sip_media_application_alexa_skill_configuration" => {
                self.plan_sip_media_application_alexa_skill_configuration(current_state, desired_input).await
            }
            "phone_number" => {
                self.plan_phone_number(current_state, desired_input).await
            }
            "sip_media_application" => {
                self.plan_sip_media_application(current_state, desired_input).await
            }
            "voice_connector_termination" => {
                self.plan_voice_connector_termination(current_state, desired_input).await
            }
            "voice_connector_termination_health" => {
                self.plan_voice_connector_termination_health(current_state, desired_input).await
            }
            "voice_connector_termination_credentials" => {
                self.plan_voice_connector_termination_credentials(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "chime_sdk",
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
            "sip_media_application_logging_configuration" => {
                self.create_sip_media_application_logging_configuration(input).await
            }
            "voice_connector_group" => {
                self.create_voice_connector_group(input).await
            }
            "voice_connector_external_systems_configuration" => {
                self.create_voice_connector_external_systems_configuration(input).await
            }
            "speaker_search_task" => {
                self.create_speaker_search_task(input).await
            }
            "voice_connector_emergency_calling_configuration" => {
                self.create_voice_connector_emergency_calling_configuration(input).await
            }
            "voice_profile_domain" => {
                self.create_voice_profile_domain(input).await
            }
            "phone_number_settings" => {
                self.create_phone_number_settings(input).await
            }
            "global_settings" => {
                self.create_global_settings(input).await
            }
            "voice_connector_logging_configuration" => {
                self.create_voice_connector_logging_configuration(input).await
            }
            "sip_media_application_call" => {
                self.create_sip_media_application_call(input).await
            }
            "voice_connector" => {
                self.create_voice_connector(input).await
            }
            "proxy_session" => {
                self.create_proxy_session(input).await
            }
            "phone_number_order" => {
                self.create_phone_number_order(input).await
            }
            "voice_connector_proxy" => {
                self.create_voice_connector_proxy(input).await
            }
            "voice_profile" => {
                self.create_voice_profile(input).await
            }
            "voice_connector_origination" => {
                self.create_voice_connector_origination(input).await
            }
            "sip_rule" => {
                self.create_sip_rule(input).await
            }
            "voice_tone_analysis_task" => {
                self.create_voice_tone_analysis_task(input).await
            }
            "voice_connector_streaming_configuration" => {
                self.create_voice_connector_streaming_configuration(input).await
            }
            "sip_media_application_alexa_skill_configuration" => {
                self.create_sip_media_application_alexa_skill_configuration(input).await
            }
            "phone_number" => {
                self.create_phone_number(input).await
            }
            "sip_media_application" => {
                self.create_sip_media_application(input).await
            }
            "voice_connector_termination" => {
                self.create_voice_connector_termination(input).await
            }
            "voice_connector_termination_health" => {
                self.create_voice_connector_termination_health(input).await
            }
            "voice_connector_termination_credentials" => {
                self.create_voice_connector_termination_credentials(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "chime_sdk",
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
            "sip_media_application_logging_configuration" => {
                self.read_sip_media_application_logging_configuration(id).await
            }
            "voice_connector_group" => {
                self.read_voice_connector_group(id).await
            }
            "voice_connector_external_systems_configuration" => {
                self.read_voice_connector_external_systems_configuration(id).await
            }
            "speaker_search_task" => {
                self.read_speaker_search_task(id).await
            }
            "voice_connector_emergency_calling_configuration" => {
                self.read_voice_connector_emergency_calling_configuration(id).await
            }
            "voice_profile_domain" => {
                self.read_voice_profile_domain(id).await
            }
            "phone_number_settings" => {
                self.read_phone_number_settings(id).await
            }
            "global_settings" => {
                self.read_global_settings(id).await
            }
            "voice_connector_logging_configuration" => {
                self.read_voice_connector_logging_configuration(id).await
            }
            "sip_media_application_call" => {
                self.read_sip_media_application_call(id).await
            }
            "voice_connector" => {
                self.read_voice_connector(id).await
            }
            "proxy_session" => {
                self.read_proxy_session(id).await
            }
            "phone_number_order" => {
                self.read_phone_number_order(id).await
            }
            "voice_connector_proxy" => {
                self.read_voice_connector_proxy(id).await
            }
            "voice_profile" => {
                self.read_voice_profile(id).await
            }
            "voice_connector_origination" => {
                self.read_voice_connector_origination(id).await
            }
            "sip_rule" => {
                self.read_sip_rule(id).await
            }
            "voice_tone_analysis_task" => {
                self.read_voice_tone_analysis_task(id).await
            }
            "voice_connector_streaming_configuration" => {
                self.read_voice_connector_streaming_configuration(id).await
            }
            "sip_media_application_alexa_skill_configuration" => {
                self.read_sip_media_application_alexa_skill_configuration(id).await
            }
            "phone_number" => {
                self.read_phone_number(id).await
            }
            "sip_media_application" => {
                self.read_sip_media_application(id).await
            }
            "voice_connector_termination" => {
                self.read_voice_connector_termination(id).await
            }
            "voice_connector_termination_health" => {
                self.read_voice_connector_termination_health(id).await
            }
            "voice_connector_termination_credentials" => {
                self.read_voice_connector_termination_credentials(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "chime_sdk",
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
            "sip_media_application_logging_configuration" => {
                self.update_sip_media_application_logging_configuration(id, input).await
            }
            "voice_connector_group" => {
                self.update_voice_connector_group(id, input).await
            }
            "voice_connector_external_systems_configuration" => {
                self.update_voice_connector_external_systems_configuration(id, input).await
            }
            "speaker_search_task" => {
                self.update_speaker_search_task(id, input).await
            }
            "voice_connector_emergency_calling_configuration" => {
                self.update_voice_connector_emergency_calling_configuration(id, input).await
            }
            "voice_profile_domain" => {
                self.update_voice_profile_domain(id, input).await
            }
            "phone_number_settings" => {
                self.update_phone_number_settings(id, input).await
            }
            "global_settings" => {
                self.update_global_settings(id, input).await
            }
            "voice_connector_logging_configuration" => {
                self.update_voice_connector_logging_configuration(id, input).await
            }
            "sip_media_application_call" => {
                self.update_sip_media_application_call(id, input).await
            }
            "voice_connector" => {
                self.update_voice_connector(id, input).await
            }
            "proxy_session" => {
                self.update_proxy_session(id, input).await
            }
            "phone_number_order" => {
                self.update_phone_number_order(id, input).await
            }
            "voice_connector_proxy" => {
                self.update_voice_connector_proxy(id, input).await
            }
            "voice_profile" => {
                self.update_voice_profile(id, input).await
            }
            "voice_connector_origination" => {
                self.update_voice_connector_origination(id, input).await
            }
            "sip_rule" => {
                self.update_sip_rule(id, input).await
            }
            "voice_tone_analysis_task" => {
                self.update_voice_tone_analysis_task(id, input).await
            }
            "voice_connector_streaming_configuration" => {
                self.update_voice_connector_streaming_configuration(id, input).await
            }
            "sip_media_application_alexa_skill_configuration" => {
                self.update_sip_media_application_alexa_skill_configuration(id, input).await
            }
            "phone_number" => {
                self.update_phone_number(id, input).await
            }
            "sip_media_application" => {
                self.update_sip_media_application(id, input).await
            }
            "voice_connector_termination" => {
                self.update_voice_connector_termination(id, input).await
            }
            "voice_connector_termination_health" => {
                self.update_voice_connector_termination_health(id, input).await
            }
            "voice_connector_termination_credentials" => {
                self.update_voice_connector_termination_credentials(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "chime_sdk",
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
            "sip_media_application_logging_configuration" => {
                self.delete_sip_media_application_logging_configuration(id).await
            }
            "voice_connector_group" => {
                self.delete_voice_connector_group(id).await
            }
            "voice_connector_external_systems_configuration" => {
                self.delete_voice_connector_external_systems_configuration(id).await
            }
            "speaker_search_task" => {
                self.delete_speaker_search_task(id).await
            }
            "voice_connector_emergency_calling_configuration" => {
                self.delete_voice_connector_emergency_calling_configuration(id).await
            }
            "voice_profile_domain" => {
                self.delete_voice_profile_domain(id).await
            }
            "phone_number_settings" => {
                self.delete_phone_number_settings(id).await
            }
            "global_settings" => {
                self.delete_global_settings(id).await
            }
            "voice_connector_logging_configuration" => {
                self.delete_voice_connector_logging_configuration(id).await
            }
            "sip_media_application_call" => {
                self.delete_sip_media_application_call(id).await
            }
            "voice_connector" => {
                self.delete_voice_connector(id).await
            }
            "proxy_session" => {
                self.delete_proxy_session(id).await
            }
            "phone_number_order" => {
                self.delete_phone_number_order(id).await
            }
            "voice_connector_proxy" => {
                self.delete_voice_connector_proxy(id).await
            }
            "voice_profile" => {
                self.delete_voice_profile(id).await
            }
            "voice_connector_origination" => {
                self.delete_voice_connector_origination(id).await
            }
            "sip_rule" => {
                self.delete_sip_rule(id).await
            }
            "voice_tone_analysis_task" => {
                self.delete_voice_tone_analysis_task(id).await
            }
            "voice_connector_streaming_configuration" => {
                self.delete_voice_connector_streaming_configuration(id).await
            }
            "sip_media_application_alexa_skill_configuration" => {
                self.delete_sip_media_application_alexa_skill_configuration(id).await
            }
            "phone_number" => {
                self.delete_phone_number(id).await
            }
            "sip_media_application" => {
                self.delete_sip_media_application(id).await
            }
            "voice_connector_termination" => {
                self.delete_voice_connector_termination(id).await
            }
            "voice_connector_termination_health" => {
                self.delete_voice_connector_termination_health(id).await
            }
            "voice_connector_termination_credentials" => {
                self.delete_voice_connector_termination_credentials(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "chime_sdk",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Sip_media_application_logging_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sip_media_application_logging_configuration resource
    async fn plan_sip_media_application_logging_configuration(
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

    /// Create a new sip_media_application_logging_configuration resource
    async fn create_sip_media_application_logging_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let sip_media_application_logging_configuration = input.get_optional_string("sip_media_application_logging_configuration")?;
            let sip_media_application_id = input.get_string("sip_media_application_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_sip_media_application_logging_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("sip_media_application_logging_configuration", sip_media_application_logging_configuration.unwrap_or_default())
                .with_field("sip_media_application_id", sip_media_application_id.unwrap_or_default())
            )
        })
    }

    /// Read a sip_media_application_logging_configuration resource
    async fn read_sip_media_application_logging_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_sip_media_application_logging_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a sip_media_application_logging_configuration resource
    async fn update_sip_media_application_logging_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let sip_media_application_logging_configuration = input.get_optional_string("sip_media_application_logging_configuration")?;
            let sip_media_application_id = input.get_string("sip_media_application_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_sip_media_application_logging_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("sip_media_application_logging_configuration", sip_media_application_logging_configuration.unwrap_or_default())
                .with_field("sip_media_application_id", sip_media_application_id.unwrap_or_default())
            )
        })
    }

    /// Delete a sip_media_application_logging_configuration resource
    async fn delete_sip_media_application_logging_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_sip_media_application_logging_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector_group resource
    async fn plan_voice_connector_group(
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

    /// Create a new voice_connector_group resource
    async fn create_voice_connector_group(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let voice_connector_items = input.get_optional_string("voice_connector_items")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector_group()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("voice_connector_items", voice_connector_items.unwrap_or_default())
            )
        })
    }

    /// Read a voice_connector_group resource
    async fn read_voice_connector_group(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector_group resource
    async fn update_voice_connector_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let voice_connector_items = input.get_optional_string("voice_connector_items")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector_group()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("voice_connector_items", voice_connector_items.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_connector_group resource
    async fn delete_voice_connector_group(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector_external_systems_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector_external_systems_configuration resource
    async fn plan_voice_connector_external_systems_configuration(
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

    /// Create a new voice_connector_external_systems_configuration resource
    async fn create_voice_connector_external_systems_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let contact_center_system_types = input.get_optional_string("contact_center_system_types")?;
            let session_border_controller_types = input.get_optional_string("session_border_controller_types")?;
            let voice_connector_id = input.get_string("voice_connector_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector_external_systems_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("contact_center_system_types", contact_center_system_types.unwrap_or_default())
                .with_field("session_border_controller_types", session_border_controller_types.unwrap_or_default())
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
            )
        })
    }

    /// Read a voice_connector_external_systems_configuration resource
    async fn read_voice_connector_external_systems_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector_external_systems_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector_external_systems_configuration resource
    async fn update_voice_connector_external_systems_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let contact_center_system_types = input.get_optional_string("contact_center_system_types")?;
            let session_border_controller_types = input.get_optional_string("session_border_controller_types")?;
            let voice_connector_id = input.get_string("voice_connector_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector_external_systems_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("contact_center_system_types", contact_center_system_types.unwrap_or_default())
                .with_field("session_border_controller_types", session_border_controller_types.unwrap_or_default())
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_connector_external_systems_configuration resource
    async fn delete_voice_connector_external_systems_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector_external_systems_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Speaker_search_task resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a speaker_search_task resource
    async fn plan_speaker_search_task(
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

    /// Create a new speaker_search_task resource
    async fn create_speaker_search_task(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_speaker_search_task()
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

    /// Read a speaker_search_task resource
    async fn read_speaker_search_task(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_speaker_search_task()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a speaker_search_task resource
    async fn update_speaker_search_task(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_speaker_search_task()
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

    /// Delete a speaker_search_task resource
    async fn delete_speaker_search_task(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_speaker_search_task()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector_emergency_calling_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector_emergency_calling_configuration resource
    async fn plan_voice_connector_emergency_calling_configuration(
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

    /// Create a new voice_connector_emergency_calling_configuration resource
    async fn create_voice_connector_emergency_calling_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let emergency_calling_configuration = input.get_string("emergency_calling_configuration")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector_emergency_calling_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("emergency_calling_configuration", emergency_calling_configuration.unwrap_or_default())
            )
        })
    }

    /// Read a voice_connector_emergency_calling_configuration resource
    async fn read_voice_connector_emergency_calling_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector_emergency_calling_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector_emergency_calling_configuration resource
    async fn update_voice_connector_emergency_calling_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let emergency_calling_configuration = input.get_string("emergency_calling_configuration")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector_emergency_calling_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("emergency_calling_configuration", emergency_calling_configuration.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_connector_emergency_calling_configuration resource
    async fn delete_voice_connector_emergency_calling_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector_emergency_calling_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_profile_domain resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_profile_domain resource
    async fn plan_voice_profile_domain(
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

    /// Create a new voice_profile_domain resource
    async fn create_voice_profile_domain(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let client_request_token = input.get_optional_string("client_request_token")?;
            let server_side_encryption_configuration = input.get_string("server_side_encryption_configuration")?;
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_profile_domain()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("description", description.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("server_side_encryption_configuration", server_side_encryption_configuration.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a voice_profile_domain resource
    async fn read_voice_profile_domain(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_profile_domain()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_profile_domain resource
    async fn update_voice_profile_domain(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let client_request_token = input.get_optional_string("client_request_token")?;
            let server_side_encryption_configuration = input.get_string("server_side_encryption_configuration")?;
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_profile_domain()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("description", description.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("server_side_encryption_configuration", server_side_encryption_configuration.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_profile_domain resource
    async fn delete_voice_profile_domain(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_profile_domain()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Phone_number_settings resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a phone_number_settings resource
    async fn plan_phone_number_settings(
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

    /// Create a new phone_number_settings resource
    async fn create_phone_number_settings(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let calling_name = input.get_string("calling_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_phone_number_settings()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("calling_name", calling_name.unwrap_or_default())
            )
        })
    }

    /// Read a phone_number_settings resource
    async fn read_phone_number_settings(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_phone_number_settings()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a phone_number_settings resource
    async fn update_phone_number_settings(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let calling_name = input.get_string("calling_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_phone_number_settings()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("calling_name", calling_name.unwrap_or_default())
            )
        })
    }

    /// Delete a phone_number_settings resource
    async fn delete_phone_number_settings(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_phone_number_settings()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Global_settings resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a global_settings resource
    async fn plan_global_settings(
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

    /// Create a new global_settings resource
    async fn create_global_settings(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector = input.get_optional_string("voice_connector")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_global_settings()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("voice_connector", voice_connector.unwrap_or_default())
            )
        })
    }

    /// Read a global_settings resource
    async fn read_global_settings(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_global_settings()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a global_settings resource
    async fn update_global_settings(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector = input.get_optional_string("voice_connector")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_global_settings()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("voice_connector", voice_connector.unwrap_or_default())
            )
        })
    }

    /// Delete a global_settings resource
    async fn delete_global_settings(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_global_settings()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector_logging_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector_logging_configuration resource
    async fn plan_voice_connector_logging_configuration(
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

    /// Create a new voice_connector_logging_configuration resource
    async fn create_voice_connector_logging_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let logging_configuration = input.get_string("logging_configuration")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector_logging_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("logging_configuration", logging_configuration.unwrap_or_default())
            )
        })
    }

    /// Read a voice_connector_logging_configuration resource
    async fn read_voice_connector_logging_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector_logging_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector_logging_configuration resource
    async fn update_voice_connector_logging_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let logging_configuration = input.get_string("logging_configuration")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector_logging_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("logging_configuration", logging_configuration.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_connector_logging_configuration resource
    async fn delete_voice_connector_logging_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector_logging_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Sip_media_application_call resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sip_media_application_call resource
    async fn plan_sip_media_application_call(
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

    /// Create a new sip_media_application_call resource
    async fn create_sip_media_application_call(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let sip_headers = input.get_optional_string("sip_headers")?;
            let from_phone_number = input.get_string("from_phone_number")?;
            let arguments_map = input.get_optional_string("arguments_map")?;
            let to_phone_number = input.get_string("to_phone_number")?;
            let sip_media_application_id = input.get_string("sip_media_application_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_sip_media_application_call()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("sip_headers", sip_headers.unwrap_or_default())
                .with_field("from_phone_number", from_phone_number.unwrap_or_default())
                .with_field("arguments_map", arguments_map.unwrap_or_default())
                .with_field("to_phone_number", to_phone_number.unwrap_or_default())
                .with_field("sip_media_application_id", sip_media_application_id.unwrap_or_default())
            )
        })
    }

    /// Read a sip_media_application_call resource
    async fn read_sip_media_application_call(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_sip_media_application_call()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a sip_media_application_call resource
    async fn update_sip_media_application_call(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let sip_headers = input.get_optional_string("sip_headers")?;
            let from_phone_number = input.get_string("from_phone_number")?;
            let arguments_map = input.get_optional_string("arguments_map")?;
            let to_phone_number = input.get_string("to_phone_number")?;
            let sip_media_application_id = input.get_string("sip_media_application_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_sip_media_application_call()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("sip_headers", sip_headers.unwrap_or_default())
                .with_field("from_phone_number", from_phone_number.unwrap_or_default())
                .with_field("arguments_map", arguments_map.unwrap_or_default())
                .with_field("to_phone_number", to_phone_number.unwrap_or_default())
                .with_field("sip_media_application_id", sip_media_application_id.unwrap_or_default())
            )
        })
    }

    /// Delete a sip_media_application_call resource
    async fn delete_sip_media_application_call(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_sip_media_application_call()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector resource
    async fn plan_voice_connector(
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

    /// Create a new voice_connector resource
    async fn create_voice_connector(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let require_encryption = input.get_string("require_encryption")?;
            let tags = input.get_optional_string("tags")?;
            let integration_type = input.get_optional_string("integration_type")?;
            let network_type = input.get_optional_string("network_type")?;
            let aws_region = input.get_optional_string("aws_region")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("require_encryption", require_encryption.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("integration_type", integration_type.unwrap_or_default())
                .with_field("network_type", network_type.unwrap_or_default())
                .with_field("aws_region", aws_region.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a voice_connector resource
    async fn read_voice_connector(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector resource
    async fn update_voice_connector(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let require_encryption = input.get_string("require_encryption")?;
            let tags = input.get_optional_string("tags")?;
            let integration_type = input.get_optional_string("integration_type")?;
            let network_type = input.get_optional_string("network_type")?;
            let aws_region = input.get_optional_string("aws_region")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("require_encryption", require_encryption.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("integration_type", integration_type.unwrap_or_default())
                .with_field("network_type", network_type.unwrap_or_default())
                .with_field("aws_region", aws_region.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_connector resource
    async fn delete_voice_connector(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Proxy_session resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a proxy_session resource
    async fn plan_proxy_session(
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

    /// Create a new proxy_session resource
    async fn create_proxy_session(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let expiry_minutes = input.get_optional_string("expiry_minutes")?;
            let capabilities = input.get_string("capabilities")?;
            let geo_match_params = input.get_optional_string("geo_match_params")?;
            let participant_phone_numbers = input.get_string("participant_phone_numbers")?;
            let number_selection_behavior = input.get_optional_string("number_selection_behavior")?;
            let geo_match_level = input.get_optional_string("geo_match_level")?;
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let name = input.get_optional_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_proxy_session()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("expiry_minutes", expiry_minutes.unwrap_or_default())
                .with_field("capabilities", capabilities.unwrap_or_default())
                .with_field("geo_match_params", geo_match_params.unwrap_or_default())
                .with_field("participant_phone_numbers", participant_phone_numbers.unwrap_or_default())
                .with_field("number_selection_behavior", number_selection_behavior.unwrap_or_default())
                .with_field("geo_match_level", geo_match_level.unwrap_or_default())
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a proxy_session resource
    async fn read_proxy_session(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_proxy_session()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a proxy_session resource
    async fn update_proxy_session(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let expiry_minutes = input.get_optional_string("expiry_minutes")?;
            let capabilities = input.get_string("capabilities")?;
            let geo_match_params = input.get_optional_string("geo_match_params")?;
            let participant_phone_numbers = input.get_string("participant_phone_numbers")?;
            let number_selection_behavior = input.get_optional_string("number_selection_behavior")?;
            let geo_match_level = input.get_optional_string("geo_match_level")?;
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let name = input.get_optional_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_proxy_session()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("expiry_minutes", expiry_minutes.unwrap_or_default())
                .with_field("capabilities", capabilities.unwrap_or_default())
                .with_field("geo_match_params", geo_match_params.unwrap_or_default())
                .with_field("participant_phone_numbers", participant_phone_numbers.unwrap_or_default())
                .with_field("number_selection_behavior", number_selection_behavior.unwrap_or_default())
                .with_field("geo_match_level", geo_match_level.unwrap_or_default())
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a proxy_session resource
    async fn delete_proxy_session(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_proxy_session()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Phone_number_order resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a phone_number_order resource
    async fn plan_phone_number_order(
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

    /// Create a new phone_number_order resource
    async fn create_phone_number_order(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let product_type = input.get_string("product_type")?;
            let e164_phone_numbers = input.get_string("e164_phone_numbers")?;
            let name = input.get_optional_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_phone_number_order()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("product_type", product_type.unwrap_or_default())
                .with_field("e164_phone_numbers", e164_phone_numbers.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a phone_number_order resource
    async fn read_phone_number_order(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_phone_number_order()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a phone_number_order resource
    async fn update_phone_number_order(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let product_type = input.get_string("product_type")?;
            let e164_phone_numbers = input.get_string("e164_phone_numbers")?;
            let name = input.get_optional_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_phone_number_order()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("product_type", product_type.unwrap_or_default())
                .with_field("e164_phone_numbers", e164_phone_numbers.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a phone_number_order resource
    async fn delete_phone_number_order(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_phone_number_order()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector_proxy resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector_proxy resource
    async fn plan_voice_connector_proxy(
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

    /// Create a new voice_connector_proxy resource
    async fn create_voice_connector_proxy(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let phone_number_pool_countries = input.get_string("phone_number_pool_countries")?;
            let fall_back_phone_number = input.get_optional_string("fall_back_phone_number")?;
            let default_session_expiry_minutes = input.get_string("default_session_expiry_minutes")?;
            let disabled = input.get_optional_string("disabled")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector_proxy()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("phone_number_pool_countries", phone_number_pool_countries.unwrap_or_default())
                .with_field("fall_back_phone_number", fall_back_phone_number.unwrap_or_default())
                .with_field("default_session_expiry_minutes", default_session_expiry_minutes.unwrap_or_default())
                .with_field("disabled", disabled.unwrap_or_default())
            )
        })
    }

    /// Read a voice_connector_proxy resource
    async fn read_voice_connector_proxy(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector_proxy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector_proxy resource
    async fn update_voice_connector_proxy(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let phone_number_pool_countries = input.get_string("phone_number_pool_countries")?;
            let fall_back_phone_number = input.get_optional_string("fall_back_phone_number")?;
            let default_session_expiry_minutes = input.get_string("default_session_expiry_minutes")?;
            let disabled = input.get_optional_string("disabled")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector_proxy()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("phone_number_pool_countries", phone_number_pool_countries.unwrap_or_default())
                .with_field("fall_back_phone_number", fall_back_phone_number.unwrap_or_default())
                .with_field("default_session_expiry_minutes", default_session_expiry_minutes.unwrap_or_default())
                .with_field("disabled", disabled.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_connector_proxy resource
    async fn delete_voice_connector_proxy(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector_proxy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_profile resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_profile resource
    async fn plan_voice_profile(
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

    /// Create a new voice_profile resource
    async fn create_voice_profile(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let speaker_search_task_id = input.get_string("speaker_search_task_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_profile()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("speaker_search_task_id", speaker_search_task_id.unwrap_or_default())
            )
        })
    }

    /// Read a voice_profile resource
    async fn read_voice_profile(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_profile()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_profile resource
    async fn update_voice_profile(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let speaker_search_task_id = input.get_string("speaker_search_task_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_profile()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("speaker_search_task_id", speaker_search_task_id.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_profile resource
    async fn delete_voice_profile(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_profile()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector_origination resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector_origination resource
    async fn plan_voice_connector_origination(
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

    /// Create a new voice_connector_origination resource
    async fn create_voice_connector_origination(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let origination = input.get_string("origination")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector_origination()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("origination", origination.unwrap_or_default())
            )
        })
    }

    /// Read a voice_connector_origination resource
    async fn read_voice_connector_origination(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector_origination()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector_origination resource
    async fn update_voice_connector_origination(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let origination = input.get_string("origination")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector_origination()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("origination", origination.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_connector_origination resource
    async fn delete_voice_connector_origination(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector_origination()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Sip_rule resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sip_rule resource
    async fn plan_sip_rule(
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

    /// Create a new sip_rule resource
    async fn create_sip_rule(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let disabled = input.get_optional_string("disabled")?;
            let target_applications = input.get_optional_string("target_applications")?;
            let trigger_type = input.get_string("trigger_type")?;
            let trigger_value = input.get_string("trigger_value")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_sip_rule()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("disabled", disabled.unwrap_or_default())
                .with_field("target_applications", target_applications.unwrap_or_default())
                .with_field("trigger_type", trigger_type.unwrap_or_default())
                .with_field("trigger_value", trigger_value.unwrap_or_default())
            )
        })
    }

    /// Read a sip_rule resource
    async fn read_sip_rule(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_sip_rule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a sip_rule resource
    async fn update_sip_rule(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let disabled = input.get_optional_string("disabled")?;
            let target_applications = input.get_optional_string("target_applications")?;
            let trigger_type = input.get_string("trigger_type")?;
            let trigger_value = input.get_string("trigger_value")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_sip_rule()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("disabled", disabled.unwrap_or_default())
                .with_field("target_applications", target_applications.unwrap_or_default())
                .with_field("trigger_type", trigger_type.unwrap_or_default())
                .with_field("trigger_value", trigger_value.unwrap_or_default())
            )
        })
    }

    /// Delete a sip_rule resource
    async fn delete_sip_rule(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_sip_rule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_tone_analysis_task resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_tone_analysis_task resource
    async fn plan_voice_tone_analysis_task(
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

    /// Create a new voice_tone_analysis_task resource
    async fn create_voice_tone_analysis_task(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_tone_analysis_task()
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

    /// Read a voice_tone_analysis_task resource
    async fn read_voice_tone_analysis_task(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_tone_analysis_task()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_tone_analysis_task resource
    async fn update_voice_tone_analysis_task(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_tone_analysis_task()
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

    /// Delete a voice_tone_analysis_task resource
    async fn delete_voice_tone_analysis_task(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_tone_analysis_task()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector_streaming_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector_streaming_configuration resource
    async fn plan_voice_connector_streaming_configuration(
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

    /// Create a new voice_connector_streaming_configuration resource
    async fn create_voice_connector_streaming_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let streaming_configuration = input.get_string("streaming_configuration")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector_streaming_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("streaming_configuration", streaming_configuration.unwrap_or_default())
            )
        })
    }

    /// Read a voice_connector_streaming_configuration resource
    async fn read_voice_connector_streaming_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector_streaming_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector_streaming_configuration resource
    async fn update_voice_connector_streaming_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let streaming_configuration = input.get_string("streaming_configuration")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector_streaming_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("streaming_configuration", streaming_configuration.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_connector_streaming_configuration resource
    async fn delete_voice_connector_streaming_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector_streaming_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Sip_media_application_alexa_skill_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sip_media_application_alexa_skill_configuration resource
    async fn plan_sip_media_application_alexa_skill_configuration(
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

    /// Create a new sip_media_application_alexa_skill_configuration resource
    async fn create_sip_media_application_alexa_skill_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let sip_media_application_id = input.get_string("sip_media_application_id")?;
            let sip_media_application_alexa_skill_configuration = input.get_optional_string("sip_media_application_alexa_skill_configuration")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_sip_media_application_alexa_skill_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("sip_media_application_id", sip_media_application_id.unwrap_or_default())
                .with_field("sip_media_application_alexa_skill_configuration", sip_media_application_alexa_skill_configuration.unwrap_or_default())
            )
        })
    }

    /// Read a sip_media_application_alexa_skill_configuration resource
    async fn read_sip_media_application_alexa_skill_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_sip_media_application_alexa_skill_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a sip_media_application_alexa_skill_configuration resource
    async fn update_sip_media_application_alexa_skill_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let sip_media_application_id = input.get_string("sip_media_application_id")?;
            let sip_media_application_alexa_skill_configuration = input.get_optional_string("sip_media_application_alexa_skill_configuration")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_sip_media_application_alexa_skill_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("sip_media_application_id", sip_media_application_id.unwrap_or_default())
                .with_field("sip_media_application_alexa_skill_configuration", sip_media_application_alexa_skill_configuration.unwrap_or_default())
            )
        })
    }

    /// Delete a sip_media_application_alexa_skill_configuration resource
    async fn delete_sip_media_application_alexa_skill_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_sip_media_application_alexa_skill_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Phone_number resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a phone_number resource
    async fn plan_phone_number(
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

    /// Create a new phone_number resource
    async fn create_phone_number(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let phone_number_id = input.get_string("phone_number_id")?;
            let name = input.get_optional_string("name")?;
            let calling_name = input.get_optional_string("calling_name")?;
            let product_type = input.get_optional_string("product_type")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_phone_number()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("phone_number_id", phone_number_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("calling_name", calling_name.unwrap_or_default())
                .with_field("product_type", product_type.unwrap_or_default())
            )
        })
    }

    /// Read a phone_number resource
    async fn read_phone_number(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_phone_number()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a phone_number resource
    async fn update_phone_number(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let phone_number_id = input.get_string("phone_number_id")?;
            let name = input.get_optional_string("name")?;
            let calling_name = input.get_optional_string("calling_name")?;
            let product_type = input.get_optional_string("product_type")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_phone_number()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("phone_number_id", phone_number_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("calling_name", calling_name.unwrap_or_default())
                .with_field("product_type", product_type.unwrap_or_default())
            )
        })
    }

    /// Delete a phone_number resource
    async fn delete_phone_number(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_phone_number()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Sip_media_application resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sip_media_application resource
    async fn plan_sip_media_application(
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

    /// Create a new sip_media_application resource
    async fn create_sip_media_application(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let endpoints = input.get_string("endpoints")?;
            let tags = input.get_optional_string("tags")?;
            let aws_region = input.get_string("aws_region")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_sip_media_application()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("endpoints", endpoints.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("aws_region", aws_region.unwrap_or_default())
            )
        })
    }

    /// Read a sip_media_application resource
    async fn read_sip_media_application(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_sip_media_application()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a sip_media_application resource
    async fn update_sip_media_application(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let endpoints = input.get_string("endpoints")?;
            let tags = input.get_optional_string("tags")?;
            let aws_region = input.get_string("aws_region")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_sip_media_application()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("endpoints", endpoints.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("aws_region", aws_region.unwrap_or_default())
            )
        })
    }

    /// Delete a sip_media_application resource
    async fn delete_sip_media_application(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_sip_media_application()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector_termination resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector_termination resource
    async fn plan_voice_connector_termination(
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

    /// Create a new voice_connector_termination resource
    async fn create_voice_connector_termination(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let termination = input.get_string("termination")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector_termination()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("termination", termination.unwrap_or_default())
            )
        })
    }

    /// Read a voice_connector_termination resource
    async fn read_voice_connector_termination(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector_termination()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector_termination resource
    async fn update_voice_connector_termination(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let voice_connector_id = input.get_string("voice_connector_id")?;
            let termination = input.get_string("termination")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector_termination()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
                .with_field("termination", termination.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_connector_termination resource
    async fn delete_voice_connector_termination(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector_termination()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector_termination_health resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector_termination_health resource
    async fn plan_voice_connector_termination_health(
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

    /// Create a new voice_connector_termination_health resource
    async fn create_voice_connector_termination_health(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector_termination_health()
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

    /// Read a voice_connector_termination_health resource
    async fn read_voice_connector_termination_health(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector_termination_health()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector_termination_health resource
    async fn update_voice_connector_termination_health(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector_termination_health()
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

    /// Delete a voice_connector_termination_health resource
    async fn delete_voice_connector_termination_health(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector_termination_health()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Voice_connector_termination_credentials resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a voice_connector_termination_credentials resource
    async fn plan_voice_connector_termination_credentials(
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

    /// Create a new voice_connector_termination_credentials resource
    async fn create_voice_connector_termination_credentials(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let credentials = input.get_optional_string("credentials")?;
            let voice_connector_id = input.get_string("voice_connector_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .create_voice_connector_termination_credentials()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("credentials", credentials.unwrap_or_default())
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
            )
        })
    }

    /// Read a voice_connector_termination_credentials resource
    async fn read_voice_connector_termination_credentials(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .describe_voice_connector_termination_credentials()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a voice_connector_termination_credentials resource
    async fn update_voice_connector_termination_credentials(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let credentials = input.get_optional_string("credentials")?;
            let voice_connector_id = input.get_string("voice_connector_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.chime_sdk_client
            //     .update_voice_connector_termination_credentials()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("credentials", credentials.unwrap_or_default())
                .with_field("voice_connector_id", voice_connector_id.unwrap_or_default())
            )
        })
    }

    /// Delete a voice_connector_termination_credentials resource
    async fn delete_voice_connector_termination_credentials(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.chime_sdk_client
            //     .delete_voice_connector_termination_credentials()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
