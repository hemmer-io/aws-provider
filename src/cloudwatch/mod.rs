//! Cloudwatch service for Aws provider
//!
//! This module handles all cloudwatch resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Cloudwatch service handler
pub struct CloudwatchService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CloudwatchService<'a> {
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
            "metric_widget_image" => {
                self.plan_metric_widget_image(current_state, desired_input).await
            }
            "anomaly_detector" => {
                self.plan_anomaly_detector(current_state, desired_input).await
            }
            "insight_rule" => {
                self.plan_insight_rule(current_state, desired_input).await
            }
            "alarms" => {
                self.plan_alarms(current_state, desired_input).await
            }
            "metric_alarm" => {
                self.plan_metric_alarm(current_state, desired_input).await
            }
            "alarms_for_metric" => {
                self.plan_alarms_for_metric(current_state, desired_input).await
            }
            "metric_stream" => {
                self.plan_metric_stream(current_state, desired_input).await
            }
            "dashboards" => {
                self.plan_dashboards(current_state, desired_input).await
            }
            "metric_statistics" => {
                self.plan_metric_statistics(current_state, desired_input).await
            }
            "managed_insight_rules" => {
                self.plan_managed_insight_rules(current_state, desired_input).await
            }
            "alarm_contributors" => {
                self.plan_alarm_contributors(current_state, desired_input).await
            }
            "insight_rule_report" => {
                self.plan_insight_rule_report(current_state, desired_input).await
            }
            "insight_rules" => {
                self.plan_insight_rules(current_state, desired_input).await
            }
            "alarm_history" => {
                self.plan_alarm_history(current_state, desired_input).await
            }
            "dashboard" => {
                self.plan_dashboard(current_state, desired_input).await
            }
            "composite_alarm" => {
                self.plan_composite_alarm(current_state, desired_input).await
            }
            "metric_data" => {
                self.plan_metric_data(current_state, desired_input).await
            }
            "anomaly_detectors" => {
                self.plan_anomaly_detectors(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudwatch",
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
            "metric_widget_image" => {
                self.create_metric_widget_image(input).await
            }
            "anomaly_detector" => {
                self.create_anomaly_detector(input).await
            }
            "insight_rule" => {
                self.create_insight_rule(input).await
            }
            "alarms" => {
                self.create_alarms(input).await
            }
            "metric_alarm" => {
                self.create_metric_alarm(input).await
            }
            "alarms_for_metric" => {
                self.create_alarms_for_metric(input).await
            }
            "metric_stream" => {
                self.create_metric_stream(input).await
            }
            "dashboards" => {
                self.create_dashboards(input).await
            }
            "metric_statistics" => {
                self.create_metric_statistics(input).await
            }
            "managed_insight_rules" => {
                self.create_managed_insight_rules(input).await
            }
            "alarm_contributors" => {
                self.create_alarm_contributors(input).await
            }
            "insight_rule_report" => {
                self.create_insight_rule_report(input).await
            }
            "insight_rules" => {
                self.create_insight_rules(input).await
            }
            "alarm_history" => {
                self.create_alarm_history(input).await
            }
            "dashboard" => {
                self.create_dashboard(input).await
            }
            "composite_alarm" => {
                self.create_composite_alarm(input).await
            }
            "metric_data" => {
                self.create_metric_data(input).await
            }
            "anomaly_detectors" => {
                self.create_anomaly_detectors(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudwatch",
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
            "metric_widget_image" => {
                self.read_metric_widget_image(id).await
            }
            "anomaly_detector" => {
                self.read_anomaly_detector(id).await
            }
            "insight_rule" => {
                self.read_insight_rule(id).await
            }
            "alarms" => {
                self.read_alarms(id).await
            }
            "metric_alarm" => {
                self.read_metric_alarm(id).await
            }
            "alarms_for_metric" => {
                self.read_alarms_for_metric(id).await
            }
            "metric_stream" => {
                self.read_metric_stream(id).await
            }
            "dashboards" => {
                self.read_dashboards(id).await
            }
            "metric_statistics" => {
                self.read_metric_statistics(id).await
            }
            "managed_insight_rules" => {
                self.read_managed_insight_rules(id).await
            }
            "alarm_contributors" => {
                self.read_alarm_contributors(id).await
            }
            "insight_rule_report" => {
                self.read_insight_rule_report(id).await
            }
            "insight_rules" => {
                self.read_insight_rules(id).await
            }
            "alarm_history" => {
                self.read_alarm_history(id).await
            }
            "dashboard" => {
                self.read_dashboard(id).await
            }
            "composite_alarm" => {
                self.read_composite_alarm(id).await
            }
            "metric_data" => {
                self.read_metric_data(id).await
            }
            "anomaly_detectors" => {
                self.read_anomaly_detectors(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudwatch",
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
            "metric_widget_image" => {
                self.update_metric_widget_image(id, input).await
            }
            "anomaly_detector" => {
                self.update_anomaly_detector(id, input).await
            }
            "insight_rule" => {
                self.update_insight_rule(id, input).await
            }
            "alarms" => {
                self.update_alarms(id, input).await
            }
            "metric_alarm" => {
                self.update_metric_alarm(id, input).await
            }
            "alarms_for_metric" => {
                self.update_alarms_for_metric(id, input).await
            }
            "metric_stream" => {
                self.update_metric_stream(id, input).await
            }
            "dashboards" => {
                self.update_dashboards(id, input).await
            }
            "metric_statistics" => {
                self.update_metric_statistics(id, input).await
            }
            "managed_insight_rules" => {
                self.update_managed_insight_rules(id, input).await
            }
            "alarm_contributors" => {
                self.update_alarm_contributors(id, input).await
            }
            "insight_rule_report" => {
                self.update_insight_rule_report(id, input).await
            }
            "insight_rules" => {
                self.update_insight_rules(id, input).await
            }
            "alarm_history" => {
                self.update_alarm_history(id, input).await
            }
            "dashboard" => {
                self.update_dashboard(id, input).await
            }
            "composite_alarm" => {
                self.update_composite_alarm(id, input).await
            }
            "metric_data" => {
                self.update_metric_data(id, input).await
            }
            "anomaly_detectors" => {
                self.update_anomaly_detectors(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudwatch",
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
            "metric_widget_image" => {
                self.delete_metric_widget_image(id).await
            }
            "anomaly_detector" => {
                self.delete_anomaly_detector(id).await
            }
            "insight_rule" => {
                self.delete_insight_rule(id).await
            }
            "alarms" => {
                self.delete_alarms(id).await
            }
            "metric_alarm" => {
                self.delete_metric_alarm(id).await
            }
            "alarms_for_metric" => {
                self.delete_alarms_for_metric(id).await
            }
            "metric_stream" => {
                self.delete_metric_stream(id).await
            }
            "dashboards" => {
                self.delete_dashboards(id).await
            }
            "metric_statistics" => {
                self.delete_metric_statistics(id).await
            }
            "managed_insight_rules" => {
                self.delete_managed_insight_rules(id).await
            }
            "alarm_contributors" => {
                self.delete_alarm_contributors(id).await
            }
            "insight_rule_report" => {
                self.delete_insight_rule_report(id).await
            }
            "insight_rules" => {
                self.delete_insight_rules(id).await
            }
            "alarm_history" => {
                self.delete_alarm_history(id).await
            }
            "dashboard" => {
                self.delete_dashboard(id).await
            }
            "composite_alarm" => {
                self.delete_composite_alarm(id).await
            }
            "metric_data" => {
                self.delete_metric_data(id).await
            }
            "anomaly_detectors" => {
                self.delete_anomaly_detectors(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudwatch",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Metric_widget_image resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a metric_widget_image resource
    async fn plan_metric_widget_image(
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

    /// Create a new metric_widget_image resource
    async fn create_metric_widget_image(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_metric_widget_image()
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

    /// Read a metric_widget_image resource
    async fn read_metric_widget_image(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_metric_widget_image()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a metric_widget_image resource
    async fn update_metric_widget_image(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_metric_widget_image()
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

    /// Delete a metric_widget_image resource
    async fn delete_metric_widget_image(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_metric_widget_image()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Anomaly_detector resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a anomaly_detector resource
    async fn plan_anomaly_detector(
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

    /// Create a new anomaly_detector resource
    async fn create_anomaly_detector(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let dimensions = input.get_optional_string("dimensions")?;
            let single_metric_anomaly_detector = input.get_optional_string("single_metric_anomaly_detector")?;
            let stat = input.get_optional_string("stat")?;
            let metric_name = input.get_optional_string("metric_name")?;
            let configuration = input.get_optional_string("configuration")?;
            let metric_characteristics = input.get_optional_string("metric_characteristics")?;
            let metric_math_anomaly_detector = input.get_optional_string("metric_math_anomaly_detector")?;
            let namespace = input.get_optional_string("namespace")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_anomaly_detector()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("dimensions", dimensions.unwrap_or_default())
                .with_field("single_metric_anomaly_detector", single_metric_anomaly_detector.unwrap_or_default())
                .with_field("stat", stat.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
                .with_field("configuration", configuration.unwrap_or_default())
                .with_field("metric_characteristics", metric_characteristics.unwrap_or_default())
                .with_field("metric_math_anomaly_detector", metric_math_anomaly_detector.unwrap_or_default())
                .with_field("namespace", namespace.unwrap_or_default())
            )
        })
    }

    /// Read a anomaly_detector resource
    async fn read_anomaly_detector(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_anomaly_detector()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a anomaly_detector resource
    async fn update_anomaly_detector(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let dimensions = input.get_optional_string("dimensions")?;
            let single_metric_anomaly_detector = input.get_optional_string("single_metric_anomaly_detector")?;
            let stat = input.get_optional_string("stat")?;
            let metric_name = input.get_optional_string("metric_name")?;
            let configuration = input.get_optional_string("configuration")?;
            let metric_characteristics = input.get_optional_string("metric_characteristics")?;
            let metric_math_anomaly_detector = input.get_optional_string("metric_math_anomaly_detector")?;
            let namespace = input.get_optional_string("namespace")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_anomaly_detector()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("dimensions", dimensions.unwrap_or_default())
                .with_field("single_metric_anomaly_detector", single_metric_anomaly_detector.unwrap_or_default())
                .with_field("stat", stat.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
                .with_field("configuration", configuration.unwrap_or_default())
                .with_field("metric_characteristics", metric_characteristics.unwrap_or_default())
                .with_field("metric_math_anomaly_detector", metric_math_anomaly_detector.unwrap_or_default())
                .with_field("namespace", namespace.unwrap_or_default())
            )
        })
    }

    /// Delete a anomaly_detector resource
    async fn delete_anomaly_detector(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_anomaly_detector()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Insight_rule resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a insight_rule resource
    async fn plan_insight_rule(
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

    /// Create a new insight_rule resource
    async fn create_insight_rule(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let apply_on_transformed_logs = input.get_optional_string("apply_on_transformed_logs")?;
            let rule_state = input.get_optional_string("rule_state")?;
            let rule_name = input.get_string("rule_name")?;
            let tags = input.get_optional_string("tags")?;
            let rule_definition = input.get_string("rule_definition")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_insight_rule()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("apply_on_transformed_logs", apply_on_transformed_logs.unwrap_or_default())
                .with_field("rule_state", rule_state.unwrap_or_default())
                .with_field("rule_name", rule_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("rule_definition", rule_definition.unwrap_or_default())
            )
        })
    }

    /// Read a insight_rule resource
    async fn read_insight_rule(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_insight_rule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a insight_rule resource
    async fn update_insight_rule(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let apply_on_transformed_logs = input.get_optional_string("apply_on_transformed_logs")?;
            let rule_state = input.get_optional_string("rule_state")?;
            let rule_name = input.get_string("rule_name")?;
            let tags = input.get_optional_string("tags")?;
            let rule_definition = input.get_string("rule_definition")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_insight_rule()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("apply_on_transformed_logs", apply_on_transformed_logs.unwrap_or_default())
                .with_field("rule_state", rule_state.unwrap_or_default())
                .with_field("rule_name", rule_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("rule_definition", rule_definition.unwrap_or_default())
            )
        })
    }

    /// Delete a insight_rule resource
    async fn delete_insight_rule(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_insight_rule()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Alarms resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a alarms resource
    async fn plan_alarms(
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

    /// Create a new alarms resource
    async fn create_alarms(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_alarms()
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

    /// Read a alarms resource
    async fn read_alarms(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_alarms()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a alarms resource
    async fn update_alarms(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_alarms()
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

    /// Delete a alarms resource
    async fn delete_alarms(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_alarms()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Metric_alarm resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a metric_alarm resource
    async fn plan_metric_alarm(
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

    /// Create a new metric_alarm resource
    async fn create_metric_alarm(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let alarm_name = input.get_string("alarm_name")?;
            let ok_actions = input.get_optional_string("ok_actions")?;
            let actions_enabled = input.get_optional_string("actions_enabled")?;
            let insufficient_data_actions = input.get_optional_string("insufficient_data_actions")?;
            let alarm_description = input.get_optional_string("alarm_description")?;
            let namespace = input.get_optional_string("namespace")?;
            let dimensions = input.get_optional_string("dimensions")?;
            let threshold = input.get_optional_string("threshold")?;
            let statistic = input.get_optional_string("statistic")?;
            let evaluate_low_sample_count_percentile = input.get_optional_string("evaluate_low_sample_count_percentile")?;
            let treat_missing_data = input.get_optional_string("treat_missing_data")?;
            let metric_name = input.get_optional_string("metric_name")?;
            let alarm_actions = input.get_optional_string("alarm_actions")?;
            let evaluation_periods = input.get_string("evaluation_periods")?;
            let comparison_operator = input.get_string("comparison_operator")?;
            let metrics = input.get_optional_string("metrics")?;
            let datapoints_to_alarm = input.get_optional_string("datapoints_to_alarm")?;
            let tags = input.get_optional_string("tags")?;
            let threshold_metric_id = input.get_optional_string("threshold_metric_id")?;
            let extended_statistic = input.get_optional_string("extended_statistic")?;
            let period = input.get_optional_string("period")?;
            let unit = input.get_optional_string("unit")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_metric_alarm()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("alarm_name", alarm_name.unwrap_or_default())
                .with_field("ok_actions", ok_actions.unwrap_or_default())
                .with_field("actions_enabled", actions_enabled.unwrap_or_default())
                .with_field("insufficient_data_actions", insufficient_data_actions.unwrap_or_default())
                .with_field("alarm_description", alarm_description.unwrap_or_default())
                .with_field("namespace", namespace.unwrap_or_default())
                .with_field("dimensions", dimensions.unwrap_or_default())
                .with_field("threshold", threshold.unwrap_or_default())
                .with_field("statistic", statistic.unwrap_or_default())
                .with_field("evaluate_low_sample_count_percentile", evaluate_low_sample_count_percentile.unwrap_or_default())
                .with_field("treat_missing_data", treat_missing_data.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
                .with_field("alarm_actions", alarm_actions.unwrap_or_default())
                .with_field("evaluation_periods", evaluation_periods.unwrap_or_default())
                .with_field("comparison_operator", comparison_operator.unwrap_or_default())
                .with_field("metrics", metrics.unwrap_or_default())
                .with_field("datapoints_to_alarm", datapoints_to_alarm.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("threshold_metric_id", threshold_metric_id.unwrap_or_default())
                .with_field("extended_statistic", extended_statistic.unwrap_or_default())
                .with_field("period", period.unwrap_or_default())
                .with_field("unit", unit.unwrap_or_default())
            )
        })
    }

    /// Read a metric_alarm resource
    async fn read_metric_alarm(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_metric_alarm()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a metric_alarm resource
    async fn update_metric_alarm(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let alarm_name = input.get_string("alarm_name")?;
            let ok_actions = input.get_optional_string("ok_actions")?;
            let actions_enabled = input.get_optional_string("actions_enabled")?;
            let insufficient_data_actions = input.get_optional_string("insufficient_data_actions")?;
            let alarm_description = input.get_optional_string("alarm_description")?;
            let namespace = input.get_optional_string("namespace")?;
            let dimensions = input.get_optional_string("dimensions")?;
            let threshold = input.get_optional_string("threshold")?;
            let statistic = input.get_optional_string("statistic")?;
            let evaluate_low_sample_count_percentile = input.get_optional_string("evaluate_low_sample_count_percentile")?;
            let treat_missing_data = input.get_optional_string("treat_missing_data")?;
            let metric_name = input.get_optional_string("metric_name")?;
            let alarm_actions = input.get_optional_string("alarm_actions")?;
            let evaluation_periods = input.get_string("evaluation_periods")?;
            let comparison_operator = input.get_string("comparison_operator")?;
            let metrics = input.get_optional_string("metrics")?;
            let datapoints_to_alarm = input.get_optional_string("datapoints_to_alarm")?;
            let tags = input.get_optional_string("tags")?;
            let threshold_metric_id = input.get_optional_string("threshold_metric_id")?;
            let extended_statistic = input.get_optional_string("extended_statistic")?;
            let period = input.get_optional_string("period")?;
            let unit = input.get_optional_string("unit")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_metric_alarm()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("alarm_name", alarm_name.unwrap_or_default())
                .with_field("ok_actions", ok_actions.unwrap_or_default())
                .with_field("actions_enabled", actions_enabled.unwrap_or_default())
                .with_field("insufficient_data_actions", insufficient_data_actions.unwrap_or_default())
                .with_field("alarm_description", alarm_description.unwrap_or_default())
                .with_field("namespace", namespace.unwrap_or_default())
                .with_field("dimensions", dimensions.unwrap_or_default())
                .with_field("threshold", threshold.unwrap_or_default())
                .with_field("statistic", statistic.unwrap_or_default())
                .with_field("evaluate_low_sample_count_percentile", evaluate_low_sample_count_percentile.unwrap_or_default())
                .with_field("treat_missing_data", treat_missing_data.unwrap_or_default())
                .with_field("metric_name", metric_name.unwrap_or_default())
                .with_field("alarm_actions", alarm_actions.unwrap_or_default())
                .with_field("evaluation_periods", evaluation_periods.unwrap_or_default())
                .with_field("comparison_operator", comparison_operator.unwrap_or_default())
                .with_field("metrics", metrics.unwrap_or_default())
                .with_field("datapoints_to_alarm", datapoints_to_alarm.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("threshold_metric_id", threshold_metric_id.unwrap_or_default())
                .with_field("extended_statistic", extended_statistic.unwrap_or_default())
                .with_field("period", period.unwrap_or_default())
                .with_field("unit", unit.unwrap_or_default())
            )
        })
    }

    /// Delete a metric_alarm resource
    async fn delete_metric_alarm(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_metric_alarm()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Alarms_for_metric resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a alarms_for_metric resource
    async fn plan_alarms_for_metric(
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

    /// Create a new alarms_for_metric resource
    async fn create_alarms_for_metric(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_alarms_for_metric()
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

    /// Read a alarms_for_metric resource
    async fn read_alarms_for_metric(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_alarms_for_metric()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a alarms_for_metric resource
    async fn update_alarms_for_metric(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_alarms_for_metric()
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

    /// Delete a alarms_for_metric resource
    async fn delete_alarms_for_metric(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_alarms_for_metric()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Metric_stream resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a metric_stream resource
    async fn plan_metric_stream(
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

    /// Create a new metric_stream resource
    async fn create_metric_stream(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let include_filters = input.get_optional_string("include_filters")?;
            let include_linked_accounts_metrics = input.get_optional_string("include_linked_accounts_metrics")?;
            let name = input.get_string("name")?;
            let output_format = input.get_string("output_format")?;
            let firehose_arn = input.get_string("firehose_arn")?;
            let role_arn = input.get_string("role_arn")?;
            let tags = input.get_optional_string("tags")?;
            let statistics_configurations = input.get_optional_string("statistics_configurations")?;
            let exclude_filters = input.get_optional_string("exclude_filters")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_metric_stream()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("include_filters", include_filters.unwrap_or_default())
                .with_field("include_linked_accounts_metrics", include_linked_accounts_metrics.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("output_format", output_format.unwrap_or_default())
                .with_field("firehose_arn", firehose_arn.unwrap_or_default())
                .with_field("role_arn", role_arn.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("statistics_configurations", statistics_configurations.unwrap_or_default())
                .with_field("exclude_filters", exclude_filters.unwrap_or_default())
            )
        })
    }

    /// Read a metric_stream resource
    async fn read_metric_stream(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_metric_stream()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a metric_stream resource
    async fn update_metric_stream(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let include_filters = input.get_optional_string("include_filters")?;
            let include_linked_accounts_metrics = input.get_optional_string("include_linked_accounts_metrics")?;
            let name = input.get_string("name")?;
            let output_format = input.get_string("output_format")?;
            let firehose_arn = input.get_string("firehose_arn")?;
            let role_arn = input.get_string("role_arn")?;
            let tags = input.get_optional_string("tags")?;
            let statistics_configurations = input.get_optional_string("statistics_configurations")?;
            let exclude_filters = input.get_optional_string("exclude_filters")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_metric_stream()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("include_filters", include_filters.unwrap_or_default())
                .with_field("include_linked_accounts_metrics", include_linked_accounts_metrics.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("output_format", output_format.unwrap_or_default())
                .with_field("firehose_arn", firehose_arn.unwrap_or_default())
                .with_field("role_arn", role_arn.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("statistics_configurations", statistics_configurations.unwrap_or_default())
                .with_field("exclude_filters", exclude_filters.unwrap_or_default())
            )
        })
    }

    /// Delete a metric_stream resource
    async fn delete_metric_stream(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_metric_stream()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Dashboards resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dashboards resource
    async fn plan_dashboards(
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

    /// Create a new dashboards resource
    async fn create_dashboards(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_dashboards()
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

    /// Read a dashboards resource
    async fn read_dashboards(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_dashboards()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a dashboards resource
    async fn update_dashboards(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_dashboards()
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

    /// Delete a dashboards resource
    async fn delete_dashboards(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_dashboards()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Metric_statistics resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a metric_statistics resource
    async fn plan_metric_statistics(
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

    /// Create a new metric_statistics resource
    async fn create_metric_statistics(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_metric_statistics()
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

    /// Read a metric_statistics resource
    async fn read_metric_statistics(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_metric_statistics()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a metric_statistics resource
    async fn update_metric_statistics(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_metric_statistics()
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

    /// Delete a metric_statistics resource
    async fn delete_metric_statistics(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_metric_statistics()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Managed_insight_rules resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a managed_insight_rules resource
    async fn plan_managed_insight_rules(
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

    /// Create a new managed_insight_rules resource
    async fn create_managed_insight_rules(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let managed_rules = input.get_string("managed_rules")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_managed_insight_rules()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("managed_rules", managed_rules.unwrap_or_default())
            )
        })
    }

    /// Read a managed_insight_rules resource
    async fn read_managed_insight_rules(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_managed_insight_rules()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a managed_insight_rules resource
    async fn update_managed_insight_rules(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let managed_rules = input.get_string("managed_rules")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_managed_insight_rules()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("managed_rules", managed_rules.unwrap_or_default())
            )
        })
    }

    /// Delete a managed_insight_rules resource
    async fn delete_managed_insight_rules(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_managed_insight_rules()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Alarm_contributors resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a alarm_contributors resource
    async fn plan_alarm_contributors(
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

    /// Create a new alarm_contributors resource
    async fn create_alarm_contributors(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_alarm_contributors()
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

    /// Read a alarm_contributors resource
    async fn read_alarm_contributors(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_alarm_contributors()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a alarm_contributors resource
    async fn update_alarm_contributors(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_alarm_contributors()
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

    /// Delete a alarm_contributors resource
    async fn delete_alarm_contributors(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_alarm_contributors()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Insight_rule_report resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a insight_rule_report resource
    async fn plan_insight_rule_report(
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

    /// Create a new insight_rule_report resource
    async fn create_insight_rule_report(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_insight_rule_report()
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

    /// Read a insight_rule_report resource
    async fn read_insight_rule_report(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_insight_rule_report()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a insight_rule_report resource
    async fn update_insight_rule_report(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_insight_rule_report()
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

    /// Delete a insight_rule_report resource
    async fn delete_insight_rule_report(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_insight_rule_report()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Insight_rules resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a insight_rules resource
    async fn plan_insight_rules(
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

    /// Create a new insight_rules resource
    async fn create_insight_rules(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_insight_rules()
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

    /// Read a insight_rules resource
    async fn read_insight_rules(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_insight_rules()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a insight_rules resource
    async fn update_insight_rules(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_insight_rules()
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

    /// Delete a insight_rules resource
    async fn delete_insight_rules(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_insight_rules()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Alarm_history resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a alarm_history resource
    async fn plan_alarm_history(
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

    /// Create a new alarm_history resource
    async fn create_alarm_history(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_alarm_history()
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

    /// Read a alarm_history resource
    async fn read_alarm_history(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_alarm_history()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a alarm_history resource
    async fn update_alarm_history(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_alarm_history()
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

    /// Delete a alarm_history resource
    async fn delete_alarm_history(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_alarm_history()
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
    async fn create_dashboard(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let dashboard_body = input.get_string("dashboard_body")?;
            let dashboard_name = input.get_string("dashboard_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_dashboard()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("dashboard_body", dashboard_body.unwrap_or_default())
                .with_field("dashboard_name", dashboard_name.unwrap_or_default())
            )
        })
    }

    /// Read a dashboard resource
    async fn read_dashboard(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_dashboard()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a dashboard resource
    async fn update_dashboard(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let dashboard_body = input.get_string("dashboard_body")?;
            let dashboard_name = input.get_string("dashboard_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_dashboard()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("dashboard_body", dashboard_body.unwrap_or_default())
                .with_field("dashboard_name", dashboard_name.unwrap_or_default())
            )
        })
    }

    /// Delete a dashboard resource
    async fn delete_dashboard(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_dashboard()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Composite_alarm resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a composite_alarm resource
    async fn plan_composite_alarm(
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

    /// Create a new composite_alarm resource
    async fn create_composite_alarm(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let alarm_actions = input.get_optional_string("alarm_actions")?;
            let actions_enabled = input.get_optional_string("actions_enabled")?;
            let actions_suppressor_extension_period = input.get_optional_string("actions_suppressor_extension_period")?;
            let ok_actions = input.get_optional_string("ok_actions")?;
            let alarm_name = input.get_string("alarm_name")?;
            let alarm_description = input.get_optional_string("alarm_description")?;
            let alarm_rule = input.get_string("alarm_rule")?;
            let insufficient_data_actions = input.get_optional_string("insufficient_data_actions")?;
            let tags = input.get_optional_string("tags")?;
            let actions_suppressor = input.get_optional_string("actions_suppressor")?;
            let actions_suppressor_wait_period = input.get_optional_string("actions_suppressor_wait_period")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_composite_alarm()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("alarm_actions", alarm_actions.unwrap_or_default())
                .with_field("actions_enabled", actions_enabled.unwrap_or_default())
                .with_field("actions_suppressor_extension_period", actions_suppressor_extension_period.unwrap_or_default())
                .with_field("ok_actions", ok_actions.unwrap_or_default())
                .with_field("alarm_name", alarm_name.unwrap_or_default())
                .with_field("alarm_description", alarm_description.unwrap_or_default())
                .with_field("alarm_rule", alarm_rule.unwrap_or_default())
                .with_field("insufficient_data_actions", insufficient_data_actions.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("actions_suppressor", actions_suppressor.unwrap_or_default())
                .with_field("actions_suppressor_wait_period", actions_suppressor_wait_period.unwrap_or_default())
            )
        })
    }

    /// Read a composite_alarm resource
    async fn read_composite_alarm(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_composite_alarm()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a composite_alarm resource
    async fn update_composite_alarm(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let alarm_actions = input.get_optional_string("alarm_actions")?;
            let actions_enabled = input.get_optional_string("actions_enabled")?;
            let actions_suppressor_extension_period = input.get_optional_string("actions_suppressor_extension_period")?;
            let ok_actions = input.get_optional_string("ok_actions")?;
            let alarm_name = input.get_string("alarm_name")?;
            let alarm_description = input.get_optional_string("alarm_description")?;
            let alarm_rule = input.get_string("alarm_rule")?;
            let insufficient_data_actions = input.get_optional_string("insufficient_data_actions")?;
            let tags = input.get_optional_string("tags")?;
            let actions_suppressor = input.get_optional_string("actions_suppressor")?;
            let actions_suppressor_wait_period = input.get_optional_string("actions_suppressor_wait_period")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_composite_alarm()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("alarm_actions", alarm_actions.unwrap_or_default())
                .with_field("actions_enabled", actions_enabled.unwrap_or_default())
                .with_field("actions_suppressor_extension_period", actions_suppressor_extension_period.unwrap_or_default())
                .with_field("ok_actions", ok_actions.unwrap_or_default())
                .with_field("alarm_name", alarm_name.unwrap_or_default())
                .with_field("alarm_description", alarm_description.unwrap_or_default())
                .with_field("alarm_rule", alarm_rule.unwrap_or_default())
                .with_field("insufficient_data_actions", insufficient_data_actions.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("actions_suppressor", actions_suppressor.unwrap_or_default())
                .with_field("actions_suppressor_wait_period", actions_suppressor_wait_period.unwrap_or_default())
            )
        })
    }

    /// Delete a composite_alarm resource
    async fn delete_composite_alarm(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_composite_alarm()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Metric_data resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a metric_data resource
    async fn plan_metric_data(
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

    /// Create a new metric_data resource
    async fn create_metric_data(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let namespace = input.get_string("namespace")?;
            let strict_entity_validation = input.get_optional_string("strict_entity_validation")?;
            let entity_metric_data = input.get_optional_string("entity_metric_data")?;
            let metric_data = input.get_optional_string("metric_data")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_metric_data()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("namespace", namespace.unwrap_or_default())
                .with_field("strict_entity_validation", strict_entity_validation.unwrap_or_default())
                .with_field("entity_metric_data", entity_metric_data.unwrap_or_default())
                .with_field("metric_data", metric_data.unwrap_or_default())
            )
        })
    }

    /// Read a metric_data resource
    async fn read_metric_data(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_metric_data()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a metric_data resource
    async fn update_metric_data(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let namespace = input.get_string("namespace")?;
            let strict_entity_validation = input.get_optional_string("strict_entity_validation")?;
            let entity_metric_data = input.get_optional_string("entity_metric_data")?;
            let metric_data = input.get_optional_string("metric_data")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_metric_data()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("namespace", namespace.unwrap_or_default())
                .with_field("strict_entity_validation", strict_entity_validation.unwrap_or_default())
                .with_field("entity_metric_data", entity_metric_data.unwrap_or_default())
                .with_field("metric_data", metric_data.unwrap_or_default())
            )
        })
    }

    /// Delete a metric_data resource
    async fn delete_metric_data(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_metric_data()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Anomaly_detectors resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a anomaly_detectors resource
    async fn plan_anomaly_detectors(
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

    /// Create a new anomaly_detectors resource
    async fn create_anomaly_detectors(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .create_anomaly_detectors()
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

    /// Read a anomaly_detectors resource
    async fn read_anomaly_detectors(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .describe_anomaly_detectors()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a anomaly_detectors resource
    async fn update_anomaly_detectors(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudwatch_client
            //     .update_anomaly_detectors()
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

    /// Delete a anomaly_detectors resource
    async fn delete_anomaly_detectors(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudwatch_client
            //     .delete_anomaly_detectors()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
