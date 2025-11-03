//! Managedblockchain service for Aws provider
//!
//! This module handles all managedblockchain resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Managedblockchain service handler
pub struct ManagedblockchainService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ManagedblockchainService<'a> {
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
            "node" => {
                self.plan_node(current_state, desired_input).await
            }
            "network" => {
                self.plan_network(current_state, desired_input).await
            }
            "accessor" => {
                self.plan_accessor(current_state, desired_input).await
            }
            "member" => {
                self.plan_member(current_state, desired_input).await
            }
            "proposal" => {
                self.plan_proposal(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "managedblockchain",
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
            "node" => {
                self.create_node(input).await
            }
            "network" => {
                self.create_network(input).await
            }
            "accessor" => {
                self.create_accessor(input).await
            }
            "member" => {
                self.create_member(input).await
            }
            "proposal" => {
                self.create_proposal(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "managedblockchain",
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
            "node" => {
                self.read_node(id).await
            }
            "network" => {
                self.read_network(id).await
            }
            "accessor" => {
                self.read_accessor(id).await
            }
            "member" => {
                self.read_member(id).await
            }
            "proposal" => {
                self.read_proposal(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "managedblockchain",
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
            "node" => {
                self.update_node(id, input).await
            }
            "network" => {
                self.update_network(id, input).await
            }
            "accessor" => {
                self.update_accessor(id, input).await
            }
            "member" => {
                self.update_member(id, input).await
            }
            "proposal" => {
                self.update_proposal(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "managedblockchain",
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
            "node" => {
                self.delete_node(id).await
            }
            "network" => {
                self.delete_network(id).await
            }
            "accessor" => {
                self.delete_accessor(id).await
            }
            "member" => {
                self.delete_member(id).await
            }
            "proposal" => {
                self.delete_proposal(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "managedblockchain",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Node resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a node resource
    async fn plan_node(
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

    /// Create a new node resource
    async fn create_node(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let network_id = input.get_string("network_id")?;
            let member_id = input.get_optional_string("member_id")?;
            let node_configuration = input.get_string("node_configuration")?;
            let tags = input.get_optional_string("tags")?;
            let client_request_token = input.get_string("client_request_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .create_node()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("network_id", network_id.unwrap_or_default())
                .with_field("member_id", member_id.unwrap_or_default())
                .with_field("node_configuration", node_configuration.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
            )
        })
    }

    /// Read a node resource
    async fn read_node(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .describe_node()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a node resource
    async fn update_node(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let network_id = input.get_string("network_id")?;
            let member_id = input.get_optional_string("member_id")?;
            let node_configuration = input.get_string("node_configuration")?;
            let tags = input.get_optional_string("tags")?;
            let client_request_token = input.get_string("client_request_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .update_node()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("network_id", network_id.unwrap_or_default())
                .with_field("member_id", member_id.unwrap_or_default())
                .with_field("node_configuration", node_configuration.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
            )
        })
    }

    /// Delete a node resource
    async fn delete_node(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.managedblockchain_client
            //     .delete_node()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Network resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a network resource
    async fn plan_network(
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

    /// Create a new network resource
    async fn create_network(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let tags = input.get_optional_string("tags")?;
            let framework = input.get_string("framework")?;
            let member_configuration = input.get_string("member_configuration")?;
            let framework_version = input.get_string("framework_version")?;
            let name = input.get_string("name")?;
            let voting_policy = input.get_string("voting_policy")?;
            let client_request_token = input.get_string("client_request_token")?;
            let framework_configuration = input.get_optional_string("framework_configuration")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .create_network()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("description", description.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("framework", framework.unwrap_or_default())
                .with_field("member_configuration", member_configuration.unwrap_or_default())
                .with_field("framework_version", framework_version.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("voting_policy", voting_policy.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("framework_configuration", framework_configuration.unwrap_or_default())
            )
        })
    }

    /// Read a network resource
    async fn read_network(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .describe_network()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a network resource
    async fn update_network(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let description = input.get_optional_string("description")?;
            let tags = input.get_optional_string("tags")?;
            let framework = input.get_string("framework")?;
            let member_configuration = input.get_string("member_configuration")?;
            let framework_version = input.get_string("framework_version")?;
            let name = input.get_string("name")?;
            let voting_policy = input.get_string("voting_policy")?;
            let client_request_token = input.get_string("client_request_token")?;
            let framework_configuration = input.get_optional_string("framework_configuration")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .update_network()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("description", description.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("framework", framework.unwrap_or_default())
                .with_field("member_configuration", member_configuration.unwrap_or_default())
                .with_field("framework_version", framework_version.unwrap_or_default())
                .with_field("name", name.unwrap_or_default())
                .with_field("voting_policy", voting_policy.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("framework_configuration", framework_configuration.unwrap_or_default())
            )
        })
    }

    /// Delete a network resource
    async fn delete_network(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.managedblockchain_client
            //     .delete_network()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Accessor resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a accessor resource
    async fn plan_accessor(
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

    /// Create a new accessor resource
    async fn create_accessor(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let network_type = input.get_optional_string("network_type")?;
            let client_request_token = input.get_string("client_request_token")?;
            let tags = input.get_optional_string("tags")?;
            let accessor_type = input.get_string("accessor_type")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .create_accessor()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("network_type", network_type.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("accessor_type", accessor_type.unwrap_or_default())
            )
        })
    }

    /// Read a accessor resource
    async fn read_accessor(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .describe_accessor()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a accessor resource
    async fn update_accessor(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let network_type = input.get_optional_string("network_type")?;
            let client_request_token = input.get_string("client_request_token")?;
            let tags = input.get_optional_string("tags")?;
            let accessor_type = input.get_string("accessor_type")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .update_accessor()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("network_type", network_type.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("accessor_type", accessor_type.unwrap_or_default())
            )
        })
    }

    /// Delete a accessor resource
    async fn delete_accessor(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.managedblockchain_client
            //     .delete_accessor()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Member resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a member resource
    async fn plan_member(
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

    /// Create a new member resource
    async fn create_member(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let member_configuration = input.get_string("member_configuration")?;
            let client_request_token = input.get_string("client_request_token")?;
            let invitation_id = input.get_string("invitation_id")?;
            let network_id = input.get_string("network_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .create_member()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("member_configuration", member_configuration.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("invitation_id", invitation_id.unwrap_or_default())
                .with_field("network_id", network_id.unwrap_or_default())
            )
        })
    }

    /// Read a member resource
    async fn read_member(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .describe_member()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a member resource
    async fn update_member(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let member_configuration = input.get_string("member_configuration")?;
            let client_request_token = input.get_string("client_request_token")?;
            let invitation_id = input.get_string("invitation_id")?;
            let network_id = input.get_string("network_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .update_member()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("member_configuration", member_configuration.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("invitation_id", invitation_id.unwrap_or_default())
                .with_field("network_id", network_id.unwrap_or_default())
            )
        })
    }

    /// Delete a member resource
    async fn delete_member(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.managedblockchain_client
            //     .delete_member()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Proposal resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a proposal resource
    async fn plan_proposal(
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

    /// Create a new proposal resource
    async fn create_proposal(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let description = input.get_optional_string("description")?;
            let client_request_token = input.get_string("client_request_token")?;
            let member_id = input.get_string("member_id")?;
            let network_id = input.get_string("network_id")?;
            let actions = input.get_string("actions")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .create_proposal()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("tags", tags.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("member_id", member_id.unwrap_or_default())
                .with_field("network_id", network_id.unwrap_or_default())
                .with_field("actions", actions.unwrap_or_default())
            )
        })
    }

    /// Read a proposal resource
    async fn read_proposal(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .describe_proposal()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a proposal resource
    async fn update_proposal(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let tags = input.get_optional_string("tags")?;
            let description = input.get_optional_string("description")?;
            let client_request_token = input.get_string("client_request_token")?;
            let member_id = input.get_string("member_id")?;
            let network_id = input.get_string("network_id")?;
            let actions = input.get_string("actions")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.managedblockchain_client
            //     .update_proposal()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("tags", tags.unwrap_or_default())
                .with_field("description", description.unwrap_or_default())
                .with_field("client_request_token", client_request_token.unwrap_or_default())
                .with_field("member_id", member_id.unwrap_or_default())
                .with_field("network_id", network_id.unwrap_or_default())
                .with_field("actions", actions.unwrap_or_default())
            )
        })
    }

    /// Delete a proposal resource
    async fn delete_proposal(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.managedblockchain_client
            //     .delete_proposal()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
