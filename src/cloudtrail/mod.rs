//! Cloudtrail service for Aws provider
//!
//! This module handles all cloudtrail resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Cloudtrail service handler
pub struct CloudtrailService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CloudtrailService<'a> {
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
            "query" => self.plan_query(current_state, desired_input).await,
            "event_data_store" => {
                self.plan_event_data_store(current_state, desired_input)
                    .await
            }
            "trail" => self.plan_trail(current_state, desired_input).await,
            "event_configuration" => {
                self.plan_event_configuration(current_state, desired_input)
                    .await
            }
            "dashboard" => self.plan_dashboard(current_state, desired_input).await,
            "trail_status" => self.plan_trail_status(current_state, desired_input).await,
            "resource_policy" => {
                self.plan_resource_policy(current_state, desired_input)
                    .await
            }
            "event_selectors" => {
                self.plan_event_selectors(current_state, desired_input)
                    .await
            }
            "channel" => self.plan_channel(current_state, desired_input).await,
            "trails" => self.plan_trails(current_state, desired_input).await,
            "query_results" => self.plan_query_results(current_state, desired_input).await,
            "insight_selectors" => {
                self.plan_insight_selectors(current_state, desired_input)
                    .await
            }
            "import" => self.plan_import(current_state, desired_input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudtrail", resource_name
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
            "query" => self.create_query(input).await,
            "event_data_store" => self.create_event_data_store(input).await,
            "trail" => self.create_trail(input).await,
            "event_configuration" => self.create_event_configuration(input).await,
            "dashboard" => self.create_dashboard(input).await,
            "trail_status" => self.create_trail_status(input).await,
            "resource_policy" => self.create_resource_policy(input).await,
            "event_selectors" => self.create_event_selectors(input).await,
            "channel" => self.create_channel(input).await,
            "trails" => self.create_trails(input).await,
            "query_results" => self.create_query_results(input).await,
            "insight_selectors" => self.create_insight_selectors(input).await,
            "import" => self.create_import(input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudtrail", resource_name
            ))),
        }
    }

    /// Read resource state
    pub async fn read_resource(&self, resource_name: &str, id: &str) -> Result<ResourceOutput> {
        match resource_name {
            "query" => self.read_query(id).await,
            "event_data_store" => self.read_event_data_store(id).await,
            "trail" => self.read_trail(id).await,
            "event_configuration" => self.read_event_configuration(id).await,
            "dashboard" => self.read_dashboard(id).await,
            "trail_status" => self.read_trail_status(id).await,
            "resource_policy" => self.read_resource_policy(id).await,
            "event_selectors" => self.read_event_selectors(id).await,
            "channel" => self.read_channel(id).await,
            "trails" => self.read_trails(id).await,
            "query_results" => self.read_query_results(id).await,
            "insight_selectors" => self.read_insight_selectors(id).await,
            "import" => self.read_import(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudtrail", resource_name
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
            "query" => self.update_query(id, input).await,
            "event_data_store" => self.update_event_data_store(id, input).await,
            "trail" => self.update_trail(id, input).await,
            "event_configuration" => self.update_event_configuration(id, input).await,
            "dashboard" => self.update_dashboard(id, input).await,
            "trail_status" => self.update_trail_status(id, input).await,
            "resource_policy" => self.update_resource_policy(id, input).await,
            "event_selectors" => self.update_event_selectors(id, input).await,
            "channel" => self.update_channel(id, input).await,
            "trails" => self.update_trails(id, input).await,
            "query_results" => self.update_query_results(id, input).await,
            "insight_selectors" => self.update_insight_selectors(id, input).await,
            "import" => self.update_import(id, input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudtrail", resource_name
            ))),
        }
    }

    /// Delete a resource
    pub async fn delete_resource(&self, resource_name: &str, id: &str) -> Result<()> {
        match resource_name {
            "query" => self.delete_query(id).await,
            "event_data_store" => self.delete_event_data_store(id).await,
            "trail" => self.delete_trail(id).await,
            "event_configuration" => self.delete_event_configuration(id).await,
            "dashboard" => self.delete_dashboard(id).await,
            "trail_status" => self.delete_trail_status(id).await,
            "resource_policy" => self.delete_resource_policy(id).await,
            "event_selectors" => self.delete_event_selectors(id).await,
            "channel" => self.delete_channel(id).await,
            "trails" => self.delete_trails(id).await,
            "query_results" => self.delete_query_results(id).await,
            "insight_selectors" => self.delete_insight_selectors(id).await,
            "import" => self.delete_import(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudtrail", resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================

    // ------------------------------------------------------------------------
    // Query resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a query resource
    async fn plan_query(
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

    /// Create a new query resource
    async fn create_query(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_query()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a query resource
    async fn read_query(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_query()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a query resource
    async fn update_query(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_query()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a query resource
    async fn delete_query(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_query()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Event_data_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a event_data_store resource
    async fn plan_event_data_store(
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

    /// Create a new event_data_store resource
    async fn create_event_data_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags_list = input.get_optional_string("tags_list")?;
            let kms_key_id = input.get_optional_string("kms_key_id")?;
            let billing_mode = input.get_optional_string("billing_mode")?;
            let retention_period = input.get_optional_string("retention_period")?;
            let termination_protection_enabled =
                input.get_optional_string("termination_protection_enabled")?;
            let advanced_event_selectors = input.get_optional_string("advanced_event_selectors")?;
            let organization_enabled = input.get_optional_string("organization_enabled")?;
            let start_ingestion = input.get_optional_string("start_ingestion")?;
            let name = input.get_string("name")?;
            let multi_region_enabled = input.get_optional_string("multi_region_enabled")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_event_data_store()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("tags_list", tags_list.unwrap_or_default())
                .with_field("kms_key_id", kms_key_id.unwrap_or_default())
                .with_field("billing_mode", billing_mode.unwrap_or_default())
                .with_field("retention_period", retention_period.unwrap_or_default())
                .with_field(
                    "termination_protection_enabled",
                    termination_protection_enabled.unwrap_or_default(),
                )
                .with_field(
                    "advanced_event_selectors",
                    advanced_event_selectors.unwrap_or_default(),
                )
                .with_field(
                    "organization_enabled",
                    organization_enabled.unwrap_or_default(),
                )
                .with_field("start_ingestion", start_ingestion.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field(
                    "multi_region_enabled",
                    multi_region_enabled.unwrap_or_default(),
                ))
        })
    }

    /// Read a event_data_store resource
    async fn read_event_data_store(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_event_data_store()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a event_data_store resource
    async fn update_event_data_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags_list = input.get_optional_string("tags_list")?;
            let kms_key_id = input.get_optional_string("kms_key_id")?;
            let billing_mode = input.get_optional_string("billing_mode")?;
            let retention_period = input.get_optional_string("retention_period")?;
            let termination_protection_enabled =
                input.get_optional_string("termination_protection_enabled")?;
            let advanced_event_selectors = input.get_optional_string("advanced_event_selectors")?;
            let organization_enabled = input.get_optional_string("organization_enabled")?;
            let start_ingestion = input.get_optional_string("start_ingestion")?;
            let name = input.get_string("name")?;
            let multi_region_enabled = input.get_optional_string("multi_region_enabled")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_event_data_store()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("tags_list", tags_list.unwrap_or_default())
                .with_field("kms_key_id", kms_key_id.unwrap_or_default())
                .with_field("billing_mode", billing_mode.unwrap_or_default())
                .with_field("retention_period", retention_period.unwrap_or_default())
                .with_field(
                    "termination_protection_enabled",
                    termination_protection_enabled.unwrap_or_default(),
                )
                .with_field(
                    "advanced_event_selectors",
                    advanced_event_selectors.unwrap_or_default(),
                )
                .with_field(
                    "organization_enabled",
                    organization_enabled.unwrap_or_default(),
                )
                .with_field("start_ingestion", start_ingestion.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field(
                    "multi_region_enabled",
                    multi_region_enabled.unwrap_or_default(),
                ))
        })
    }

    /// Delete a event_data_store resource
    async fn delete_event_data_store(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_event_data_store()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Trail resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a trail resource
    async fn plan_trail(
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

    /// Create a new trail resource
    async fn create_trail(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cloud_watch_logs_log_group_arn =
                input.get_optional_string("cloud_watch_logs_log_group_arn")?;
            let is_organization_trail = input.get_optional_string("is_organization_trail")?;
            let sns_topic_name = input.get_optional_string("sns_topic_name")?;
            let s3_key_prefix = input.get_optional_string("s3_key_prefix")?;
            let enable_log_file_validation =
                input.get_optional_string("enable_log_file_validation")?;
            let cloud_watch_logs_role_arn =
                input.get_optional_string("cloud_watch_logs_role_arn")?;
            let tags_list = input.get_optional_string("tags_list")?;
            let is_multi_region_trail = input.get_optional_string("is_multi_region_trail")?;
            let kms_key_id = input.get_optional_string("kms_key_id")?;
            let name = input.get_string("name")?;
            let s3_bucket_name = input.get_string("s3_bucket_name")?;
            let include_global_service_events =
                input.get_optional_string("include_global_service_events")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_trail()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field(
                    "cloud_watch_logs_log_group_arn",
                    cloud_watch_logs_log_group_arn.unwrap_or_default(),
                )
                .with_field(
                    "is_organization_trail",
                    is_organization_trail.unwrap_or_default(),
                )
                .with_field("sns_topic_name", sns_topic_name.unwrap_or_default())
                .with_field("s3_key_prefix", s3_key_prefix.unwrap_or_default())
                .with_field(
                    "enable_log_file_validation",
                    enable_log_file_validation.unwrap_or_default(),
                )
                .with_field(
                    "cloud_watch_logs_role_arn",
                    cloud_watch_logs_role_arn.unwrap_or_default(),
                )
                .with_field("tags_list", tags_list.unwrap_or_default())
                .with_field(
                    "is_multi_region_trail",
                    is_multi_region_trail.unwrap_or_default(),
                )
                .with_field("kms_key_id", kms_key_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("s3_bucket_name", s3_bucket_name.unwrap_or_default())
                .with_field(
                    "include_global_service_events",
                    include_global_service_events.unwrap_or_default(),
                ))
        })
    }

    /// Read a trail resource
    async fn read_trail(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_trail()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a trail resource
    async fn update_trail(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cloud_watch_logs_log_group_arn =
                input.get_optional_string("cloud_watch_logs_log_group_arn")?;
            let is_organization_trail = input.get_optional_string("is_organization_trail")?;
            let sns_topic_name = input.get_optional_string("sns_topic_name")?;
            let s3_key_prefix = input.get_optional_string("s3_key_prefix")?;
            let enable_log_file_validation =
                input.get_optional_string("enable_log_file_validation")?;
            let cloud_watch_logs_role_arn =
                input.get_optional_string("cloud_watch_logs_role_arn")?;
            let tags_list = input.get_optional_string("tags_list")?;
            let is_multi_region_trail = input.get_optional_string("is_multi_region_trail")?;
            let kms_key_id = input.get_optional_string("kms_key_id")?;
            let name = input.get_string("name")?;
            let s3_bucket_name = input.get_string("s3_bucket_name")?;
            let include_global_service_events =
                input.get_optional_string("include_global_service_events")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_trail()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field(
                    "cloud_watch_logs_log_group_arn",
                    cloud_watch_logs_log_group_arn.unwrap_or_default(),
                )
                .with_field(
                    "is_organization_trail",
                    is_organization_trail.unwrap_or_default(),
                )
                .with_field("sns_topic_name", sns_topic_name.unwrap_or_default())
                .with_field("s3_key_prefix", s3_key_prefix.unwrap_or_default())
                .with_field(
                    "enable_log_file_validation",
                    enable_log_file_validation.unwrap_or_default(),
                )
                .with_field(
                    "cloud_watch_logs_role_arn",
                    cloud_watch_logs_role_arn.unwrap_or_default(),
                )
                .with_field("tags_list", tags_list.unwrap_or_default())
                .with_field(
                    "is_multi_region_trail",
                    is_multi_region_trail.unwrap_or_default(),
                )
                .with_field("kms_key_id", kms_key_id.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("s3_bucket_name", s3_bucket_name.unwrap_or_default())
                .with_field(
                    "include_global_service_events",
                    include_global_service_events.unwrap_or_default(),
                ))
        })
    }

    /// Delete a trail resource
    async fn delete_trail(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_trail()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Event_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a event_configuration resource
    async fn plan_event_configuration(
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

    /// Create a new event_configuration resource
    async fn create_event_configuration(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let context_key_selectors = input.get_string("context_key_selectors")?;
            let event_data_store = input.get_optional_string("event_data_store")?;
            let max_event_size = input.get_string("max_event_size")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_event_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field(
                    "context_key_selectors",
                    context_key_selectors.unwrap_or_default(),
                )
                .with_field("event_data_store", event_data_store.unwrap_or_default())
                .with_field("max_event_size", max_event_size.unwrap_or_default()))
        })
    }

    /// Read a event_configuration resource
    async fn read_event_configuration(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_event_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a event_configuration resource
    async fn update_event_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let context_key_selectors = input.get_string("context_key_selectors")?;
            let event_data_store = input.get_optional_string("event_data_store")?;
            let max_event_size = input.get_string("max_event_size")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_event_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field(
                    "context_key_selectors",
                    context_key_selectors.unwrap_or_default(),
                )
                .with_field("event_data_store", event_data_store.unwrap_or_default())
                .with_field("max_event_size", max_event_size.unwrap_or_default()))
        })
    }

    /// Delete a event_configuration resource
    async fn delete_event_configuration(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_event_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Dashboard resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dashboard resource
    async fn plan_dashboard(
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

    /// Create a new dashboard resource
    async fn create_dashboard(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let refresh_schedule = input.get_optional_string("refresh_schedule")?;
            let name = input.get_string("name")?;
            let tags_list = input.get_optional_string("tags_list")?;
            let termination_protection_enabled =
                input.get_optional_string("termination_protection_enabled")?;
            let widgets = input.get_optional_string("widgets")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_dashboard()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("refresh_schedule", refresh_schedule.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("tags_list", tags_list.unwrap_or_default())
                .with_field(
                    "termination_protection_enabled",
                    termination_protection_enabled.unwrap_or_default(),
                )
                .with_field("widgets", widgets.unwrap_or_default()))
        })
    }

    /// Read a dashboard resource
    async fn read_dashboard(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_dashboard()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a dashboard resource
    async fn update_dashboard(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let refresh_schedule = input.get_optional_string("refresh_schedule")?;
            let name = input.get_string("name")?;
            let tags_list = input.get_optional_string("tags_list")?;
            let termination_protection_enabled =
                input.get_optional_string("termination_protection_enabled")?;
            let widgets = input.get_optional_string("widgets")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_dashboard()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("refresh_schedule", refresh_schedule.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("tags_list", tags_list.unwrap_or_default())
                .with_field(
                    "termination_protection_enabled",
                    termination_protection_enabled.unwrap_or_default(),
                )
                .with_field("widgets", widgets.unwrap_or_default()))
        })
    }

    /// Delete a dashboard resource
    async fn delete_dashboard(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_dashboard()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Trail_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a trail_status resource
    async fn plan_trail_status(
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

    /// Create a new trail_status resource
    async fn create_trail_status(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_trail_status()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a trail_status resource
    async fn read_trail_status(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_trail_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a trail_status resource
    async fn update_trail_status(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_trail_status()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a trail_status resource
    async fn delete_trail_status(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_trail_status()
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
    async fn create_resource_policy(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_arn = input.get_string("resource_arn")?;
            let resource_policy = input.get_string("resource_policy")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_resource_policy()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("resource_arn", resource_arn.unwrap_or_default())
                .with_field("resource_policy", resource_policy.unwrap_or_default()))
        })
    }

    /// Read a resource_policy resource
    async fn read_resource_policy(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_resource_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
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

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
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
                .with_field("resource_policy", resource_policy.unwrap_or_default()))
        })
    }

    /// Delete a resource_policy resource
    async fn delete_resource_policy(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_resource_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Event_selectors resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a event_selectors resource
    async fn plan_event_selectors(
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

    /// Create a new event_selectors resource
    async fn create_event_selectors(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let advanced_event_selectors = input.get_optional_string("advanced_event_selectors")?;
            let event_selectors = input.get_optional_string("event_selectors")?;
            let trail_name = input.get_string("trail_name")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_event_selectors()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field(
                    "advanced_event_selectors",
                    advanced_event_selectors.unwrap_or_default(),
                )
                .with_field("event_selectors", event_selectors.unwrap_or_default())
                .with_field("trail_name", trail_name.unwrap_or_default()))
        })
    }

    /// Read a event_selectors resource
    async fn read_event_selectors(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_event_selectors()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a event_selectors resource
    async fn update_event_selectors(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let advanced_event_selectors = input.get_optional_string("advanced_event_selectors")?;
            let event_selectors = input.get_optional_string("event_selectors")?;
            let trail_name = input.get_string("trail_name")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_event_selectors()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field(
                    "advanced_event_selectors",
                    advanced_event_selectors.unwrap_or_default(),
                )
                .with_field("event_selectors", event_selectors.unwrap_or_default())
                .with_field("trail_name", trail_name.unwrap_or_default()))
        })
    }

    /// Delete a event_selectors resource
    async fn delete_event_selectors(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_event_selectors()
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
    async fn create_channel(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let destinations = input.get_string("destinations")?;
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;
            let source = input.get_string("source")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_channel()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("destinations", destinations.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("source", source.unwrap_or_default()))
        })
    }

    /// Read a channel resource
    async fn read_channel(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_channel()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a channel resource
    async fn update_channel(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let destinations = input.get_string("destinations")?;
            let tags = input.get_optional_string("tags")?;
            let name = input.get_string("name")?;
            let source = input.get_string("source")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_channel()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("destinations", destinations.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("source", source.unwrap_or_default()))
        })
    }

    /// Delete a channel resource
    async fn delete_channel(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_channel()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Trails resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a trails resource
    async fn plan_trails(
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

    /// Create a new trails resource
    async fn create_trails(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_trails()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a trails resource
    async fn read_trails(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_trails()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a trails resource
    async fn update_trails(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_trails()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a trails resource
    async fn delete_trails(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_trails()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Query_results resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a query_results resource
    async fn plan_query_results(
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

    /// Create a new query_results resource
    async fn create_query_results(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_query_results()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a query_results resource
    async fn read_query_results(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_query_results()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a query_results resource
    async fn update_query_results(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_query_results()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a query_results resource
    async fn delete_query_results(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_query_results()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Insight_selectors resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a insight_selectors resource
    async fn plan_insight_selectors(
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

    /// Create a new insight_selectors resource
    async fn create_insight_selectors(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let insights_destination = input.get_optional_string("insights_destination")?;
            let insight_selectors = input.get_string("insight_selectors")?;
            let event_data_store = input.get_optional_string("event_data_store")?;
            let trail_name = input.get_optional_string("trail_name")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_insight_selectors()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field(
                    "insights_destination",
                    insights_destination.unwrap_or_default(),
                )
                .with_field("insight_selectors", insight_selectors.unwrap_or_default())
                .with_field("event_data_store", event_data_store.unwrap_or_default())
                .with_field("trail_name", trail_name.unwrap_or_default()))
        })
    }

    /// Read a insight_selectors resource
    async fn read_insight_selectors(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_insight_selectors()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a insight_selectors resource
    async fn update_insight_selectors(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let insights_destination = input.get_optional_string("insights_destination")?;
            let insight_selectors = input.get_string("insight_selectors")?;
            let event_data_store = input.get_optional_string("event_data_store")?;
            let trail_name = input.get_optional_string("trail_name")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_insight_selectors()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field(
                    "insights_destination",
                    insights_destination.unwrap_or_default(),
                )
                .with_field("insight_selectors", insight_selectors.unwrap_or_default())
                .with_field("event_data_store", event_data_store.unwrap_or_default())
                .with_field("trail_name", trail_name.unwrap_or_default()))
        })
    }

    /// Delete a insight_selectors resource
    async fn delete_insight_selectors(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_insight_selectors()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Import resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a import resource
    async fn plan_import(
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

    /// Create a new import resource
    async fn create_import(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .create_import()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a import resource
    async fn read_import(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .describe_import()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a import resource
    async fn update_import(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudtrail_client
            //     .update_import()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a import resource
    async fn delete_import(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudtrail_client
            //     .delete_import()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }
}
