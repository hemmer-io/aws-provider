//! Cloudhsm service for Aws provider
//!
//! This module handles all cloudhsm resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Cloudhsm service handler
pub struct CloudhsmService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CloudhsmService<'a> {
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
            "config" => {
                self.plan_config(current_state, desired_input).await
            }
            "hapg" => {
                self.plan_hapg(current_state, desired_input).await
            }
            "hsm" => {
                self.plan_hsm(current_state, desired_input).await
            }
            "luna_client" => {
                self.plan_luna_client(current_state, desired_input).await
            }
            "hsm" => {
                self.plan_hsm(current_state, desired_input).await
            }
            "resource_policy" => {
                self.plan_resource_policy(current_state, desired_input).await
            }
            "backups" => {
                self.plan_backups(current_state, desired_input).await
            }
            "clusters" => {
                self.plan_clusters(current_state, desired_input).await
            }
            "cluster" => {
                self.plan_cluster(current_state, desired_input).await
            }
            "backup" => {
                self.plan_backup(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudhsm",
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
            "config" => {
                self.create_config(input).await
            }
            "hapg" => {
                self.create_hapg(input).await
            }
            "hsm" => {
                self.create_hsm(input).await
            }
            "luna_client" => {
                self.create_luna_client(input).await
            }
            "hsm" => {
                self.create_hsm(input).await
            }
            "resource_policy" => {
                self.create_resource_policy(input).await
            }
            "backups" => {
                self.create_backups(input).await
            }
            "clusters" => {
                self.create_clusters(input).await
            }
            "cluster" => {
                self.create_cluster(input).await
            }
            "backup" => {
                self.create_backup(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudhsm",
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
            "config" => {
                self.read_config(id).await
            }
            "hapg" => {
                self.read_hapg(id).await
            }
            "hsm" => {
                self.read_hsm(id).await
            }
            "luna_client" => {
                self.read_luna_client(id).await
            }
            "hsm" => {
                self.read_hsm(id).await
            }
            "resource_policy" => {
                self.read_resource_policy(id).await
            }
            "backups" => {
                self.read_backups(id).await
            }
            "clusters" => {
                self.read_clusters(id).await
            }
            "cluster" => {
                self.read_cluster(id).await
            }
            "backup" => {
                self.read_backup(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudhsm",
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
            "config" => {
                self.update_config(id, input).await
            }
            "hapg" => {
                self.update_hapg(id, input).await
            }
            "hsm" => {
                self.update_hsm(id, input).await
            }
            "luna_client" => {
                self.update_luna_client(id, input).await
            }
            "hsm" => {
                self.update_hsm(id, input).await
            }
            "resource_policy" => {
                self.update_resource_policy(id, input).await
            }
            "backups" => {
                self.update_backups(id, input).await
            }
            "clusters" => {
                self.update_clusters(id, input).await
            }
            "cluster" => {
                self.update_cluster(id, input).await
            }
            "backup" => {
                self.update_backup(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudhsm",
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
            "config" => {
                self.delete_config(id).await
            }
            "hapg" => {
                self.delete_hapg(id).await
            }
            "hsm" => {
                self.delete_hsm(id).await
            }
            "luna_client" => {
                self.delete_luna_client(id).await
            }
            "hsm" => {
                self.delete_hsm(id).await
            }
            "resource_policy" => {
                self.delete_resource_policy(id).await
            }
            "backups" => {
                self.delete_backups(id).await
            }
            "clusters" => {
                self.delete_clusters(id).await
            }
            "cluster" => {
                self.delete_cluster(id).await
            }
            "backup" => {
                self.delete_backup(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "cloudhsm",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a config resource
    async fn plan_config(
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

    /// Create a new config resource
    async fn create_config(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .create_config()
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

    /// Read a config resource
    async fn read_config(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .describe_config()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a config resource
    async fn update_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .update_config()
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

    /// Delete a config resource
    async fn delete_config(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudhsm_client
            //     .delete_config()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Hapg resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hapg resource
    async fn plan_hapg(
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

    /// Create a new hapg resource
    async fn create_hapg(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let label = input.get_string("label")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .create_hapg()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("label", label.unwrap_or_default())
            )
        })
    }

    /// Read a hapg resource
    async fn read_hapg(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .describe_hapg()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a hapg resource
    async fn update_hapg(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let label = input.get_string("label")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .update_hapg()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("label", label.unwrap_or_default())
            )
        })
    }

    /// Delete a hapg resource
    async fn delete_hapg(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudhsm_client
            //     .delete_hapg()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Hsm resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hsm resource
    async fn plan_hsm(
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

    /// Create a new hsm resource
    async fn create_hsm(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let ssh_key = input.get_string("ssh_key")?;
            let eni_ip = input.get_optional_string("eni_ip")?;
            let iam_role_arn = input.get_string("iam_role_arn")?;
            let subnet_id = input.get_string("subnet_id")?;
            let external_id = input.get_optional_string("external_id")?;
            let subscription_type = input.get_string("subscription_type")?;
            let syslog_ip = input.get_optional_string("syslog_ip")?;
            let client_token = input.get_optional_string("client_token")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .create_hsm()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("ssh_key", ssh_key.unwrap_or_default())
                .with_field("eni_ip", eni_ip.unwrap_or_default())
                .with_field("iam_role_arn", iam_role_arn.unwrap_or_default())
                .with_field("subnet_id", subnet_id.unwrap_or_default())
                .with_field("external_id", external_id.unwrap_or_default())
                .with_field("subscription_type", subscription_type.unwrap_or_default())
                .with_field("syslog_ip", syslog_ip.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
            )
        })
    }

    /// Read a hsm resource
    async fn read_hsm(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .describe_hsm()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a hsm resource
    async fn update_hsm(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let ssh_key = input.get_string("ssh_key")?;
            let eni_ip = input.get_optional_string("eni_ip")?;
            let iam_role_arn = input.get_string("iam_role_arn")?;
            let subnet_id = input.get_string("subnet_id")?;
            let external_id = input.get_optional_string("external_id")?;
            let subscription_type = input.get_string("subscription_type")?;
            let syslog_ip = input.get_optional_string("syslog_ip")?;
            let client_token = input.get_optional_string("client_token")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .update_hsm()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("ssh_key", ssh_key.unwrap_or_default())
                .with_field("eni_ip", eni_ip.unwrap_or_default())
                .with_field("iam_role_arn", iam_role_arn.unwrap_or_default())
                .with_field("subnet_id", subnet_id.unwrap_or_default())
                .with_field("external_id", external_id.unwrap_or_default())
                .with_field("subscription_type", subscription_type.unwrap_or_default())
                .with_field("syslog_ip", syslog_ip.unwrap_or_default())
                .with_field("client_token", client_token.unwrap_or_default())
            )
        })
    }

    /// Delete a hsm resource
    async fn delete_hsm(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudhsm_client
            //     .delete_hsm()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Luna_client resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a luna_client resource
    async fn plan_luna_client(
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

    /// Create a new luna_client resource
    async fn create_luna_client(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let label = input.get_optional_string("label")?;
            let certificate = input.get_string("certificate")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .create_luna_client()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("label", label.unwrap_or_default())
                .with_field("certificate", certificate.unwrap_or_default())
            )
        })
    }

    /// Read a luna_client resource
    async fn read_luna_client(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .describe_luna_client()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a luna_client resource
    async fn update_luna_client(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let label = input.get_optional_string("label")?;
            let certificate = input.get_string("certificate")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .update_luna_client()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("label", label.unwrap_or_default())
                .with_field("certificate", certificate.unwrap_or_default())
            )
        })
    }

    /// Delete a luna_client resource
    async fn delete_luna_client(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudhsm_client
            //     .delete_luna_client()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Hsm resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hsm resource
    async fn plan_hsm(
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

    /// Create a new hsm resource
    async fn create_hsm(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cluster_id = input.get_string("cluster_id")?;
            let availability_zone = input.get_string("availability_zone")?;
            let ip_address = input.get_optional_string("ip_address")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .create_hsm()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("cluster_id", cluster_id.unwrap_or_default())
                .with_field("availability_zone", availability_zone.unwrap_or_default())
                .with_field("ip_address", ip_address.unwrap_or_default())
            )
        })
    }

    /// Read a hsm resource
    async fn read_hsm(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .describe_hsm()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a hsm resource
    async fn update_hsm(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let cluster_id = input.get_string("cluster_id")?;
            let availability_zone = input.get_string("availability_zone")?;
            let ip_address = input.get_optional_string("ip_address")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .update_hsm()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("cluster_id", cluster_id.unwrap_or_default())
                .with_field("availability_zone", availability_zone.unwrap_or_default())
                .with_field("ip_address", ip_address.unwrap_or_default())
            )
        })
    }

    /// Delete a hsm resource
    async fn delete_hsm(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudhsm_client
            //     .delete_hsm()
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
            let policy = input.get_optional_string("policy")?;
            let resource_arn = input.get_optional_string("resource_arn")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .create_resource_policy()
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

    /// Read a resource_policy resource
    async fn read_resource_policy(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudhsm_client
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
            let policy = input.get_optional_string("policy")?;
            let resource_arn = input.get_optional_string("resource_arn")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .update_resource_policy()
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

    /// Delete a resource_policy resource
    async fn delete_resource_policy(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudhsm_client
            //     .delete_resource_policy()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Backups resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a backups resource
    async fn plan_backups(
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

    /// Create a new backups resource
    async fn create_backups(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .create_backups()
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

    /// Read a backups resource
    async fn read_backups(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .describe_backups()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a backups resource
    async fn update_backups(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .update_backups()
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

    /// Delete a backups resource
    async fn delete_backups(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudhsm_client
            //     .delete_backups()
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
    async fn create_clusters(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .create_clusters()
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

    /// Read a clusters resource
    async fn read_clusters(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .describe_clusters()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a clusters resource
    async fn update_clusters(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .update_clusters()
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

    /// Delete a clusters resource
    async fn delete_clusters(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudhsm_client
            //     .delete_clusters()
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
    async fn create_cluster(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let hsm_type = input.get_string("hsm_type")?;
            let source_backup_id = input.get_optional_string("source_backup_id")?;
            let network_type = input.get_optional_string("network_type")?;
            let backup_retention_policy = input.get_optional_string("backup_retention_policy")?;
            let tag_list = input.get_optional_string("tag_list")?;
            let mode = input.get_optional_string("mode")?;
            let subnet_ids = input.get_string("subnet_ids")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .create_cluster()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("hsm_type", hsm_type.unwrap_or_default())
                .with_field("source_backup_id", source_backup_id.unwrap_or_default())
                .with_field("network_type", network_type.unwrap_or_default())
                .with_field("backup_retention_policy", backup_retention_policy.unwrap_or_default())
                .with_field("tag_list", tag_list.unwrap_or_default())
                .with_field("mode", mode.unwrap_or_default())
                .with_field("subnet_ids", subnet_ids.unwrap_or_default())
            )
        })
    }

    /// Read a cluster resource
    async fn read_cluster(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .describe_cluster()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a cluster resource
    async fn update_cluster(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let hsm_type = input.get_string("hsm_type")?;
            let source_backup_id = input.get_optional_string("source_backup_id")?;
            let network_type = input.get_optional_string("network_type")?;
            let backup_retention_policy = input.get_optional_string("backup_retention_policy")?;
            let tag_list = input.get_optional_string("tag_list")?;
            let mode = input.get_optional_string("mode")?;
            let subnet_ids = input.get_string("subnet_ids")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .update_cluster()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("hsm_type", hsm_type.unwrap_or_default())
                .with_field("source_backup_id", source_backup_id.unwrap_or_default())
                .with_field("network_type", network_type.unwrap_or_default())
                .with_field("backup_retention_policy", backup_retention_policy.unwrap_or_default())
                .with_field("tag_list", tag_list.unwrap_or_default())
                .with_field("mode", mode.unwrap_or_default())
                .with_field("subnet_ids", subnet_ids.unwrap_or_default())
            )
        })
    }

    /// Delete a cluster resource
    async fn delete_cluster(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudhsm_client
            //     .delete_cluster()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Backup resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a backup resource
    async fn plan_backup(
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

    /// Create a new backup resource
    async fn create_backup(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .create_backup()
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

    /// Read a backup resource
    async fn read_backup(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .describe_backup()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a backup resource
    async fn update_backup(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.cloudhsm_client
            //     .update_backup()
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

    /// Delete a backup resource
    async fn delete_backup(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.cloudhsm_client
            //     .delete_backup()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
