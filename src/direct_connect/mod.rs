//! Direct_connect service for Aws provider
//!
//! This module handles all direct_connect resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Direct_connect service handler
pub struct Direct_connectService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Direct_connectService<'a> {
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
            "transit_virtual_interface" => {
                self.plan_transit_virtual_interface(current_state, desired_input).await
            }
            "direct_connect_gateway_associations" => {
                self.plan_direct_connect_gateway_associations(current_state, desired_input).await
            }
            "router_configuration" => {
                self.plan_router_configuration(current_state, desired_input).await
            }
            "interconnect" => {
                self.plan_interconnect(current_state, desired_input).await
            }
            "lag" => {
                self.plan_lag(current_state, desired_input).await
            }
            "connections" => {
                self.plan_connections(current_state, desired_input).await
            }
            "direct_connect_gateway_association_proposals" => {
                self.plan_direct_connect_gateway_association_proposals(current_state, desired_input).await
            }
            "lags" => {
                self.plan_lags(current_state, desired_input).await
            }
            "virtual_gateways" => {
                self.plan_virtual_gateways(current_state, desired_input).await
            }
            "connection" => {
                self.plan_connection(current_state, desired_input).await
            }
            "virtual_interfaces" => {
                self.plan_virtual_interfaces(current_state, desired_input).await
            }
            "hosted_connections" => {
                self.plan_hosted_connections(current_state, desired_input).await
            }
            "tags" => {
                self.plan_tags(current_state, desired_input).await
            }
            "direct_connect_gateway_attachments" => {
                self.plan_direct_connect_gateway_attachments(current_state, desired_input).await
            }
            "customer_metadata" => {
                self.plan_customer_metadata(current_state, desired_input).await
            }
            "public_virtual_interface" => {
                self.plan_public_virtual_interface(current_state, desired_input).await
            }
            "bgp_peer" => {
                self.plan_bgp_peer(current_state, desired_input).await
            }
            "virtual_interface_attributes" => {
                self.plan_virtual_interface_attributes(current_state, desired_input).await
            }
            "connections_on_interconnect" => {
                self.plan_connections_on_interconnect(current_state, desired_input).await
            }
            "virtual_interface" => {
                self.plan_virtual_interface(current_state, desired_input).await
            }
            "interconnect_loa" => {
                self.plan_interconnect_loa(current_state, desired_input).await
            }
            "loa" => {
                self.plan_loa(current_state, desired_input).await
            }
            "private_virtual_interface" => {
                self.plan_private_virtual_interface(current_state, desired_input).await
            }
            "locations" => {
                self.plan_locations(current_state, desired_input).await
            }
            "direct_connect_gateway" => {
                self.plan_direct_connect_gateway(current_state, desired_input).await
            }
            "interconnects" => {
                self.plan_interconnects(current_state, desired_input).await
            }
            "connection_loa" => {
                self.plan_connection_loa(current_state, desired_input).await
            }
            "direct_connect_gateways" => {
                self.plan_direct_connect_gateways(current_state, desired_input).await
            }
            "direct_connect_gateway_association_proposal" => {
                self.plan_direct_connect_gateway_association_proposal(current_state, desired_input).await
            }
            "direct_connect_gateway_association" => {
                self.plan_direct_connect_gateway_association(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "direct_connect",
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
            "transit_virtual_interface" => {
                self.create_transit_virtual_interface(input).await
            }
            "direct_connect_gateway_associations" => {
                self.create_direct_connect_gateway_associations(input).await
            }
            "router_configuration" => {
                self.create_router_configuration(input).await
            }
            "interconnect" => {
                self.create_interconnect(input).await
            }
            "lag" => {
                self.create_lag(input).await
            }
            "connections" => {
                self.create_connections(input).await
            }
            "direct_connect_gateway_association_proposals" => {
                self.create_direct_connect_gateway_association_proposals(input).await
            }
            "lags" => {
                self.create_lags(input).await
            }
            "virtual_gateways" => {
                self.create_virtual_gateways(input).await
            }
            "connection" => {
                self.create_connection(input).await
            }
            "virtual_interfaces" => {
                self.create_virtual_interfaces(input).await
            }
            "hosted_connections" => {
                self.create_hosted_connections(input).await
            }
            "tags" => {
                self.create_tags(input).await
            }
            "direct_connect_gateway_attachments" => {
                self.create_direct_connect_gateway_attachments(input).await
            }
            "customer_metadata" => {
                self.create_customer_metadata(input).await
            }
            "public_virtual_interface" => {
                self.create_public_virtual_interface(input).await
            }
            "bgp_peer" => {
                self.create_bgp_peer(input).await
            }
            "virtual_interface_attributes" => {
                self.create_virtual_interface_attributes(input).await
            }
            "connections_on_interconnect" => {
                self.create_connections_on_interconnect(input).await
            }
            "virtual_interface" => {
                self.create_virtual_interface(input).await
            }
            "interconnect_loa" => {
                self.create_interconnect_loa(input).await
            }
            "loa" => {
                self.create_loa(input).await
            }
            "private_virtual_interface" => {
                self.create_private_virtual_interface(input).await
            }
            "locations" => {
                self.create_locations(input).await
            }
            "direct_connect_gateway" => {
                self.create_direct_connect_gateway(input).await
            }
            "interconnects" => {
                self.create_interconnects(input).await
            }
            "connection_loa" => {
                self.create_connection_loa(input).await
            }
            "direct_connect_gateways" => {
                self.create_direct_connect_gateways(input).await
            }
            "direct_connect_gateway_association_proposal" => {
                self.create_direct_connect_gateway_association_proposal(input).await
            }
            "direct_connect_gateway_association" => {
                self.create_direct_connect_gateway_association(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "direct_connect",
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
            "transit_virtual_interface" => {
                self.read_transit_virtual_interface(id).await
            }
            "direct_connect_gateway_associations" => {
                self.read_direct_connect_gateway_associations(id).await
            }
            "router_configuration" => {
                self.read_router_configuration(id).await
            }
            "interconnect" => {
                self.read_interconnect(id).await
            }
            "lag" => {
                self.read_lag(id).await
            }
            "connections" => {
                self.read_connections(id).await
            }
            "direct_connect_gateway_association_proposals" => {
                self.read_direct_connect_gateway_association_proposals(id).await
            }
            "lags" => {
                self.read_lags(id).await
            }
            "virtual_gateways" => {
                self.read_virtual_gateways(id).await
            }
            "connection" => {
                self.read_connection(id).await
            }
            "virtual_interfaces" => {
                self.read_virtual_interfaces(id).await
            }
            "hosted_connections" => {
                self.read_hosted_connections(id).await
            }
            "tags" => {
                self.read_tags(id).await
            }
            "direct_connect_gateway_attachments" => {
                self.read_direct_connect_gateway_attachments(id).await
            }
            "customer_metadata" => {
                self.read_customer_metadata(id).await
            }
            "public_virtual_interface" => {
                self.read_public_virtual_interface(id).await
            }
            "bgp_peer" => {
                self.read_bgp_peer(id).await
            }
            "virtual_interface_attributes" => {
                self.read_virtual_interface_attributes(id).await
            }
            "connections_on_interconnect" => {
                self.read_connections_on_interconnect(id).await
            }
            "virtual_interface" => {
                self.read_virtual_interface(id).await
            }
            "interconnect_loa" => {
                self.read_interconnect_loa(id).await
            }
            "loa" => {
                self.read_loa(id).await
            }
            "private_virtual_interface" => {
                self.read_private_virtual_interface(id).await
            }
            "locations" => {
                self.read_locations(id).await
            }
            "direct_connect_gateway" => {
                self.read_direct_connect_gateway(id).await
            }
            "interconnects" => {
                self.read_interconnects(id).await
            }
            "connection_loa" => {
                self.read_connection_loa(id).await
            }
            "direct_connect_gateways" => {
                self.read_direct_connect_gateways(id).await
            }
            "direct_connect_gateway_association_proposal" => {
                self.read_direct_connect_gateway_association_proposal(id).await
            }
            "direct_connect_gateway_association" => {
                self.read_direct_connect_gateway_association(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "direct_connect",
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
            "transit_virtual_interface" => {
                self.update_transit_virtual_interface(id, input).await
            }
            "direct_connect_gateway_associations" => {
                self.update_direct_connect_gateway_associations(id, input).await
            }
            "router_configuration" => {
                self.update_router_configuration(id, input).await
            }
            "interconnect" => {
                self.update_interconnect(id, input).await
            }
            "lag" => {
                self.update_lag(id, input).await
            }
            "connections" => {
                self.update_connections(id, input).await
            }
            "direct_connect_gateway_association_proposals" => {
                self.update_direct_connect_gateway_association_proposals(id, input).await
            }
            "lags" => {
                self.update_lags(id, input).await
            }
            "virtual_gateways" => {
                self.update_virtual_gateways(id, input).await
            }
            "connection" => {
                self.update_connection(id, input).await
            }
            "virtual_interfaces" => {
                self.update_virtual_interfaces(id, input).await
            }
            "hosted_connections" => {
                self.update_hosted_connections(id, input).await
            }
            "tags" => {
                self.update_tags(id, input).await
            }
            "direct_connect_gateway_attachments" => {
                self.update_direct_connect_gateway_attachments(id, input).await
            }
            "customer_metadata" => {
                self.update_customer_metadata(id, input).await
            }
            "public_virtual_interface" => {
                self.update_public_virtual_interface(id, input).await
            }
            "bgp_peer" => {
                self.update_bgp_peer(id, input).await
            }
            "virtual_interface_attributes" => {
                self.update_virtual_interface_attributes(id, input).await
            }
            "connections_on_interconnect" => {
                self.update_connections_on_interconnect(id, input).await
            }
            "virtual_interface" => {
                self.update_virtual_interface(id, input).await
            }
            "interconnect_loa" => {
                self.update_interconnect_loa(id, input).await
            }
            "loa" => {
                self.update_loa(id, input).await
            }
            "private_virtual_interface" => {
                self.update_private_virtual_interface(id, input).await
            }
            "locations" => {
                self.update_locations(id, input).await
            }
            "direct_connect_gateway" => {
                self.update_direct_connect_gateway(id, input).await
            }
            "interconnects" => {
                self.update_interconnects(id, input).await
            }
            "connection_loa" => {
                self.update_connection_loa(id, input).await
            }
            "direct_connect_gateways" => {
                self.update_direct_connect_gateways(id, input).await
            }
            "direct_connect_gateway_association_proposal" => {
                self.update_direct_connect_gateway_association_proposal(id, input).await
            }
            "direct_connect_gateway_association" => {
                self.update_direct_connect_gateway_association(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "direct_connect",
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
            "transit_virtual_interface" => {
                self.delete_transit_virtual_interface(id).await
            }
            "direct_connect_gateway_associations" => {
                self.delete_direct_connect_gateway_associations(id).await
            }
            "router_configuration" => {
                self.delete_router_configuration(id).await
            }
            "interconnect" => {
                self.delete_interconnect(id).await
            }
            "lag" => {
                self.delete_lag(id).await
            }
            "connections" => {
                self.delete_connections(id).await
            }
            "direct_connect_gateway_association_proposals" => {
                self.delete_direct_connect_gateway_association_proposals(id).await
            }
            "lags" => {
                self.delete_lags(id).await
            }
            "virtual_gateways" => {
                self.delete_virtual_gateways(id).await
            }
            "connection" => {
                self.delete_connection(id).await
            }
            "virtual_interfaces" => {
                self.delete_virtual_interfaces(id).await
            }
            "hosted_connections" => {
                self.delete_hosted_connections(id).await
            }
            "tags" => {
                self.delete_tags(id).await
            }
            "direct_connect_gateway_attachments" => {
                self.delete_direct_connect_gateway_attachments(id).await
            }
            "customer_metadata" => {
                self.delete_customer_metadata(id).await
            }
            "public_virtual_interface" => {
                self.delete_public_virtual_interface(id).await
            }
            "bgp_peer" => {
                self.delete_bgp_peer(id).await
            }
            "virtual_interface_attributes" => {
                self.delete_virtual_interface_attributes(id).await
            }
            "connections_on_interconnect" => {
                self.delete_connections_on_interconnect(id).await
            }
            "virtual_interface" => {
                self.delete_virtual_interface(id).await
            }
            "interconnect_loa" => {
                self.delete_interconnect_loa(id).await
            }
            "loa" => {
                self.delete_loa(id).await
            }
            "private_virtual_interface" => {
                self.delete_private_virtual_interface(id).await
            }
            "locations" => {
                self.delete_locations(id).await
            }
            "direct_connect_gateway" => {
                self.delete_direct_connect_gateway(id).await
            }
            "interconnects" => {
                self.delete_interconnects(id).await
            }
            "connection_loa" => {
                self.delete_connection_loa(id).await
            }
            "direct_connect_gateways" => {
                self.delete_direct_connect_gateways(id).await
            }
            "direct_connect_gateway_association_proposal" => {
                self.delete_direct_connect_gateway_association_proposal(id).await
            }
            "direct_connect_gateway_association" => {
                self.delete_direct_connect_gateway_association(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "direct_connect",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Transit_virtual_interface resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a transit_virtual_interface resource
    async fn plan_transit_virtual_interface(
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

    /// Create a new transit_virtual_interface resource
    async fn create_transit_virtual_interface(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let new_transit_virtual_interface = input.get_string("new_transit_virtual_interface")?;
            let connection_id = input.get_string("connection_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_transit_virtual_interface()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("new_transit_virtual_interface", new_transit_virtual_interface.unwrap_or_default())
                .with_field("connection_id", connection_id.unwrap_or_default())
            )
        })
    }

    /// Read a transit_virtual_interface resource
    async fn read_transit_virtual_interface(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_transit_virtual_interface()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a transit_virtual_interface resource
    async fn update_transit_virtual_interface(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let new_transit_virtual_interface = input.get_string("new_transit_virtual_interface")?;
            let connection_id = input.get_string("connection_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_transit_virtual_interface()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("new_transit_virtual_interface", new_transit_virtual_interface.unwrap_or_default())
                .with_field("connection_id", connection_id.unwrap_or_default())
            )
        })
    }

    /// Delete a transit_virtual_interface resource
    async fn delete_transit_virtual_interface(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_transit_virtual_interface()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Direct_connect_gateway_associations resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a direct_connect_gateway_associations resource
    async fn plan_direct_connect_gateway_associations(
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

    /// Create a new direct_connect_gateway_associations resource
    async fn create_direct_connect_gateway_associations(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_direct_connect_gateway_associations()
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

    /// Read a direct_connect_gateway_associations resource
    async fn read_direct_connect_gateway_associations(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_direct_connect_gateway_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a direct_connect_gateway_associations resource
    async fn update_direct_connect_gateway_associations(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_direct_connect_gateway_associations()
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

    /// Delete a direct_connect_gateway_associations resource
    async fn delete_direct_connect_gateway_associations(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_direct_connect_gateway_associations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Router_configuration resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a router_configuration resource
    async fn plan_router_configuration(
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

    /// Create a new router_configuration resource
    async fn create_router_configuration(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_router_configuration()
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

    /// Read a router_configuration resource
    async fn read_router_configuration(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_router_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a router_configuration resource
    async fn update_router_configuration(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_router_configuration()
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

    /// Delete a router_configuration resource
    async fn delete_router_configuration(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_router_configuration()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Interconnect resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a interconnect resource
    async fn plan_interconnect(
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

    /// Create a new interconnect resource
    async fn create_interconnect(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let bandwidth = input.get_string("bandwidth")?;
            let request_mac_sec = input.get_optional_string("request_mac_sec")?;
            let lag_id = input.get_optional_string("lag_id")?;
            let location = input.get_string("location")?;
            let provider_name = input.get_optional_string("provider_name")?;
            let tags = input.get_optional_string("tags")?;
            let interconnect_name = input.get_string("interconnect_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_interconnect()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("bandwidth", bandwidth.unwrap_or_default())
                .with_field("request_mac_sec", request_mac_sec.unwrap_or_default())
                .with_field("lag_id", lag_id.unwrap_or_default())
                .with_field("location", location.unwrap_or_default())
                .with_field("provider_name", provider_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("interconnect_name", interconnect_name.unwrap_or_default())
            )
        })
    }

    /// Read a interconnect resource
    async fn read_interconnect(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_interconnect()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a interconnect resource
    async fn update_interconnect(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let bandwidth = input.get_string("bandwidth")?;
            let request_mac_sec = input.get_optional_string("request_mac_sec")?;
            let lag_id = input.get_optional_string("lag_id")?;
            let location = input.get_string("location")?;
            let provider_name = input.get_optional_string("provider_name")?;
            let tags = input.get_optional_string("tags")?;
            let interconnect_name = input.get_string("interconnect_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_interconnect()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("bandwidth", bandwidth.unwrap_or_default())
                .with_field("request_mac_sec", request_mac_sec.unwrap_or_default())
                .with_field("lag_id", lag_id.unwrap_or_default())
                .with_field("location", location.unwrap_or_default())
                .with_field("provider_name", provider_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("interconnect_name", interconnect_name.unwrap_or_default())
            )
        })
    }

    /// Delete a interconnect resource
    async fn delete_interconnect(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_interconnect()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Lag resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a lag resource
    async fn plan_lag(
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

    /// Create a new lag resource
    async fn create_lag(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let lag_name = input.get_string("lag_name")?;
            let connection_id = input.get_optional_string("connection_id")?;
            let tags = input.get_optional_string("tags")?;
            let child_connection_tags = input.get_optional_string("child_connection_tags")?;
            let number_of_connections = input.get_string("number_of_connections")?;
            let provider_name = input.get_optional_string("provider_name")?;
            let request_mac_sec = input.get_optional_string("request_mac_sec")?;
            let connections_bandwidth = input.get_string("connections_bandwidth")?;
            let location = input.get_string("location")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_lag()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("lag_name", lag_name.unwrap_or_default())
                .with_field("connection_id", connection_id.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("child_connection_tags", child_connection_tags.unwrap_or_default())
                .with_field("number_of_connections", number_of_connections.unwrap_or_default())
                .with_field("provider_name", provider_name.unwrap_or_default())
                .with_field("request_mac_sec", request_mac_sec.unwrap_or_default())
                .with_field("connections_bandwidth", connections_bandwidth.unwrap_or_default())
                .with_field("location", location.unwrap_or_default())
            )
        })
    }

    /// Read a lag resource
    async fn read_lag(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_lag()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a lag resource
    async fn update_lag(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let lag_name = input.get_string("lag_name")?;
            let connection_id = input.get_optional_string("connection_id")?;
            let tags = input.get_optional_string("tags")?;
            let child_connection_tags = input.get_optional_string("child_connection_tags")?;
            let number_of_connections = input.get_string("number_of_connections")?;
            let provider_name = input.get_optional_string("provider_name")?;
            let request_mac_sec = input.get_optional_string("request_mac_sec")?;
            let connections_bandwidth = input.get_string("connections_bandwidth")?;
            let location = input.get_string("location")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_lag()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("lag_name", lag_name.unwrap_or_default())
                .with_field("connection_id", connection_id.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("child_connection_tags", child_connection_tags.unwrap_or_default())
                .with_field("number_of_connections", number_of_connections.unwrap_or_default())
                .with_field("provider_name", provider_name.unwrap_or_default())
                .with_field("request_mac_sec", request_mac_sec.unwrap_or_default())
                .with_field("connections_bandwidth", connections_bandwidth.unwrap_or_default())
                .with_field("location", location.unwrap_or_default())
            )
        })
    }

    /// Delete a lag resource
    async fn delete_lag(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_lag()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connections resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connections resource
    async fn plan_connections(
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

    /// Create a new connections resource
    async fn create_connections(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_connections()
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

    /// Read a connections resource
    async fn read_connections(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_connections()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connections resource
    async fn update_connections(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_connections()
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

    /// Delete a connections resource
    async fn delete_connections(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_connections()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Direct_connect_gateway_association_proposals resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a direct_connect_gateway_association_proposals resource
    async fn plan_direct_connect_gateway_association_proposals(
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

    /// Create a new direct_connect_gateway_association_proposals resource
    async fn create_direct_connect_gateway_association_proposals(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_direct_connect_gateway_association_proposals()
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

    /// Read a direct_connect_gateway_association_proposals resource
    async fn read_direct_connect_gateway_association_proposals(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_direct_connect_gateway_association_proposals()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a direct_connect_gateway_association_proposals resource
    async fn update_direct_connect_gateway_association_proposals(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_direct_connect_gateway_association_proposals()
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

    /// Delete a direct_connect_gateway_association_proposals resource
    async fn delete_direct_connect_gateway_association_proposals(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_direct_connect_gateway_association_proposals()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Lags resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a lags resource
    async fn plan_lags(
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

    /// Create a new lags resource
    async fn create_lags(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_lags()
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

    /// Read a lags resource
    async fn read_lags(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_lags()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a lags resource
    async fn update_lags(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_lags()
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

    /// Delete a lags resource
    async fn delete_lags(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_lags()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Virtual_gateways resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a virtual_gateways resource
    async fn plan_virtual_gateways(
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

    /// Create a new virtual_gateways resource
    async fn create_virtual_gateways(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_virtual_gateways()
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

    /// Read a virtual_gateways resource
    async fn read_virtual_gateways(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_virtual_gateways()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a virtual_gateways resource
    async fn update_virtual_gateways(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_virtual_gateways()
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

    /// Delete a virtual_gateways resource
    async fn delete_virtual_gateways(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_virtual_gateways()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connection resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connection resource
    async fn plan_connection(
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

    /// Create a new connection resource
    async fn create_connection(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let location = input.get_string("location")?;
            let bandwidth = input.get_string("bandwidth")?;
            let lag_id = input.get_optional_string("lag_id")?;
            let provider_name = input.get_optional_string("provider_name")?;
            let request_mac_sec = input.get_optional_string("request_mac_sec")?;
            let tags = input.get_optional_string("tags")?;
            let connection_name = input.get_string("connection_name")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_connection()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("location", location.unwrap_or_default())
                .with_field("bandwidth", bandwidth.unwrap_or_default())
                .with_field("lag_id", lag_id.unwrap_or_default())
                .with_field("provider_name", provider_name.unwrap_or_default())
                .with_field("request_mac_sec", request_mac_sec.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("connection_name", connection_name.unwrap_or_default())
            )
        })
    }

    /// Read a connection resource
    async fn read_connection(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_connection()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connection resource
    async fn update_connection(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let location = input.get_string("location")?;
            let bandwidth = input.get_string("bandwidth")?;
            let lag_id = input.get_optional_string("lag_id")?;
            let provider_name = input.get_optional_string("provider_name")?;
            let request_mac_sec = input.get_optional_string("request_mac_sec")?;
            let tags = input.get_optional_string("tags")?;
            let connection_name = input.get_string("connection_name")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_connection()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("location", location.unwrap_or_default())
                .with_field("bandwidth", bandwidth.unwrap_or_default())
                .with_field("lag_id", lag_id.unwrap_or_default())
                .with_field("provider_name", provider_name.unwrap_or_default())
                .with_field("request_mac_sec", request_mac_sec.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
                .with_field("connection_name", connection_name.unwrap_or_default())
            )
        })
    }

    /// Delete a connection resource
    async fn delete_connection(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_connection()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Virtual_interfaces resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a virtual_interfaces resource
    async fn plan_virtual_interfaces(
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

    /// Create a new virtual_interfaces resource
    async fn create_virtual_interfaces(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_virtual_interfaces()
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

    /// Read a virtual_interfaces resource
    async fn read_virtual_interfaces(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_virtual_interfaces()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a virtual_interfaces resource
    async fn update_virtual_interfaces(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_virtual_interfaces()
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

    /// Delete a virtual_interfaces resource
    async fn delete_virtual_interfaces(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_virtual_interfaces()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Hosted_connections resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hosted_connections resource
    async fn plan_hosted_connections(
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

    /// Create a new hosted_connections resource
    async fn create_hosted_connections(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_hosted_connections()
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

    /// Read a hosted_connections resource
    async fn read_hosted_connections(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_hosted_connections()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a hosted_connections resource
    async fn update_hosted_connections(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_hosted_connections()
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

    /// Delete a hosted_connections resource
    async fn delete_hosted_connections(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_hosted_connections()
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
            // let result = self.provider.direct_connect_client
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
            // let result = self.provider.direct_connect_client
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
            // let result = self.provider.direct_connect_client
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
            // self.provider.direct_connect_client
            //     .delete_tags()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Direct_connect_gateway_attachments resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a direct_connect_gateway_attachments resource
    async fn plan_direct_connect_gateway_attachments(
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

    /// Create a new direct_connect_gateway_attachments resource
    async fn create_direct_connect_gateway_attachments(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_direct_connect_gateway_attachments()
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

    /// Read a direct_connect_gateway_attachments resource
    async fn read_direct_connect_gateway_attachments(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_direct_connect_gateway_attachments()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a direct_connect_gateway_attachments resource
    async fn update_direct_connect_gateway_attachments(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_direct_connect_gateway_attachments()
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

    /// Delete a direct_connect_gateway_attachments resource
    async fn delete_direct_connect_gateway_attachments(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_direct_connect_gateway_attachments()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Customer_metadata resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a customer_metadata resource
    async fn plan_customer_metadata(
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

    /// Create a new customer_metadata resource
    async fn create_customer_metadata(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_customer_metadata()
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

    /// Read a customer_metadata resource
    async fn read_customer_metadata(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_customer_metadata()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a customer_metadata resource
    async fn update_customer_metadata(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_customer_metadata()
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

    /// Delete a customer_metadata resource
    async fn delete_customer_metadata(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_customer_metadata()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Public_virtual_interface resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a public_virtual_interface resource
    async fn plan_public_virtual_interface(
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

    /// Create a new public_virtual_interface resource
    async fn create_public_virtual_interface(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let connection_id = input.get_string("connection_id")?;
            let new_public_virtual_interface = input.get_string("new_public_virtual_interface")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_public_virtual_interface()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("connection_id", connection_id.unwrap_or_default())
                .with_field("new_public_virtual_interface", new_public_virtual_interface.unwrap_or_default())
            )
        })
    }

    /// Read a public_virtual_interface resource
    async fn read_public_virtual_interface(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_public_virtual_interface()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a public_virtual_interface resource
    async fn update_public_virtual_interface(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let connection_id = input.get_string("connection_id")?;
            let new_public_virtual_interface = input.get_string("new_public_virtual_interface")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_public_virtual_interface()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("connection_id", connection_id.unwrap_or_default())
                .with_field("new_public_virtual_interface", new_public_virtual_interface.unwrap_or_default())
            )
        })
    }

    /// Delete a public_virtual_interface resource
    async fn delete_public_virtual_interface(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_public_virtual_interface()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Bgp_peer resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a bgp_peer resource
    async fn plan_bgp_peer(
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

    /// Create a new bgp_peer resource
    async fn create_bgp_peer(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let virtual_interface_id = input.get_optional_string("virtual_interface_id")?;
            let new_bgp_peer = input.get_optional_string("new_bgp_peer")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_bgp_peer()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("virtual_interface_id", virtual_interface_id.unwrap_or_default())
                .with_field("new_bgp_peer", new_bgp_peer.unwrap_or_default())
            )
        })
    }

    /// Read a bgp_peer resource
    async fn read_bgp_peer(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_bgp_peer()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a bgp_peer resource
    async fn update_bgp_peer(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let virtual_interface_id = input.get_optional_string("virtual_interface_id")?;
            let new_bgp_peer = input.get_optional_string("new_bgp_peer")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_bgp_peer()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("virtual_interface_id", virtual_interface_id.unwrap_or_default())
                .with_field("new_bgp_peer", new_bgp_peer.unwrap_or_default())
            )
        })
    }

    /// Delete a bgp_peer resource
    async fn delete_bgp_peer(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_bgp_peer()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Virtual_interface_attributes resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a virtual_interface_attributes resource
    async fn plan_virtual_interface_attributes(
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

    /// Create a new virtual_interface_attributes resource
    async fn create_virtual_interface_attributes(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let virtual_interface_name = input.get_optional_string("virtual_interface_name")?;
            let virtual_interface_id = input.get_string("virtual_interface_id")?;
            let enable_site_link = input.get_optional_string("enable_site_link")?;
            let mtu = input.get_optional_string("mtu")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_virtual_interface_attributes()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("virtual_interface_name", virtual_interface_name.unwrap_or_default())
                .with_field("virtual_interface_id", virtual_interface_id.unwrap_or_default())
                .with_field("enable_site_link", enable_site_link.unwrap_or_default())
                .with_field("mtu", mtu.unwrap_or_default())
            )
        })
    }

    /// Read a virtual_interface_attributes resource
    async fn read_virtual_interface_attributes(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_virtual_interface_attributes()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a virtual_interface_attributes resource
    async fn update_virtual_interface_attributes(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let virtual_interface_name = input.get_optional_string("virtual_interface_name")?;
            let virtual_interface_id = input.get_string("virtual_interface_id")?;
            let enable_site_link = input.get_optional_string("enable_site_link")?;
            let mtu = input.get_optional_string("mtu")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_virtual_interface_attributes()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("virtual_interface_name", virtual_interface_name.unwrap_or_default())
                .with_field("virtual_interface_id", virtual_interface_id.unwrap_or_default())
                .with_field("enable_site_link", enable_site_link.unwrap_or_default())
                .with_field("mtu", mtu.unwrap_or_default())
            )
        })
    }

    /// Delete a virtual_interface_attributes resource
    async fn delete_virtual_interface_attributes(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_virtual_interface_attributes()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connections_on_interconnect resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connections_on_interconnect resource
    async fn plan_connections_on_interconnect(
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

    /// Create a new connections_on_interconnect resource
    async fn create_connections_on_interconnect(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_connections_on_interconnect()
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

    /// Read a connections_on_interconnect resource
    async fn read_connections_on_interconnect(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_connections_on_interconnect()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connections_on_interconnect resource
    async fn update_connections_on_interconnect(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_connections_on_interconnect()
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

    /// Delete a connections_on_interconnect resource
    async fn delete_connections_on_interconnect(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_connections_on_interconnect()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Virtual_interface resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a virtual_interface resource
    async fn plan_virtual_interface(
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

    /// Create a new virtual_interface resource
    async fn create_virtual_interface(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_virtual_interface()
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

    /// Read a virtual_interface resource
    async fn read_virtual_interface(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_virtual_interface()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a virtual_interface resource
    async fn update_virtual_interface(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_virtual_interface()
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

    /// Delete a virtual_interface resource
    async fn delete_virtual_interface(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_virtual_interface()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Interconnect_loa resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a interconnect_loa resource
    async fn plan_interconnect_loa(
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

    /// Create a new interconnect_loa resource
    async fn create_interconnect_loa(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_interconnect_loa()
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

    /// Read a interconnect_loa resource
    async fn read_interconnect_loa(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_interconnect_loa()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a interconnect_loa resource
    async fn update_interconnect_loa(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_interconnect_loa()
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

    /// Delete a interconnect_loa resource
    async fn delete_interconnect_loa(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_interconnect_loa()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Loa resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a loa resource
    async fn plan_loa(
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

    /// Create a new loa resource
    async fn create_loa(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_loa()
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

    /// Read a loa resource
    async fn read_loa(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_loa()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a loa resource
    async fn update_loa(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_loa()
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

    /// Delete a loa resource
    async fn delete_loa(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_loa()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Private_virtual_interface resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a private_virtual_interface resource
    async fn plan_private_virtual_interface(
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

    /// Create a new private_virtual_interface resource
    async fn create_private_virtual_interface(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let new_private_virtual_interface = input.get_string("new_private_virtual_interface")?;
            let connection_id = input.get_string("connection_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_private_virtual_interface()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("new_private_virtual_interface", new_private_virtual_interface.unwrap_or_default())
                .with_field("connection_id", connection_id.unwrap_or_default())
            )
        })
    }

    /// Read a private_virtual_interface resource
    async fn read_private_virtual_interface(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_private_virtual_interface()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a private_virtual_interface resource
    async fn update_private_virtual_interface(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let new_private_virtual_interface = input.get_string("new_private_virtual_interface")?;
            let connection_id = input.get_string("connection_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_private_virtual_interface()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("new_private_virtual_interface", new_private_virtual_interface.unwrap_or_default())
                .with_field("connection_id", connection_id.unwrap_or_default())
            )
        })
    }

    /// Delete a private_virtual_interface resource
    async fn delete_private_virtual_interface(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_private_virtual_interface()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Locations resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a locations resource
    async fn plan_locations(
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

    /// Create a new locations resource
    async fn create_locations(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_locations()
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

    /// Read a locations resource
    async fn read_locations(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_locations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a locations resource
    async fn update_locations(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_locations()
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

    /// Delete a locations resource
    async fn delete_locations(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_locations()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Direct_connect_gateway resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a direct_connect_gateway resource
    async fn plan_direct_connect_gateway(
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

    /// Create a new direct_connect_gateway resource
    async fn create_direct_connect_gateway(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let amazon_side_asn = input.get_optional_string("amazon_side_asn")?;
            let direct_connect_gateway_name = input.get_string("direct_connect_gateway_name")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_direct_connect_gateway()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("amazon_side_asn", amazon_side_asn.unwrap_or_default())
                .with_field("direct_connect_gateway_name", direct_connect_gateway_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Read a direct_connect_gateway resource
    async fn read_direct_connect_gateway(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_direct_connect_gateway()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a direct_connect_gateway resource
    async fn update_direct_connect_gateway(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let amazon_side_asn = input.get_optional_string("amazon_side_asn")?;
            let direct_connect_gateway_name = input.get_string("direct_connect_gateway_name")?;
            let tags = input.get_optional_string("tags")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_direct_connect_gateway()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("amazon_side_asn", amazon_side_asn.unwrap_or_default())
                .with_field("direct_connect_gateway_name", direct_connect_gateway_name.unwrap_or_default())
                .with_field("tags", tags.unwrap_or_default())
            )
        })
    }

    /// Delete a direct_connect_gateway resource
    async fn delete_direct_connect_gateway(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_direct_connect_gateway()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Interconnects resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a interconnects resource
    async fn plan_interconnects(
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

    /// Create a new interconnects resource
    async fn create_interconnects(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_interconnects()
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

    /// Read a interconnects resource
    async fn read_interconnects(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_interconnects()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a interconnects resource
    async fn update_interconnects(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_interconnects()
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

    /// Delete a interconnects resource
    async fn delete_interconnects(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_interconnects()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Connection_loa resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connection_loa resource
    async fn plan_connection_loa(
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

    /// Create a new connection_loa resource
    async fn create_connection_loa(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_connection_loa()
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

    /// Read a connection_loa resource
    async fn read_connection_loa(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_connection_loa()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a connection_loa resource
    async fn update_connection_loa(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_connection_loa()
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

    /// Delete a connection_loa resource
    async fn delete_connection_loa(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_connection_loa()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Direct_connect_gateways resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a direct_connect_gateways resource
    async fn plan_direct_connect_gateways(
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

    /// Create a new direct_connect_gateways resource
    async fn create_direct_connect_gateways(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_direct_connect_gateways()
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

    /// Read a direct_connect_gateways resource
    async fn read_direct_connect_gateways(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_direct_connect_gateways()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a direct_connect_gateways resource
    async fn update_direct_connect_gateways(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_direct_connect_gateways()
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

    /// Delete a direct_connect_gateways resource
    async fn delete_direct_connect_gateways(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_direct_connect_gateways()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Direct_connect_gateway_association_proposal resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a direct_connect_gateway_association_proposal resource
    async fn plan_direct_connect_gateway_association_proposal(
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

    /// Create a new direct_connect_gateway_association_proposal resource
    async fn create_direct_connect_gateway_association_proposal(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let add_allowed_prefixes_to_direct_connect_gateway = input.get_optional_string("add_allowed_prefixes_to_direct_connect_gateway")?;
            let remove_allowed_prefixes_to_direct_connect_gateway = input.get_optional_string("remove_allowed_prefixes_to_direct_connect_gateway")?;
            let gateway_id = input.get_string("gateway_id")?;
            let direct_connect_gateway_id = input.get_string("direct_connect_gateway_id")?;
            let direct_connect_gateway_owner_account = input.get_string("direct_connect_gateway_owner_account")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_direct_connect_gateway_association_proposal()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("add_allowed_prefixes_to_direct_connect_gateway", add_allowed_prefixes_to_direct_connect_gateway.unwrap_or_default())
                .with_field("remove_allowed_prefixes_to_direct_connect_gateway", remove_allowed_prefixes_to_direct_connect_gateway.unwrap_or_default())
                .with_field("gateway_id", gateway_id.unwrap_or_default())
                .with_field("direct_connect_gateway_id", direct_connect_gateway_id.unwrap_or_default())
                .with_field("direct_connect_gateway_owner_account", direct_connect_gateway_owner_account.unwrap_or_default())
            )
        })
    }

    /// Read a direct_connect_gateway_association_proposal resource
    async fn read_direct_connect_gateway_association_proposal(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_direct_connect_gateway_association_proposal()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a direct_connect_gateway_association_proposal resource
    async fn update_direct_connect_gateway_association_proposal(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let add_allowed_prefixes_to_direct_connect_gateway = input.get_optional_string("add_allowed_prefixes_to_direct_connect_gateway")?;
            let remove_allowed_prefixes_to_direct_connect_gateway = input.get_optional_string("remove_allowed_prefixes_to_direct_connect_gateway")?;
            let gateway_id = input.get_string("gateway_id")?;
            let direct_connect_gateway_id = input.get_string("direct_connect_gateway_id")?;
            let direct_connect_gateway_owner_account = input.get_string("direct_connect_gateway_owner_account")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_direct_connect_gateway_association_proposal()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("add_allowed_prefixes_to_direct_connect_gateway", add_allowed_prefixes_to_direct_connect_gateway.unwrap_or_default())
                .with_field("remove_allowed_prefixes_to_direct_connect_gateway", remove_allowed_prefixes_to_direct_connect_gateway.unwrap_or_default())
                .with_field("gateway_id", gateway_id.unwrap_or_default())
                .with_field("direct_connect_gateway_id", direct_connect_gateway_id.unwrap_or_default())
                .with_field("direct_connect_gateway_owner_account", direct_connect_gateway_owner_account.unwrap_or_default())
            )
        })
    }

    /// Delete a direct_connect_gateway_association_proposal resource
    async fn delete_direct_connect_gateway_association_proposal(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_direct_connect_gateway_association_proposal()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


    // ------------------------------------------------------------------------
    // Direct_connect_gateway_association resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a direct_connect_gateway_association resource
    async fn plan_direct_connect_gateway_association(
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

    /// Create a new direct_connect_gateway_association resource
    async fn create_direct_connect_gateway_association(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // Use the runtime to execute async SDK calls
        self.provider.runtime().block_on(async {
            // Extract input fields
            let gateway_id = input.get_optional_string("gateway_id")?;
            let add_allowed_prefixes_to_direct_connect_gateway = input.get_optional_string("add_allowed_prefixes_to_direct_connect_gateway")?;
            let direct_connect_gateway_id = input.get_string("direct_connect_gateway_id")?;
            let virtual_gateway_id = input.get_optional_string("virtual_gateway_id")?;


            // TODO: Call AWS SDK to create the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .create_direct_connect_gateway_association()
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to create resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id("placeholder-id")
                .with_field("gateway_id", gateway_id.unwrap_or_default())
                .with_field("add_allowed_prefixes_to_direct_connect_gateway", add_allowed_prefixes_to_direct_connect_gateway.unwrap_or_default())
                .with_field("direct_connect_gateway_id", direct_connect_gateway_id.unwrap_or_default())
                .with_field("virtual_gateway_id", virtual_gateway_id.unwrap_or_default())
            )
        })
    }

    /// Read a direct_connect_gateway_association resource
    async fn read_direct_connect_gateway_association(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to read the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .describe_direct_connect_gateway_association()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to read resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id))
        })
    }

    /// Update a direct_connect_gateway_association resource
    async fn update_direct_connect_gateway_association(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        self.provider.runtime().block_on(async {
            // Extract input fields
            let gateway_id = input.get_optional_string("gateway_id")?;
            let add_allowed_prefixes_to_direct_connect_gateway = input.get_optional_string("add_allowed_prefixes_to_direct_connect_gateway")?;
            let direct_connect_gateway_id = input.get_string("direct_connect_gateway_id")?;
            let virtual_gateway_id = input.get_optional_string("virtual_gateway_id")?;


            // TODO: Call AWS SDK to update the resource
            // Example:
            // let result = self.provider.direct_connect_client
            //     .update_direct_connect_gateway_association()
            //     .set_id(id.to_string())
            //     .set_name(name)
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to update resource: {}", e)))?;

            // Return placeholder output
            Ok(ResourceOutput::new()
                .with_id(id)
                .with_field("gateway_id", gateway_id.unwrap_or_default())
                .with_field("add_allowed_prefixes_to_direct_connect_gateway", add_allowed_prefixes_to_direct_connect_gateway.unwrap_or_default())
                .with_field("direct_connect_gateway_id", direct_connect_gateway_id.unwrap_or_default())
                .with_field("virtual_gateway_id", virtual_gateway_id.unwrap_or_default())
            )
        })
    }

    /// Delete a direct_connect_gateway_association resource
    async fn delete_direct_connect_gateway_association(
        &self,
        id: &str,
    ) -> Result<()> {
        self.provider.runtime().block_on(async {
            // TODO: Call AWS SDK to delete the resource
            // Example:
            // self.provider.direct_connect_client
            //     .delete_direct_connect_gateway_association()
            //     .set_id(id.to_string())
            //     .send()
            //     .await
            //     .map_err(|e| hemmer_core::HemmerError::Provider(format!("Failed to delete resource: {}", e)))?;

            Ok(())
        })
    }


}
