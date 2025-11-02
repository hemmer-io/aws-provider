//! Ecs service for Aws provider
//!
//! This module handles all ecs resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Ecs service handler
pub struct EcsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> EcsService<'a> {
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
            "service_deployments" => {
                self.plan_service_deployments(current_state, desired_input)
                    .await
            }
            "services" => self.plan_services(current_state, desired_input).await,
            "capacity_provider" => {
                self.plan_capacity_provider(current_state, desired_input)
                    .await
            }
            "cluster_capacity_providers" => {
                self.plan_cluster_capacity_providers(current_state, desired_input)
                    .await
            }
            "task_set" => self.plan_task_set(current_state, desired_input).await,
            "tasks" => self.plan_tasks(current_state, desired_input).await,
            "cluster_settings" => {
                self.plan_cluster_settings(current_state, desired_input)
                    .await
            }
            "task_protection" => {
                self.plan_task_protection(current_state, desired_input)
                    .await
            }
            "task_definitions" => {
                self.plan_task_definitions(current_state, desired_input)
                    .await
            }
            "account_setting_default" => {
                self.plan_account_setting_default(current_state, desired_input)
                    .await
            }
            "container_agent" => {
                self.plan_container_agent(current_state, desired_input)
                    .await
            }
            "service_primary_task_set" => {
                self.plan_service_primary_task_set(current_state, desired_input)
                    .await
            }
            "attributes" => self.plan_attributes(current_state, desired_input).await,
            "service" => self.plan_service(current_state, desired_input).await,
            "task_definition" => {
                self.plan_task_definition(current_state, desired_input)
                    .await
            }
            "service_revisions" => {
                self.plan_service_revisions(current_state, desired_input)
                    .await
            }
            "task_sets" => self.plan_task_sets(current_state, desired_input).await,
            "clusters" => self.plan_clusters(current_state, desired_input).await,
            "container_instances_state" => {
                self.plan_container_instances_state(current_state, desired_input)
                    .await
            }
            "account_setting" => {
                self.plan_account_setting(current_state, desired_input)
                    .await
            }
            "capacity_providers" => {
                self.plan_capacity_providers(current_state, desired_input)
                    .await
            }
            "cluster" => self.plan_cluster(current_state, desired_input).await,
            "container_instances" => {
                self.plan_container_instances(current_state, desired_input)
                    .await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ecs", resource_name
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
            "service_deployments" => self.create_service_deployments(input).await,
            "services" => self.create_services(input).await,
            "capacity_provider" => self.create_capacity_provider(input).await,
            "cluster_capacity_providers" => self.create_cluster_capacity_providers(input).await,
            "task_set" => self.create_task_set(input).await,
            "tasks" => self.create_tasks(input).await,
            "cluster_settings" => self.create_cluster_settings(input).await,
            "task_protection" => self.create_task_protection(input).await,
            "task_definitions" => self.create_task_definitions(input).await,
            "account_setting_default" => self.create_account_setting_default(input).await,
            "container_agent" => self.create_container_agent(input).await,
            "service_primary_task_set" => self.create_service_primary_task_set(input).await,
            "attributes" => self.create_attributes(input).await,
            "service" => self.create_service(input).await,
            "task_definition" => self.create_task_definition(input).await,
            "service_revisions" => self.create_service_revisions(input).await,
            "task_sets" => self.create_task_sets(input).await,
            "clusters" => self.create_clusters(input).await,
            "container_instances_state" => self.create_container_instances_state(input).await,
            "account_setting" => self.create_account_setting(input).await,
            "capacity_providers" => self.create_capacity_providers(input).await,
            "cluster" => self.create_cluster(input).await,
            "container_instances" => self.create_container_instances(input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ecs", resource_name
            ))),
        }
    }

    /// Read resource state
    pub async fn read_resource(&self, resource_name: &str, id: &str) -> Result<ResourceOutput> {
        match resource_name {
            "service_deployments" => self.read_service_deployments(id).await,
            "services" => self.read_services(id).await,
            "capacity_provider" => self.read_capacity_provider(id).await,
            "cluster_capacity_providers" => self.read_cluster_capacity_providers(id).await,
            "task_set" => self.read_task_set(id).await,
            "tasks" => self.read_tasks(id).await,
            "cluster_settings" => self.read_cluster_settings(id).await,
            "task_protection" => self.read_task_protection(id).await,
            "task_definitions" => self.read_task_definitions(id).await,
            "account_setting_default" => self.read_account_setting_default(id).await,
            "container_agent" => self.read_container_agent(id).await,
            "service_primary_task_set" => self.read_service_primary_task_set(id).await,
            "attributes" => self.read_attributes(id).await,
            "service" => self.read_service(id).await,
            "task_definition" => self.read_task_definition(id).await,
            "service_revisions" => self.read_service_revisions(id).await,
            "task_sets" => self.read_task_sets(id).await,
            "clusters" => self.read_clusters(id).await,
            "container_instances_state" => self.read_container_instances_state(id).await,
            "account_setting" => self.read_account_setting(id).await,
            "capacity_providers" => self.read_capacity_providers(id).await,
            "cluster" => self.read_cluster(id).await,
            "container_instances" => self.read_container_instances(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ecs", resource_name
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
            "service_deployments" => self.update_service_deployments(id, input).await,
            "services" => self.update_services(id, input).await,
            "capacity_provider" => self.update_capacity_provider(id, input).await,
            "cluster_capacity_providers" => self.update_cluster_capacity_providers(id, input).await,
            "task_set" => self.update_task_set(id, input).await,
            "tasks" => self.update_tasks(id, input).await,
            "cluster_settings" => self.update_cluster_settings(id, input).await,
            "task_protection" => self.update_task_protection(id, input).await,
            "task_definitions" => self.update_task_definitions(id, input).await,
            "account_setting_default" => self.update_account_setting_default(id, input).await,
            "container_agent" => self.update_container_agent(id, input).await,
            "service_primary_task_set" => self.update_service_primary_task_set(id, input).await,
            "attributes" => self.update_attributes(id, input).await,
            "service" => self.update_service(id, input).await,
            "task_definition" => self.update_task_definition(id, input).await,
            "service_revisions" => self.update_service_revisions(id, input).await,
            "task_sets" => self.update_task_sets(id, input).await,
            "clusters" => self.update_clusters(id, input).await,
            "container_instances_state" => self.update_container_instances_state(id, input).await,
            "account_setting" => self.update_account_setting(id, input).await,
            "capacity_providers" => self.update_capacity_providers(id, input).await,
            "cluster" => self.update_cluster(id, input).await,
            "container_instances" => self.update_container_instances(id, input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ecs", resource_name
            ))),
        }
    }

    /// Delete a resource
    pub async fn delete_resource(&self, resource_name: &str, id: &str) -> Result<()> {
        match resource_name {
            "service_deployments" => self.delete_service_deployments(id).await,
            "services" => self.delete_services(id).await,
            "capacity_provider" => self.delete_capacity_provider(id).await,
            "cluster_capacity_providers" => self.delete_cluster_capacity_providers(id).await,
            "task_set" => self.delete_task_set(id).await,
            "tasks" => self.delete_tasks(id).await,
            "cluster_settings" => self.delete_cluster_settings(id).await,
            "task_protection" => self.delete_task_protection(id).await,
            "task_definitions" => self.delete_task_definitions(id).await,
            "account_setting_default" => self.delete_account_setting_default(id).await,
            "container_agent" => self.delete_container_agent(id).await,
            "service_primary_task_set" => self.delete_service_primary_task_set(id).await,
            "attributes" => self.delete_attributes(id).await,
            "service" => self.delete_service(id).await,
            "task_definition" => self.delete_task_definition(id).await,
            "service_revisions" => self.delete_service_revisions(id).await,
            "task_sets" => self.delete_task_sets(id).await,
            "clusters" => self.delete_clusters(id).await,
            "container_instances_state" => self.delete_container_instances_state(id).await,
            "account_setting" => self.delete_account_setting(id).await,
            "capacity_providers" => self.delete_capacity_providers(id).await,
            "cluster" => self.delete_cluster(id).await,
            "container_instances" => self.delete_container_instances(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "ecs", resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================

    // ------------------------------------------------------------------------
    // Service_deployments resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a service_deployments resource
    async fn plan_service_deployments(
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

    /// Create a new service_deployments resource
    async fn create_service_deployments(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_service_deployments()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a service_deployments resource
    async fn read_service_deployments(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_service_deployments()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a service_deployments resource
    async fn update_service_deployments(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_service_deployments()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a service_deployments resource
    async fn delete_service_deployments(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_service_deployments()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Services resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a services resource
    async fn plan_services(
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

    /// Create a new services resource
    async fn create_services(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_services()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a services resource
    async fn read_services(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_services()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a services resource
    async fn update_services(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_services()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a services resource
    async fn delete_services(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_services()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Capacity_provider resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a capacity_provider resource
    async fn plan_capacity_provider(
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

    /// Create a new capacity_provider resource
    async fn create_capacity_provider(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let managed_instances_provider =
                input.get_optional_string("managed_instances_provider")?;
            let tags = input.get_optional_string("tags")?;
            let auto_scaling_group_provider =
                input.get_optional_string("auto_scaling_group_provider")?;
            let cluster = input.get_optional_string("cluster")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_capacity_provider()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field(
                    "managed_instances_provider",
                    managed_instances_provider.unwrap_or_default(),
                )
                .with_field("tags", tags.unwrap_or_default())
                .with_field(
                    "auto_scaling_group_provider",
                    auto_scaling_group_provider.unwrap_or_default(),
                )
                .with_field("cluster", cluster.unwrap_or_default()))
        })
    }

    /// Read a capacity_provider resource
    async fn read_capacity_provider(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_capacity_provider()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a capacity_provider resource
    async fn update_capacity_provider(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let managed_instances_provider =
                input.get_optional_string("managed_instances_provider")?;
            let tags = input.get_optional_string("tags")?;
            let auto_scaling_group_provider =
                input.get_optional_string("auto_scaling_group_provider")?;
            let cluster = input.get_optional_string("cluster")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_capacity_provider()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field(
                    "managed_instances_provider",
                    managed_instances_provider.unwrap_or_default(),
                )
                .with_field("tags", tags.unwrap_or_default())
                .with_field(
                    "auto_scaling_group_provider",
                    auto_scaling_group_provider.unwrap_or_default(),
                )
                .with_field("cluster", cluster.unwrap_or_default()))
        })
    }

    /// Delete a capacity_provider resource
    async fn delete_capacity_provider(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_capacity_provider()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Cluster_capacity_providers resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cluster_capacity_providers resource
    async fn plan_cluster_capacity_providers(
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

    /// Create a new cluster_capacity_providers resource
    async fn create_cluster_capacity_providers(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let capacity_providers = input.get_string("capacity_providers")?;
            let default_capacity_provider_strategy =
                input.get_string("default_capacity_provider_strategy")?;
            let cluster = input.get_string("cluster")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_cluster_capacity_providers()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("capacity_providers", capacity_providers.unwrap_or_default())
                .with_field(
                    "default_capacity_provider_strategy",
                    default_capacity_provider_strategy.unwrap_or_default(),
                )
                .with_field("cluster", cluster.unwrap_or_default()))
        })
    }

    /// Read a cluster_capacity_providers resource
    async fn read_cluster_capacity_providers(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_cluster_capacity_providers()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a cluster_capacity_providers resource
    async fn update_cluster_capacity_providers(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let capacity_providers = input.get_string("capacity_providers")?;
            let default_capacity_provider_strategy =
                input.get_string("default_capacity_provider_strategy")?;
            let cluster = input.get_string("cluster")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_cluster_capacity_providers()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("capacity_providers", capacity_providers.unwrap_or_default())
                .with_field(
                    "default_capacity_provider_strategy",
                    default_capacity_provider_strategy.unwrap_or_default(),
                )
                .with_field("cluster", cluster.unwrap_or_default()))
        })
    }

    /// Delete a cluster_capacity_providers resource
    async fn delete_cluster_capacity_providers(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_cluster_capacity_providers()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Task_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a task_set resource
    async fn plan_task_set(
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

    /// Create a new task_set resource
    async fn create_task_set(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let service = input.get_string("service")?;
            let launch_type = input.get_optional_string("launch_type")?;
            let service_registries = input.get_optional_string("service_registries")?;
            let platform_version = input.get_optional_string("platform_version")?;
            let client_token = input.get_optional_string("client_token")?;
            let tags = input.get_optional_string("tags")?;
            let task_definition = input.get_string("task_definition")?;
            let cluster = input.get_string("cluster")?;
            let scale = input.get_optional_string("scale")?;
            let external_id = input.get_optional_string("external_id")?;
            let load_balancers = input.get_optional_string("load_balancers")?;
            let network_configuration = input.get_optional_string("network_configuration")?;
            let capacity_provider_strategy =
                input.get_optional_string("capacity_provider_strategy")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_task_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("service", service.unwrap_or_default())
                .with_field("launch_type", launch_type.unwrap_or_default())
                .with_field("service_registries", service_registries.unwrap_or_default())
                .with_field("platform_version", platform_version.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("task_definition", task_definition.unwrap_or_default())
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("scale", scale.unwrap_or_default())
                .with_field("external_id", external_id.unwrap_or_default())
                .with_field("load_balancers", load_balancers.unwrap_or_default())
                .with_field(
                    "network_configuration",
                    network_configuration.unwrap_or_default(),
                )
                .with_field(
                    "capacity_provider_strategy",
                    capacity_provider_strategy.unwrap_or_default(),
                ))
        })
    }

    /// Read a task_set resource
    async fn read_task_set(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_task_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a task_set resource
    async fn update_task_set(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let service = input.get_string("service")?;
            let launch_type = input.get_optional_string("launch_type")?;
            let service_registries = input.get_optional_string("service_registries")?;
            let platform_version = input.get_optional_string("platform_version")?;
            let client_token = input.get_optional_string("client_token")?;
            let tags = input.get_optional_string("tags")?;
            let task_definition = input.get_string("task_definition")?;
            let cluster = input.get_string("cluster")?;
            let scale = input.get_optional_string("scale")?;
            let external_id = input.get_optional_string("external_id")?;
            let load_balancers = input.get_optional_string("load_balancers")?;
            let network_configuration = input.get_optional_string("network_configuration")?;
            let capacity_provider_strategy =
                input.get_optional_string("capacity_provider_strategy")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_task_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("service", service.unwrap_or_default())
                .with_field("launch_type", launch_type.unwrap_or_default())
                .with_field("service_registries", service_registries.unwrap_or_default())
                .with_field("platform_version", platform_version.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("task_definition", task_definition.unwrap_or_default())
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("scale", scale.unwrap_or_default())
                .with_field("external_id", external_id.unwrap_or_default())
                .with_field("load_balancers", load_balancers.unwrap_or_default())
                .with_field(
                    "network_configuration",
                    network_configuration.unwrap_or_default(),
                )
                .with_field(
                    "capacity_provider_strategy",
                    capacity_provider_strategy.unwrap_or_default(),
                ))
        })
    }

    /// Delete a task_set resource
    async fn delete_task_set(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_task_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Tasks resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tasks resource
    async fn plan_tasks(
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

    /// Create a new tasks resource
    async fn create_tasks(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_tasks()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a tasks resource
    async fn read_tasks(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_tasks()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a tasks resource
    async fn update_tasks(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_tasks()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a tasks resource
    async fn delete_tasks(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_tasks()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Cluster_settings resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cluster_settings resource
    async fn plan_cluster_settings(
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

    /// Create a new cluster_settings resource
    async fn create_cluster_settings(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cluster = input.get_string("cluster")?;
            let settings = input.get_string("settings")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_cluster_settings()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("settings", settings.unwrap_or_default()))
        })
    }

    /// Read a cluster_settings resource
    async fn read_cluster_settings(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_cluster_settings()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a cluster_settings resource
    async fn update_cluster_settings(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cluster = input.get_string("cluster")?;
            let settings = input.get_string("settings")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_cluster_settings()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("settings", settings.unwrap_or_default()))
        })
    }

    /// Delete a cluster_settings resource
    async fn delete_cluster_settings(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_cluster_settings()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Task_protection resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a task_protection resource
    async fn plan_task_protection(
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

    /// Create a new task_protection resource
    async fn create_task_protection(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tasks = input.get_string("tasks")?;
            let protection_enabled = input.get_string("protection_enabled")?;
            let cluster = input.get_string("cluster")?;
            let expires_in_minutes = input.get_optional_string("expires_in_minutes")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_task_protection()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("tasks", tasks.unwrap_or_default())
                .with_field("protection_enabled", protection_enabled.unwrap_or_default())
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("expires_in_minutes", expires_in_minutes.unwrap_or_default()))
        })
    }

    /// Read a task_protection resource
    async fn read_task_protection(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_task_protection()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a task_protection resource
    async fn update_task_protection(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tasks = input.get_string("tasks")?;
            let protection_enabled = input.get_string("protection_enabled")?;
            let cluster = input.get_string("cluster")?;
            let expires_in_minutes = input.get_optional_string("expires_in_minutes")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_task_protection()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("tasks", tasks.unwrap_or_default())
                .with_field("protection_enabled", protection_enabled.unwrap_or_default())
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("expires_in_minutes", expires_in_minutes.unwrap_or_default()))
        })
    }

    /// Delete a task_protection resource
    async fn delete_task_protection(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_task_protection()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Task_definitions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a task_definitions resource
    async fn plan_task_definitions(
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

    /// Create a new task_definitions resource
    async fn create_task_definitions(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_task_definitions()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a task_definitions resource
    async fn read_task_definitions(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_task_definitions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a task_definitions resource
    async fn update_task_definitions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_task_definitions()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a task_definitions resource
    async fn delete_task_definitions(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_task_definitions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Account_setting_default resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a account_setting_default resource
    async fn plan_account_setting_default(
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

    /// Create a new account_setting_default resource
    async fn create_account_setting_default(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let value = input.get_string("value")?;
            let name = input.get_string("name")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_account_setting_default()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("value", value.unwrap_or_default())
                .with_field("name", name.unwrap_or_default()))
        })
    }

    /// Read a account_setting_default resource
    async fn read_account_setting_default(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_account_setting_default()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a account_setting_default resource
    async fn update_account_setting_default(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let value = input.get_string("value")?;
            let name = input.get_string("name")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_account_setting_default()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("value", value.unwrap_or_default())
                .with_field("name", name.unwrap_or_default()))
        })
    }

    /// Delete a account_setting_default resource
    async fn delete_account_setting_default(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_account_setting_default()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Container_agent resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a container_agent resource
    async fn plan_container_agent(
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

    /// Create a new container_agent resource
    async fn create_container_agent(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cluster = input.get_optional_string("cluster")?;
            let container_instance = input.get_string("container_instance")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_container_agent()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("container_instance", container_instance.unwrap_or_default()))
        })
    }

    /// Read a container_agent resource
    async fn read_container_agent(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_container_agent()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a container_agent resource
    async fn update_container_agent(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cluster = input.get_optional_string("cluster")?;
            let container_instance = input.get_string("container_instance")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_container_agent()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("container_instance", container_instance.unwrap_or_default()))
        })
    }

    /// Delete a container_agent resource
    async fn delete_container_agent(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_container_agent()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Service_primary_task_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a service_primary_task_set resource
    async fn plan_service_primary_task_set(
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

    /// Create a new service_primary_task_set resource
    async fn create_service_primary_task_set(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let service = input.get_string("service")?;
            let cluster = input.get_string("cluster")?;
            let primary_task_set = input.get_string("primary_task_set")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_service_primary_task_set()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("service", service.unwrap_or_default())
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("primary_task_set", primary_task_set.unwrap_or_default()))
        })
    }

    /// Read a service_primary_task_set resource
    async fn read_service_primary_task_set(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_service_primary_task_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a service_primary_task_set resource
    async fn update_service_primary_task_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let service = input.get_string("service")?;
            let cluster = input.get_string("cluster")?;
            let primary_task_set = input.get_string("primary_task_set")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_service_primary_task_set()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("service", service.unwrap_or_default())
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("primary_task_set", primary_task_set.unwrap_or_default()))
        })
    }

    /// Delete a service_primary_task_set resource
    async fn delete_service_primary_task_set(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_service_primary_task_set()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Attributes resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a attributes resource
    async fn plan_attributes(
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

    /// Create a new attributes resource
    async fn create_attributes(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cluster = input.get_optional_string("cluster")?;
            let attributes = input.get_string("attributes")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_attributes()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("attributes", attributes.unwrap_or_default()))
        })
    }

    /// Read a attributes resource
    async fn read_attributes(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_attributes()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a attributes resource
    async fn update_attributes(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cluster = input.get_optional_string("cluster")?;
            let attributes = input.get_string("attributes")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_attributes()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("attributes", attributes.unwrap_or_default()))
        })
    }

    /// Delete a attributes resource
    async fn delete_attributes(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_attributes()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Service resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a service resource
    async fn plan_service(
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

    /// Create a new service resource
    async fn create_service(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let service_connect_configuration =
                input.get_optional_string("service_connect_configuration")?;
            let availability_zone_rebalancing =
                input.get_optional_string("availability_zone_rebalancing")?;
            let cluster = input.get_optional_string("cluster")?;
            let role = input.get_optional_string("role")?;
            let health_check_grace_period_seconds =
                input.get_optional_string("health_check_grace_period_seconds")?;
            let platform_version = input.get_optional_string("platform_version")?;
            let enable_ecs_managed_tags = input.get_optional_string("enable_ecs_managed_tags")?;
            let load_balancers = input.get_optional_string("load_balancers")?;
            let network_configuration = input.get_optional_string("network_configuration")?;
            let client_token = input.get_optional_string("client_token")?;
            let deployment_configuration = input.get_optional_string("deployment_configuration")?;
            let service_name = input.get_string("service_name")?;
            let task_definition = input.get_optional_string("task_definition")?;
            let placement_constraints = input.get_optional_string("placement_constraints")?;
            let propagate_tags = input.get_optional_string("propagate_tags")?;
            let enable_execute_command = input.get_optional_string("enable_execute_command")?;
            let capacity_provider_strategy =
                input.get_optional_string("capacity_provider_strategy")?;
            let volume_configurations = input.get_optional_string("volume_configurations")?;
            let launch_type = input.get_optional_string("launch_type")?;
            let service_registries = input.get_optional_string("service_registries")?;
            let desired_count = input.get_optional_string("desired_count")?;
            let deployment_controller = input.get_optional_string("deployment_controller")?;
            let vpc_lattice_configurations =
                input.get_optional_string("vpc_lattice_configurations")?;
            let scheduling_strategy = input.get_optional_string("scheduling_strategy")?;
            let placement_strategy = input.get_optional_string("placement_strategy")?;
            let tags = input.get_optional_string("tags")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_service()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field(
                    "service_connect_configuration",
                    service_connect_configuration.unwrap_or_default(),
                )
                .with_field(
                    "availability_zone_rebalancing",
                    availability_zone_rebalancing.unwrap_or_default(),
                )
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("role", role.unwrap_or_default())
                .with_field(
                    "health_check_grace_period_seconds",
                    health_check_grace_period_seconds.unwrap_or_default(),
                )
                .with_field("platform_version", platform_version.unwrap_or_default())
                .with_field(
                    "enable_ecs_managed_tags",
                    enable_ecs_managed_tags.unwrap_or_default(),
                )
                .with_field("load_balancers", load_balancers.unwrap_or_default())
                .with_field(
                    "network_configuration",
                    network_configuration.unwrap_or_default(),
                )
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field(
                    "deployment_configuration",
                    deployment_configuration.unwrap_or_default(),
                )
                .with_field("service_name", service_name.unwrap_or_default())
                .with_field("task_definition", task_definition.unwrap_or_default())
                .with_field(
                    "placement_constraints",
                    placement_constraints.unwrap_or_default(),
                )
                .with_field("propagate_tags", propagate_tags.unwrap_or_default())
                .with_field(
                    "enable_execute_command",
                    enable_execute_command.unwrap_or_default(),
                )
                .with_field(
                    "capacity_provider_strategy",
                    capacity_provider_strategy.unwrap_or_default(),
                )
                .with_field(
                    "volume_configurations",
                    volume_configurations.unwrap_or_default(),
                )
                .with_field("launch_type", launch_type.unwrap_or_default())
                .with_field("service_registries", service_registries.unwrap_or_default())
                .with_field("desired_count", desired_count.unwrap_or_default())
                .with_field(
                    "deployment_controller",
                    deployment_controller.unwrap_or_default(),
                )
                .with_field(
                    "vpc_lattice_configurations",
                    vpc_lattice_configurations.unwrap_or_default(),
                )
                .with_field(
                    "scheduling_strategy",
                    scheduling_strategy.unwrap_or_default(),
                )
                .with_field("placement_strategy", placement_strategy.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default()))
        })
    }

    /// Read a service resource
    async fn read_service(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_service()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a service resource
    async fn update_service(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let service_connect_configuration =
                input.get_optional_string("service_connect_configuration")?;
            let availability_zone_rebalancing =
                input.get_optional_string("availability_zone_rebalancing")?;
            let cluster = input.get_optional_string("cluster")?;
            let role = input.get_optional_string("role")?;
            let health_check_grace_period_seconds =
                input.get_optional_string("health_check_grace_period_seconds")?;
            let platform_version = input.get_optional_string("platform_version")?;
            let enable_ecs_managed_tags = input.get_optional_string("enable_ecs_managed_tags")?;
            let load_balancers = input.get_optional_string("load_balancers")?;
            let network_configuration = input.get_optional_string("network_configuration")?;
            let client_token = input.get_optional_string("client_token")?;
            let deployment_configuration = input.get_optional_string("deployment_configuration")?;
            let service_name = input.get_string("service_name")?;
            let task_definition = input.get_optional_string("task_definition")?;
            let placement_constraints = input.get_optional_string("placement_constraints")?;
            let propagate_tags = input.get_optional_string("propagate_tags")?;
            let enable_execute_command = input.get_optional_string("enable_execute_command")?;
            let capacity_provider_strategy =
                input.get_optional_string("capacity_provider_strategy")?;
            let volume_configurations = input.get_optional_string("volume_configurations")?;
            let launch_type = input.get_optional_string("launch_type")?;
            let service_registries = input.get_optional_string("service_registries")?;
            let desired_count = input.get_optional_string("desired_count")?;
            let deployment_controller = input.get_optional_string("deployment_controller")?;
            let vpc_lattice_configurations =
                input.get_optional_string("vpc_lattice_configurations")?;
            let scheduling_strategy = input.get_optional_string("scheduling_strategy")?;
            let placement_strategy = input.get_optional_string("placement_strategy")?;
            let tags = input.get_optional_string("tags")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_service()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field(
                    "service_connect_configuration",
                    service_connect_configuration.unwrap_or_default(),
                )
                .with_field(
                    "availability_zone_rebalancing",
                    availability_zone_rebalancing.unwrap_or_default(),
                )
                .with_field("cluster", cluster.unwrap_or_default())
                .with_field("role", role.unwrap_or_default())
                .with_field(
                    "health_check_grace_period_seconds",
                    health_check_grace_period_seconds.unwrap_or_default(),
                )
                .with_field("platform_version", platform_version.unwrap_or_default())
                .with_field(
                    "enable_ecs_managed_tags",
                    enable_ecs_managed_tags.unwrap_or_default(),
                )
                .with_field("load_balancers", load_balancers.unwrap_or_default())
                .with_field(
                    "network_configuration",
                    network_configuration.unwrap_or_default(),
                )
                .with_field("client_token", client_token.unwrap_or_default())
                .with_field(
                    "deployment_configuration",
                    deployment_configuration.unwrap_or_default(),
                )
                .with_field("service_name", service_name.unwrap_or_default())
                .with_field("task_definition", task_definition.unwrap_or_default())
                .with_field(
                    "placement_constraints",
                    placement_constraints.unwrap_or_default(),
                )
                .with_field("propagate_tags", propagate_tags.unwrap_or_default())
                .with_field(
                    "enable_execute_command",
                    enable_execute_command.unwrap_or_default(),
                )
                .with_field(
                    "capacity_provider_strategy",
                    capacity_provider_strategy.unwrap_or_default(),
                )
                .with_field(
                    "volume_configurations",
                    volume_configurations.unwrap_or_default(),
                )
                .with_field("launch_type", launch_type.unwrap_or_default())
                .with_field("service_registries", service_registries.unwrap_or_default())
                .with_field("desired_count", desired_count.unwrap_or_default())
                .with_field(
                    "deployment_controller",
                    deployment_controller.unwrap_or_default(),
                )
                .with_field(
                    "vpc_lattice_configurations",
                    vpc_lattice_configurations.unwrap_or_default(),
                )
                .with_field(
                    "scheduling_strategy",
                    scheduling_strategy.unwrap_or_default(),
                )
                .with_field("placement_strategy", placement_strategy.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default()))
        })
    }

    /// Delete a service resource
    async fn delete_service(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_service()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Task_definition resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a task_definition resource
    async fn plan_task_definition(
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

    /// Create a new task_definition resource
    async fn create_task_definition(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_task_definition()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a task_definition resource
    async fn read_task_definition(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_task_definition()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a task_definition resource
    async fn update_task_definition(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_task_definition()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a task_definition resource
    async fn delete_task_definition(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_task_definition()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Service_revisions resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a service_revisions resource
    async fn plan_service_revisions(
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

    /// Create a new service_revisions resource
    async fn create_service_revisions(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_service_revisions()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a service_revisions resource
    async fn read_service_revisions(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_service_revisions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a service_revisions resource
    async fn update_service_revisions(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_service_revisions()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a service_revisions resource
    async fn delete_service_revisions(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_service_revisions()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Task_sets resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a task_sets resource
    async fn plan_task_sets(
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

    /// Create a new task_sets resource
    async fn create_task_sets(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_task_sets()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a task_sets resource
    async fn read_task_sets(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_task_sets()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a task_sets resource
    async fn update_task_sets(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_task_sets()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a task_sets resource
    async fn delete_task_sets(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_task_sets()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Clusters resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a clusters resource
    async fn plan_clusters(
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

    /// Create a new clusters resource
    async fn create_clusters(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_clusters()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a clusters resource
    async fn read_clusters(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_clusters()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a clusters resource
    async fn update_clusters(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_clusters()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a clusters resource
    async fn delete_clusters(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_clusters()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Container_instances_state resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a container_instances_state resource
    async fn plan_container_instances_state(
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

    /// Create a new container_instances_state resource
    async fn create_container_instances_state(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let container_instances = input.get_string("container_instances")?;
            let status = input.get_string("status")?;
            let cluster = input.get_optional_string("cluster")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_container_instances_state()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field(
                    "container_instances",
                    container_instances.unwrap_or_default(),
                )
                .with_field("status", status.unwrap_or_default())
                .with_field("cluster", cluster.unwrap_or_default()))
        })
    }

    /// Read a container_instances_state resource
    async fn read_container_instances_state(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_container_instances_state()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a container_instances_state resource
    async fn update_container_instances_state(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let container_instances = input.get_string("container_instances")?;
            let status = input.get_string("status")?;
            let cluster = input.get_optional_string("cluster")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_container_instances_state()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field(
                    "container_instances",
                    container_instances.unwrap_or_default(),
                )
                .with_field("status", status.unwrap_or_default())
                .with_field("cluster", cluster.unwrap_or_default()))
        })
    }

    /// Delete a container_instances_state resource
    async fn delete_container_instances_state(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_container_instances_state()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Account_setting resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a account_setting resource
    async fn plan_account_setting(
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

    /// Create a new account_setting resource
    async fn create_account_setting(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let value = input.get_string("value")?;
            let principal_arn = input.get_optional_string("principal_arn")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_account_setting()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("name", name.unwrap_or_default())
                .with_field("value", value.unwrap_or_default())
                .with_field("principal_arn", principal_arn.unwrap_or_default()))
        })
    }

    /// Read a account_setting resource
    async fn read_account_setting(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_account_setting()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a account_setting resource
    async fn update_account_setting(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let name = input.get_string("name")?;
            let value = input.get_string("value")?;
            let principal_arn = input.get_optional_string("principal_arn")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_account_setting()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("name", name.unwrap_or_default())
                .with_field("value", value.unwrap_or_default())
                .with_field("principal_arn", principal_arn.unwrap_or_default()))
        })
    }

    /// Delete a account_setting resource
    async fn delete_account_setting(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_account_setting()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Capacity_providers resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a capacity_providers resource
    async fn plan_capacity_providers(
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

    /// Create a new capacity_providers resource
    async fn create_capacity_providers(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_capacity_providers()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a capacity_providers resource
    async fn read_capacity_providers(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_capacity_providers()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a capacity_providers resource
    async fn update_capacity_providers(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_capacity_providers()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a capacity_providers resource
    async fn delete_capacity_providers(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_capacity_providers()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Cluster resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cluster resource
    async fn plan_cluster(
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

    /// Create a new cluster resource
    async fn create_cluster(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let configuration = input.get_optional_string("configuration")?;
            let settings = input.get_optional_string("settings")?;
            let cluster_name = input.get_optional_string("cluster_name")?;
            let service_connect_defaults = input.get_optional_string("service_connect_defaults")?;
            let tags = input.get_optional_string("tags")?;
            let capacity_providers = input.get_optional_string("capacity_providers")?;
            let default_capacity_provider_strategy =
                input.get_optional_string("default_capacity_provider_strategy")?;

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_cluster()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("configuration", configuration.unwrap_or_default())
                .with_field("settings", settings.unwrap_or_default())
                .with_field("cluster_name", cluster_name.unwrap_or_default())
                .with_field(
                    "service_connect_defaults",
                    service_connect_defaults.unwrap_or_default(),
                )
                .with_field("tags", tags.unwrap_or_default())
                .with_field("capacity_providers", capacity_providers.unwrap_or_default())
                .with_field(
                    "default_capacity_provider_strategy",
                    default_capacity_provider_strategy.unwrap_or_default(),
                ))
        })
    }

    /// Read a cluster resource
    async fn read_cluster(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_cluster()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a cluster resource
    async fn update_cluster(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let configuration = input.get_optional_string("configuration")?;
            let settings = input.get_optional_string("settings")?;
            let cluster_name = input.get_optional_string("cluster_name")?;
            let service_connect_defaults = input.get_optional_string("service_connect_defaults")?;
            let tags = input.get_optional_string("tags")?;
            let capacity_providers = input.get_optional_string("capacity_providers")?;
            let default_capacity_provider_strategy =
                input.get_optional_string("default_capacity_provider_strategy")?;

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_cluster()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("configuration", configuration.unwrap_or_default())
                .with_field("settings", settings.unwrap_or_default())
                .with_field("cluster_name", cluster_name.unwrap_or_default())
                .with_field(
                    "service_connect_defaults",
                    service_connect_defaults.unwrap_or_default(),
                )
                .with_field("tags", tags.unwrap_or_default())
                .with_field("capacity_providers", capacity_providers.unwrap_or_default())
                .with_field(
                    "default_capacity_provider_strategy",
                    default_capacity_provider_strategy.unwrap_or_default(),
                ))
        })
    }

    /// Delete a cluster resource
    async fn delete_cluster(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_cluster()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }

    // ------------------------------------------------------------------------
    // Container_instances resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a container_instances resource
    async fn plan_container_instances(
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

    /// Create a new container_instances resource
    async fn create_container_instances(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .create_container_instances()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id("placeholder-id"))
        })
    }

    /// Read a container_instances resource
    async fn read_container_instances(&self, id: &str) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .describe_container_instances()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Update a container_instances resource
    async fn update_container_instances(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields

            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.ecs_client
            //     .update_container_instances()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new().with_id(id))
        })
    }

    /// Delete a container_instances resource
    async fn delete_container_instances(&self, id: &str) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.ecs_client
            //     .delete_container_instances()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }
}
