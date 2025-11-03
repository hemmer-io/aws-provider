//! Waf service for Aws provider
//!
//! This module handles all waf resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Waf service handler
pub struct WafService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> WafService<'a> {
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
            "rate_based_rule" => {
                self.plan_rate_based_rule(current_state, desired_input).await
            }
            "regex_pattern_set" => {
                self.plan_regex_pattern_set(current_state, desired_input).await
            }
            "xss_match_set" => {
                self.plan_xss_match_set(current_state, desired_input).await
            }
            "sampled_requests" => {
                self.plan_sampled_requests(current_state, desired_input).await
            }
            "rule" => {
                self.plan_rule(current_state, desired_input).await
            }
            "rule_group" => {
                self.plan_rule_group(current_state, desired_input).await
            }
            "byte_match_set" => {
                self.plan_byte_match_set(current_state, desired_input).await
            }
            "regex_match_set" => {
                self.plan_regex_match_set(current_state, desired_input).await
            }
            "size_constraint_set" => {
                self.plan_size_constraint_set(current_state, desired_input).await
            }
            "web_acl" => {
                self.plan_web_acl(current_state, desired_input).await
            }
            "geo_match_set" => {
                self.plan_geo_match_set(current_state, desired_input).await
            }
            "permission_policy" => {
                self.plan_permission_policy(current_state, desired_input).await
            }
            "ip_set" => {
                self.plan_ip_set(current_state, desired_input).await
            }
            "logging_configuration" => {
                self.plan_logging_configuration(current_state, desired_input).await
            }
            "sql_injection_match_set" => {
                self.plan_sql_injection_match_set(current_state, desired_input).await
            }
            "rate_based_rule_managed_keys" => {
                self.plan_rate_based_rule_managed_keys(current_state, desired_input).await
            }
            "web_acl_migration_stack" => {
                self.plan_web_acl_migration_stack(current_state, desired_input).await
            }
            "change_token_status" => {
                self.plan_change_token_status(current_state, desired_input).await
            }
            "change_token" => {
                self.plan_change_token(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "waf",
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
            "rate_based_rule" => {
                self.create_rate_based_rule(input).await
            }
            "regex_pattern_set" => {
                self.create_regex_pattern_set(input).await
            }
            "xss_match_set" => {
                self.create_xss_match_set(input).await
            }
            "sampled_requests" => {
                self.create_sampled_requests(input).await
            }
            "rule" => {
                self.create_rule(input).await
            }
            "rule_group" => {
                self.create_rule_group(input).await
            }
            "byte_match_set" => {
                self.create_byte_match_set(input).await
            }
            "regex_match_set" => {
                self.create_regex_match_set(input).await
            }
            "size_constraint_set" => {
                self.create_size_constraint_set(input).await
            }
            "web_acl" => {
                self.create_web_acl(input).await
            }
            "geo_match_set" => {
                self.create_geo_match_set(input).await
            }
            "permission_policy" => {
                self.create_permission_policy(input).await
            }
            "ip_set" => {
                self.create_ip_set(input).await
            }
            "logging_configuration" => {
                self.create_logging_configuration(input).await
            }
            "sql_injection_match_set" => {
                self.create_sql_injection_match_set(input).await
            }
            "rate_based_rule_managed_keys" => {
                self.create_rate_based_rule_managed_keys(input).await
            }
            "web_acl_migration_stack" => {
                self.create_web_acl_migration_stack(input).await
            }
            "change_token_status" => {
                self.create_change_token_status(input).await
            }
            "change_token" => {
                self.create_change_token(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "waf",
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
            "rate_based_rule" => {
                self.read_rate_based_rule(id).await
            }
            "regex_pattern_set" => {
                self.read_regex_pattern_set(id).await
            }
            "xss_match_set" => {
                self.read_xss_match_set(id).await
            }
            "sampled_requests" => {
                self.read_sampled_requests(id).await
            }
            "rule" => {
                self.read_rule(id).await
            }
            "rule_group" => {
                self.read_rule_group(id).await
            }
            "byte_match_set" => {
                self.read_byte_match_set(id).await
            }
            "regex_match_set" => {
                self.read_regex_match_set(id).await
            }
            "size_constraint_set" => {
                self.read_size_constraint_set(id).await
            }
            "web_acl" => {
                self.read_web_acl(id).await
            }
            "geo_match_set" => {
                self.read_geo_match_set(id).await
            }
            "permission_policy" => {
                self.read_permission_policy(id).await
            }
            "ip_set" => {
                self.read_ip_set(id).await
            }
            "logging_configuration" => {
                self.read_logging_configuration(id).await
            }
            "sql_injection_match_set" => {
                self.read_sql_injection_match_set(id).await
            }
            "rate_based_rule_managed_keys" => {
                self.read_rate_based_rule_managed_keys(id).await
            }
            "web_acl_migration_stack" => {
                self.read_web_acl_migration_stack(id).await
            }
            "change_token_status" => {
                self.read_change_token_status(id).await
            }
            "change_token" => {
                self.read_change_token(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "waf",
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
            "rate_based_rule" => {
                self.update_rate_based_rule(id, input).await
            }
            "regex_pattern_set" => {
                self.update_regex_pattern_set(id, input).await
            }
            "xss_match_set" => {
                self.update_xss_match_set(id, input).await
            }
            "sampled_requests" => {
                self.update_sampled_requests(id, input).await
            }
            "rule" => {
                self.update_rule(id, input).await
            }
            "rule_group" => {
                self.update_rule_group(id, input).await
            }
            "byte_match_set" => {
                self.update_byte_match_set(id, input).await
            }
            "regex_match_set" => {
                self.update_regex_match_set(id, input).await
            }
            "size_constraint_set" => {
                self.update_size_constraint_set(id, input).await
            }
            "web_acl" => {
                self.update_web_acl(id, input).await
            }
            "geo_match_set" => {
                self.update_geo_match_set(id, input).await
            }
            "permission_policy" => {
                self.update_permission_policy(id, input).await
            }
            "ip_set" => {
                self.update_ip_set(id, input).await
            }
            "logging_configuration" => {
                self.update_logging_configuration(id, input).await
            }
            "sql_injection_match_set" => {
                self.update_sql_injection_match_set(id, input).await
            }
            "rate_based_rule_managed_keys" => {
                self.update_rate_based_rule_managed_keys(id, input).await
            }
            "web_acl_migration_stack" => {
                self.update_web_acl_migration_stack(id, input).await
            }
            "change_token_status" => {
                self.update_change_token_status(id, input).await
            }
            "change_token" => {
                self.update_change_token(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "waf",
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
            "rate_based_rule" => {
                self.delete_rate_based_rule(id).await
            }
            "regex_pattern_set" => {
                self.delete_regex_pattern_set(id).await
            }
            "xss_match_set" => {
                self.delete_xss_match_set(id).await
            }
            "sampled_requests" => {
                self.delete_sampled_requests(id).await
            }
            "rule" => {
                self.delete_rule(id).await
            }
            "rule_group" => {
                self.delete_rule_group(id).await
            }
            "byte_match_set" => {
                self.delete_byte_match_set(id).await
            }
            "regex_match_set" => {
                self.delete_regex_match_set(id).await
            }
            "size_constraint_set" => {
                self.delete_size_constraint_set(id).await
            }
            "web_acl" => {
                self.delete_web_acl(id).await
            }
            "geo_match_set" => {
                self.delete_geo_match_set(id).await
            }
            "permission_policy" => {
                self.delete_permission_policy(id).await
            }
            "ip_set" => {
                self.delete_ip_set(id).await
            }
            "logging_configuration" => {
                self.delete_logging_configuration(id).await
            }
            "sql_injection_match_set" => {
                self.delete_sql_injection_match_set(id).await
            }
            "rate_based_rule_managed_keys" => {
                self.delete_rate_based_rule_managed_keys(id).await
            }
            "web_acl_migration_stack" => {
                self.delete_web_acl_migration_stack(id).await
            }
            "change_token_status" => {
                self.delete_change_token_status(id).await
            }
            "change_token" => {
                self.delete_change_token(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "waf",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Rate_based_rule resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rate_based_rule resource
    async fn plan_rate_based_rule(
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

    /// Create a new rate_based_rule resource
    async fn create_rate_based_rule(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let rate_key = input.get_string("rate_key")?;
            let change_token = input.get_string("change_token")?;
            let rate_limit = input.get_string("rate_limit")?;
            let tags = input.get_optional_string("tags")?;
            let metric_name = input.get_string("metric_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_rate_based_rule()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("rate_key", rate_key.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("rate_limit", rate_limit.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
            )
        })
    }

    /// Read a rate_based_rule resource
    async fn read_rate_based_rule(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_rate_based_rule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a rate_based_rule resource
    async fn update_rate_based_rule(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let rate_key = input.get_string("rate_key")?;
            let change_token = input.get_string("change_token")?;
            let rate_limit = input.get_string("rate_limit")?;
            let tags = input.get_optional_string("tags")?;
            let metric_name = input.get_string("metric_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_rate_based_rule()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("rate_key", rate_key.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("rate_limit", rate_limit.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
            )
        })
    }

    /// Delete a rate_based_rule resource
    async fn delete_rate_based_rule(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_rate_based_rule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Regex_pattern_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a regex_pattern_set resource
    async fn plan_regex_pattern_set(
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

    /// Create a new regex_pattern_set resource
    async fn create_regex_pattern_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let change_token = input.get_string("change_token")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_regex_pattern_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a regex_pattern_set resource
    async fn read_regex_pattern_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_regex_pattern_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a regex_pattern_set resource
    async fn update_regex_pattern_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let change_token = input.get_string("change_token")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_regex_pattern_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a regex_pattern_set resource
    async fn delete_regex_pattern_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_regex_pattern_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Xss_match_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a xss_match_set resource
    async fn plan_xss_match_set(
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

    /// Create a new xss_match_set resource
    async fn create_xss_match_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let change_token = input.get_string("change_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_xss_match_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
            )
        })
    }

    /// Read a xss_match_set resource
    async fn read_xss_match_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_xss_match_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a xss_match_set resource
    async fn update_xss_match_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let change_token = input.get_string("change_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_xss_match_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
            )
        })
    }

    /// Delete a xss_match_set resource
    async fn delete_xss_match_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_xss_match_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Sampled_requests resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sampled_requests resource
    async fn plan_sampled_requests(
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

    /// Create a new sampled_requests resource
    async fn create_sampled_requests(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_sampled_requests()
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

    /// Read a sampled_requests resource
    async fn read_sampled_requests(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_sampled_requests()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a sampled_requests resource
    async fn update_sampled_requests(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_sampled_requests()
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

    /// Delete a sampled_requests resource
    async fn delete_sampled_requests(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_sampled_requests()
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
            let change_token = input.get_string("change_token")?;
            let metric_name = input.get_string("metric_name")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_rule()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
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
            // let result = self.provider.waf_client
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
            let change_token = input.get_string("change_token")?;
            let metric_name = input.get_string("metric_name")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
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
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
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
            // self.provider.waf_client
            //     .delete_rule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Rule_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rule_group resource
    async fn plan_rule_group(
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

    /// Create a new rule_group resource
    async fn create_rule_group(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let metric_name = input.get_string("metric_name")?;
            let change_token = input.get_string("change_token")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_rule_group()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("tags", tags.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a rule_group resource
    async fn read_rule_group(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_rule_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a rule_group resource
    async fn update_rule_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let metric_name = input.get_string("metric_name")?;
            let change_token = input.get_string("change_token")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_rule_group()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("tags", tags.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a rule_group resource
    async fn delete_rule_group(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_rule_group()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Byte_match_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a byte_match_set resource
    async fn plan_byte_match_set(
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

    /// Create a new byte_match_set resource
    async fn create_byte_match_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let change_token = input.get_string("change_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_byte_match_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
            )
        })
    }

    /// Read a byte_match_set resource
    async fn read_byte_match_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_byte_match_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a byte_match_set resource
    async fn update_byte_match_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let change_token = input.get_string("change_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_byte_match_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
            )
        })
    }

    /// Delete a byte_match_set resource
    async fn delete_byte_match_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_byte_match_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Regex_match_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a regex_match_set resource
    async fn plan_regex_match_set(
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

    /// Create a new regex_match_set resource
    async fn create_regex_match_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let change_token = input.get_string("change_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_regex_match_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
            )
        })
    }

    /// Read a regex_match_set resource
    async fn read_regex_match_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_regex_match_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a regex_match_set resource
    async fn update_regex_match_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let change_token = input.get_string("change_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_regex_match_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
            )
        })
    }

    /// Delete a regex_match_set resource
    async fn delete_regex_match_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_regex_match_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Size_constraint_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a size_constraint_set resource
    async fn plan_size_constraint_set(
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

    /// Create a new size_constraint_set resource
    async fn create_size_constraint_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let change_token = input.get_string("change_token")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_size_constraint_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a size_constraint_set resource
    async fn read_size_constraint_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_size_constraint_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a size_constraint_set resource
    async fn update_size_constraint_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let change_token = input.get_string("change_token")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_size_constraint_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a size_constraint_set resource
    async fn delete_size_constraint_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_size_constraint_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Web_acl resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a web_acl resource
    async fn plan_web_acl(
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

    /// Create a new web_acl resource
    async fn create_web_acl(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let change_token = input.get_string("change_token")?;
            let default_action = input.get_string("default_action")?;
            let metric_name = input.get_string("metric_name")?;
            let name = input.get_string("name")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_web_acl()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("default_action", default_action.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Read a web_acl resource
    async fn read_web_acl(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_web_acl()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a web_acl resource
    async fn update_web_acl(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let change_token = input.get_string("change_token")?;
            let default_action = input.get_string("default_action")?;
            let metric_name = input.get_string("metric_name")?;
            let name = input.get_string("name")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_web_acl()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("default_action", default_action.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Delete a web_acl resource
    async fn delete_web_acl(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_web_acl()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Geo_match_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a geo_match_set resource
    async fn plan_geo_match_set(
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

    /// Create a new geo_match_set resource
    async fn create_geo_match_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let change_token = input.get_string("change_token")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_geo_match_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a geo_match_set resource
    async fn read_geo_match_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_geo_match_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a geo_match_set resource
    async fn update_geo_match_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let change_token = input.get_string("change_token")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_geo_match_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a geo_match_set resource
    async fn delete_geo_match_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_geo_match_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Permission_policy resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a permission_policy resource
    async fn plan_permission_policy(
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

    /// Create a new permission_policy resource
    async fn create_permission_policy(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let policy = input.get_string("policy")?;
            let resource_arn = input.get_string("resource_arn")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_permission_policy()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("policy", policy.unwrap_or_default())
                .with_field("resource_arn", resource_arn.unwrap_or_default())
            )
        })
    }

    /// Read a permission_policy resource
    async fn read_permission_policy(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_permission_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a permission_policy resource
    async fn update_permission_policy(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let policy = input.get_string("policy")?;
            let resource_arn = input.get_string("resource_arn")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_permission_policy()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("policy", policy.unwrap_or_default())
                .with_field("resource_arn", resource_arn.unwrap_or_default())
            )
        })
    }

    /// Delete a permission_policy resource
    async fn delete_permission_policy(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_permission_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Ip_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ip_set resource
    async fn plan_ip_set(
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

    /// Create a new ip_set resource
    async fn create_ip_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let change_token = input.get_string("change_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_ip_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
            )
        })
    }

    /// Read a ip_set resource
    async fn read_ip_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_ip_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a ip_set resource
    async fn update_ip_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let change_token = input.get_string("change_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_ip_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("change_token", change_token.unwrap_or_default())
            )
        })
    }

    /// Delete a ip_set resource
    async fn delete_ip_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_ip_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Logging_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a logging_configuration resource
    async fn plan_logging_configuration(
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

    /// Create a new logging_configuration resource
    async fn create_logging_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let logging_configuration = input.get_string("logging_configuration")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_logging_configuration()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("logging_configuration", logging_configuration.unwrap_or_default())
            )
        })
    }

    /// Read a logging_configuration resource
    async fn read_logging_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_logging_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a logging_configuration resource
    async fn update_logging_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let logging_configuration = input.get_string("logging_configuration")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_logging_configuration()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("logging_configuration", logging_configuration.unwrap_or_default())
            )
        })
    }

    /// Delete a logging_configuration resource
    async fn delete_logging_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_logging_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Sql_injection_match_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sql_injection_match_set resource
    async fn plan_sql_injection_match_set(
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

    /// Create a new sql_injection_match_set resource
    async fn create_sql_injection_match_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let change_token = input.get_string("change_token")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_sql_injection_match_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Read a sql_injection_match_set resource
    async fn read_sql_injection_match_set(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_sql_injection_match_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a sql_injection_match_set resource
    async fn update_sql_injection_match_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let change_token = input.get_string("change_token")?;
            let name = input.get_string("name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_sql_injection_match_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("change_token", change_token.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
            )
        })
    }

    /// Delete a sql_injection_match_set resource
    async fn delete_sql_injection_match_set(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_sql_injection_match_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Rate_based_rule_managed_keys resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rate_based_rule_managed_keys resource
    async fn plan_rate_based_rule_managed_keys(
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

    /// Create a new rate_based_rule_managed_keys resource
    async fn create_rate_based_rule_managed_keys(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_rate_based_rule_managed_keys()
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

    /// Read a rate_based_rule_managed_keys resource
    async fn read_rate_based_rule_managed_keys(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_rate_based_rule_managed_keys()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a rate_based_rule_managed_keys resource
    async fn update_rate_based_rule_managed_keys(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_rate_based_rule_managed_keys()
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

    /// Delete a rate_based_rule_managed_keys resource
    async fn delete_rate_based_rule_managed_keys(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_rate_based_rule_managed_keys()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Web_acl_migration_stack resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a web_acl_migration_stack resource
    async fn plan_web_acl_migration_stack(
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

    /// Create a new web_acl_migration_stack resource
    async fn create_web_acl_migration_stack(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let s3_bucket_name = input.get_string("s3_bucket_name")?;
            let ignore_unsupported_type = input.get_string("ignore_unsupported_type")?;
            let web_acl_id = input.get_string("web_acl_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_web_acl_migration_stack()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("s3_bucket_name", s3_bucket_name.unwrap_or_default())
                .with_field("ignore_unsupported_type", ignore_unsupported_type.unwrap_or_default())
                .with_field("web_acl_id", web_acl_id.unwrap_or_default())
            )
        })
    }

    /// Read a web_acl_migration_stack resource
    async fn read_web_acl_migration_stack(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_web_acl_migration_stack()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a web_acl_migration_stack resource
    async fn update_web_acl_migration_stack(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let s3_bucket_name = input.get_string("s3_bucket_name")?;
            let ignore_unsupported_type = input.get_string("ignore_unsupported_type")?;
            let web_acl_id = input.get_string("web_acl_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_web_acl_migration_stack()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("s3_bucket_name", s3_bucket_name.unwrap_or_default())
                .with_field("ignore_unsupported_type", ignore_unsupported_type.unwrap_or_default())
                .with_field("web_acl_id", web_acl_id.unwrap_or_default())
            )
        })
    }

    /// Delete a web_acl_migration_stack resource
    async fn delete_web_acl_migration_stack(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_web_acl_migration_stack()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Change_token_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a change_token_status resource
    async fn plan_change_token_status(
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

    /// Create a new change_token_status resource
    async fn create_change_token_status(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_change_token_status()
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

    /// Read a change_token_status resource
    async fn read_change_token_status(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_change_token_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a change_token_status resource
    async fn update_change_token_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_change_token_status()
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

    /// Delete a change_token_status resource
    async fn delete_change_token_status(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_change_token_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Change_token resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a change_token resource
    async fn plan_change_token(
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

    /// Create a new change_token resource
    async fn create_change_token(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.waf_client
            //     .create_change_token()
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

    /// Read a change_token resource
    async fn read_change_token(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.waf_client
            //     .describe_change_token()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a change_token resource
    async fn update_change_token(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.waf_client
            //     .update_change_token()
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

    /// Delete a change_token resource
    async fn delete_change_token(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.waf_client
            //     .delete_change_token()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
