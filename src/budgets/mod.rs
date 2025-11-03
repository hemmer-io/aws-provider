//! Budgets service for Aws provider
//!
//! This module handles all budgets resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Budgets service handler
pub struct BudgetsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> BudgetsService<'a> {
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
            "subscriber" => {
                self.plan_subscriber(current_state, desired_input).await
            }
            "budget_notifications_for_account" => {
                self.plan_budget_notifications_for_account(current_state, desired_input).await
            }
            "notifications_for_budget" => {
                self.plan_notifications_for_budget(current_state, desired_input).await
            }
            "budget_action_histories" => {
                self.plan_budget_action_histories(current_state, desired_input).await
            }
            "subscribers_for_notification" => {
                self.plan_subscribers_for_notification(current_state, desired_input).await
            }
            "budgets" => {
                self.plan_budgets(current_state, desired_input).await
            }
            "budget_performance_history" => {
                self.plan_budget_performance_history(current_state, desired_input).await
            }
            "notification" => {
                self.plan_notification(current_state, desired_input).await
            }
            "budget_action" => {
                self.plan_budget_action(current_state, desired_input).await
            }
            "budget_actions_for_budget" => {
                self.plan_budget_actions_for_budget(current_state, desired_input).await
            }
            "budget" => {
                self.plan_budget(current_state, desired_input).await
            }
            "budget_actions_for_account" => {
                self.plan_budget_actions_for_account(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "budgets",
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
            "subscriber" => {
                self.create_subscriber(input).await
            }
            "budget_notifications_for_account" => {
                self.create_budget_notifications_for_account(input).await
            }
            "notifications_for_budget" => {
                self.create_notifications_for_budget(input).await
            }
            "budget_action_histories" => {
                self.create_budget_action_histories(input).await
            }
            "subscribers_for_notification" => {
                self.create_subscribers_for_notification(input).await
            }
            "budgets" => {
                self.create_budgets(input).await
            }
            "budget_performance_history" => {
                self.create_budget_performance_history(input).await
            }
            "notification" => {
                self.create_notification(input).await
            }
            "budget_action" => {
                self.create_budget_action(input).await
            }
            "budget_actions_for_budget" => {
                self.create_budget_actions_for_budget(input).await
            }
            "budget" => {
                self.create_budget(input).await
            }
            "budget_actions_for_account" => {
                self.create_budget_actions_for_account(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "budgets",
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
            "subscriber" => {
                self.read_subscriber(id).await
            }
            "budget_notifications_for_account" => {
                self.read_budget_notifications_for_account(id).await
            }
            "notifications_for_budget" => {
                self.read_notifications_for_budget(id).await
            }
            "budget_action_histories" => {
                self.read_budget_action_histories(id).await
            }
            "subscribers_for_notification" => {
                self.read_subscribers_for_notification(id).await
            }
            "budgets" => {
                self.read_budgets(id).await
            }
            "budget_performance_history" => {
                self.read_budget_performance_history(id).await
            }
            "notification" => {
                self.read_notification(id).await
            }
            "budget_action" => {
                self.read_budget_action(id).await
            }
            "budget_actions_for_budget" => {
                self.read_budget_actions_for_budget(id).await
            }
            "budget" => {
                self.read_budget(id).await
            }
            "budget_actions_for_account" => {
                self.read_budget_actions_for_account(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "budgets",
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
            "subscriber" => {
                self.update_subscriber(id, input).await
            }
            "budget_notifications_for_account" => {
                self.update_budget_notifications_for_account(id, input).await
            }
            "notifications_for_budget" => {
                self.update_notifications_for_budget(id, input).await
            }
            "budget_action_histories" => {
                self.update_budget_action_histories(id, input).await
            }
            "subscribers_for_notification" => {
                self.update_subscribers_for_notification(id, input).await
            }
            "budgets" => {
                self.update_budgets(id, input).await
            }
            "budget_performance_history" => {
                self.update_budget_performance_history(id, input).await
            }
            "notification" => {
                self.update_notification(id, input).await
            }
            "budget_action" => {
                self.update_budget_action(id, input).await
            }
            "budget_actions_for_budget" => {
                self.update_budget_actions_for_budget(id, input).await
            }
            "budget" => {
                self.update_budget(id, input).await
            }
            "budget_actions_for_account" => {
                self.update_budget_actions_for_account(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "budgets",
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
            "subscriber" => {
                self.delete_subscriber(id).await
            }
            "budget_notifications_for_account" => {
                self.delete_budget_notifications_for_account(id).await
            }
            "notifications_for_budget" => {
                self.delete_notifications_for_budget(id).await
            }
            "budget_action_histories" => {
                self.delete_budget_action_histories(id).await
            }
            "subscribers_for_notification" => {
                self.delete_subscribers_for_notification(id).await
            }
            "budgets" => {
                self.delete_budgets(id).await
            }
            "budget_performance_history" => {
                self.delete_budget_performance_history(id).await
            }
            "notification" => {
                self.delete_notification(id).await
            }
            "budget_action" => {
                self.delete_budget_action(id).await
            }
            "budget_actions_for_budget" => {
                self.delete_budget_actions_for_budget(id).await
            }
            "budget" => {
                self.delete_budget(id).await
            }
            "budget_actions_for_account" => {
                self.delete_budget_actions_for_account(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "budgets",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Subscriber resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a subscriber resource
    async fn plan_subscriber(
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

    /// Create a new subscriber resource
    async fn create_subscriber(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let notification = input.get_string("notification")?;
            let subscriber = input.get_string("subscriber")?;
            let account_id = input.get_string("account_id")?;
            let budget_name = input.get_string("budget_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_subscriber()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("notification", notification.unwrap_or_default())
                .with_field("subscriber", subscriber.unwrap_or_default())
                .with_field("account_id", account_id.unwrap_or_default())
                .with_field("budget_name", budget_name.unwrap_or_default())
            )
        })
    }

    /// Read a subscriber resource
    async fn read_subscriber(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_subscriber()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a subscriber resource
    async fn update_subscriber(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let notification = input.get_string("notification")?;
            let subscriber = input.get_string("subscriber")?;
            let account_id = input.get_string("account_id")?;
            let budget_name = input.get_string("budget_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_subscriber()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("notification", notification.unwrap_or_default())
                .with_field("subscriber", subscriber.unwrap_or_default())
                .with_field("account_id", account_id.unwrap_or_default())
                .with_field("budget_name", budget_name.unwrap_or_default())
            )
        })
    }

    /// Delete a subscriber resource
    async fn delete_subscriber(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_subscriber()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Budget_notifications_for_account resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a budget_notifications_for_account resource
    async fn plan_budget_notifications_for_account(
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

    /// Create a new budget_notifications_for_account resource
    async fn create_budget_notifications_for_account(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_budget_notifications_for_account()
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

    /// Read a budget_notifications_for_account resource
    async fn read_budget_notifications_for_account(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_budget_notifications_for_account()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a budget_notifications_for_account resource
    async fn update_budget_notifications_for_account(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_budget_notifications_for_account()
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

    /// Delete a budget_notifications_for_account resource
    async fn delete_budget_notifications_for_account(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_budget_notifications_for_account()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Notifications_for_budget resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a notifications_for_budget resource
    async fn plan_notifications_for_budget(
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

    /// Create a new notifications_for_budget resource
    async fn create_notifications_for_budget(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_notifications_for_budget()
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

    /// Read a notifications_for_budget resource
    async fn read_notifications_for_budget(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_notifications_for_budget()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a notifications_for_budget resource
    async fn update_notifications_for_budget(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_notifications_for_budget()
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

    /// Delete a notifications_for_budget resource
    async fn delete_notifications_for_budget(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_notifications_for_budget()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Budget_action_histories resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a budget_action_histories resource
    async fn plan_budget_action_histories(
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

    /// Create a new budget_action_histories resource
    async fn create_budget_action_histories(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_budget_action_histories()
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

    /// Read a budget_action_histories resource
    async fn read_budget_action_histories(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_budget_action_histories()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a budget_action_histories resource
    async fn update_budget_action_histories(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_budget_action_histories()
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

    /// Delete a budget_action_histories resource
    async fn delete_budget_action_histories(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_budget_action_histories()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Subscribers_for_notification resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a subscribers_for_notification resource
    async fn plan_subscribers_for_notification(
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

    /// Create a new subscribers_for_notification resource
    async fn create_subscribers_for_notification(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_subscribers_for_notification()
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

    /// Read a subscribers_for_notification resource
    async fn read_subscribers_for_notification(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_subscribers_for_notification()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a subscribers_for_notification resource
    async fn update_subscribers_for_notification(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_subscribers_for_notification()
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

    /// Delete a subscribers_for_notification resource
    async fn delete_subscribers_for_notification(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_subscribers_for_notification()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Budgets resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a budgets resource
    async fn plan_budgets(
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

    /// Create a new budgets resource
    async fn create_budgets(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_budgets()
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

    /// Read a budgets resource
    async fn read_budgets(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_budgets()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a budgets resource
    async fn update_budgets(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_budgets()
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

    /// Delete a budgets resource
    async fn delete_budgets(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_budgets()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Budget_performance_history resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a budget_performance_history resource
    async fn plan_budget_performance_history(
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

    /// Create a new budget_performance_history resource
    async fn create_budget_performance_history(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_budget_performance_history()
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

    /// Read a budget_performance_history resource
    async fn read_budget_performance_history(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_budget_performance_history()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a budget_performance_history resource
    async fn update_budget_performance_history(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_budget_performance_history()
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

    /// Delete a budget_performance_history resource
    async fn delete_budget_performance_history(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_budget_performance_history()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Notification resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a notification resource
    async fn plan_notification(
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

    /// Create a new notification resource
    async fn create_notification(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let account_id = input.get_string("account_id")?;
            let notification = input.get_string("notification")?;
            let subscribers = input.get_string("subscribers")?;
            let budget_name = input.get_string("budget_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_notification()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("account_id", account_id.unwrap_or_default())
                .with_field("notification", notification.unwrap_or_default())
                .with_field("subscribers", subscribers.unwrap_or_default())
                .with_field("budget_name", budget_name.unwrap_or_default())
            )
        })
    }

    /// Read a notification resource
    async fn read_notification(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_notification()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a notification resource
    async fn update_notification(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let account_id = input.get_string("account_id")?;
            let notification = input.get_string("notification")?;
            let subscribers = input.get_string("subscribers")?;
            let budget_name = input.get_string("budget_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_notification()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("account_id", account_id.unwrap_or_default())
                .with_field("notification", notification.unwrap_or_default())
                .with_field("subscribers", subscribers.unwrap_or_default())
                .with_field("budget_name", budget_name.unwrap_or_default())
            )
        })
    }

    /// Delete a notification resource
    async fn delete_notification(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_notification()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Budget_action resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a budget_action resource
    async fn plan_budget_action(
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

    /// Create a new budget_action resource
    async fn create_budget_action(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let notification_type = input.get_string("notification_type")?;
            let account_id = input.get_string("account_id")?;
            let definition = input.get_string("definition")?;
            let approval_model = input.get_string("approval_model")?;
            let budget_name = input.get_string("budget_name")?;
            let subscribers = input.get_string("subscribers")?;
            let action_threshold = input.get_string("action_threshold")?;
            let execution_role_arn = input.get_string("execution_role_arn")?;
            let resource_tags = input.get_optional_string("resource_tags")?;
            let action_type = input.get_string("action_type")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_budget_action()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("notification_type", notification_type.unwrap_or_default())
                .with_field("account_id", account_id.unwrap_or_default())
                .with_field("definition", definition.unwrap_or_default())
                .with_field("approval_model", approval_model.unwrap_or_default())
                .with_field("budget_name", budget_name.unwrap_or_default())
                .with_field("subscribers", subscribers.unwrap_or_default())
                .with_field("action_threshold", action_threshold.unwrap_or_default())
                .with_field("execution_role_arn", execution_role_arn.unwrap_or_default())
                .with_field("resource_tags", resource_tags.unwrap_or_default())
                .with_field("action_type", action_type.unwrap_or_default())
            )
        })
    }

    /// Read a budget_action resource
    async fn read_budget_action(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_budget_action()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a budget_action resource
    async fn update_budget_action(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let notification_type = input.get_string("notification_type")?;
            let account_id = input.get_string("account_id")?;
            let definition = input.get_string("definition")?;
            let approval_model = input.get_string("approval_model")?;
            let budget_name = input.get_string("budget_name")?;
            let subscribers = input.get_string("subscribers")?;
            let action_threshold = input.get_string("action_threshold")?;
            let execution_role_arn = input.get_string("execution_role_arn")?;
            let resource_tags = input.get_optional_string("resource_tags")?;
            let action_type = input.get_string("action_type")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_budget_action()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("notification_type", notification_type.unwrap_or_default())
                .with_field("account_id", account_id.unwrap_or_default())
                .with_field("definition", definition.unwrap_or_default())
                .with_field("approval_model", approval_model.unwrap_or_default())
                .with_field("budget_name", budget_name.unwrap_or_default())
                .with_field("subscribers", subscribers.unwrap_or_default())
                .with_field("action_threshold", action_threshold.unwrap_or_default())
                .with_field("execution_role_arn", execution_role_arn.unwrap_or_default())
                .with_field("resource_tags", resource_tags.unwrap_or_default())
                .with_field("action_type", action_type.unwrap_or_default())
            )
        })
    }

    /// Delete a budget_action resource
    async fn delete_budget_action(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_budget_action()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Budget_actions_for_budget resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a budget_actions_for_budget resource
    async fn plan_budget_actions_for_budget(
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

    /// Create a new budget_actions_for_budget resource
    async fn create_budget_actions_for_budget(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_budget_actions_for_budget()
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

    /// Read a budget_actions_for_budget resource
    async fn read_budget_actions_for_budget(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_budget_actions_for_budget()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a budget_actions_for_budget resource
    async fn update_budget_actions_for_budget(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_budget_actions_for_budget()
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

    /// Delete a budget_actions_for_budget resource
    async fn delete_budget_actions_for_budget(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_budget_actions_for_budget()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Budget resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a budget resource
    async fn plan_budget(
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

    /// Create a new budget resource
    async fn create_budget(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_tags = input.get_optional_string("resource_tags")?;
            let account_id = input.get_string("account_id")?;
            let budget = input.get_string("budget")?;
            let notifications_with_subscribers = input.get_optional_string("notifications_with_subscribers")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_budget()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("resource_tags", resource_tags.unwrap_or_default())
                .with_field("account_id", account_id.unwrap_or_default())
                .with_field("budget", budget.unwrap_or_default())
                .with_field("notifications_with_subscribers", notifications_with_subscribers.unwrap_or_default())
            )
        })
    }

    /// Read a budget resource
    async fn read_budget(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_budget()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a budget resource
    async fn update_budget(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let resource_tags = input.get_optional_string("resource_tags")?;
            let account_id = input.get_string("account_id")?;
            let budget = input.get_string("budget")?;
            let notifications_with_subscribers = input.get_optional_string("notifications_with_subscribers")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_budget()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("resource_tags", resource_tags.unwrap_or_default())
                .with_field("account_id", account_id.unwrap_or_default())
                .with_field("budget", budget.unwrap_or_default())
                .with_field("notifications_with_subscribers", notifications_with_subscribers.unwrap_or_default())
            )
        })
    }

    /// Delete a budget resource
    async fn delete_budget(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_budget()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Budget_actions_for_account resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a budget_actions_for_account resource
    async fn plan_budget_actions_for_account(
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

    /// Create a new budget_actions_for_account resource
    async fn create_budget_actions_for_account(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .create_budget_actions_for_account()
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

    /// Read a budget_actions_for_account resource
    async fn read_budget_actions_for_account(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .describe_budget_actions_for_account()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a budget_actions_for_account resource
    async fn update_budget_actions_for_account(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.budgets_client
            //     .update_budget_actions_for_account()
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

    /// Delete a budget_actions_for_account resource
    async fn delete_budget_actions_for_account(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.budgets_client
            //     .delete_budget_actions_for_account()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
