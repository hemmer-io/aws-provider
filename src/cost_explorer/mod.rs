//! Cost_explorer service for Aws provider
//!
//! This module handles all cost_explorer resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Cost_explorer service handler
pub struct Cost_explorerService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cost_explorerService<'a> {
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
            "anomaly_monitor" => {
                self.plan_anomaly_monitor(current_state, desired_input).await
            }
            "cost_and_usage_comparisons" => {
                self.plan_cost_and_usage_comparisons(current_state, desired_input).await
            }
            "savings_plan_purchase_recommendation_details" => {
                self.plan_savings_plan_purchase_recommendation_details(current_state, desired_input).await
            }
            "commitment_purchase_analysis" => {
                self.plan_commitment_purchase_analysis(current_state, desired_input).await
            }
            "anomaly_monitors" => {
                self.plan_anomaly_monitors(current_state, desired_input).await
            }
            "savings_plans_utilization" => {
                self.plan_savings_plans_utilization(current_state, desired_input).await
            }
            "tags" => {
                self.plan_tags(current_state, desired_input).await
            }
            "cost_and_usage_with_resources" => {
                self.plan_cost_and_usage_with_resources(current_state, desired_input).await
            }
            "rightsizing_recommendation" => {
                self.plan_rightsizing_recommendation(current_state, desired_input).await
            }
            "cost_allocation_tags_status" => {
                self.plan_cost_allocation_tags_status(current_state, desired_input).await
            }
            "anomalies" => {
                self.plan_anomalies(current_state, desired_input).await
            }
            "reservation_purchase_recommendation" => {
                self.plan_reservation_purchase_recommendation(current_state, desired_input).await
            }
            "dimension_values" => {
                self.plan_dimension_values(current_state, desired_input).await
            }
            "cost_category_definition" => {
                self.plan_cost_category_definition(current_state, desired_input).await
            }
            "savings_plans_purchase_recommendation" => {
                self.plan_savings_plans_purchase_recommendation(current_state, desired_input).await
            }
            "anomaly_subscriptions" => {
                self.plan_anomaly_subscriptions(current_state, desired_input).await
            }
            "approximate_usage_records" => {
                self.plan_approximate_usage_records(current_state, desired_input).await
            }
            "cost_forecast" => {
                self.plan_cost_forecast(current_state, desired_input).await
            }
            "cost_categories" => {
                self.plan_cost_categories(current_state, desired_input).await
            }
            "anomaly_subscription" => {
                self.plan_anomaly_subscription(current_state, desired_input).await
            }
            "savings_plans_utilization_details" => {
                self.plan_savings_plans_utilization_details(current_state, desired_input).await
            }
            "cost_comparison_drivers" => {
                self.plan_cost_comparison_drivers(current_state, desired_input).await
            }
            "usage_forecast" => {
                self.plan_usage_forecast(current_state, desired_input).await
            }
            "reservation_coverage" => {
                self.plan_reservation_coverage(current_state, desired_input).await
            }
            "cost_and_usage" => {
                self.plan_cost_and_usage(current_state, desired_input).await
            }
            "savings_plans_coverage" => {
                self.plan_savings_plans_coverage(current_state, desired_input).await
            }
            "reservation_utilization" => {
                self.plan_reservation_utilization(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cost_explorer",
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
            "anomaly_monitor" => {
                self.create_anomaly_monitor(input).await
            }
            "cost_and_usage_comparisons" => {
                self.create_cost_and_usage_comparisons(input).await
            }
            "savings_plan_purchase_recommendation_details" => {
                self.create_savings_plan_purchase_recommendation_details(input).await
            }
            "commitment_purchase_analysis" => {
                self.create_commitment_purchase_analysis(input).await
            }
            "anomaly_monitors" => {
                self.create_anomaly_monitors(input).await
            }
            "savings_plans_utilization" => {
                self.create_savings_plans_utilization(input).await
            }
            "tags" => {
                self.create_tags(input).await
            }
            "cost_and_usage_with_resources" => {
                self.create_cost_and_usage_with_resources(input).await
            }
            "rightsizing_recommendation" => {
                self.create_rightsizing_recommendation(input).await
            }
            "cost_allocation_tags_status" => {
                self.create_cost_allocation_tags_status(input).await
            }
            "anomalies" => {
                self.create_anomalies(input).await
            }
            "reservation_purchase_recommendation" => {
                self.create_reservation_purchase_recommendation(input).await
            }
            "dimension_values" => {
                self.create_dimension_values(input).await
            }
            "cost_category_definition" => {
                self.create_cost_category_definition(input).await
            }
            "savings_plans_purchase_recommendation" => {
                self.create_savings_plans_purchase_recommendation(input).await
            }
            "anomaly_subscriptions" => {
                self.create_anomaly_subscriptions(input).await
            }
            "approximate_usage_records" => {
                self.create_approximate_usage_records(input).await
            }
            "cost_forecast" => {
                self.create_cost_forecast(input).await
            }
            "cost_categories" => {
                self.create_cost_categories(input).await
            }
            "anomaly_subscription" => {
                self.create_anomaly_subscription(input).await
            }
            "savings_plans_utilization_details" => {
                self.create_savings_plans_utilization_details(input).await
            }
            "cost_comparison_drivers" => {
                self.create_cost_comparison_drivers(input).await
            }
            "usage_forecast" => {
                self.create_usage_forecast(input).await
            }
            "reservation_coverage" => {
                self.create_reservation_coverage(input).await
            }
            "cost_and_usage" => {
                self.create_cost_and_usage(input).await
            }
            "savings_plans_coverage" => {
                self.create_savings_plans_coverage(input).await
            }
            "reservation_utilization" => {
                self.create_reservation_utilization(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cost_explorer",
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
            "anomaly_monitor" => {
                self.read_anomaly_monitor(id).await
            }
            "cost_and_usage_comparisons" => {
                self.read_cost_and_usage_comparisons(id).await
            }
            "savings_plan_purchase_recommendation_details" => {
                self.read_savings_plan_purchase_recommendation_details(id).await
            }
            "commitment_purchase_analysis" => {
                self.read_commitment_purchase_analysis(id).await
            }
            "anomaly_monitors" => {
                self.read_anomaly_monitors(id).await
            }
            "savings_plans_utilization" => {
                self.read_savings_plans_utilization(id).await
            }
            "tags" => {
                self.read_tags(id).await
            }
            "cost_and_usage_with_resources" => {
                self.read_cost_and_usage_with_resources(id).await
            }
            "rightsizing_recommendation" => {
                self.read_rightsizing_recommendation(id).await
            }
            "cost_allocation_tags_status" => {
                self.read_cost_allocation_tags_status(id).await
            }
            "anomalies" => {
                self.read_anomalies(id).await
            }
            "reservation_purchase_recommendation" => {
                self.read_reservation_purchase_recommendation(id).await
            }
            "dimension_values" => {
                self.read_dimension_values(id).await
            }
            "cost_category_definition" => {
                self.read_cost_category_definition(id).await
            }
            "savings_plans_purchase_recommendation" => {
                self.read_savings_plans_purchase_recommendation(id).await
            }
            "anomaly_subscriptions" => {
                self.read_anomaly_subscriptions(id).await
            }
            "approximate_usage_records" => {
                self.read_approximate_usage_records(id).await
            }
            "cost_forecast" => {
                self.read_cost_forecast(id).await
            }
            "cost_categories" => {
                self.read_cost_categories(id).await
            }
            "anomaly_subscription" => {
                self.read_anomaly_subscription(id).await
            }
            "savings_plans_utilization_details" => {
                self.read_savings_plans_utilization_details(id).await
            }
            "cost_comparison_drivers" => {
                self.read_cost_comparison_drivers(id).await
            }
            "usage_forecast" => {
                self.read_usage_forecast(id).await
            }
            "reservation_coverage" => {
                self.read_reservation_coverage(id).await
            }
            "cost_and_usage" => {
                self.read_cost_and_usage(id).await
            }
            "savings_plans_coverage" => {
                self.read_savings_plans_coverage(id).await
            }
            "reservation_utilization" => {
                self.read_reservation_utilization(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cost_explorer",
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
            "anomaly_monitor" => {
                self.update_anomaly_monitor(id, input).await
            }
            "cost_and_usage_comparisons" => {
                self.update_cost_and_usage_comparisons(id, input).await
            }
            "savings_plan_purchase_recommendation_details" => {
                self.update_savings_plan_purchase_recommendation_details(id, input).await
            }
            "commitment_purchase_analysis" => {
                self.update_commitment_purchase_analysis(id, input).await
            }
            "anomaly_monitors" => {
                self.update_anomaly_monitors(id, input).await
            }
            "savings_plans_utilization" => {
                self.update_savings_plans_utilization(id, input).await
            }
            "tags" => {
                self.update_tags(id, input).await
            }
            "cost_and_usage_with_resources" => {
                self.update_cost_and_usage_with_resources(id, input).await
            }
            "rightsizing_recommendation" => {
                self.update_rightsizing_recommendation(id, input).await
            }
            "cost_allocation_tags_status" => {
                self.update_cost_allocation_tags_status(id, input).await
            }
            "anomalies" => {
                self.update_anomalies(id, input).await
            }
            "reservation_purchase_recommendation" => {
                self.update_reservation_purchase_recommendation(id, input).await
            }
            "dimension_values" => {
                self.update_dimension_values(id, input).await
            }
            "cost_category_definition" => {
                self.update_cost_category_definition(id, input).await
            }
            "savings_plans_purchase_recommendation" => {
                self.update_savings_plans_purchase_recommendation(id, input).await
            }
            "anomaly_subscriptions" => {
                self.update_anomaly_subscriptions(id, input).await
            }
            "approximate_usage_records" => {
                self.update_approximate_usage_records(id, input).await
            }
            "cost_forecast" => {
                self.update_cost_forecast(id, input).await
            }
            "cost_categories" => {
                self.update_cost_categories(id, input).await
            }
            "anomaly_subscription" => {
                self.update_anomaly_subscription(id, input).await
            }
            "savings_plans_utilization_details" => {
                self.update_savings_plans_utilization_details(id, input).await
            }
            "cost_comparison_drivers" => {
                self.update_cost_comparison_drivers(id, input).await
            }
            "usage_forecast" => {
                self.update_usage_forecast(id, input).await
            }
            "reservation_coverage" => {
                self.update_reservation_coverage(id, input).await
            }
            "cost_and_usage" => {
                self.update_cost_and_usage(id, input).await
            }
            "savings_plans_coverage" => {
                self.update_savings_plans_coverage(id, input).await
            }
            "reservation_utilization" => {
                self.update_reservation_utilization(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cost_explorer",
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
            "anomaly_monitor" => {
                self.delete_anomaly_monitor(id).await
            }
            "cost_and_usage_comparisons" => {
                self.delete_cost_and_usage_comparisons(id).await
            }
            "savings_plan_purchase_recommendation_details" => {
                self.delete_savings_plan_purchase_recommendation_details(id).await
            }
            "commitment_purchase_analysis" => {
                self.delete_commitment_purchase_analysis(id).await
            }
            "anomaly_monitors" => {
                self.delete_anomaly_monitors(id).await
            }
            "savings_plans_utilization" => {
                self.delete_savings_plans_utilization(id).await
            }
            "tags" => {
                self.delete_tags(id).await
            }
            "cost_and_usage_with_resources" => {
                self.delete_cost_and_usage_with_resources(id).await
            }
            "rightsizing_recommendation" => {
                self.delete_rightsizing_recommendation(id).await
            }
            "cost_allocation_tags_status" => {
                self.delete_cost_allocation_tags_status(id).await
            }
            "anomalies" => {
                self.delete_anomalies(id).await
            }
            "reservation_purchase_recommendation" => {
                self.delete_reservation_purchase_recommendation(id).await
            }
            "dimension_values" => {
                self.delete_dimension_values(id).await
            }
            "cost_category_definition" => {
                self.delete_cost_category_definition(id).await
            }
            "savings_plans_purchase_recommendation" => {
                self.delete_savings_plans_purchase_recommendation(id).await
            }
            "anomaly_subscriptions" => {
                self.delete_anomaly_subscriptions(id).await
            }
            "approximate_usage_records" => {
                self.delete_approximate_usage_records(id).await
            }
            "cost_forecast" => {
                self.delete_cost_forecast(id).await
            }
            "cost_categories" => {
                self.delete_cost_categories(id).await
            }
            "anomaly_subscription" => {
                self.delete_anomaly_subscription(id).await
            }
            "savings_plans_utilization_details" => {
                self.delete_savings_plans_utilization_details(id).await
            }
            "cost_comparison_drivers" => {
                self.delete_cost_comparison_drivers(id).await
            }
            "usage_forecast" => {
                self.delete_usage_forecast(id).await
            }
            "reservation_coverage" => {
                self.delete_reservation_coverage(id).await
            }
            "cost_and_usage" => {
                self.delete_cost_and_usage(id).await
            }
            "savings_plans_coverage" => {
                self.delete_savings_plans_coverage(id).await
            }
            "reservation_utilization" => {
                self.delete_reservation_utilization(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cost_explorer",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Anomaly_monitor resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a anomaly_monitor resource
    async fn plan_anomaly_monitor(
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

    /// Create a new anomaly_monitor resource
    async fn create_anomaly_monitor(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_tags = input.get_optional_string("resource_tags")?;
            let anomaly_monitor = input.get_string("anomaly_monitor")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_anomaly_monitor()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("resource_tags", resource_tags.unwrap_or_default())
                .with_field("anomaly_monitor", anomaly_monitor.unwrap_or_default())
            )
        })
    }

    /// Read a anomaly_monitor resource
    async fn read_anomaly_monitor(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_anomaly_monitor()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a anomaly_monitor resource
    async fn update_anomaly_monitor(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_tags = input.get_optional_string("resource_tags")?;
            let anomaly_monitor = input.get_string("anomaly_monitor")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_anomaly_monitor()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("resource_tags", resource_tags.unwrap_or_default())
                .with_field("anomaly_monitor", anomaly_monitor.unwrap_or_default())
            )
        })
    }

    /// Delete a anomaly_monitor resource
    async fn delete_anomaly_monitor(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_anomaly_monitor()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Cost_and_usage_comparisons resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cost_and_usage_comparisons resource
    async fn plan_cost_and_usage_comparisons(
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

    /// Create a new cost_and_usage_comparisons resource
    async fn create_cost_and_usage_comparisons(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_cost_and_usage_comparisons()
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

    /// Read a cost_and_usage_comparisons resource
    async fn read_cost_and_usage_comparisons(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_cost_and_usage_comparisons()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a cost_and_usage_comparisons resource
    async fn update_cost_and_usage_comparisons(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_cost_and_usage_comparisons()
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

    /// Delete a cost_and_usage_comparisons resource
    async fn delete_cost_and_usage_comparisons(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_cost_and_usage_comparisons()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Savings_plan_purchase_recommendation_details resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a savings_plan_purchase_recommendation_details resource
    async fn plan_savings_plan_purchase_recommendation_details(
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

    /// Create a new savings_plan_purchase_recommendation_details resource
    async fn create_savings_plan_purchase_recommendation_details(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_savings_plan_purchase_recommendation_details()
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

    /// Read a savings_plan_purchase_recommendation_details resource
    async fn read_savings_plan_purchase_recommendation_details(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_savings_plan_purchase_recommendation_details()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a savings_plan_purchase_recommendation_details resource
    async fn update_savings_plan_purchase_recommendation_details(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_savings_plan_purchase_recommendation_details()
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

    /// Delete a savings_plan_purchase_recommendation_details resource
    async fn delete_savings_plan_purchase_recommendation_details(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_savings_plan_purchase_recommendation_details()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Commitment_purchase_analysis resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a commitment_purchase_analysis resource
    async fn plan_commitment_purchase_analysis(
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

    /// Create a new commitment_purchase_analysis resource
    async fn create_commitment_purchase_analysis(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_commitment_purchase_analysis()
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

    /// Read a commitment_purchase_analysis resource
    async fn read_commitment_purchase_analysis(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_commitment_purchase_analysis()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a commitment_purchase_analysis resource
    async fn update_commitment_purchase_analysis(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_commitment_purchase_analysis()
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

    /// Delete a commitment_purchase_analysis resource
    async fn delete_commitment_purchase_analysis(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_commitment_purchase_analysis()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Anomaly_monitors resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a anomaly_monitors resource
    async fn plan_anomaly_monitors(
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

    /// Create a new anomaly_monitors resource
    async fn create_anomaly_monitors(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_anomaly_monitors()
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

    /// Read a anomaly_monitors resource
    async fn read_anomaly_monitors(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_anomaly_monitors()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a anomaly_monitors resource
    async fn update_anomaly_monitors(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_anomaly_monitors()
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

    /// Delete a anomaly_monitors resource
    async fn delete_anomaly_monitors(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_anomaly_monitors()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Savings_plans_utilization resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a savings_plans_utilization resource
    async fn plan_savings_plans_utilization(
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

    /// Create a new savings_plans_utilization resource
    async fn create_savings_plans_utilization(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_savings_plans_utilization()
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

    /// Read a savings_plans_utilization resource
    async fn read_savings_plans_utilization(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_savings_plans_utilization()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a savings_plans_utilization resource
    async fn update_savings_plans_utilization(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_savings_plans_utilization()
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

    /// Delete a savings_plans_utilization resource
    async fn delete_savings_plans_utilization(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_savings_plans_utilization()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Tags resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tags resource
    async fn plan_tags(
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

    /// Create a new tags resource
    async fn create_tags(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_tags()
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

    /// Read a tags resource
    async fn read_tags(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_tags()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a tags resource
    async fn update_tags(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_tags()
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

    /// Delete a tags resource
    async fn delete_tags(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_tags()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Cost_and_usage_with_resources resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cost_and_usage_with_resources resource
    async fn plan_cost_and_usage_with_resources(
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

    /// Create a new cost_and_usage_with_resources resource
    async fn create_cost_and_usage_with_resources(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_cost_and_usage_with_resources()
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

    /// Read a cost_and_usage_with_resources resource
    async fn read_cost_and_usage_with_resources(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_cost_and_usage_with_resources()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a cost_and_usage_with_resources resource
    async fn update_cost_and_usage_with_resources(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_cost_and_usage_with_resources()
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

    /// Delete a cost_and_usage_with_resources resource
    async fn delete_cost_and_usage_with_resources(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_cost_and_usage_with_resources()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Rightsizing_recommendation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rightsizing_recommendation resource
    async fn plan_rightsizing_recommendation(
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

    /// Create a new rightsizing_recommendation resource
    async fn create_rightsizing_recommendation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_rightsizing_recommendation()
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

    /// Read a rightsizing_recommendation resource
    async fn read_rightsizing_recommendation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_rightsizing_recommendation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a rightsizing_recommendation resource
    async fn update_rightsizing_recommendation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_rightsizing_recommendation()
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

    /// Delete a rightsizing_recommendation resource
    async fn delete_rightsizing_recommendation(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_rightsizing_recommendation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Cost_allocation_tags_status resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cost_allocation_tags_status resource
    async fn plan_cost_allocation_tags_status(
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

    /// Create a new cost_allocation_tags_status resource
    async fn create_cost_allocation_tags_status(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cost_allocation_tags_status = input.get_string("cost_allocation_tags_status")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_cost_allocation_tags_status()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("cost_allocation_tags_status", cost_allocation_tags_status.unwrap_or_default())
            )
        })
    }

    /// Read a cost_allocation_tags_status resource
    async fn read_cost_allocation_tags_status(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_cost_allocation_tags_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a cost_allocation_tags_status resource
    async fn update_cost_allocation_tags_status(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cost_allocation_tags_status = input.get_string("cost_allocation_tags_status")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_cost_allocation_tags_status()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("cost_allocation_tags_status", cost_allocation_tags_status.unwrap_or_default())
            )
        })
    }

    /// Delete a cost_allocation_tags_status resource
    async fn delete_cost_allocation_tags_status(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_cost_allocation_tags_status()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Anomalies resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a anomalies resource
    async fn plan_anomalies(
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

    /// Create a new anomalies resource
    async fn create_anomalies(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_anomalies()
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

    /// Read a anomalies resource
    async fn read_anomalies(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_anomalies()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a anomalies resource
    async fn update_anomalies(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_anomalies()
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

    /// Delete a anomalies resource
    async fn delete_anomalies(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_anomalies()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Reservation_purchase_recommendation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a reservation_purchase_recommendation resource
    async fn plan_reservation_purchase_recommendation(
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

    /// Create a new reservation_purchase_recommendation resource
    async fn create_reservation_purchase_recommendation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_reservation_purchase_recommendation()
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

    /// Read a reservation_purchase_recommendation resource
    async fn read_reservation_purchase_recommendation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_reservation_purchase_recommendation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a reservation_purchase_recommendation resource
    async fn update_reservation_purchase_recommendation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_reservation_purchase_recommendation()
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

    /// Delete a reservation_purchase_recommendation resource
    async fn delete_reservation_purchase_recommendation(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_reservation_purchase_recommendation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Dimension_values resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dimension_values resource
    async fn plan_dimension_values(
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

    /// Create a new dimension_values resource
    async fn create_dimension_values(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_dimension_values()
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

    /// Read a dimension_values resource
    async fn read_dimension_values(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_dimension_values()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a dimension_values resource
    async fn update_dimension_values(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_dimension_values()
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

    /// Delete a dimension_values resource
    async fn delete_dimension_values(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_dimension_values()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Cost_category_definition resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cost_category_definition resource
    async fn plan_cost_category_definition(
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

    /// Create a new cost_category_definition resource
    async fn create_cost_category_definition(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_tags = input.get_optional_string("resource_tags")?;
            let rules = input.get_string("rules")?;
            let name = input.get_string("name")?;
            let default_value = input.get_optional_string("default_value")?;
            let effective_start = input.get_optional_string("effective_start")?;
            let rule_version = input.get_string("rule_version")?;
            let split_charge_rules = input.get_optional_string("split_charge_rules")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_cost_category_definition()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("resource_tags", resource_tags.unwrap_or_default())
                .with_field("rules", rules.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("default_value", default_value.unwrap_or_default())
                .with_field("effective_start", effective_start.unwrap_or_default())
                .with_field("rule_version", rule_version.unwrap_or_default())
                .with_field("split_charge_rules", split_charge_rules.unwrap_or_default())
            )
        })
    }

    /// Read a cost_category_definition resource
    async fn read_cost_category_definition(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_cost_category_definition()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a cost_category_definition resource
    async fn update_cost_category_definition(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_tags = input.get_optional_string("resource_tags")?;
            let rules = input.get_string("rules")?;
            let name = input.get_string("name")?;
            let default_value = input.get_optional_string("default_value")?;
            let effective_start = input.get_optional_string("effective_start")?;
            let rule_version = input.get_string("rule_version")?;
            let split_charge_rules = input.get_optional_string("split_charge_rules")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_cost_category_definition()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("resource_tags", resource_tags.unwrap_or_default())
                .with_field("rules", rules.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("default_value", default_value.unwrap_or_default())
                .with_field("effective_start", effective_start.unwrap_or_default())
                .with_field("rule_version", rule_version.unwrap_or_default())
                .with_field("split_charge_rules", split_charge_rules.unwrap_or_default())
            )
        })
    }

    /// Delete a cost_category_definition resource
    async fn delete_cost_category_definition(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_cost_category_definition()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Savings_plans_purchase_recommendation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a savings_plans_purchase_recommendation resource
    async fn plan_savings_plans_purchase_recommendation(
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

    /// Create a new savings_plans_purchase_recommendation resource
    async fn create_savings_plans_purchase_recommendation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_savings_plans_purchase_recommendation()
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

    /// Read a savings_plans_purchase_recommendation resource
    async fn read_savings_plans_purchase_recommendation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_savings_plans_purchase_recommendation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a savings_plans_purchase_recommendation resource
    async fn update_savings_plans_purchase_recommendation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_savings_plans_purchase_recommendation()
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

    /// Delete a savings_plans_purchase_recommendation resource
    async fn delete_savings_plans_purchase_recommendation(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_savings_plans_purchase_recommendation()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Anomaly_subscriptions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a anomaly_subscriptions resource
    async fn plan_anomaly_subscriptions(
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

    /// Create a new anomaly_subscriptions resource
    async fn create_anomaly_subscriptions(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_anomaly_subscriptions()
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

    /// Read a anomaly_subscriptions resource
    async fn read_anomaly_subscriptions(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_anomaly_subscriptions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a anomaly_subscriptions resource
    async fn update_anomaly_subscriptions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_anomaly_subscriptions()
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

    /// Delete a anomaly_subscriptions resource
    async fn delete_anomaly_subscriptions(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_anomaly_subscriptions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Approximate_usage_records resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a approximate_usage_records resource
    async fn plan_approximate_usage_records(
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

    /// Create a new approximate_usage_records resource
    async fn create_approximate_usage_records(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_approximate_usage_records()
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

    /// Read a approximate_usage_records resource
    async fn read_approximate_usage_records(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_approximate_usage_records()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a approximate_usage_records resource
    async fn update_approximate_usage_records(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_approximate_usage_records()
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

    /// Delete a approximate_usage_records resource
    async fn delete_approximate_usage_records(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_approximate_usage_records()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Cost_forecast resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cost_forecast resource
    async fn plan_cost_forecast(
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

    /// Create a new cost_forecast resource
    async fn create_cost_forecast(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_cost_forecast()
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

    /// Read a cost_forecast resource
    async fn read_cost_forecast(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_cost_forecast()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a cost_forecast resource
    async fn update_cost_forecast(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_cost_forecast()
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

    /// Delete a cost_forecast resource
    async fn delete_cost_forecast(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_cost_forecast()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Cost_categories resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cost_categories resource
    async fn plan_cost_categories(
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

    /// Create a new cost_categories resource
    async fn create_cost_categories(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_cost_categories()
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

    /// Read a cost_categories resource
    async fn read_cost_categories(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_cost_categories()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a cost_categories resource
    async fn update_cost_categories(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_cost_categories()
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

    /// Delete a cost_categories resource
    async fn delete_cost_categories(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_cost_categories()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Anomaly_subscription resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a anomaly_subscription resource
    async fn plan_anomaly_subscription(
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

    /// Create a new anomaly_subscription resource
    async fn create_anomaly_subscription(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let anomaly_subscription = input.get_string("anomaly_subscription")?;
            let resource_tags = input.get_optional_string("resource_tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_anomaly_subscription()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("anomaly_subscription", anomaly_subscription.unwrap_or_default())
                .with_field("resource_tags", resource_tags.unwrap_or_default())
            )
        })
    }

    /// Read a anomaly_subscription resource
    async fn read_anomaly_subscription(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_anomaly_subscription()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a anomaly_subscription resource
    async fn update_anomaly_subscription(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let anomaly_subscription = input.get_string("anomaly_subscription")?;
            let resource_tags = input.get_optional_string("resource_tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_anomaly_subscription()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("anomaly_subscription", anomaly_subscription.unwrap_or_default())
                .with_field("resource_tags", resource_tags.unwrap_or_default())
            )
        })
    }

    /// Delete a anomaly_subscription resource
    async fn delete_anomaly_subscription(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_anomaly_subscription()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Savings_plans_utilization_details resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a savings_plans_utilization_details resource
    async fn plan_savings_plans_utilization_details(
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

    /// Create a new savings_plans_utilization_details resource
    async fn create_savings_plans_utilization_details(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_savings_plans_utilization_details()
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

    /// Read a savings_plans_utilization_details resource
    async fn read_savings_plans_utilization_details(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_savings_plans_utilization_details()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a savings_plans_utilization_details resource
    async fn update_savings_plans_utilization_details(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_savings_plans_utilization_details()
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

    /// Delete a savings_plans_utilization_details resource
    async fn delete_savings_plans_utilization_details(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_savings_plans_utilization_details()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Cost_comparison_drivers resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cost_comparison_drivers resource
    async fn plan_cost_comparison_drivers(
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

    /// Create a new cost_comparison_drivers resource
    async fn create_cost_comparison_drivers(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_cost_comparison_drivers()
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

    /// Read a cost_comparison_drivers resource
    async fn read_cost_comparison_drivers(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_cost_comparison_drivers()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a cost_comparison_drivers resource
    async fn update_cost_comparison_drivers(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_cost_comparison_drivers()
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

    /// Delete a cost_comparison_drivers resource
    async fn delete_cost_comparison_drivers(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_cost_comparison_drivers()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Usage_forecast resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a usage_forecast resource
    async fn plan_usage_forecast(
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

    /// Create a new usage_forecast resource
    async fn create_usage_forecast(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_usage_forecast()
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

    /// Read a usage_forecast resource
    async fn read_usage_forecast(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_usage_forecast()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a usage_forecast resource
    async fn update_usage_forecast(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_usage_forecast()
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

    /// Delete a usage_forecast resource
    async fn delete_usage_forecast(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_usage_forecast()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Reservation_coverage resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a reservation_coverage resource
    async fn plan_reservation_coverage(
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

    /// Create a new reservation_coverage resource
    async fn create_reservation_coverage(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_reservation_coverage()
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

    /// Read a reservation_coverage resource
    async fn read_reservation_coverage(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_reservation_coverage()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a reservation_coverage resource
    async fn update_reservation_coverage(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_reservation_coverage()
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

    /// Delete a reservation_coverage resource
    async fn delete_reservation_coverage(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_reservation_coverage()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Cost_and_usage resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cost_and_usage resource
    async fn plan_cost_and_usage(
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

    /// Create a new cost_and_usage resource
    async fn create_cost_and_usage(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_cost_and_usage()
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

    /// Read a cost_and_usage resource
    async fn read_cost_and_usage(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_cost_and_usage()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a cost_and_usage resource
    async fn update_cost_and_usage(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_cost_and_usage()
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

    /// Delete a cost_and_usage resource
    async fn delete_cost_and_usage(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_cost_and_usage()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Savings_plans_coverage resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a savings_plans_coverage resource
    async fn plan_savings_plans_coverage(
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

    /// Create a new savings_plans_coverage resource
    async fn create_savings_plans_coverage(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_savings_plans_coverage()
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

    /// Read a savings_plans_coverage resource
    async fn read_savings_plans_coverage(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_savings_plans_coverage()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a savings_plans_coverage resource
    async fn update_savings_plans_coverage(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_savings_plans_coverage()
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

    /// Delete a savings_plans_coverage resource
    async fn delete_savings_plans_coverage(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_savings_plans_coverage()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Reservation_utilization resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a reservation_utilization resource
    async fn plan_reservation_utilization(
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

    /// Create a new reservation_utilization resource
    async fn create_reservation_utilization(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .create_reservation_utilization()
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

    /// Read a reservation_utilization resource
    async fn read_reservation_utilization(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .describe_reservation_utilization()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a reservation_utilization resource
    async fn update_reservation_utilization(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cost_explorer_client
            //     .update_reservation_utilization()
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

    /// Delete a reservation_utilization resource
    async fn delete_reservation_utilization(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cost_explorer_client
            //     .delete_reservation_utilization()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
