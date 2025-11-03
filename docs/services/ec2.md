# Ec2 Service



**Resources**: 335

---

## Overview

The ec2 service provides access to 335 resource types:

- [Vpcs](#vpcs) [R]
- [Spot_datafeed_subscription](#spot_datafeed_subscription) [CRD]
- [Aws_network_performance_data](#aws_network_performance_data) [R]
- [Transit_gateway_connects](#transit_gateway_connects) [R]
- [Transit_gateway_attachments](#transit_gateway_attachments) [R]
- [Instances](#instances) [R]
- [Local_gateway_route_table_virtual_interface_group_associations](#local_gateway_route_table_virtual_interface_group_associations) [R]
- [Export_tasks](#export_tasks) [R]
- [Capacity_reservation_billing_requests](#capacity_reservation_billing_requests) [R]
- [Capacity_reservations](#capacity_reservations) [R]
- [Id_format](#id_format) [R]
- [Groups_for_capacity_reservation](#groups_for_capacity_reservation) [R]
- [Regions](#regions) [R]
- [Delegate_mac_volume_ownership_task](#delegate_mac_volume_ownership_task) [C]
- [Subnet_cidr_reservations](#subnet_cidr_reservations) [R]
- [Verified_access_group_policy](#verified_access_group_policy) [R]
- [Managed_prefix_lists](#managed_prefix_lists) [R]
- [Capacity_reservation_usage](#capacity_reservation_usage) [R]
- [Launch_template_versions](#launch_template_versions) [RD]
- [Subnet](#subnet) [CD]
- [Vpc_endpoint_service_configuration](#vpc_endpoint_service_configuration) [C]
- [Transit_gateway_connect_peers](#transit_gateway_connect_peers) [R]
- [Network_insights_access_scope_content](#network_insights_access_scope_content) [R]
- [Subnet_cidr_reservation](#subnet_cidr_reservation) [CD]
- [Vpc](#vpc) [CD]
- [Import_snapshot_tasks](#import_snapshot_tasks) [R]
- [Ipam_prefix_list_resolver_target](#ipam_prefix_list_resolver_target) [CD]
- [Instance_credit_specifications](#instance_credit_specifications) [R]
- [Fleet](#fleet) [C]
- [Fleets](#fleets) [RD]
- [Instance_attribute](#instance_attribute) [R]
- [Launch_templates](#launch_templates) [R]
- [Reserved_instances](#reserved_instances) [R]
- [Reserved_instances_listings](#reserved_instances_listings) [R]
- [Image_usage_report_entries](#image_usage_report_entries) [R]
- [Vpc_peering_connection](#vpc_peering_connection) [CD]
- [Store_image_task](#store_image_task) [C]
- [Iam_instance_profile_associations](#iam_instance_profile_associations) [R]
- [Route_server_associations](#route_server_associations) [R]
- [Snapshot_block_public_access_state](#snapshot_block_public_access_state) [R]
- [Local_gateway_route_table_vpc_association](#local_gateway_route_table_vpc_association) [CD]
- [Principal_id_format](#principal_id_format) [R]
- [Vpc_block_public_access_exclusion](#vpc_block_public_access_exclusion) [CD]
- [Scheduled_instances](#scheduled_instances) [R]
- [Fleet_history](#fleet_history) [R]
- [Key_pairs](#key_pairs) [R]
- [Ipam_resource_discovery](#ipam_resource_discovery) [CD]
- [Capacity_manager_organizations_access](#capacity_manager_organizations_access) [U]
- [Local_gateway_route](#local_gateway_route) [CD]
- [Traffic_mirror_target](#traffic_mirror_target) [CD]
- [Client_vpn_endpoint](#client_vpn_endpoint) [CD]
- [Transit_gateway_policy_table](#transit_gateway_policy_table) [CD]
- [Customer_gateways](#customer_gateways) [R]
- [Ipam_prefix_list_resolver_targets](#ipam_prefix_list_resolver_targets) [R]
- [Ipam_resource_discoveries](#ipam_resource_discoveries) [R]
- [Reserved_instances_exchange_quote](#reserved_instances_exchange_quote) [R]
- [Vpc_peering_connections](#vpc_peering_connections) [R]
- [Route_table](#route_table) [CD]
- [Prefix_lists](#prefix_lists) [R]
- [Network_insights_access_scope_analysis](#network_insights_access_scope_analysis) [D]
- [Console_output](#console_output) [R]
- [Security_group_rule_descriptions_egress](#security_group_rule_descriptions_egress) [U]
- [Volume_status](#volume_status) [R]
- [Vpc_classic_link](#vpc_classic_link) [R]
- [Queued_reserved_instances](#queued_reserved_instances) [D]
- [Import_image_tasks](#import_image_tasks) [R]
- [Instance_event_notification_attributes](#instance_event_notification_attributes) [R]
- [Spot_fleet_requests](#spot_fleet_requests) [R]
- [Capacity_blocks](#capacity_blocks) [R]
- [Client_vpn_target_networks](#client_vpn_target_networks) [R]
- [Mac_system_integrity_protection_modification_task](#mac_system_integrity_protection_modification_task) [C]
- [Capacity_reservation_fleets](#capacity_reservation_fleets) [R]
- [Transit_gateway_multicast_domain](#transit_gateway_multicast_domain) [CD]
- [Fpga_image_attribute](#fpga_image_attribute) [R]
- [Mac_modification_tasks](#mac_modification_tasks) [R]
- [Vpc_endpoints](#vpc_endpoints) [RD]
- [Network_insights_access_scope_analyses](#network_insights_access_scope_analyses) [R]
- [Fpga_images](#fpga_images) [R]
- [Traffic_mirror_sessions](#traffic_mirror_sessions) [R]
- [Vpc_endpoint_connections](#vpc_endpoint_connections) [R]
- [Coip_pools](#coip_pools) [R]
- [Vpc_endpoint_service_permissions](#vpc_endpoint_service_permissions) [R]
- [Fast_launch_images](#fast_launch_images) [R]
- [Instance_event_window](#instance_event_window) [CD]
- [Ebs_encryption_by_default](#ebs_encryption_by_default) [R]
- [Launch_template_version](#launch_template_version) [C]
- [Transit_gateway_peering_attachment](#transit_gateway_peering_attachment) [CD]
- [Instance_event_windows](#instance_event_windows) [R]
- [Vpc_endpoint_associations](#vpc_endpoint_associations) [R]
- [Declarative_policies_reports](#declarative_policies_reports) [R]
- [Traffic_mirror_filter_rules](#traffic_mirror_filter_rules) [R]
- [Tags](#tags) [CRD]
- [Image_attribute](#image_attribute) [R]
- [Verified_access_instances](#verified_access_instances) [R]
- [Allowed_images_settings](#allowed_images_settings) [R]
- [Declarative_policies_report_summary](#declarative_policies_report_summary) [R]
- [Transit_gateway_attachment_propagations](#transit_gateway_attachment_propagations) [R]
- [Instance_status](#instance_status) [R]
- [Vpn_gateway](#vpn_gateway) [CD]
- [Network_interface_permission](#network_interface_permission) [CD]
- [Volumes_modifications](#volumes_modifications) [R]
- [Instance_uefi_data](#instance_uefi_data) [R]
- [Carrier_gateway](#carrier_gateway) [CD]
- [Launch_template](#launch_template) [CD]
- [Associated_enclave_certificate_iam_roles](#associated_enclave_certificate_iam_roles) [R]
- [Verified_access_endpoint_policy](#verified_access_endpoint_policy) [R]
- [Key_pair](#key_pair) [CD]
- [Conversion_tasks](#conversion_tasks) [R]
- [Local_gateway_route_tables](#local_gateway_route_tables) [R]
- [Capacity_block_extension_history](#capacity_block_extension_history) [R]
- [Outpost_lags](#outpost_lags) [R]
- [Instance_tpm_ek_pub](#instance_tpm_ek_pub) [R]
- [Moving_addresses](#moving_addresses) [R]
- [Volume_attribute](#volume_attribute) [R]
- [Ipam_discovered_public_addresses](#ipam_discovered_public_addresses) [R]
- [Capacity_manager_metric_data](#capacity_manager_metric_data) [R]
- [Transit_gateway_peering_attachments](#transit_gateway_peering_attachments) [R]
- [Security_group](#security_group) [CD]
- [Capacity_manager_data_export](#capacity_manager_data_export) [CD]
- [Transit_gateway_connect_peer](#transit_gateway_connect_peer) [CD]
- [Verified_access_endpoint](#verified_access_endpoint) [CD]
- [Network_interfaces](#network_interfaces) [R]
- [Ipam_resource_cidrs](#ipam_resource_cidrs) [R]
- [Egress_only_internet_gateways](#egress_only_internet_gateways) [R]
- [Aws_network_performance_metric_subscriptions](#aws_network_performance_metric_subscriptions) [R]
- [Mac_hosts](#mac_hosts) [R]
- [Vpc_block_public_access_options](#vpc_block_public_access_options) [R]
- [Associated_ipv6_pool_cidrs](#associated_ipv6_pool_cidrs) [R]
- [Local_gateway_route_table_vpc_associations](#local_gateway_route_table_vpc_associations) [R]
- [Network_acl_entry](#network_acl_entry) [CD]
- [Host_reservation_offerings](#host_reservation_offerings) [R]
- [Flow_logs_integration_template](#flow_logs_integration_template) [R]
- [Spot_fleet_instances](#spot_fleet_instances) [R]
- [Transit_gateway_policy_tables](#transit_gateway_policy_tables) [R]
- [Ipam_pool_allocations](#ipam_pool_allocations) [R]
- [Public_ipv4_pools](#public_ipv4_pools) [R]
- [Network_insights_access_scope_analysis_findings](#network_insights_access_scope_analysis_findings) [R]
- [Security_group_rule_descriptions_ingress](#security_group_rule_descriptions_ingress) [U]
- [Internet_gateway](#internet_gateway) [CD]
- [Coip_pool_usage](#coip_pool_usage) [R]
- [Reserved_instances_listing](#reserved_instances_listing) [C]
- [Transit_gateway_route](#transit_gateway_route) [CD]
- [Capacity_block_offerings](#capacity_block_offerings) [R]
- [Instance_connect_endpoints](#instance_connect_endpoints) [R]
- [Transit_gateways](#transit_gateways) [R]
- [Launch_template_data](#launch_template_data) [R]
- [Console_screenshot](#console_screenshot) [R]
- [Traffic_mirror_targets](#traffic_mirror_targets) [R]
- [Volume](#volume) [CD]
- [Ipam](#ipam) [CD]
- [Image_usage_reports](#image_usage_reports) [R]
- [Default_credit_specification](#default_credit_specification) [R]
- [Customer_gateway](#customer_gateway) [CD]
- [Local_gateway_route_table_virtual_interface_group_association](#local_gateway_route_table_virtual_interface_group_association) [CD]
- [Ipam_discovered_accounts](#ipam_discovered_accounts) [R]
- [Transit_gateway_route_tables](#transit_gateway_route_tables) [R]
- [Security_groups_for_vpc](#security_groups_for_vpc) [R]
- [Instance_topology](#instance_topology) [R]
- [Managed_prefix_list_associations](#managed_prefix_list_associations) [R]
- [Vpn_gateways](#vpn_gateways) [R]
- [Ipam_external_resource_verification_token](#ipam_external_resource_verification_token) [CD]
- [Vpc_endpoint_connection_notifications](#vpc_endpoint_connection_notifications) [RD]
- [Vpc_block_public_access_exclusions](#vpc_block_public_access_exclusions) [R]
- [Vpn_connection_device_sample_configuration](#vpn_connection_device_sample_configuration) [R]
- [Image_references](#image_references) [R]
- [Local_gateway_virtual_interface_groups](#local_gateway_virtual_interface_groups) [R]
- [Ipam_pools](#ipam_pools) [R]
- [Route_servers](#route_servers) [R]
- [Traffic_mirror_filters](#traffic_mirror_filters) [R]
- [Transit_gateway](#transit_gateway) [CD]
- [Instance_image_metadata](#instance_image_metadata) [R]
- [Route_server_propagations](#route_server_propagations) [R]
- [Transit_gateway_policy_table_associations](#transit_gateway_policy_table_associations) [R]
- [Replace_root_volume_task](#replace_root_volume_task) [C]
- [Verified_access_groups](#verified_access_groups) [R]
- [Verified_access_instance_logging_configurations](#verified_access_instance_logging_configurations) [R]
- [Fast_snapshot_restores](#fast_snapshot_restores) [R]
- [Ipam_pool](#ipam_pool) [CD]
- [Capacity_block_extension_offerings](#capacity_block_extension_offerings) [R]
- [Ipam_prefix_list_resolver_versions](#ipam_prefix_list_resolver_versions) [R]
- [Account_attributes](#account_attributes) [R]
- [Default_vpc](#default_vpc) [C]
- [Fleet_instances](#fleet_instances) [R]
- [Spot_fleet_request_history](#spot_fleet_request_history) [R]
- [Route_tables](#route_tables) [R]
- [Image_usage_report](#image_usage_report) [CD]
- [Snapshot](#snapshot) [CD]
- [Availability_zones](#availability_zones) [R]
- [Network_insights_access_scopes](#network_insights_access_scopes) [R]
- [Route_server_peers](#route_server_peers) [R]
- [Security_group_vpc_associations](#security_group_vpc_associations) [R]
- [Ebs_default_kms_key_id](#ebs_default_kms_key_id) [R]
- [Coip_pool](#coip_pool) [CD]
- [Network_insights_access_scope](#network_insights_access_scope) [CD]
- [Ipam_discovered_resource_cidrs](#ipam_discovered_resource_cidrs) [R]
- [Route_server_peer](#route_server_peer) [CD]
- [Managed_prefix_list](#managed_prefix_list) [CD]
- [Capacity_manager_attributes](#capacity_manager_attributes) [R]
- [Vpn_connection](#vpn_connection) [CD]
- [Verified_access_instance](#verified_access_instance) [CD]
- [Dhcp_options](#dhcp_options) [CRD]
- [Client_vpn_endpoints](#client_vpn_endpoints) [R]
- [Client_vpn_connections](#client_vpn_connections) [R]
- [Stale_security_groups](#stale_security_groups) [R]
- [Ipam_byoasn](#ipam_byoasn) [R]
- [Traffic_mirror_filter_rule](#traffic_mirror_filter_rule) [CD]
- [Ipam_prefix_list_resolver_rules](#ipam_prefix_list_resolver_rules) [R]
- [Vpc_classic_link_dns_support](#vpc_classic_link_dns_support) [R]
- [Network_acl](#network_acl) [CD]
- [Transit_gateway_route_table](#transit_gateway_route_table) [CD]
- [Export_image_tasks](#export_image_tasks) [R]
- [Internet_gateways](#internet_gateways) [R]
- [Ipam_external_resource_verification_tokens](#ipam_external_resource_verification_tokens) [R]
- [Default_subnet](#default_subnet) [C]
- [Vpc_endpoint_connection_notification](#vpc_endpoint_connection_notification) [C]
- [Elastic_gpus](#elastic_gpus) [R]
- [Scheduled_instance_availability](#scheduled_instance_availability) [R]
- [Transit_gateway_multicast_domains](#transit_gateway_multicast_domains) [R]
- [Instance_type_offerings](#instance_type_offerings) [R]
- [Addresses](#addresses) [R]
- [Verified_access_trust_providers](#verified_access_trust_providers) [R]
- [Route_server_endpoint](#route_server_endpoint) [CD]
- [Service_link_virtual_interfaces](#service_link_virtual_interfaces) [R]
- [Snapshot_attribute](#snapshot_attribute) [R]
- [Serial_console_access_status](#serial_console_access_status) [R]
- [Capacity_reservation_fleet](#capacity_reservation_fleet) [C]
- [Vpc_endpoint_services](#vpc_endpoint_services) [R]
- [Bundle_tasks](#bundle_tasks) [R]
- [Snapshot_tier_status](#snapshot_tier_status) [R]
- [Reserved_instances_modifications](#reserved_instances_modifications) [R]
- [Transit_gateway_connect](#transit_gateway_connect) [CD]
- [Network_acls](#network_acls) [R]
- [Vpc_endpoint_service_configurations](#vpc_endpoint_service_configurations) [RD]
- [Network_insights_analyses](#network_insights_analyses) [R]
- [Vpn_connection_route](#vpn_connection_route) [CD]
- [Subnets](#subnets) [R]
- [Security_group_rules](#security_group_rules) [R]
- [Address_transfers](#address_transfers) [R]
- [Traffic_mirror_session](#traffic_mirror_session) [CD]
- [Security_groups](#security_groups) [R]
- [Placement_groups](#placement_groups) [R]
- [Transit_gateway_prefix_list_reference](#transit_gateway_prefix_list_reference) [CD]
- [Capacity_block_status](#capacity_block_status) [R]
- [Restore_image_task](#restore_image_task) [C]
- [Placement_group](#placement_group) [CD]
- [Local_gateway_route_table](#local_gateway_route_table) [CD]
- [Security_group_references](#security_group_references) [R]
- [Volumes](#volumes) [R]
- [Local_gateway_virtual_interface](#local_gateway_virtual_interface) [CD]
- [Vpn_tunnel_replacement_status](#vpn_tunnel_replacement_status) [R]
- [Active_vpn_tunnel_status](#active_vpn_tunnel_status) [R]
- [Ipam_scopes](#ipam_scopes) [R]
- [Verified_access_group](#verified_access_group) [CD]
- [Client_vpn_authorization_rules](#client_vpn_authorization_rules) [R]
- [Transit_gateway_route_table_propagations](#transit_gateway_route_table_propagations) [R]
- [Network_insights_analysis](#network_insights_analysis) [D]
- [Password_data](#password_data) [R]
- [Host_reservations](#host_reservations) [R]
- [Hosts](#hosts) [R]
- [Vpn_connection_device_types](#vpn_connection_device_types) [R]
- [Network_interface_permissions](#network_interface_permissions) [R]
- [Ipam_address_history](#ipam_address_history) [R]
- [Vpc_endpoint](#vpc_endpoint) [C]
- [Ipam_prefix_list_resolver](#ipam_prefix_list_resolver) [CD]
- [Aggregate_id_format](#aggregate_id_format) [R]
- [Transit_gateway_route_table_announcements](#transit_gateway_route_table_announcements) [R]
- [Flow_logs](#flow_logs) [CRD]
- [Network_insights_paths](#network_insights_paths) [R]
- [Image_block_public_access_state](#image_block_public_access_state) [R]
- [Transit_gateway_policy_table_entries](#transit_gateway_policy_table_entries) [R]
- [Reserved_instances_offerings](#reserved_instances_offerings) [R]
- [Transit_gateway_route_table_announcement](#transit_gateway_route_table_announcement) [CD]
- [Ipam_prefix_list_resolvers](#ipam_prefix_list_resolvers) [R]
- [Verified_access_endpoint_targets](#verified_access_endpoint_targets) [R]
- [Instance_types_from_instance_requirements](#instance_types_from_instance_requirements) [R]
- [Carrier_gateways](#carrier_gateways) [R]
- [Transit_gateway_vpc_attachments](#transit_gateway_vpc_attachments) [R]
- [Snapshots](#snapshots) [CR]
- [Local_gateway_virtual_interfaces](#local_gateway_virtual_interfaces) [R]
- [Ipam_resource_discovery_associations](#ipam_resource_discovery_associations) [R]
- [Nat_gateways](#nat_gateways) [R]
- [Images](#images) [R]
- [Image](#image) [C]
- [Nat_gateway](#nat_gateway) [CD]
- [Route_server_endpoints](#route_server_endpoints) [R]
- [Route_server](#route_server) [CD]
- [Addresses_attribute](#addresses_attribute) [R]
- [Host_reservation_purchase_preview](#host_reservation_purchase_preview) [R]
- [Instance_types](#instance_types) [R]
- [Route_server_routing_database](#route_server_routing_database) [R]
- [Trunk_interface_associations](#trunk_interface_associations) [R]
- [Verified_access_endpoints](#verified_access_endpoints) [R]
- [Verified_access_trust_provider](#verified_access_trust_provider) [CD]
- [Spot_price_history](#spot_price_history) [R]
- [Vpc_attribute](#vpc_attribute) [R]
- [Spot_instance_requests](#spot_instance_requests) [R]
- [Instance_metadata_defaults](#instance_metadata_defaults) [R]
- [Ipam_prefix_list_resolver_version_entries](#ipam_prefix_list_resolver_version_entries) [R]
- [Network_insights_path](#network_insights_path) [CD]
- [Ipam_scope](#ipam_scope) [CD]
- [Identity_id_format](#identity_id_format) [R]
- [Managed_prefix_list_entries](#managed_prefix_list_entries) [R]
- [Transit_gateway_multicast_domain_associations](#transit_gateway_multicast_domain_associations) [R]
- [Spot_placement_scores](#spot_placement_scores) [R]
- [Transit_gateway_prefix_list_references](#transit_gateway_prefix_list_references) [R]
- [Client_vpn_routes](#client_vpn_routes) [R]
- [Ipv6_pools](#ipv6_pools) [R]
- [Transit_gateway_route_table_associations](#transit_gateway_route_table_associations) [R]
- [Instance_export_task](#instance_export_task) [C]
- [Traffic_mirror_filter](#traffic_mirror_filter) [CD]
- [Transit_gateway_vpc_attachment](#transit_gateway_vpc_attachment) [CD]
- [Instance_connect_endpoint](#instance_connect_endpoint) [CD]
- [Capacity_reservation_by_splitting](#capacity_reservation_by_splitting) [C]
- [Local_gateways](#local_gateways) [R]
- [Locked_snapshots](#locked_snapshots) [R]
- [Vpn_connections](#vpn_connections) [R]
- [Fpga_image](#fpga_image) [CD]
- [Network_interface_attribute](#network_interface_attribute) [R]
- [Ipam_pool_cidrs](#ipam_pool_cidrs) [R]
- [Ipams](#ipams) [R]
- [Public_ipv4_pool](#public_ipv4_pool) [CD]
- [Byoip_cidrs](#byoip_cidrs) [R]
- [Capacity_reservation_topology](#capacity_reservation_topology) [R]
- [Classic_link_instances](#classic_link_instances) [R]
- [Local_gateway_virtual_interface_group](#local_gateway_virtual_interface_group) [CD]
- [Route](#route) [CD]
- [Capacity_manager_metric_dimensions](#capacity_manager_metric_dimensions) [R]
- [Coip_cidr](#coip_cidr) [CD]
- [Egress_only_internet_gateway](#egress_only_internet_gateway) [CD]
- [Capacity_reservation](#capacity_reservation) [C]
- [Client_vpn_route](#client_vpn_route) [CD]
- [Network_interface](#network_interface) [CD]
- [Capacity_manager_data_exports](#capacity_manager_data_exports) [R]
- [Store_image_tasks](#store_image_tasks) [R]
- [Replace_root_volume_tasks](#replace_root_volume_tasks) [R]

---

## Resources


### Vpcs

Vpcs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |
| `vpcs` | Vec<String> | <p>Information about the VPCs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpcs outputs
vpcs_id = vpcs.id
vpcs_next_token = vpcs.next_token
vpcs_vpcs = vpcs.vpcs
```

---


### Spot_datafeed_subscription

SpotDatafeedSubscription resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bucket` | String | ✅ | <p>The name of the Amazon S3 bucket in which to store the Spot Instance data feed. For
            more information about bucket names, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html">Bucket naming rules</a> 
            in the <i>Amazon S3 User Guide</i>.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually
            making the request, and provides an error response. If you have the required
            permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is
            <code>UnauthorizedOperation</code>.</p> |
| `prefix` | String |  | <p>The prefix for the data feed file names.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spot_datafeed_subscription` | String | <p>The Spot Instance data feed subscription.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create spot_datafeed_subscription
spot_datafeed_subscription = provider.ec2.Spot_datafeed_subscription {
    bucket = "value"  # <p>The name of the Amazon S3 bucket in which to store the Spot Instance data feed. For
            more information about bucket names, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html">Bucket naming rules</a> 
            in the <i>Amazon S3 User Guide</i>.</p>
}

# Access spot_datafeed_subscription outputs
spot_datafeed_subscription_id = spot_datafeed_subscription.id
spot_datafeed_subscription_spot_datafeed_subscription = spot_datafeed_subscription.spot_datafeed_subscription
```

---


### Aws_network_performance_data

AwsNetworkPerformanceData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_responses` | Vec<String> | <p>The list of data responses.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aws_network_performance_data outputs
aws_network_performance_data_id = aws_network_performance_data.id
aws_network_performance_data_data_responses = aws_network_performance_data.data_responses
aws_network_performance_data_next_token = aws_network_performance_data.next_token
```

---


### Transit_gateway_connects

TransitGatewayConnects resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_connects` | Vec<String> | <p>Information about the Connect attachments.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_connects outputs
transit_gateway_connects_id = transit_gateway_connects.id
transit_gateway_connects_transit_gateway_connects = transit_gateway_connects.transit_gateway_connects
transit_gateway_connects_next_token = transit_gateway_connects.next_token
```

---


### Transit_gateway_attachments

TransitGatewayAttachments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `transit_gateway_attachments` | Vec<String> | <p>Information about the attachments.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_attachments outputs
transit_gateway_attachments_id = transit_gateway_attachments.id
transit_gateway_attachments_next_token = transit_gateway_attachments.next_token
transit_gateway_attachments_transit_gateway_attachments = transit_gateway_attachments.transit_gateway_attachments
```

---


### Instances

Instances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `reservations` | Vec<String> | <p>Information about the reservations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instances outputs
instances_id = instances.id
instances_next_token = instances.next_token
instances_reservations = instances.reservations
```

---


### Local_gateway_route_table_virtual_interface_group_associations

LocalGatewayRouteTableVirtualInterfaceGroupAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `local_gateway_route_table_virtual_interface_group_associations` | Vec<String> | <p>Information about the associations.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access local_gateway_route_table_virtual_interface_group_associations outputs
local_gateway_route_table_virtual_interface_group_associations_id = local_gateway_route_table_virtual_interface_group_associations.id
local_gateway_route_table_virtual_interface_group_associations_local_gateway_route_table_virtual_interface_group_associations = local_gateway_route_table_virtual_interface_group_associations.local_gateway_route_table_virtual_interface_group_associations
local_gateway_route_table_virtual_interface_group_associations_next_token = local_gateway_route_table_virtual_interface_group_associations.next_token
```

---


### Export_tasks

ExportTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `export_tasks` | Vec<String> | <p>Information about the export tasks.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access export_tasks outputs
export_tasks_id = export_tasks.id
export_tasks_export_tasks = export_tasks.export_tasks
```

---


### Capacity_reservation_billing_requests

CapacityReservationBillingRequests resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_reservation_billing_requests` | Vec<String> | <p>Information about the request.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_reservation_billing_requests outputs
capacity_reservation_billing_requests_id = capacity_reservation_billing_requests.id
capacity_reservation_billing_requests_capacity_reservation_billing_requests = capacity_reservation_billing_requests.capacity_reservation_billing_requests
capacity_reservation_billing_requests_next_token = capacity_reservation_billing_requests.next_token
```

---


### Capacity_reservations

CapacityReservations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `capacity_reservations` | Vec<String> | <p>Information about the Capacity Reservations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_reservations outputs
capacity_reservations_id = capacity_reservations.id
capacity_reservations_next_token = capacity_reservations.next_token
capacity_reservations_capacity_reservations = capacity_reservations.capacity_reservations
```

---


### Id_format

IdFormat resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statuses` | Vec<String> | <p>Information about the ID format for the resource.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access id_format outputs
id_format_id = id_format.id
id_format_statuses = id_format.statuses
```

---


### Groups_for_capacity_reservation

GroupsForCapacityReservation resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `capacity_reservation_groups` | Vec<String> | <p>Information about the resource groups to which the Capacity Reservation has been
			added.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access groups_for_capacity_reservation outputs
groups_for_capacity_reservation_id = groups_for_capacity_reservation.id
groups_for_capacity_reservation_next_token = groups_for_capacity_reservation.next_token
groups_for_capacity_reservation_capacity_reservation_groups = groups_for_capacity_reservation.capacity_reservation_groups
```

---


### Regions

Regions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regions` | Vec<String> | <p>Information about the Regions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access regions outputs
regions_id = regions.id
regions_regions = regions.regions
```

---


### Delegate_mac_volume_ownership_task

DelegateMacVolumeOwnershipTask resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `mac_credentials` | String | ✅ | <p>Specifies the following credentials:</p>
         <ul>
            <li>
               <p>
                  <b>Internal disk administrative user</b>
               </p>
               <ul>
                  <li>
                     <p>
                        <b>Username</b> - Only the default administrative user 
                     (<code>aws-managed-user</code>) is supported and it is used by default. You can't 
                     specify a different administrative user.</p>
                  </li>
                  <li>
                     <p>
                        <b>Password</b> - If you did not change the default 
                     password for <code>aws-managed-user</code>, specify the default password, which is 
                     <i>blank</i>. Otherwise, specify your password.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>Amazon EBS root volume administrative user</b>
               </p>
               <ul>
                  <li>
                     <p>
                        <b>Username</b> - If you did not change the default 
                     administrative user, specify <code>ec2-user</code>. Otherwise, specify the username 
                     for your administrative user.</p>
                  </li>
                  <li>
                     <p>
                        <b>Password</b> - Specify the password for the 
                     administrative user.</p>
                  </li>
               </ul>
            </li>
         </ul>
         <p>The credentials must be specified in the following JSON format:</p>
         <p>
            <code>{
  "internalDiskPassword":"<i>internal-disk-admin_password</i>",
  "rootVolumeUsername":"<i>root-volume-admin_username</i>",
  "rootVolumepassword":"<i>root-volume-admin_password</i>"
}</code>
         </p> |
| `instance_id` | String | ✅ | <p>The ID of the Amazon EC2 Mac instance.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the volume ownership delegation task.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create delegate_mac_volume_ownership_task
delegate_mac_volume_ownership_task = provider.ec2.Delegate_mac_volume_ownership_task {
    mac_credentials = "value"  # <p>Specifies the following credentials:</p>
         <ul>
            <li>
               <p>
                  <b>Internal disk administrative user</b>
               </p>
               <ul>
                  <li>
                     <p>
                        <b>Username</b> - Only the default administrative user 
                     (<code>aws-managed-user</code>) is supported and it is used by default. You can't 
                     specify a different administrative user.</p>
                  </li>
                  <li>
                     <p>
                        <b>Password</b> - If you did not change the default 
                     password for <code>aws-managed-user</code>, specify the default password, which is 
                     <i>blank</i>. Otherwise, specify your password.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>Amazon EBS root volume administrative user</b>
               </p>
               <ul>
                  <li>
                     <p>
                        <b>Username</b> - If you did not change the default 
                     administrative user, specify <code>ec2-user</code>. Otherwise, specify the username 
                     for your administrative user.</p>
                  </li>
                  <li>
                     <p>
                        <b>Password</b> - Specify the password for the 
                     administrative user.</p>
                  </li>
               </ul>
            </li>
         </ul>
         <p>The credentials must be specified in the following JSON format:</p>
         <p>
            <code>{
  "internalDiskPassword":"<i>internal-disk-admin_password</i>",
  "rootVolumeUsername":"<i>root-volume-admin_username</i>",
  "rootVolumepassword":"<i>root-volume-admin_password</i>"
}</code>
         </p>
    instance_id = "value"  # <p>The ID of the Amazon EC2 Mac instance.</p>
}

```

---


### Subnet_cidr_reservations

SubnetCidrReservations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subnet_ipv6_cidr_reservations` | Vec<String> | <p>Information about the IPv6 subnet CIDR reservations.</p> |
| `subnet_ipv4_cidr_reservations` | Vec<String> | <p>Information about the IPv4 subnet CIDR reservations.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access subnet_cidr_reservations outputs
subnet_cidr_reservations_id = subnet_cidr_reservations.id
subnet_cidr_reservations_subnet_ipv6_cidr_reservations = subnet_cidr_reservations.subnet_ipv6_cidr_reservations
subnet_cidr_reservations_subnet_ipv4_cidr_reservations = subnet_cidr_reservations.subnet_ipv4_cidr_reservations
subnet_cidr_reservations_next_token = subnet_cidr_reservations.next_token
```

---


### Verified_access_group_policy

VerifiedAccessGroupPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_document` | String | <p>The Verified Access policy document.</p> |
| `policy_enabled` | bool | <p>The status of the Verified Access policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access verified_access_group_policy outputs
verified_access_group_policy_id = verified_access_group_policy.id
verified_access_group_policy_policy_document = verified_access_group_policy.policy_document
verified_access_group_policy_policy_enabled = verified_access_group_policy.policy_enabled
```

---


### Managed_prefix_lists

ManagedPrefixLists resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `prefix_lists` | Vec<String> | <p>Information about the prefix lists.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_prefix_lists outputs
managed_prefix_lists_id = managed_prefix_lists.id
managed_prefix_lists_next_token = managed_prefix_lists.next_token
managed_prefix_lists_prefix_lists = managed_prefix_lists.prefix_lists
```

---


### Capacity_reservation_usage

CapacityReservationUsage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_reservation_id` | String | <p>The ID of the Capacity Reservation.</p> |
| `instance_type` | String | <p>The type of instance for which the Capacity Reservation reserves capacity.</p> |
| `total_instance_count` | i64 | <p>The number of instances for which the Capacity Reservation reserves capacity.</p> |
| `available_instance_count` | i64 | <p>The remaining capacity. Indicates the number of instances that can be launched in the
			Capacity Reservation.</p> |
| `state` | String | <p>The current state of the Capacity Reservation. A Capacity Reservation can be in one of
			the following states:</p>
         <ul>
            <li>
               <p>
                  <code>active</code> - The capacity is available for use.</p>
            </li>
            <li>
               <p>
                  <code>expired</code> - The Capacity Reservation expired automatically at the date and time 
		specified in your reservation request. The reserved capacity is no longer available for your use.</p>
            </li>
            <li>
               <p>
                  <code>cancelled</code> - The Capacity Reservation was canceled. The reserved capacity is no 
		longer available for your use.</p>
            </li>
            <li>
               <p>
                  <code>pending</code> - The Capacity Reservation request was successful but the capacity 
		provisioning is still pending.</p>
            </li>
            <li>
               <p>
                  <code>failed</code> - The Capacity Reservation request has failed. A request can fail due to 
		request parameters that are not valid, capacity constraints, or instance limit constraints. You 
		can view a failed request for 60 minutes.</p>
            </li>
            <li>
               <p>
                  <code>scheduled</code> - (<i>Future-dated Capacity Reservations</i>) The 
		future-dated Capacity Reservation request was approved and the Capacity Reservation is scheduled 
		for delivery on the requested start date.</p>
            </li>
            <li>
               <p>
                  <code>payment-pending</code> - (<i>Capacity Blocks</i>) The upfront 
	    payment has not been processed yet.</p>
            </li>
            <li>
               <p>
                  <code>payment-failed</code> - (<i>Capacity Blocks</i>) The upfront 
	    payment was not processed in the 12-hour time frame. Your Capacity Block was released.</p>
            </li>
            <li>
               <p>
                  <code>assessing</code> - (<i>Future-dated Capacity Reservations</i>) 
		Amazon EC2 is assessing your request for a future-dated Capacity Reservation.</p>
            </li>
            <li>
               <p>
                  <code>delayed</code> - (<i>Future-dated Capacity Reservations</i>) Amazon EC2 
		encountered a delay in provisioning the requested future-dated Capacity Reservation. Amazon EC2 is 
		unable to deliver the requested capacity by the requested start date and time.</p>
            </li>
            <li>
               <p>
                  <code>unsupported</code> - (<i>Future-dated Capacity Reservations</i>) Amazon EC2 
		can't support the future-dated Capacity Reservation request due to capacity constraints. You can view 
		unsupported requests for 30 days. The Capacity Reservation will not be delivered.</p>
            </li>
         </ul> |
| `instance_usages` | Vec<String> | <p>Information about the Capacity Reservation usage.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_reservation_usage outputs
capacity_reservation_usage_id = capacity_reservation_usage.id
capacity_reservation_usage_capacity_reservation_id = capacity_reservation_usage.capacity_reservation_id
capacity_reservation_usage_instance_type = capacity_reservation_usage.instance_type
capacity_reservation_usage_total_instance_count = capacity_reservation_usage.total_instance_count
capacity_reservation_usage_available_instance_count = capacity_reservation_usage.available_instance_count
capacity_reservation_usage_state = capacity_reservation_usage.state
capacity_reservation_usage_instance_usages = capacity_reservation_usage.instance_usages
capacity_reservation_usage_next_token = capacity_reservation_usage.next_token
```

---


### Launch_template_versions

LaunchTemplateVersions resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `launch_template_versions` | Vec<String> | <p>Information about the launch template versions.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code>
            when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access launch_template_versions outputs
launch_template_versions_id = launch_template_versions.id
launch_template_versions_launch_template_versions = launch_template_versions.launch_template_versions
launch_template_versions_next_token = launch_template_versions.next_token
```

---


### Subnet

Subnet resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ipv6_cidr_block` | String |  | <p>The IPv6 network range for the subnet, in CIDR notation. This parameter is required
            for an IPv6 only subnet.</p> |
| `outpost_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Outpost. If you specify an Outpost ARN, you must also
        specify the Availability Zone of the Outpost subnet.</p> |
| `ipv6_ipam_pool_id` | String |  | <p>An IPv6 IPAM pool ID for the subnet.</p> |
| `ipv4_ipam_pool_id` | String |  | <p>An IPv4 IPAM pool ID for the subnet.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `ipv6_netmask_length` | i64 |  | <p>An IPv6 netmask length for the subnet.</p> |
| `ipv6_native` | bool |  | <p>Indicates whether to create an IPv6 only subnet.</p> |
| `cidr_block` | String |  | <p>The IPv4 network range for the subnet, in CIDR notation. For example, <code>10.0.0.0/24</code>. 
           We modify the specified CIDR block to its canonical form; for example, if you specify 
           <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
         <p>This parameter is not supported for an IPv6 only subnet.</p> |
| `vpc_id` | String | ✅ | <p>The ID of the VPC.</p> |
| `ipv4_netmask_length` | i64 |  | <p>An IPv4 netmask length for the subnet.</p> |
| `availability_zone` | String |  | <p>The Availability Zone or Local Zone for the subnet.</p>
         <p>Default: Amazon Web Services selects one for you. If you create more than one subnet in your VPC, we 
          do not necessarily select a different zone for each subnet.</p>
         <p>To create a subnet in a Local Zone, set this value to the Local Zone ID, for example
          <code>us-west-2-lax-1a</code>. For information about the Regions that support Local Zones, 
           see <a href="https://docs.aws.amazon.com/local-zones/latest/ug/available-local-zones.html">Available Local Zones</a>.</p>
         <p>To create a subnet in an Outpost, set this value to the Availability Zone for the
           Outpost and specify the Outpost ARN.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the subnet.</p> |
| `availability_zone_id` | String |  | <p>The AZ ID or the Local Zone ID of the subnet.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subnet
subnet = provider.ec2.Subnet {
    vpc_id = "value"  # <p>The ID of the VPC.</p>
}

```

---


### Vpc_endpoint_service_configuration

VpcEndpointServiceConfiguration resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `acceptance_required` | bool |  | <p>Indicates whether requests from service consumers to create an endpoint to your service must
            be accepted manually.</p> |
| `gateway_load_balancer_arns` | Vec<String> |  | <p>The Amazon Resource Names (ARNs) of the Gateway Load Balancers.</p> |
| `supported_regions` | Vec<String> |  | <p>The Regions from which service consumers can access the service.</p> |
| `network_load_balancer_arns` | Vec<String> |  | <p>The Amazon Resource Names (ARNs) of the Network Load Balancers.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.
            For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure
                idempotency</a>.</p> |
| `supported_ip_address_types` | Vec<String> |  | <p>The supported IP address types. The possible values are <code>ipv4</code> and <code>ipv6</code>.</p> |
| `private_dns_name` | String |  | <p>(Interface endpoint configuration) The private DNS name to assign to the VPC endpoint service.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to associate with the service.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_endpoint_service_configuration
vpc_endpoint_service_configuration = provider.ec2.Vpc_endpoint_service_configuration {
}

```

---


### Transit_gateway_connect_peers

TransitGatewayConnectPeers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_connect_peers` | Vec<String> | <p>Information about the Connect peers.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_connect_peers outputs
transit_gateway_connect_peers_id = transit_gateway_connect_peers.id
transit_gateway_connect_peers_transit_gateway_connect_peers = transit_gateway_connect_peers.transit_gateway_connect_peers
transit_gateway_connect_peers_next_token = transit_gateway_connect_peers.next_token
```

---


### Network_insights_access_scope_content

NetworkInsightsAccessScopeContent resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network_insights_access_scope_content` | String | <p>The Network Access Scope content.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_insights_access_scope_content outputs
network_insights_access_scope_content_id = network_insights_access_scope_content.id
network_insights_access_scope_content_network_insights_access_scope_content = network_insights_access_scope_content.network_insights_access_scope_content
```

---


### Subnet_cidr_reservation

SubnetCidrReservation resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reservation_type` | String | ✅ | <p>The type of reservation. The reservation type determines how the reserved IP addresses are 
            assigned to resources.</p>
         <ul>
            <li>
               <p>
                  <code>prefix</code> - Amazon Web Services assigns the reserved IP addresses to 
                    network interfaces.</p>
            </li>
            <li>
               <p>
                  <code>explicit</code> - You assign the reserved IP addresses to network interfaces.</p>
            </li>
         </ul> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the subnet CIDR reservation.</p> |
| `description` | String |  | <p>The description to assign to the subnet CIDR reservation.</p> |
| `subnet_id` | String | ✅ | <p>The ID of the subnet.</p> |
| `cidr` | String | ✅ | <p>The IPv4 or IPV6 CIDR range to reserve.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create subnet_cidr_reservation
subnet_cidr_reservation = provider.ec2.Subnet_cidr_reservation {
    reservation_type = "value"  # <p>The type of reservation. The reservation type determines how the reserved IP addresses are 
            assigned to resources.</p>
         <ul>
            <li>
               <p>
                  <code>prefix</code> - Amazon Web Services assigns the reserved IP addresses to 
                    network interfaces.</p>
            </li>
            <li>
               <p>
                  <code>explicit</code> - You assign the reserved IP addresses to network interfaces.</p>
            </li>
         </ul>
    subnet_id = "value"  # <p>The ID of the subnet.</p>
    cidr = "value"  # <p>The IPv4 or IPV6 CIDR range to reserve.</p>
}

```

---


### Vpc

Vpc resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ipv6_cidr_block_network_border_group` | String |  | <p>The name of the location from which we advertise the IPV6 CIDR block. Use this parameter to limit the address to this location.</p>
         <p> You must set <code>AmazonProvidedIpv6CidrBlock</code> to <code>true</code> to use this parameter.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the VPC.</p> |
| `ipv4_netmask_length` | i64 |  | <p>The netmask length of the IPv4 CIDR you want to allocate to this VPC from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p> |
| `ipv6_pool` | String |  | <p>The ID of an IPv6 address pool from which to allocate the IPv6 CIDR block.</p> |
| `ipv6_ipam_pool_id` | String |  | <p>The ID of an IPv6 IPAM pool which will be used to allocate this VPC an IPv6 CIDR. IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across Amazon Web Services Regions and accounts throughout your Amazon Web Services Organization. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p> |
| `ipv6_netmask_length` | i64 |  | <p>The netmask length of the IPv6 CIDR you want to allocate to this VPC from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p> |
| `amazon_provided_ipv6_cidr_block` | bool |  | <p>Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC.
            You cannot specify the range of IP addresses, or the size of the CIDR block.</p> |
| `instance_tenancy` | String |  | <p>The tenancy options for instances launched into the VPC. For <code>default</code>, instances
      are launched with shared tenancy by default. You can launch instances with any tenancy into a
      shared tenancy VPC. For <code>dedicated</code>, instances are launched as dedicated tenancy
      instances by default. You can only launch instances with a tenancy of <code>dedicated</code>
      or <code>host</code> into a dedicated tenancy VPC. </p>
         <p>
            <b>Important:</b> The <code>host</code> value cannot be used with this parameter. Use the <code>default</code> or <code>dedicated</code> values only.</p>
         <p>Default: <code>default</code>
         </p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `ipv4_ipam_pool_id` | String |  | <p>The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.
         
      </p> |
| `ipv6_cidr_block` | String |  | <p>The IPv6 CIDR block from the IPv6 address pool. You must also specify <code>Ipv6Pool</code> in the request.</p>
         <p>To let Amazon choose the IPv6 CIDR block for you, omit this parameter.</p> |
| `cidr_block` | String |  | <p>The IPv4 network range for the VPC, in CIDR notation. For example,
		        <code>10.0.0.0/16</code>. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc
vpc = provider.ec2.Vpc {
}

```

---


### Import_snapshot_tasks

ImportSnapshotTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to get the next page of results. This value is <code>null</code> when there are no more results
   to return.</p> |
| `import_snapshot_tasks` | Vec<String> | <p>A list of zero or more import snapshot tasks that are currently active or were completed or canceled in the
   previous 7 days.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access import_snapshot_tasks outputs
import_snapshot_tasks_id = import_snapshot_tasks.id
import_snapshot_tasks_next_token = import_snapshot_tasks.next_token
import_snapshot_tasks_import_snapshot_tasks = import_snapshot_tasks.import_snapshot_tasks
```

---


### Ipam_prefix_list_resolver_target

IpamPrefixListResolverTarget resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `prefix_list_region` | String | ✅ | <p>The Amazon Web Services Region where the prefix list is located. This is required when referencing a prefix list in a different Region.</p> |
| `ipam_prefix_list_resolver_id` | String | ✅ | <p>The ID of the IPAM prefix list resolver that will manage the synchronization of CIDRs to the target prefix list.</p> |
| `desired_version` | i64 |  | <p>The specific version of the prefix list to target. If not specified, the resolver will target the latest version.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the IPAM prefix list resolver target during creation. Tags help you organize and manage your Amazon Web Services resources.</p> |
| `prefix_list_id` | String | ✅ | <p>The ID of the managed prefix list that will be synchronized with CIDRs selected by the IPAM prefix list resolver. This prefix list becomes an IPAM managed prefix list.</p>
         <p>An IPAM-managed prefix list is a customer-managed prefix list that has been associated with an IPAM prefix list resolver target. When a prefix list becomes IPAM managed, its CIDRs are automatically synchronized based on the IPAM prefix list resolver's CIDR selection rules, and direct CIDR modifications are restricted.</p> |
| `track_latest_version` | bool | ✅ | <p>Indicates whether the resolver target should automatically track the latest version of the prefix list. When enabled, the target will always synchronize with the most current version of the prefix list.</p>
         <p>Choose this for automatic updates when you want your prefix lists to stay current with infrastructure changes without manual intervention.</p> |
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ipam_prefix_list_resolver_target
ipam_prefix_list_resolver_target = provider.ec2.Ipam_prefix_list_resolver_target {
    prefix_list_region = "value"  # <p>The Amazon Web Services Region where the prefix list is located. This is required when referencing a prefix list in a different Region.</p>
    ipam_prefix_list_resolver_id = "value"  # <p>The ID of the IPAM prefix list resolver that will manage the synchronization of CIDRs to the target prefix list.</p>
    prefix_list_id = "value"  # <p>The ID of the managed prefix list that will be synchronized with CIDRs selected by the IPAM prefix list resolver. This prefix list becomes an IPAM managed prefix list.</p>
         <p>An IPAM-managed prefix list is a customer-managed prefix list that has been associated with an IPAM prefix list resolver target. When a prefix list becomes IPAM managed, its CIDRs are automatically synchronized based on the IPAM prefix list resolver's CIDR selection rules, and direct CIDR modifications are restricted.</p>
    track_latest_version = "value"  # <p>Indicates whether the resolver target should automatically track the latest version of the prefix list. When enabled, the target will always synchronize with the most current version of the prefix list.</p>
         <p>Choose this for automatic updates when you want your prefix lists to stay current with infrastructure changes without manual intervention.</p>
}

```

---


### Instance_credit_specifications

InstanceCreditSpecifications resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_credit_specifications` | Vec<String> | <p>Information about the credit option for CPU usage of an instance.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_credit_specifications outputs
instance_credit_specifications_id = instance_credit_specifications.id
instance_credit_specifications_instance_credit_specifications = instance_credit_specifications.instance_credit_specifications
instance_credit_specifications_next_token = instance_credit_specifications.next_token
```

---


### Fleet

Fleet resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `on_demand_options` | String |  | <p>Describes the configuration of On-Demand Instances in an EC2 Fleet.</p> |
| `excess_capacity_termination_policy` | String |  | <p>Indicates whether running instances should be terminated if the total target capacity of
         the EC2 Fleet is decreased below the current size of the EC2 Fleet.</p>
         <p>Supported only for fleets of type <code>maintain</code>.</p> |
| `valid_until` | String |  | <p>The end date and time of the request, in UTC format (for example,
            <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).
         At this point, no new EC2 Fleet requests are placed or able to fulfill the request. If no value is specified, the request remains until you cancel it.</p> |
| `tag_specifications` | Vec<String> |  | <p>The key-value pair for tagging the EC2 Fleet request on creation. For more information, see 
         <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-resources">Tag your resources</a>.</p>
         <p>If the fleet type is <code>instant</code>, specify a resource type of <code>fleet</code> 
         to tag the fleet or <code>instance</code> to tag the instances at launch.</p>
         <p>If the fleet type is <code>maintain</code> or <code>request</code>, specify a resource
         type of <code>fleet</code> to tag the fleet. You cannot specify a resource type of
            <code>instance</code>. To tag instances at launch, specify the tags in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html#create-launch-template">launch template</a>.</p> |
| `launch_template_configs` | Vec<String> | ✅ | <p>The configuration for the EC2 Fleet.</p> |
| `spot_options` | String |  | <p>Describes the configuration of Spot Instances in an EC2 Fleet.</p> |
| `replace_unhealthy_instances` | bool |  | <p>Indicates whether EC2 Fleet should replace unhealthy Spot Instances. Supported only for
         fleets of type <code>maintain</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/manage-ec2-fleet.html#ec2-fleet-health-checks">EC2 Fleet
            health checks</a> in the <i>Amazon EC2 User Guide</i>.</p> |
| `type` | String |  | <p>The fleet type. The default value is <code>maintain</code>.</p>
         <ul>
            <li>
               <p>
                  <code>maintain</code> - The EC2 Fleet places an asynchronous request for your desired
               capacity, and continues to maintain your desired Spot capacity by replenishing
               interrupted Spot Instances.</p>
            </li>
            <li>
               <p>
                  <code>request</code> - The EC2 Fleet places an asynchronous one-time request for your
               desired capacity, but does submit Spot requests in alternative capacity pools if Spot
               capacity is unavailable, and does not maintain Spot capacity if Spot Instances are
               interrupted.</p>
            </li>
            <li>
               <p>
                  <code>instant</code> - The EC2 Fleet places a synchronous one-time request for your
               desired capacity, and returns errors for any instances that could not be
               launched.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet-request-type.html">EC2 Fleet
            request types</a> in the <i>Amazon EC2 User Guide</i>.</p> |
| `context` | String |  | <p>Reserved.</p> |
| `target_capacity_specification` | String | ✅ | <p>The number of units to request.</p> |
| `terminate_instances_with_expiration` | bool |  | <p>Indicates whether running instances should be terminated when the EC2 Fleet expires.</p> |
| `valid_from` | String |  | <p>The start date and time of the request, in UTC format (for example,
            <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).
         The default is to start fulfilling the request immediately.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
          request. If you do not specify a client token, a randomly generated token is used for
          the request to ensure idempotency.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring
            idempotency</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fleet
fleet = provider.ec2.Fleet {
    launch_template_configs = "value"  # <p>The configuration for the EC2 Fleet.</p>
    target_capacity_specification = "value"  # <p>The number of units to request.</p>
}

```

---


### Fleets

Fleets resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleets` | Vec<String> | <p>Information about the EC2 Fleets.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleets outputs
fleets_id = fleets.id
fleets_fleets = fleets.fleets
fleets_next_token = fleets.next_token
```

---


### Instance_attribute

InstanceAttribute resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ramdisk_id` | String | <p>The RAM disk ID.</p> |
| `ebs_optimized` | String | <p>Indicates whether the instance is optimized for Amazon EBS I/O.</p> |
| `disable_api_termination` | String | <p>Indicates whether termination protection is enabled. If the value is <code>true</code>, 
            you can't terminate the instance using the Amazon EC2 console, command line tools, or API.</p> |
| `ena_support` | String | <p>Indicates whether enhanced networking with ENA is enabled.</p> |
| `product_codes` | Vec<String> | <p>The product codes.</p> |
| `root_device_name` | String | <p>The device name of the root device volume (for example,
            <code>/dev/sda1</code>).</p> |
| `enclave_options` | String | <p>Indicates whether the instance is enabled for Amazon Web Services Nitro Enclaves.</p> |
| `source_dest_check` | String | <p>Indicates whether source/destination checks are enabled.</p> |
| `user_data` | String | <p>The user data.</p> |
| `instance_id` | String | <p>The ID of the instance.</p> |
| `block_device_mappings` | Vec<String> | <p>The block device mapping of the instance.</p> |
| `disable_api_stop` | String | <p>Indicates whether stop protection is enabled for the instance.</p> |
| `groups` | Vec<String> | <p>The security groups associated with the instance.</p> |
| `sriov_net_support` | String | <p>Indicates whether enhanced networking with the Intel 82599 Virtual Function interface
            is enabled.</p> |
| `instance_initiated_shutdown_behavior` | String | <p>Indicates whether an instance stops or terminates when you initiate shutdown from the
            instance (using the operating system command for system shutdown).</p> |
| `kernel_id` | String | <p>The kernel ID.</p> |
| `instance_type` | String | <p>The instance type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_attribute outputs
instance_attribute_id = instance_attribute.id
instance_attribute_ramdisk_id = instance_attribute.ramdisk_id
instance_attribute_ebs_optimized = instance_attribute.ebs_optimized
instance_attribute_disable_api_termination = instance_attribute.disable_api_termination
instance_attribute_ena_support = instance_attribute.ena_support
instance_attribute_product_codes = instance_attribute.product_codes
instance_attribute_root_device_name = instance_attribute.root_device_name
instance_attribute_enclave_options = instance_attribute.enclave_options
instance_attribute_source_dest_check = instance_attribute.source_dest_check
instance_attribute_user_data = instance_attribute.user_data
instance_attribute_instance_id = instance_attribute.instance_id
instance_attribute_block_device_mappings = instance_attribute.block_device_mappings
instance_attribute_disable_api_stop = instance_attribute.disable_api_stop
instance_attribute_groups = instance_attribute.groups
instance_attribute_sriov_net_support = instance_attribute.sriov_net_support
instance_attribute_instance_initiated_shutdown_behavior = instance_attribute.instance_initiated_shutdown_behavior
instance_attribute_kernel_id = instance_attribute.kernel_id
instance_attribute_instance_type = instance_attribute.instance_type
```

---


### Launch_templates

LaunchTemplates resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code>
            when there are no more results to return.</p> |
| `launch_templates` | Vec<String> | <p>Information about the launch templates.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access launch_templates outputs
launch_templates_id = launch_templates.id
launch_templates_next_token = launch_templates.next_token
launch_templates_launch_templates = launch_templates.launch_templates
```

---


### Reserved_instances

ReservedInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_instances` | Vec<String> | <p>A list of Reserved Instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_instances outputs
reserved_instances_id = reserved_instances.id
reserved_instances_reserved_instances = reserved_instances.reserved_instances
```

---


### Reserved_instances_listings

ReservedInstancesListings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_instances_listings` | Vec<String> | <p>Information about the Reserved Instance listing.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_instances_listings outputs
reserved_instances_listings_id = reserved_instances_listings.id
reserved_instances_listings_reserved_instances_listings = reserved_instances_listings.reserved_instances_listings
```

---


### Image_usage_report_entries

ImageUsageReportEntries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `image_usage_report_entries` | Vec<String> | <p>The content of the usage reports.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_usage_report_entries outputs
image_usage_report_entries_id = image_usage_report_entries.id
image_usage_report_entries_next_token = image_usage_report_entries.next_token
image_usage_report_entries_image_usage_report_entries = image_usage_report_entries.image_usage_report_entries
```

---


### Vpc_peering_connection

VpcPeeringConnection resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_id` | String | ✅ | <p>The ID of the requester VPC. You must specify this parameter in the
			request.</p> |
| `peer_vpc_id` | String |  | <p>The ID of the VPC with which you are creating the VPC peering connection. You must
			specify this parameter in the request.</p> |
| `peer_region` | String |  | <p>The Region code for the accepter VPC, if the accepter VPC is located in a Region
            other than the Region in which you make the request.</p>
         <p>Default: The Region in which you make the request.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the peering connection.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `peer_owner_id` | String |  | <p>The Amazon Web Services account ID of the owner of the accepter VPC.</p>
         <p>Default: Your Amazon Web Services account ID</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_peering_connection
vpc_peering_connection = provider.ec2.Vpc_peering_connection {
    vpc_id = "value"  # <p>The ID of the requester VPC. You must specify this parameter in the
			request.</p>
}

```

---


### Store_image_task

StoreImageTask resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `s3_object_tags` | Vec<String> |  | <p>The tags to apply to the AMI object that will be stored in the Amazon S3 bucket. </p> |
| `bucket` | String | ✅ | <p>The name of the Amazon S3 bucket in which the AMI object will be stored. The bucket must be in
      the Region in which the request is being made. The AMI object appears in the bucket only after
      the upload task has completed. </p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
			and provides an error response. If you have the required permissions, the error response is 
			<code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `image_id` | String | ✅ | <p>The ID of the AMI.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create store_image_task
store_image_task = provider.ec2.Store_image_task {
    bucket = "value"  # <p>The name of the Amazon S3 bucket in which the AMI object will be stored. The bucket must be in
      the Region in which the request is being made. The AMI object appears in the bucket only after
      the upload task has completed. </p>
    image_id = "value"  # <p>The ID of the AMI.</p>
}

```

---


### Iam_instance_profile_associations

IamInstanceProfileAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
            This value is <code>null</code> when there are no more items to return.</p> |
| `iam_instance_profile_associations` | Vec<String> | <p>Information about the IAM instance profile associations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access iam_instance_profile_associations outputs
iam_instance_profile_associations_id = iam_instance_profile_associations.id
iam_instance_profile_associations_next_token = iam_instance_profile_associations.next_token
iam_instance_profile_associations_iam_instance_profile_associations = iam_instance_profile_associations.iam_instance_profile_associations
```

---


### Route_server_associations

RouteServerAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `route_server_associations` | Vec<String> | <p>Information about the associations for the specified route server.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access route_server_associations outputs
route_server_associations_id = route_server_associations.id
route_server_associations_route_server_associations = route_server_associations.route_server_associations
```

---


### Snapshot_block_public_access_state

SnapshotBlockPublicAccessState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_by` | String | <p>The entity that manages the state for block public access for snapshots. Possible
            values include:</p>
         <ul>
            <li>
               <p>
                  <code>account</code> - The state is managed by the account.</p>
            </li>
            <li>
               <p>
                  <code>declarative-policy</code> - The state is managed by a declarative policy and
            can't be modified by the account.</p>
            </li>
         </ul> |
| `state` | String | <p>The current state of block public access for snapshots. Possible values include:</p>
         <ul>
            <li>
               <p>
                  <code>block-all-sharing</code> - All public sharing of snapshots is blocked. Users in 
          the account can't request new public sharing. Additionally, snapshots that were already 
          publicly shared are treated as private and are not publicly available.</p>
            </li>
            <li>
               <p>
                  <code>block-new-sharing</code>  - Only new public sharing of snapshots is blocked. 
          Users in the account can't request new public sharing. However, snapshots that were 
          already publicly shared, remain publicly available.</p>
            </li>
            <li>
               <p>
                  <code>unblocked</code>  - Public sharing is not blocked. Users can publicly share 
          snapshots.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshot_block_public_access_state outputs
snapshot_block_public_access_state_id = snapshot_block_public_access_state.id
snapshot_block_public_access_state_managed_by = snapshot_block_public_access_state.managed_by
snapshot_block_public_access_state_state = snapshot_block_public_access_state.state
```

---


### Local_gateway_route_table_vpc_association

LocalGatewayRouteTableVpcAssociation resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the local gateway route table VPC association.</p> |
| `local_gateway_route_table_id` | String | ✅ | <p>The ID of the local gateway route table.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `vpc_id` | String | ✅ | <p>The ID of the VPC.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create local_gateway_route_table_vpc_association
local_gateway_route_table_vpc_association = provider.ec2.Local_gateway_route_table_vpc_association {
    local_gateway_route_table_id = "value"  # <p>The ID of the local gateway route table.</p>
    vpc_id = "value"  # <p>The ID of the VPC.</p>
}

```

---


### Principal_id_format

PrincipalIdFormat resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `principals` | Vec<String> | <p>Information about the ID format settings for the ARN.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access principal_id_format outputs
principal_id_format_id = principal_id_format.id
principal_id_format_principals = principal_id_format.principals
principal_id_format_next_token = principal_id_format.next_token
```

---


### Vpc_block_public_access_exclusion

VpcBlockPublicAccessExclusion resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vpc_id` | String |  | <p>A VPC ID.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `subnet_id` | String |  | <p>A subnet ID.</p> |
| `tag_specifications` | Vec<String> |  | <p>
            <code>tag</code> - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value.
    For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p> |
| `internet_gateway_exclusion_mode` | String | ✅ | <p>The exclusion mode for internet gateway traffic.</p>
         <ul>
            <li>
               <p>
                  <code>allow-bidirectional</code>: Allow all internet traffic to and from the excluded VPCs and subnets.</p>
            </li>
            <li>
               <p>
                  <code>allow-egress</code>: Allow outbound internet traffic from the excluded VPCs and subnets. Block inbound internet traffic to the excluded VPCs and subnets. Only applies when VPC Block Public Access is set to Bidirectional.</p>
            </li>
         </ul> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_block_public_access_exclusion
vpc_block_public_access_exclusion = provider.ec2.Vpc_block_public_access_exclusion {
    internet_gateway_exclusion_mode = "value"  # <p>The exclusion mode for internet gateway traffic.</p>
         <ul>
            <li>
               <p>
                  <code>allow-bidirectional</code>: Allow all internet traffic to and from the excluded VPCs and subnets.</p>
            </li>
            <li>
               <p>
                  <code>allow-egress</code>: Allow outbound internet traffic from the excluded VPCs and subnets. Block inbound internet traffic to the excluded VPCs and subnets. Only applies when VPC Block Public Access is set to Bidirectional.</p>
            </li>
         </ul>
}

```

---


### Scheduled_instances

ScheduledInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token required to retrieve the next set of results. This value is <code>null</code> when there are no more results to return.</p> |
| `scheduled_instance_set` | Vec<String> | <p>Information about the Scheduled Instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scheduled_instances outputs
scheduled_instances_id = scheduled_instances.id
scheduled_instances_next_token = scheduled_instances.next_token
scheduled_instances_scheduled_instance_set = scheduled_instances.scheduled_instance_set
```

---


### Fleet_history

FleetHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fleet_id` | String | <p>The ID of the EC Fleet.</p> |
| `start_time` | String | <p>The start date and time for the events, in UTC format (for example,
            <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p> |
| `history_records` | Vec<String> | <p>Information about the events in the history of the EC2 Fleet.</p> |
| `last_evaluated_time` | String | <p>The last date and time for the events, in UTC format (for example,
            <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).
         All records up to this time were retrieved.</p>
         <p>If <code>nextToken</code> indicates that there are more items, this value is not
         present.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_history outputs
fleet_history_id = fleet_history.id
fleet_history_fleet_id = fleet_history.fleet_id
fleet_history_start_time = fleet_history.start_time
fleet_history_history_records = fleet_history.history_records
fleet_history_last_evaluated_time = fleet_history.last_evaluated_time
fleet_history_next_token = fleet_history.next_token
```

---


### Key_pairs

KeyPairs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_pairs` | Vec<String> | <p>Information about the key pairs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access key_pairs outputs
key_pairs_id = key_pairs.id
key_pairs_key_pairs = key_pairs.key_pairs
```

---


### Ipam_resource_discovery

IpamResourceDiscovery resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `description` | String |  | <p>A description for the IPAM resource discovery.</p> |
| `client_token` | String |  | <p>A client token for the IPAM resource discovery.</p> |
| `tag_specifications` | Vec<String> |  | <p>Tag specifications for the IPAM resource discovery.</p> |
| `operating_regions` | Vec<String> |  | <p>Operating Regions for the IPAM resource discovery. Operating Regions are Amazon Web Services Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the Amazon Web Services Regions you select as operating Regions.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ipam_resource_discovery
ipam_resource_discovery = provider.ec2.Ipam_resource_discovery {
}

```

---


### Capacity_manager_organizations_access

CapacityManagerOrganizationsAccess resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organizations_access` | bool | ✅ | <p>
    Specifies whether to enable or disable cross-account access for Amazon Web Services Organizations. When enabled, Capacity Manager aggregates data from all accounts in your organization.
</p> |
| `client_token` | String |  | <p>
Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.
</p> |
| `dry_run` | bool |  | <p>
Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If 
you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.
</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Local_gateway_route

LocalGatewayRoute resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network_interface_id` | String |  | <p>The ID of the network interface.</p> |
| `local_gateway_route_table_id` | String | ✅ | <p>The ID of the local gateway route table.</p> |
| `destination_cidr_block` | String |  | <p>The CIDR range used for destination matches. Routing decisions are based on 
        the most specific match.</p> |
| `local_gateway_virtual_interface_group_id` | String |  | <p>The ID of the virtual interface group.</p> |
| `destination_prefix_list_id` | String |  | <p>
         The ID of the prefix list. Use a prefix list in place of <code>DestinationCidrBlock</code>. You 
         cannot use <code>DestinationPrefixListId</code> and <code>DestinationCidrBlock</code> in the same request.
      </p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create local_gateway_route
local_gateway_route = provider.ec2.Local_gateway_route {
    local_gateway_route_table_id = "value"  # <p>The ID of the local gateway route table.</p>
}

```

---


### Traffic_mirror_target

TrafficMirrorTarget resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gateway_load_balancer_endpoint_id` | String |  | <p>The ID of the Gateway Load Balancer endpoint.</p> |
| `network_interface_id` | String |  | <p>The network interface ID that is associated with the target.</p> |
| `network_load_balancer_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.</p> |
| `description` | String |  | <p>The description of the Traffic Mirror target.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the Traffic Mirror target.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure idempotency</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create traffic_mirror_target
traffic_mirror_target = provider.ec2.Traffic_mirror_target {
}

```

---


### Client_vpn_endpoint

ClientVpnEndpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `self_service_portal` | String |  | <p>Specify whether to enable the self-service portal for the Client VPN endpoint.</p>
         <p>Default Value: <code>enabled</code>
         </p> |
| `client_connect_options` | String |  | <p>The options for managing connection authorization for new client connections.</p> |
| `connection_log_options` | String | ✅ | <p>Information about the client connection logging options.</p>
         <p>If you enable client connection logging, data about client connections is sent to a
			Cloudwatch Logs log stream. The following information is logged:</p>
         <ul>
            <li>
               <p>Client connection requests</p>
            </li>
            <li>
               <p>Client connection results (successful and unsuccessful)</p>
            </li>
            <li>
               <p>Reasons for unsuccessful client connection requests</p>
            </li>
            <li>
               <p>Client connection termination time</p>
            </li>
         </ul> |
| `dns_servers` | Vec<String> |  | <p>Information about the DNS servers to be used for DNS resolution. A Client VPN endpoint can
			have up to two DNS servers. If no DNS server is specified, the DNS address configured on the device is used for the DNS server.</p> |
| `description` | String |  | <p>A brief description of the Client VPN endpoint.</p> |
| `split_tunnel` | bool |  | <p>Indicates whether split-tunnel is enabled on the Client VPN endpoint.</p>
         <p>By default, split-tunnel on a VPN endpoint is disabled.</p>
         <p>For information about split-tunnel VPN endpoints, see <a href="https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/split-tunnel-vpn.html">Split-tunnel Client VPN endpoint</a> in the 
			<i>Client VPN Administrator Guide</i>.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. 
For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `security_group_ids` | Vec<String> |  | <p>The IDs of one or more security groups to apply to the target network. You must also specify the ID of the VPC that contains the security groups.</p> |
| `vpc_id` | String |  | <p>The ID of the VPC to associate with the Client VPN endpoint. If no security group IDs are specified in the request, the default security group for the VPC is applied.</p> |
| `session_timeout_hours` | i64 |  | <p>The maximum VPN session duration time in hours.</p>
         <p>Valid values: <code>8 | 10 | 12 | 24</code>
         </p>
         <p>Default value: <code>24</code>
         </p> |
| `endpoint_ip_address_type` | String |  | <p>The IP address type for the Client VPN endpoint. Valid values are <code>ipv4</code>
			(default) for IPv4 addressing only, <code>ipv6</code> for IPv6 addressing only, or <code>dual-stack</code> for both IPv4 and IPv6
			addressing. When set to <code>dual-stack,</code> clients can connect to the endpoint
			using either IPv4 or IPv6 addresses..</p> |
| `disconnect_on_session_timeout` | bool |  | <p>Indicates whether the client VPN session is disconnected after the maximum timeout specified in <code>SessionTimeoutHours</code> is reached. If <code>true</code>, users are prompted to reconnect client VPN. If <code>false</code>, client VPN attempts to reconnect automatically. 
                   The default value is <code>true</code>.</p> |
| `traffic_ip_address_type` | String |  | <p>The IP address type for traffic within the Client VPN tunnel. Valid values are <code>ipv4</code> (default) for IPv4 traffic only, <code>ipv6</code> for IPv6 addressing only, or <code>dual-stack</code> for both IPv4 and IPv6 traffic. When set to <code>dual-stack</code>, clients can access both IPv4 and IPv6 resources through the VPN .</p> |
| `transport_protocol` | String |  | <p>The transport protocol to be used by the VPN session.</p>
         <p>Default value: <code>udp</code>
         </p> |
| `client_login_banner_options` | String |  | <p>Options for enabling a customizable text banner that will be displayed on
			Amazon Web Services provided clients when a VPN session is established.</p> |
| `client_cidr_block` | String |  | <p>The IPv4 address range, in CIDR notation, from which to assign client IP addresses. The address range cannot overlap with the local CIDR of the VPC in which the associated subnet is located, or the routes that you add manually. The address range cannot be changed after the Client VPN endpoint has been created. Client CIDR range must have a size of at least /22 and must not be greater than /12.</p> |
| `vpn_port` | i64 |  | <p>The port number to assign to the Client VPN endpoint for TCP and UDP traffic.</p>
         <p>Valid Values: <code>443</code> | <code>1194</code>
         </p>
         <p>Default Value: <code>443</code>
         </p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the Client VPN endpoint during creation.</p> |
| `server_certificate_arn` | String | ✅ | <p>The ARN of the server certificate. For more information, see 
			the <a href="https://docs.aws.amazon.com/acm/latest/userguide/">Certificate Manager User Guide</a>.</p> |
| `client_route_enforcement_options` | String |  | <p>Client route enforcement is a feature of the Client VPN service that helps enforce administrator defined routes on devices connected through the VPN. T
		his feature helps improve your security posture by ensuring that network traffic originating from a connected client is not inadvertently sent outside the VPN tunnel.</p>
         <p>Client route enforcement works by monitoring the route table of a connected device for routing policy changes to the VPN connection. If the feature detects any VPN routing policy modifications, it will automatically force an update to the route table, 
			reverting it back to the expected route configurations.</p> |
| `authentication_options` | Vec<String> | ✅ | <p>Information about the authentication method to be used to authenticate clients.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create client_vpn_endpoint
client_vpn_endpoint = provider.ec2.Client_vpn_endpoint {
    connection_log_options = "value"  # <p>Information about the client connection logging options.</p>
         <p>If you enable client connection logging, data about client connections is sent to a
			Cloudwatch Logs log stream. The following information is logged:</p>
         <ul>
            <li>
               <p>Client connection requests</p>
            </li>
            <li>
               <p>Client connection results (successful and unsuccessful)</p>
            </li>
            <li>
               <p>Reasons for unsuccessful client connection requests</p>
            </li>
            <li>
               <p>Client connection termination time</p>
            </li>
         </ul>
    server_certificate_arn = "value"  # <p>The ARN of the server certificate. For more information, see 
			the <a href="https://docs.aws.amazon.com/acm/latest/userguide/">Certificate Manager User Guide</a>.</p>
    authentication_options = "value"  # <p>Information about the authentication method to be used to authenticate clients.</p>
}

```

---


### Transit_gateway_policy_table

TransitGatewayPolicyTable resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transit_gateway_id` | String | ✅ | <p>The ID of the transit gateway used for the policy table.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags specification for the transit gateway policy table created during the request.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_policy_table
transit_gateway_policy_table = provider.ec2.Transit_gateway_policy_table {
    transit_gateway_id = "value"  # <p>The ID of the transit gateway used for the policy table.</p>
}

```

---


### Customer_gateways

CustomerGateways resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer_gateways` | Vec<String> | <p>Information about one or more customer gateways.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access customer_gateways outputs
customer_gateways_id = customer_gateways.id
customer_gateways_customer_gateways = customer_gateways.customer_gateways
```

---


### Ipam_prefix_list_resolver_targets

IpamPrefixListResolverTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `ipam_prefix_list_resolver_targets` | Vec<String> | <p>Information about the IPAM prefix list resolver Targets.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_prefix_list_resolver_targets outputs
ipam_prefix_list_resolver_targets_id = ipam_prefix_list_resolver_targets.id
ipam_prefix_list_resolver_targets_next_token = ipam_prefix_list_resolver_targets.next_token
ipam_prefix_list_resolver_targets_ipam_prefix_list_resolver_targets = ipam_prefix_list_resolver_targets.ipam_prefix_list_resolver_targets
```

---


### Ipam_resource_discoveries

IpamResourceDiscoveries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipam_resource_discoveries` | Vec<String> | <p>The resource discoveries.</p> |
| `next_token` | String | <p>Specify the pagination token from a previous request to retrieve the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_resource_discoveries outputs
ipam_resource_discoveries_id = ipam_resource_discoveries.id
ipam_resource_discoveries_ipam_resource_discoveries = ipam_resource_discoveries.ipam_resource_discoveries
ipam_resource_discoveries_next_token = ipam_resource_discoveries.next_token
```

---


### Reserved_instances_exchange_quote

ReservedInstancesExchangeQuote resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_instance_value_rollup` | String | <p>The cost associated with the Reserved Instance.</p> |
| `output_reserved_instances_will_expire_at` | String | <p>The new end date of the reservation term.</p> |
| `payment_due` | String | <p>The total true upfront charge for the exchange.</p> |
| `currency_code` | String | <p>The currency of the transaction.</p> |
| `reserved_instance_value_set` | Vec<String> | <p>The configuration of your Convertible Reserved Instances.</p> |
| `target_configuration_value_rollup` | String | <p>The cost associated with the Reserved Instance.</p> |
| `target_configuration_value_set` | Vec<String> | <p>The values of the target Convertible Reserved Instances.</p> |
| `is_valid_exchange` | bool | <p>If <code>true</code>, the exchange is valid. If <code>false</code>, the exchange cannot be
      completed.</p> |
| `validation_failure_reason` | String | <p>Describes the reason why the exchange cannot be completed.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_instances_exchange_quote outputs
reserved_instances_exchange_quote_id = reserved_instances_exchange_quote.id
reserved_instances_exchange_quote_reserved_instance_value_rollup = reserved_instances_exchange_quote.reserved_instance_value_rollup
reserved_instances_exchange_quote_output_reserved_instances_will_expire_at = reserved_instances_exchange_quote.output_reserved_instances_will_expire_at
reserved_instances_exchange_quote_payment_due = reserved_instances_exchange_quote.payment_due
reserved_instances_exchange_quote_currency_code = reserved_instances_exchange_quote.currency_code
reserved_instances_exchange_quote_reserved_instance_value_set = reserved_instances_exchange_quote.reserved_instance_value_set
reserved_instances_exchange_quote_target_configuration_value_rollup = reserved_instances_exchange_quote.target_configuration_value_rollup
reserved_instances_exchange_quote_target_configuration_value_set = reserved_instances_exchange_quote.target_configuration_value_set
reserved_instances_exchange_quote_is_valid_exchange = reserved_instances_exchange_quote.is_valid_exchange
reserved_instances_exchange_quote_validation_failure_reason = reserved_instances_exchange_quote.validation_failure_reason
```

---


### Vpc_peering_connections

VpcPeeringConnections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_peering_connections` | Vec<String> | <p>Information about the VPC peering connections.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_peering_connections outputs
vpc_peering_connections_id = vpc_peering_connections.id
vpc_peering_connections_vpc_peering_connections = vpc_peering_connections.vpc_peering_connections
vpc_peering_connections_next_token = vpc_peering_connections.next_token
```

---


### Route_table

RouteTable resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `vpc_id` | String | ✅ | <p>The ID of the VPC.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the route table.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create route_table
route_table = provider.ec2.Route_table {
    vpc_id = "value"  # <p>The ID of the VPC.</p>
}

```

---


### Prefix_lists

PrefixLists resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `prefix_lists` | Vec<String> | <p>All available prefix lists.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access prefix_lists outputs
prefix_lists_id = prefix_lists.id
prefix_lists_next_token = prefix_lists.next_token
prefix_lists_prefix_lists = prefix_lists.prefix_lists
```

---


### Network_insights_access_scope_analysis

NetworkInsightsAccessScopeAnalysis resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Console_output

ConsoleOutput resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_id` | String | <p>The ID of the instance.</p> |
| `output` | String | <p>The console output, base64-encoded. If you are using a command line tool, the tool
            decodes the output for you.</p> |
| `timestamp` | String | <p>The time at which the output was last updated.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access console_output outputs
console_output_id = console_output.id
console_output_instance_id = console_output.instance_id
console_output_output = console_output.output
console_output_timestamp = console_output.timestamp
```

---


### Security_group_rule_descriptions_egress

SecurityGroupRuleDescriptionsEgress resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `security_group_rule_descriptions` | Vec<String> |  | <p>The description for the egress security group rules. You must specify either the
            description or the IP permissions.</p> |
| `group_name` | String |  | <p>[Default VPC] The name of the security group. You must specify either the security group
			ID or the security group name.</p> |
| `group_id` | String |  | <p>The ID of the security group. You must specify either the security group ID or the
			security group name in the request. For security groups in a nondefault VPC, you must
			specify the security group ID.</p> |
| `ip_permissions` | Vec<String> |  | <p>The IP permissions for the security group rule. You must specify either the IP permissions
		    or the description.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Volume_status

VolumeStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `volume_statuses` | Vec<String> | <p>Information about the status of the volumes.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
  This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access volume_status outputs
volume_status_id = volume_status.id
volume_status_volume_statuses = volume_status.volume_statuses
volume_status_next_token = volume_status.next_token
```

---


### Vpc_classic_link

VpcClassicLink resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpcs` | Vec<String> | <p>The ClassicLink status of the VPCs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_classic_link outputs
vpc_classic_link_id = vpc_classic_link.id
vpc_classic_link_vpcs = vpc_classic_link.vpcs
```

---


### Queued_reserved_instances

QueuedReservedInstances resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Import_image_tasks

ImportImageTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_image_tasks` | Vec<String> | <p>A list of zero or more import image tasks that are currently active or were completed or canceled in the
   previous 7 days.</p> |
| `next_token` | String | <p>The token to use to get the next page of results. This value is <code>null</code> when there are no more results
   to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access import_image_tasks outputs
import_image_tasks_id = import_image_tasks.id
import_image_tasks_import_image_tasks = import_image_tasks.import_image_tasks
import_image_tasks_next_token = import_image_tasks.next_token
```

---


### Instance_event_notification_attributes

InstanceEventNotificationAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_tag_attribute` | String | <p>Information about the registered tag keys.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_event_notification_attributes outputs
instance_event_notification_attributes_id = instance_event_notification_attributes.id
instance_event_notification_attributes_instance_tag_attribute = instance_event_notification_attributes.instance_tag_attribute
```

---


### Spot_fleet_requests

SpotFleetRequests resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `spot_fleet_request_configs` | Vec<String> | <p>Information about the configuration of your Spot Fleet.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access spot_fleet_requests outputs
spot_fleet_requests_id = spot_fleet_requests.id
spot_fleet_requests_next_token = spot_fleet_requests.next_token
spot_fleet_requests_spot_fleet_request_configs = spot_fleet_requests.spot_fleet_request_configs
```

---


### Capacity_blocks

CapacityBlocks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_blocks` | Vec<String> | <p>The Capacity Blocks.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_blocks outputs
capacity_blocks_id = capacity_blocks.id
capacity_blocks_capacity_blocks = capacity_blocks.capacity_blocks
capacity_blocks_next_token = capacity_blocks.next_token
```

---


### Client_vpn_target_networks

ClientVpnTargetNetworks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_vpn_target_networks` | Vec<String> | <p>Information about the associated target networks.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access client_vpn_target_networks outputs
client_vpn_target_networks_id = client_vpn_target_networks.id
client_vpn_target_networks_client_vpn_target_networks = client_vpn_target_networks.client_vpn_target_networks
client_vpn_target_networks_next_token = client_vpn_target_networks.next_token
```

---


### Mac_system_integrity_protection_modification_task

MacSystemIntegrityProtectionModificationTask resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mac_system_integrity_protection_configuration` | String |  | <p>Specifies the overrides to selectively enable or disable individual SIP settings. 
         The individual settings you specify here override the overall SIP status you specify 
         for <b>MacSystemIntegrityProtectionStatus</b>.</p> |
| `instance_id` | String | ✅ | <p>The ID of the Amazon EC2 Mac instance.</p> |
| `mac_credentials` | String |  | <p>
            <b>[Apple silicon Mac instances only]</b> Specifies the 
         following credentials:</p>
         <ul>
            <li>
               <p>
                  <b>Internal disk administrative user</b>
               </p>
               <ul>
                  <li>
                     <p>
                        <b>Username</b> - Only the default administrative 
                     user (<code>aws-managed-user</code>) is supported and it is used by default. You 
                     can't specify a different administrative user.</p>
                  </li>
                  <li>
                     <p>
                        <b>Password</b> - If you did not change the default 
                     password for <code>aws-managed-user</code>, specify the default password, which 
                     is <i>blank</i>. Otherwise, specify your password.</p>
                  </li>
               </ul>
            </li>
            <li>
               <p>
                  <b>Amazon EBS root volume administrative user</b>
               </p>
               <ul>
                  <li>
                     <p>
                        <b>Username</b> - If you did not change the default 
                     administrative user, specify <code>ec2-user</code>. Otherwise, specify the username 
                     for your administrative user.</p>
                  </li>
                  <li>
                     <p>
                        <b>Password</b> - Specify the password for the 
                     administrative user.</p>
                  </li>
               </ul>
            </li>
         </ul>
         <p>The credentials must be specified in the following JSON format:</p>
         <p>
            <code>{
  "internalDiskPassword":"<i>internal-disk-admin_password</i>",
  "rootVolumeUsername":"<i>root-volume-admin_username</i>",
  "rootVolumepassword":"<i>root-volume-admin_password</i>"
}</code>
         </p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p> |
| `mac_system_integrity_protection_status` | String | ✅ | <p>Specifies the overall SIP status for the instance. To enable all SIP settings, specify 
         <code>enabled</code>. To disable all SIP settings, specify <code>disabled</code>.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>Specifies tags to apply to the SIP modification task.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create mac_system_integrity_protection_modification_task
mac_system_integrity_protection_modification_task = provider.ec2.Mac_system_integrity_protection_modification_task {
    instance_id = "value"  # <p>The ID of the Amazon EC2 Mac instance.</p>
    mac_system_integrity_protection_status = "value"  # <p>Specifies the overall SIP status for the instance. To enable all SIP settings, specify 
         <code>enabled</code>. To disable all SIP settings, specify <code>disabled</code>.</p>
}

```

---


### Capacity_reservation_fleets

CapacityReservationFleets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `capacity_reservation_fleets` | Vec<String> | <p>Information about the Capacity Reservation Fleets.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_reservation_fleets outputs
capacity_reservation_fleets_id = capacity_reservation_fleets.id
capacity_reservation_fleets_next_token = capacity_reservation_fleets.next_token
capacity_reservation_fleets_capacity_reservation_fleets = capacity_reservation_fleets.capacity_reservation_fleets
```

---


### Transit_gateway_multicast_domain

TransitGatewayMulticastDomain resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transit_gateway_id` | String | ✅ | <p>The ID of the transit gateway.</p> |
| `options` | String |  | <p>The options for the transit gateway multicast domain.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags for the transit gateway multicast domain.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_multicast_domain
transit_gateway_multicast_domain = provider.ec2.Transit_gateway_multicast_domain {
    transit_gateway_id = "value"  # <p>The ID of the transit gateway.</p>
}

```

---


### Fpga_image_attribute

FpgaImageAttribute resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fpga_image_attribute` | String | <p>Information about the attribute.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fpga_image_attribute outputs
fpga_image_attribute_id = fpga_image_attribute.id
fpga_image_attribute_fpga_image_attribute = fpga_image_attribute.fpga_image_attribute
```

---


### Mac_modification_tasks

MacModificationTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `mac_modification_tasks` | Vec<String> | <p>Information about the tasks.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mac_modification_tasks outputs
mac_modification_tasks_id = mac_modification_tasks.id
mac_modification_tasks_next_token = mac_modification_tasks.next_token
mac_modification_tasks_mac_modification_tasks = mac_modification_tasks.mac_modification_tasks
```

---


### Vpc_endpoints

VpcEndpoints resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p> |
| `vpc_endpoints` | Vec<String> | <p>Information about the VPC endpoints.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_endpoints outputs
vpc_endpoints_id = vpc_endpoints.id
vpc_endpoints_next_token = vpc_endpoints.next_token
vpc_endpoints_vpc_endpoints = vpc_endpoints.vpc_endpoints
```

---


### Network_insights_access_scope_analyses

NetworkInsightsAccessScopeAnalyses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network_insights_access_scope_analyses` | Vec<String> | <p>The Network Access Scope analyses.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_insights_access_scope_analyses outputs
network_insights_access_scope_analyses_id = network_insights_access_scope_analyses.id
network_insights_access_scope_analyses_network_insights_access_scope_analyses = network_insights_access_scope_analyses.network_insights_access_scope_analyses
network_insights_access_scope_analyses_next_token = network_insights_access_scope_analyses.next_token
```

---


### Fpga_images

FpgaImages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fpga_images` | Vec<String> | <p>Information about the FPGA images.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fpga_images outputs
fpga_images_id = fpga_images.id
fpga_images_fpga_images = fpga_images.fpga_images
fpga_images_next_token = fpga_images.next_token
```

---


### Traffic_mirror_sessions

TrafficMirrorSessions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `traffic_mirror_sessions` | Vec<String> | <p>Describes one or more Traffic Mirror sessions. By default, all Traffic Mirror sessions are described. Alternatively, you can filter the results.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. The value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access traffic_mirror_sessions outputs
traffic_mirror_sessions_id = traffic_mirror_sessions.id
traffic_mirror_sessions_traffic_mirror_sessions = traffic_mirror_sessions.traffic_mirror_sessions
traffic_mirror_sessions_next_token = traffic_mirror_sessions.next_token
```

---


### Vpc_endpoint_connections

VpcEndpointConnections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_endpoint_connections` | Vec<String> | <p>Information about the VPC endpoint connections.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_endpoint_connections outputs
vpc_endpoint_connections_id = vpc_endpoint_connections.id
vpc_endpoint_connections_vpc_endpoint_connections = vpc_endpoint_connections.vpc_endpoint_connections
vpc_endpoint_connections_next_token = vpc_endpoint_connections.next_token
```

---


### Coip_pools

CoipPools resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `coip_pools` | Vec<String> | <p>Information about the address pools.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access coip_pools outputs
coip_pools_id = coip_pools.id
coip_pools_coip_pools = coip_pools.coip_pools
coip_pools_next_token = coip_pools.next_token
```

---


### Vpc_endpoint_service_permissions

VpcEndpointServicePermissions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `allowed_principals` | Vec<String> | <p>Information about the allowed principals.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_endpoint_service_permissions outputs
vpc_endpoint_service_permissions_id = vpc_endpoint_service_permissions.id
vpc_endpoint_service_permissions_next_token = vpc_endpoint_service_permissions.next_token
vpc_endpoint_service_permissions_allowed_principals = vpc_endpoint_service_permissions.allowed_principals
```

---


### Fast_launch_images

FastLaunchImages resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `fast_launch_images` | Vec<String> | <p>A collection of details about the fast-launch enabled Windows images that meet the
      requested criteria.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fast_launch_images outputs
fast_launch_images_id = fast_launch_images.id
fast_launch_images_next_token = fast_launch_images.next_token
fast_launch_images_fast_launch_images = fast_launch_images.fast_launch_images
```

---


### Instance_event_window

InstanceEventWindow resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cron_expression` | String |  | <p>The cron expression for the event window, for example, <code>* 0-4,20-23 * * 1,5</code>.
         If you specify a cron expression, you can't specify a time range.</p>
         <p>Constraints:</p>
         <ul>
            <li>
               <p>Only hour and day of the week values are supported.</p>
            </li>
            <li>
               <p>For day of the week values, you can specify either integers <code>0</code> through
                  <code>6</code>, or alternative single values <code>SUN</code> through
                  <code>SAT</code>.</p>
            </li>
            <li>
               <p>The minute, month, and year must be specified by <code>*</code>.</p>
            </li>
            <li>
               <p>The hour value must be one or a multiple range, for example, <code>0-4</code> or
                  <code>0-4,20-23</code>.</p>
            </li>
            <li>
               <p>Each hour range must be >= 2 hours, for example, <code>0-2</code> or
                  <code>20-23</code>.</p>
            </li>
            <li>
               <p>The event window must be >= 4 hours. The combined total time ranges in the event
               window must be >= 4 hours.</p>
            </li>
         </ul>
         <p>For more information about cron expressions, see <a href="https://en.wikipedia.org/wiki/Cron">cron</a> on the <i>Wikipedia
            website</i>.</p> |
| `name` | String |  | <p>The name of the event window.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the event window.</p> |
| `time_ranges` | Vec<String> |  | <p>The time range for the event window. If you specify a time range, you can't specify a
         cron expression.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance_event_window
instance_event_window = provider.ec2.Instance_event_window {
}

```

---


### Ebs_encryption_by_default

EbsEncryptionByDefault resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sse_type` | String | <p>Reserved for future use.</p> |
| `ebs_encryption_by_default` | bool | <p>Indicates whether encryption by default is enabled.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ebs_encryption_by_default outputs
ebs_encryption_by_default_id = ebs_encryption_by_default.id
ebs_encryption_by_default_sse_type = ebs_encryption_by_default.sse_type
ebs_encryption_by_default_ebs_encryption_by_default = ebs_encryption_by_default.ebs_encryption_by_default
```

---


### Launch_template_version

LaunchTemplateVersion resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `launch_template_id` | String |  | <p>The ID of the launch template.</p>
         <p>You must specify either the launch template ID or the launch template name, but not
            both.</p> |
| `source_version` | String |  | <p>The version of the launch template on which to base the new version. Snapshots applied
            to the block device mapping are ignored when creating a new version unless they are
            explicitly included.</p>
         <p>If you specify this parameter, the new version inherits the launch parameters from the
            source version. If you specify additional launch parameters for the new version, they
            overwrite any corresponding launch parameters inherited from the source version.</p>
         <p>If you omit this parameter, the new version contains only the launch parameters that
            you specify for the new version.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the
            request. If a client token isn't specified, a randomly generated token is used in the
            request to ensure idempotency.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring
                idempotency</a>.</p>
         <p>Constraint: Maximum 128 ASCII characters.</p> |
| `launch_template_data` | String | ✅ | <p>The information for the launch template.</p> |
| `version_description` | String |  | <p>A description for the version of the launch template.</p> |
| `resolve_alias` | bool |  | <p>If <code>true</code>, and if a Systems Manager parameter is specified for
                <code>ImageId</code>, the AMI ID is displayed in the response for
                <code>imageID</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/create-launch-template.html#use-an-ssm-parameter-instead-of-an-ami-id">Use a Systems Manager parameter instead of an AMI ID</a> in the
                <i>Amazon EC2 User Guide</i>.</p>
         <p>Default: <code>false</code>
         </p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually
            making the request, and provides an error response. If you have the required
            permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is
                <code>UnauthorizedOperation</code>.</p> |
| `launch_template_name` | String |  | <p>The name of the launch template.</p>
         <p>You must specify either the launch template ID or the launch template name, but not
            both.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create launch_template_version
launch_template_version = provider.ec2.Launch_template_version {
    launch_template_data = "value"  # <p>The information for the launch template.</p>
}

```

---


### Transit_gateway_peering_attachment

TransitGatewayPeeringAttachment resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `options` | String |  | <p>Requests a transit gateway peering attachment.</p> |
| `peer_transit_gateway_id` | String | ✅ | <p>The ID of the peer transit gateway with which to create the peering attachment.</p> |
| `peer_region` | String | ✅ | <p>The Region where the peer transit gateway is located.</p> |
| `peer_account_id` | String | ✅ | <p>The ID of the Amazon Web Services account that owns the peer transit gateway.</p> |
| `transit_gateway_id` | String | ✅ | <p>The ID of the transit gateway.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the transit gateway peering attachment.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_peering_attachment
transit_gateway_peering_attachment = provider.ec2.Transit_gateway_peering_attachment {
    peer_transit_gateway_id = "value"  # <p>The ID of the peer transit gateway with which to create the peering attachment.</p>
    peer_region = "value"  # <p>The Region where the peer transit gateway is located.</p>
    peer_account_id = "value"  # <p>The ID of the Amazon Web Services account that owns the peer transit gateway.</p>
    transit_gateway_id = "value"  # <p>The ID of the transit gateway.</p>
}

```

---


### Instance_event_windows

InstanceEventWindows resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_event_windows` | Vec<String> | <p>Information about the event windows.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code>
         when there are no more results to return. </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_event_windows outputs
instance_event_windows_id = instance_event_windows.id
instance_event_windows_instance_event_windows = instance_event_windows.instance_event_windows
instance_event_windows_next_token = instance_event_windows.next_token
```

---


### Vpc_endpoint_associations

VpcEndpointAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_endpoint_associations` | Vec<String> | <p>Details of the endpoint associations.</p> |
| `next_token` | String | <p>The pagination token.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_endpoint_associations outputs
vpc_endpoint_associations_id = vpc_endpoint_associations.id
vpc_endpoint_associations_vpc_endpoint_associations = vpc_endpoint_associations.vpc_endpoint_associations
vpc_endpoint_associations_next_token = vpc_endpoint_associations.next_token
```

---


### Declarative_policies_reports

DeclarativePoliciesReports resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reports` | Vec<String> | <p>The report metadata.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access declarative_policies_reports outputs
declarative_policies_reports_id = declarative_policies_reports.id
declarative_policies_reports_reports = declarative_policies_reports.reports
declarative_policies_reports_next_token = declarative_policies_reports.next_token
```

---


### Traffic_mirror_filter_rules

TrafficMirrorFilterRules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `traffic_mirror_filter_rules` | Vec<String> | <p>Traffic mirror rules.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. The value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access traffic_mirror_filter_rules outputs
traffic_mirror_filter_rules_id = traffic_mirror_filter_rules.id
traffic_mirror_filter_rules_traffic_mirror_filter_rules = traffic_mirror_filter_rules.traffic_mirror_filter_rules
traffic_mirror_filter_rules_next_token = traffic_mirror_filter_rules.next_token
```

---


### Tags

Tags resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resources` | Vec<String> | ✅ | <p>The IDs of the resources, separated by spaces.</p>
         <p>Constraints: Up to 1000 resource IDs. We recommend breaking up this request into smaller batches.</p> |
| `tags` | Vec<String> | ✅ | <p>The tags. The <code>value</code> parameter is required, but if you don't want the tag to have a value,
        specify the parameter with no value, and we set the value to an empty string.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
         This value is <code>null</code> when there are no more items to return.</p> |
| `tags` | Vec<String> | <p>The tags.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create tags
tags = provider.ec2.Tags {
    resources = "value"  # <p>The IDs of the resources, separated by spaces.</p>
         <p>Constraints: Up to 1000 resource IDs. We recommend breaking up this request into smaller batches.</p>
    tags = "value"  # <p>The tags. The <code>value</code> parameter is required, but if you don't want the tag to have a value,
        specify the parameter with no value, and we set the value to an empty string.</p>
}

# Access tags outputs
tags_id = tags.id
tags_next_token = tags.next_token
tags_tags = tags.tags
```

---


### Image_attribute

ImageAttribute resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_launched_time` | String | <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time
        format</a>, when the AMI was last used to launch an EC2 instance. When the AMI is used
      to launch an instance, there is a 24-hour delay before that usage is reported.</p>
         <note>
            <p>
               <code>lastLaunchedTime</code> data is available starting April 2017.</p>
         </note> |
| `tpm_support` | String | <p>If the image is configured for NitroTPM support, the value is <code>v2.0</code>.</p> |
| `deregistration_protection` | String | <p>Indicates whether deregistration protection is enabled for the AMI.</p> |
| `launch_permissions` | Vec<String> | <p>The launch permissions.</p> |
| `kernel_id` | String | <p>The kernel ID.</p> |
| `sriov_net_support` | String | <p>Indicates whether enhanced networking with the Intel 82599 Virtual Function interface is
      enabled.</p> |
| `uefi_data` | String | <p>Base64 representation of the non-volatile UEFI variable store. To retrieve the UEFI data,
      use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetInstanceUefiData">GetInstanceUefiData</a> command. You can inspect and modify the UEFI data by using the
        <a href="https://github.com/awslabs/python-uefivars">python-uefivars tool</a> on
      GitHub. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/uefi-secure-boot.html">UEFI Secure Boot for Amazon EC2
        instances</a> in the <i>Amazon EC2 User Guide</i>.</p> |
| `product_codes` | Vec<String> | <p>The product codes.</p> |
| `block_device_mappings` | Vec<String> | <p>The block device mapping entries.</p> |
| `description` | String | <p>A description for the AMI.</p> |
| `boot_mode` | String | <p>The boot mode.</p> |
| `ramdisk_id` | String | <p>The RAM disk ID.</p> |
| `imds_support` | String | <p>If <code>v2.0</code>, it indicates that IMDSv2 is specified in the AMI. Instances launched
      from this AMI will have <code>HttpTokens</code> automatically set to <code>required</code> so
      that, by default, the instance requires that IMDSv2 is used when requesting instance metadata.
      In addition, <code>HttpPutResponseHopLimit</code> is set to <code>2</code>. For more
      information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration">Configure the AMI</a> in the <i>Amazon EC2 User Guide</i>.</p> |
| `image_id` | String | <p>The ID of the AMI.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_attribute outputs
image_attribute_id = image_attribute.id
image_attribute_last_launched_time = image_attribute.last_launched_time
image_attribute_tpm_support = image_attribute.tpm_support
image_attribute_deregistration_protection = image_attribute.deregistration_protection
image_attribute_launch_permissions = image_attribute.launch_permissions
image_attribute_kernel_id = image_attribute.kernel_id
image_attribute_sriov_net_support = image_attribute.sriov_net_support
image_attribute_uefi_data = image_attribute.uefi_data
image_attribute_product_codes = image_attribute.product_codes
image_attribute_block_device_mappings = image_attribute.block_device_mappings
image_attribute_description = image_attribute.description
image_attribute_boot_mode = image_attribute.boot_mode
image_attribute_ramdisk_id = image_attribute.ramdisk_id
image_attribute_imds_support = image_attribute.imds_support
image_attribute_image_id = image_attribute.image_id
```

---


### Verified_access_instances

VerifiedAccessInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `verified_access_instances` | Vec<String> | <p>Details about the Verified Access instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access verified_access_instances outputs
verified_access_instances_id = verified_access_instances.id
verified_access_instances_next_token = verified_access_instances.next_token
verified_access_instances_verified_access_instances = verified_access_instances.verified_access_instances
```

---


### Allowed_images_settings

AllowedImagesSettings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | <p>The current state of the Allowed AMIs setting at the account level in the specified Amazon Web Services
      Region.</p>
         <p>Possible values:</p>
         <ul>
            <li>
               <p>
                  <code>disabled</code>: All AMIs are allowed.</p>
            </li>
            <li>
               <p>
                  <code>audit-mode</code>: All AMIs are allowed, but the <code>ImageAllowed</code> field
          is set to <code>true</code> if the AMI would be allowed with the current list of criteria
          if allowed AMIs was enabled.</p>
            </li>
            <li>
               <p>
                  <code>enabled</code>: Only AMIs matching the image criteria are discoverable and
          available for use.</p>
            </li>
         </ul> |
| `image_criteria` | Vec<String> | <p>The list of criteria for images that are discoverable and usable in the account in the
      specified Amazon Web Services Region.</p> |
| `managed_by` | String | <p>The entity that manages the Allowed AMIs settings. Possible values include:</p>
         <ul>
            <li>
               <p>
                  <code>account</code> - The Allowed AMIs settings is managed by the account.</p>
            </li>
            <li>
               <p>
                  <code>declarative-policy</code> - The Allowed AMIs settings is managed by a
                    declarative policy and can't be modified by the account.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access allowed_images_settings outputs
allowed_images_settings_id = allowed_images_settings.id
allowed_images_settings_state = allowed_images_settings.state
allowed_images_settings_image_criteria = allowed_images_settings.image_criteria
allowed_images_settings_managed_by = allowed_images_settings.managed_by
```

---


### Declarative_policies_report_summary

DeclarativePoliciesReportSummary resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | <p>The time when the report generation started.</p> |
| `end_time` | String | <p>The time when the report generation ended.</p> |
| `number_of_accounts` | i64 | <p>The total number of accounts associated with the specified
            <code>targetId</code>.</p> |
| `number_of_failed_accounts` | i64 | <p>The number of accounts where attributes could not be retrieved in any Region.</p> |
| `report_id` | String | <p>The ID of the report.</p> |
| `attribute_summaries` | Vec<String> | <p>The attributes described in the report.</p> |
| `target_id` | String | <p>The root ID, organizational unit ID, or account ID.</p>
         <p>Format:</p>
         <ul>
            <li>
               <p>For root: <code>r-ab12</code>
               </p>
            </li>
            <li>
               <p>For OU: <code>ou-ab12-cdef1234</code>
               </p>
            </li>
            <li>
               <p>For account: <code>123456789012</code>
               </p>
            </li>
         </ul> |
| `s3_prefix` | String | <p>The prefix for your S3 object.</p> |
| `s3_bucket` | String | <p>The name of the Amazon S3 bucket where the report is located.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access declarative_policies_report_summary outputs
declarative_policies_report_summary_id = declarative_policies_report_summary.id
declarative_policies_report_summary_start_time = declarative_policies_report_summary.start_time
declarative_policies_report_summary_end_time = declarative_policies_report_summary.end_time
declarative_policies_report_summary_number_of_accounts = declarative_policies_report_summary.number_of_accounts
declarative_policies_report_summary_number_of_failed_accounts = declarative_policies_report_summary.number_of_failed_accounts
declarative_policies_report_summary_report_id = declarative_policies_report_summary.report_id
declarative_policies_report_summary_attribute_summaries = declarative_policies_report_summary.attribute_summaries
declarative_policies_report_summary_target_id = declarative_policies_report_summary.target_id
declarative_policies_report_summary_s3_prefix = declarative_policies_report_summary.s3_prefix
declarative_policies_report_summary_s3_bucket = declarative_policies_report_summary.s3_bucket
```

---


### Transit_gateway_attachment_propagations

TransitGatewayAttachmentPropagations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `transit_gateway_attachment_propagations` | Vec<String> | <p>Information about the propagation route tables.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_attachment_propagations outputs
transit_gateway_attachment_propagations_id = transit_gateway_attachment_propagations.id
transit_gateway_attachment_propagations_next_token = transit_gateway_attachment_propagations.next_token
transit_gateway_attachment_propagations_transit_gateway_attachment_propagations = transit_gateway_attachment_propagations.transit_gateway_attachment_propagations
```

---


### Instance_status

InstanceStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_statuses` | Vec<String> | <p>Information about the status of the instances.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_status outputs
instance_status_id = instance_status.id
instance_status_instance_statuses = instance_status.instance_statuses
instance_status_next_token = instance_status.next_token
```

---


### Vpn_gateway

VpnGateway resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the virtual private gateway.</p> |
| `type` | String | ✅ | <p>The type of VPN connection this virtual private gateway supports.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually
            making the request, and provides an error response. If you have the required
            permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is
                <code>UnauthorizedOperation</code>.</p> |
| `amazon_side_asn` | i64 |  | <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. If
            you're using a 16-bit ASN, it must be in the 64512 to 65534 range. If you're using a
            32-bit ASN, it must be in the 4200000000 to 4294967294 range.</p>
         <p>Default: 64512</p> |
| `availability_zone` | String |  | <p>The Availability Zone for the virtual private gateway.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpn_gateway
vpn_gateway = provider.ec2.Vpn_gateway {
    type = "value"  # <p>The type of VPN connection this virtual private gateway supports.</p>
}

```

---


### Network_interface_permission

NetworkInterfacePermission resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network_interface_id` | String | ✅ | <p>The ID of the network interface.</p> |
| `aws_account_id` | String |  | <p>The Amazon Web Services account ID.</p> |
| `aws_service` | String |  | <p>The Amazon Web Services service. Currently not supported.</p> |
| `permission` | String | ✅ | <p>The type of permission to grant.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually
            making the request, and provides an error response. If you have the required
            permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is
                <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create network_interface_permission
network_interface_permission = provider.ec2.Network_interface_permission {
    network_interface_id = "value"  # <p>The ID of the network interface.</p>
    permission = "value"  # <p>The type of permission to grant.</p>
}

```

---


### Volumes_modifications

VolumesModifications resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `volumes_modifications` | Vec<String> | <p>Information about the volume modifications.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
  This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access volumes_modifications outputs
volumes_modifications_id = volumes_modifications.id
volumes_modifications_volumes_modifications = volumes_modifications.volumes_modifications
volumes_modifications_next_token = volumes_modifications.next_token
```

---


### Instance_uefi_data

InstanceUefiData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_id` | String | <p>The ID of the instance from which to retrieve the UEFI data.</p> |
| `uefi_data` | String | <p>Base64 representation of the non-volatile UEFI variable store.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_uefi_data outputs
instance_uefi_data_id = instance_uefi_data.id
instance_uefi_data_instance_id = instance_uefi_data.instance_id
instance_uefi_data_uefi_data = instance_uefi_data.uefi_data
```

---


### Carrier_gateway

CarrierGateway resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_specifications` | Vec<String> |  | <p>The tags to associate with the carrier gateway.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `vpc_id` | String | ✅ | <p>The ID of the VPC to associate with the carrier gateway.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure
                idempotency</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create carrier_gateway
carrier_gateway = provider.ec2.Carrier_gateway {
    vpc_id = "value"  # <p>The ID of the VPC to associate with the carrier gateway.</p>
}

```

---


### Launch_template

LaunchTemplate resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the launch template on creation. To tag the launch template, the
            resource type must be <code>launch-template</code>.</p>
         <p>To specify the tags for the resources that are created when an instance is launched,
            you must use the <code>TagSpecifications</code> parameter in the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestLaunchTemplateData.html">launch template
                data</a> structure.</p> |
| `launch_template_name` | String | ✅ | <p>A name for the launch template.</p> |
| `version_description` | String |  | <p>A description for the first version of the launch template.</p> |
| `launch_template_data` | String | ✅ | <p>The information for the launch template.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually
            making the request, and provides an error response. If you have the required
            permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is
                <code>UnauthorizedOperation</code>.</p> |
| `operator` | String |  | <p>Reserved for internal use.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the
            request. If a client token isn't specified, a randomly generated token is used in the
            request to ensure idempotency.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring
                idempotency</a>.</p>
         <p>Constraint: Maximum 128 ASCII characters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create launch_template
launch_template = provider.ec2.Launch_template {
    launch_template_name = "value"  # <p>A name for the launch template.</p>
    launch_template_data = "value"  # <p>The information for the launch template.</p>
}

```

---


### Associated_enclave_certificate_iam_roles

AssociatedEnclaveCertificateIamRoles resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associated_roles` | Vec<String> | <p>Information about the associated IAM roles.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access associated_enclave_certificate_iam_roles outputs
associated_enclave_certificate_iam_roles_id = associated_enclave_certificate_iam_roles.id
associated_enclave_certificate_iam_roles_associated_roles = associated_enclave_certificate_iam_roles.associated_roles
```

---


### Verified_access_endpoint_policy

VerifiedAccessEndpointPolicy resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_document` | String | <p>The Verified Access policy document.</p> |
| `policy_enabled` | bool | <p>The status of the Verified Access policy.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access verified_access_endpoint_policy outputs
verified_access_endpoint_policy_id = verified_access_endpoint_policy.id
verified_access_endpoint_policy_policy_document = verified_access_endpoint_policy.policy_document
verified_access_endpoint_policy_policy_enabled = verified_access_endpoint_policy.policy_enabled
```

---


### Key_pair

KeyPair resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key_name` | String | ✅ | <p>A unique name for the key pair.</p>
         <p>Constraints: Up to 255 ASCII characters</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the new key pair.</p> |
| `key_type` | String |  | <p>The type of key pair. Note that ED25519 keys are not supported for Windows instances.</p>
         <p>Default: <code>rsa</code>
         </p> |
| `key_format` | String |  | <p>The format of the key pair.</p>
         <p>Default: <code>pem</code>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create key_pair
key_pair = provider.ec2.Key_pair {
    key_name = "value"  # <p>A unique name for the key pair.</p>
         <p>Constraints: Up to 255 ASCII characters</p>
}

```

---


### Conversion_tasks

ConversionTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conversion_tasks` | Vec<String> | <p>Information about the conversion tasks.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access conversion_tasks outputs
conversion_tasks_id = conversion_tasks.id
conversion_tasks_conversion_tasks = conversion_tasks.conversion_tasks
```

---


### Local_gateway_route_tables

LocalGatewayRouteTables resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `local_gateway_route_tables` | Vec<String> | <p>Information about the local gateway route tables.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access local_gateway_route_tables outputs
local_gateway_route_tables_id = local_gateway_route_tables.id
local_gateway_route_tables_next_token = local_gateway_route_tables.next_token
local_gateway_route_tables_local_gateway_route_tables = local_gateway_route_tables.local_gateway_route_tables
```

---


### Capacity_block_extension_history

CapacityBlockExtensionHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `capacity_block_extensions` | Vec<String> | <p>Describes one or more of your Capacity Block extensions. The results describe only the
			Capacity Block extensions in the Amazon Web Services Region that you're currently using.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_block_extension_history outputs
capacity_block_extension_history_id = capacity_block_extension_history.id
capacity_block_extension_history_next_token = capacity_block_extension_history.next_token
capacity_block_extension_history_capacity_block_extensions = capacity_block_extension_history.capacity_block_extensions
```

---


### Outpost_lags

OutpostLags resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `outpost_lags` | Vec<String> | <p>The Outpost LAGs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access outpost_lags outputs
outpost_lags_id = outpost_lags.id
outpost_lags_next_token = outpost_lags.next_token
outpost_lags_outpost_lags = outpost_lags.outpost_lags
```

---


### Instance_tpm_ek_pub

InstanceTpmEkPub resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_value` | String | <p>The public endorsement key material.</p> |
| `key_format` | String | <p>The public endorsement key format.</p> |
| `key_type` | String | <p>The public endorsement key type.</p> |
| `instance_id` | String | <p>The ID of the instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_tpm_ek_pub outputs
instance_tpm_ek_pub_id = instance_tpm_ek_pub.id
instance_tpm_ek_pub_key_value = instance_tpm_ek_pub.key_value
instance_tpm_ek_pub_key_format = instance_tpm_ek_pub.key_format
instance_tpm_ek_pub_key_type = instance_tpm_ek_pub.key_type
instance_tpm_ek_pub_instance_id = instance_tpm_ek_pub.instance_id
```

---


### Moving_addresses

MovingAddresses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `moving_address_statuses` | Vec<String> | <p>The status for each Elastic IP address.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access moving_addresses outputs
moving_addresses_id = moving_addresses.id
moving_addresses_moving_address_statuses = moving_addresses.moving_address_statuses
moving_addresses_next_token = moving_addresses.next_token
```

---


### Volume_attribute

VolumeAttribute resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_enable_io` | String | <p>The state of <code>autoEnableIO</code> attribute.</p> |
| `product_codes` | Vec<String> | <p>A list of product codes.</p> |
| `volume_id` | String | <p>The ID of the volume.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access volume_attribute outputs
volume_attribute_id = volume_attribute.id
volume_attribute_auto_enable_io = volume_attribute.auto_enable_io
volume_attribute_product_codes = volume_attribute.product_codes
volume_attribute_volume_id = volume_attribute.volume_id
```

---


### Ipam_discovered_public_addresses

IpamDiscoveredPublicAddresses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipam_discovered_public_addresses` | Vec<String> | <p>IPAM discovered public addresses.</p> |
| `oldest_sample_time` | String | <p>The oldest successful resource discovery time.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_discovered_public_addresses outputs
ipam_discovered_public_addresses_id = ipam_discovered_public_addresses.id
ipam_discovered_public_addresses_ipam_discovered_public_addresses = ipam_discovered_public_addresses.ipam_discovered_public_addresses
ipam_discovered_public_addresses_oldest_sample_time = ipam_discovered_public_addresses.oldest_sample_time
ipam_discovered_public_addresses_next_token = ipam_discovered_public_addresses.next_token
```

---


### Capacity_manager_metric_data

CapacityManagerMetricData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric_data_results` | Vec<String> | <p>
The metric data points returned by the query. Each result contains dimension values, timestamp, and metric values with their associated statistics.
</p> |
| `next_token` | String | <p>
The token to use to retrieve the next page of results. This value is null when there are no more results to return.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_manager_metric_data outputs
capacity_manager_metric_data_id = capacity_manager_metric_data.id
capacity_manager_metric_data_metric_data_results = capacity_manager_metric_data.metric_data_results
capacity_manager_metric_data_next_token = capacity_manager_metric_data.next_token
```

---


### Transit_gateway_peering_attachments

TransitGatewayPeeringAttachments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_peering_attachments` | Vec<String> | <p>The transit gateway peering attachments.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_peering_attachments outputs
transit_gateway_peering_attachments_id = transit_gateway_peering_attachments.id
transit_gateway_peering_attachments_transit_gateway_peering_attachments = transit_gateway_peering_attachments.transit_gateway_peering_attachments
transit_gateway_peering_attachments_next_token = transit_gateway_peering_attachments.next_token
```

---


### Security_group

SecurityGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | ✅ | <p>A description for the security group.</p>
         <p>Constraints: Up to 255 characters in length</p>
         <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&;{}!$*</p> |
| `group_name` | String | ✅ | <p>The name of the security group. Names are case-insensitive and must be unique within the VPC.</p>
         <p>Constraints: Up to 255 characters in length. Can't start with <code>sg-</code>.</p>
         <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&;{}!$*</p> |
| `vpc_id` | String |  | <p>The ID of the VPC. Required for a nondefault VPC.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the security group.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create security_group
security_group = provider.ec2.Security_group {
    description = "value"  # <p>A description for the security group.</p>
         <p>Constraints: Up to 255 characters in length</p>
         <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&;{}!$*</p>
    group_name = "value"  # <p>The name of the security group. Names are case-insensitive and must be unique within the VPC.</p>
         <p>Constraints: Up to 255 characters in length. Can't start with <code>sg-</code>.</p>
         <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&;{}!$*</p>
}

```

---


### Capacity_manager_data_export

CapacityManagerDataExport resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_format` | String | ✅ | <p>
The file format for the exported data. Parquet format is recommended for large datasets and better compression.
</p> |
| `dry_run` | bool |  | <p>
Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. 
If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.
</p> |
| `client_token` | String |  | <p>
Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see Ensure Idempotency.
</p> |
| `s3_bucket_name` | String | ✅ | <p>
The name of the S3 bucket where the capacity data export files will be delivered. The bucket must exist and you must have write permissions to it.
</p> |
| `tag_specifications` | Vec<String> |  | <p>
The tags to apply to the data export configuration. You can tag the export for organization and cost tracking purposes.
</p> |
| `s3_bucket_prefix` | String |  | <p>
The S3 key prefix for the exported data files. This allows you to organize exports in a specific folder structure within your bucket. If not specified, files are placed at the bucket root.
</p> |
| `schedule` | String | ✅ | <p>
The frequency at which data exports are generated. 
</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create capacity_manager_data_export
capacity_manager_data_export = provider.ec2.Capacity_manager_data_export {
    output_format = "value"  # <p>
The file format for the exported data. Parquet format is recommended for large datasets and better compression.
</p>
    s3_bucket_name = "value"  # <p>
The name of the S3 bucket where the capacity data export files will be delivered. The bucket must exist and you must have write permissions to it.
</p>
    schedule = "value"  # <p>
The frequency at which data exports are generated. 
</p>
}

```

---


### Transit_gateway_connect_peer

TransitGatewayConnectPeer resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `peer_address` | String | ✅ | <p>The peer IP address (GRE outer IP address) on the appliance side of the Connect peer.</p> |
| `bgp_options` | String |  | <p>The BGP options for the Connect peer.</p> |
| `inside_cidr_blocks` | Vec<String> | ✅ | <p>The range of inside IP addresses that are used for BGP peering. You must specify a
            size /29 IPv4 CIDR block from the <code>169.254.0.0/16</code> range. The first address
            from the range must be configured on the appliance as the BGP IP address. You can also
            optionally specify a size /125 IPv6 CIDR block from the <code>fd00::/8</code>
            range.</p> |
| `transit_gateway_attachment_id` | String | ✅ | <p>The ID of the Connect attachment.</p> |
| `transit_gateway_address` | String |  | <p>The peer IP address (GRE outer IP address) on the transit gateway side of the Connect peer, which must be
            specified from a transit gateway CIDR block. If not specified, Amazon automatically assigns
            the first available IP address from the transit gateway CIDR block.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the Connect peer.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_connect_peer
transit_gateway_connect_peer = provider.ec2.Transit_gateway_connect_peer {
    peer_address = "value"  # <p>The peer IP address (GRE outer IP address) on the appliance side of the Connect peer.</p>
    inside_cidr_blocks = "value"  # <p>The range of inside IP addresses that are used for BGP peering. You must specify a
            size /29 IPv4 CIDR block from the <code>169.254.0.0/16</code> range. The first address
            from the range must be configured on the appliance as the BGP IP address. You can also
            optionally specify a size /125 IPv6 CIDR block from the <code>fd00::/8</code>
            range.</p>
    transit_gateway_attachment_id = "value"  # <p>The ID of the Connect attachment.</p>
}

```

---


### Verified_access_endpoint

VerifiedAccessEndpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description for the Verified Access endpoint.</p> |
| `sse_specification` | String |  | <p>The options for server side encryption.</p> |
| `endpoint_domain_prefix` | String |  | <p>A custom identifier that is prepended to the DNS name that is generated for the
         endpoint.</p> |
| `network_interface_options` | String |  | <p>The network interface details. This parameter is required if the endpoint type is
            <code>network-interface</code>.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `policy_document` | String |  | <p>The Verified Access policy document.</p> |
| `rds_options` | String |  | <p>The RDS details. This parameter is required if the endpoint type is <code>rds</code>.</p> |
| `domain_certificate_arn` | String |  | <p>The ARN of the public TLS/SSL certificate in Amazon Web Services Certificate Manager to associate with the endpoint.
         The CN in the certificate must match the DNS name your end users will use to reach your
         application.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the Verified Access endpoint.</p> |
| `attachment_type` | String | ✅ | <p>The type of attachment.</p> |
| `cidr_options` | String |  | <p>The CIDR options. This parameter is required if the endpoint type is <code>cidr</code>.</p> |
| `endpoint_type` | String | ✅ | <p>The type of Verified Access endpoint to create.</p> |
| `security_group_ids` | Vec<String> |  | <p>The IDs of the security groups to associate with the Verified Access endpoint. Required if <code>AttachmentType</code> is set to <code>vpc</code>.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive token that you provide to ensure idempotency of your
            modification request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `verified_access_group_id` | String | ✅ | <p>The ID of the Verified Access group to associate the endpoint with.</p> |
| `application_domain` | String |  | <p>The DNS name for users to reach your application.</p> |
| `load_balancer_options` | String |  | <p>The load balancer details. This parameter is required if the endpoint type is
            <code>load-balancer</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create verified_access_endpoint
verified_access_endpoint = provider.ec2.Verified_access_endpoint {
    attachment_type = "value"  # <p>The type of attachment.</p>
    endpoint_type = "value"  # <p>The type of Verified Access endpoint to create.</p>
    verified_access_group_id = "value"  # <p>The ID of the Verified Access group to associate the endpoint with.</p>
}

```

---


### Network_interfaces

NetworkInterfaces resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is
                <code>null</code> when there are no more items to return.</p> |
| `network_interfaces` | Vec<String> | <p>Information about the network interfaces.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_interfaces outputs
network_interfaces_id = network_interfaces.id
network_interfaces_next_token = network_interfaces.next_token
network_interfaces_network_interfaces = network_interfaces.network_interfaces
```

---


### Ipam_resource_cidrs

IpamResourceCidrs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `ipam_resource_cidrs` | Vec<String> | <p>The resource CIDRs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_resource_cidrs outputs
ipam_resource_cidrs_id = ipam_resource_cidrs.id
ipam_resource_cidrs_next_token = ipam_resource_cidrs.next_token
ipam_resource_cidrs_ipam_resource_cidrs = ipam_resource_cidrs.ipam_resource_cidrs
```

---


### Egress_only_internet_gateways

EgressOnlyInternetGateways resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |
| `egress_only_internet_gateways` | Vec<String> | <p>Information about the egress-only internet gateways.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access egress_only_internet_gateways outputs
egress_only_internet_gateways_id = egress_only_internet_gateways.id
egress_only_internet_gateways_next_token = egress_only_internet_gateways.next_token
egress_only_internet_gateways_egress_only_internet_gateways = egress_only_internet_gateways.egress_only_internet_gateways
```

---


### Aws_network_performance_metric_subscriptions

AwsNetworkPerformanceMetricSubscriptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `subscriptions` | Vec<String> | <p>Describes the current Infrastructure Performance subscriptions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aws_network_performance_metric_subscriptions outputs
aws_network_performance_metric_subscriptions_id = aws_network_performance_metric_subscriptions.id
aws_network_performance_metric_subscriptions_next_token = aws_network_performance_metric_subscriptions.next_token
aws_network_performance_metric_subscriptions_subscriptions = aws_network_performance_metric_subscriptions.subscriptions
```

---


### Mac_hosts

MacHosts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results.</p> |
| `mac_hosts` | Vec<String> | <p>
            Information about the EC2 Mac Dedicated Hosts.
        </p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access mac_hosts outputs
mac_hosts_id = mac_hosts.id
mac_hosts_next_token = mac_hosts.next_token
mac_hosts_mac_hosts = mac_hosts.mac_hosts
```

---


### Vpc_block_public_access_options

VpcBlockPublicAccessOptions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpc_block_public_access_options` | String | <p>Details related to the options.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_block_public_access_options outputs
vpc_block_public_access_options_id = vpc_block_public_access_options.id
vpc_block_public_access_options_vpc_block_public_access_options = vpc_block_public_access_options.vpc_block_public_access_options
```

---


### Associated_ipv6_pool_cidrs

AssociatedIpv6PoolCidrs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipv6_cidr_associations` | Vec<String> | <p>Information about the IPv6 CIDR block associations.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access associated_ipv6_pool_cidrs outputs
associated_ipv6_pool_cidrs_id = associated_ipv6_pool_cidrs.id
associated_ipv6_pool_cidrs_ipv6_cidr_associations = associated_ipv6_pool_cidrs.ipv6_cidr_associations
associated_ipv6_pool_cidrs_next_token = associated_ipv6_pool_cidrs.next_token
```

---


### Local_gateway_route_table_vpc_associations

LocalGatewayRouteTableVpcAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `local_gateway_route_table_vpc_associations` | Vec<String> | <p>Information about the associations.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access local_gateway_route_table_vpc_associations outputs
local_gateway_route_table_vpc_associations_id = local_gateway_route_table_vpc_associations.id
local_gateway_route_table_vpc_associations_local_gateway_route_table_vpc_associations = local_gateway_route_table_vpc_associations.local_gateway_route_table_vpc_associations
local_gateway_route_table_vpc_associations_next_token = local_gateway_route_table_vpc_associations.next_token
```

---


### Network_acl_entry

NetworkAclEntry resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `protocol` | String | ✅ | <p>The protocol number. A value of "-1" means all protocols. If you specify "-1" or a
  			protocol number other than "6" (TCP), "17" (UDP), or "1" (ICMP), traffic on all ports is 
			allowed, regardless of any ports or ICMP types or codes that you specify. If you specify 
			protocol "58" (ICMPv6) and specify an IPv4 CIDR block, traffic for all ICMP types and 
			codes allowed, regardless of any that you specify. If you specify protocol "58" (ICMPv6) 
			and specify an IPv6 CIDR block, you must specify an ICMP type and code.</p> |
| `rule_action` | String | ✅ | <p>Indicates whether to allow or deny the traffic that matches the rule.</p> |
| `ipv6_cidr_block` | String |  | <p>The IPv6 network range to allow or deny, in CIDR notation (for example
                <code>2001:db8:1234:1a00::/64</code>).</p> |
| `icmp_type_code` | String |  | <p>ICMP protocol: The ICMP or ICMPv6 type and code. Required if specifying protocol 
		        1 (ICMP) or protocol 58 (ICMPv6) with an IPv6 CIDR block.</p> |
| `port_range` | String |  | <p>TCP or UDP protocols: The range of ports the rule applies to.
		        Required if specifying protocol 6 (TCP) or 17 (UDP).</p> |
| `rule_number` | i64 | ✅ | <p>The rule number for the entry (for example, 100). ACL entries are processed in ascending order by rule number.</p>
         <p>Constraints: Positive integer from 1 to 32766. The range 32767 to 65535 is reserved for internal use.</p> |
| `cidr_block` | String |  | <p>The IPv4 network range to allow or deny, in CIDR notation (for example
		        <code>172.16.0.0/24</code>). We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p> |
| `network_acl_id` | String | ✅ | <p>The ID of the network ACL.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `egress` | bool | ✅ | <p>Indicates whether this is an egress rule (rule is applied to traffic leaving the subnet).</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create network_acl_entry
network_acl_entry = provider.ec2.Network_acl_entry {
    protocol = "value"  # <p>The protocol number. A value of "-1" means all protocols. If you specify "-1" or a
  			protocol number other than "6" (TCP), "17" (UDP), or "1" (ICMP), traffic on all ports is 
			allowed, regardless of any ports or ICMP types or codes that you specify. If you specify 
			protocol "58" (ICMPv6) and specify an IPv4 CIDR block, traffic for all ICMP types and 
			codes allowed, regardless of any that you specify. If you specify protocol "58" (ICMPv6) 
			and specify an IPv6 CIDR block, you must specify an ICMP type and code.</p>
    rule_action = "value"  # <p>Indicates whether to allow or deny the traffic that matches the rule.</p>
    rule_number = "value"  # <p>The rule number for the entry (for example, 100). ACL entries are processed in ascending order by rule number.</p>
         <p>Constraints: Positive integer from 1 to 32766. The range 32767 to 65535 is reserved for internal use.</p>
    network_acl_id = "value"  # <p>The ID of the network ACL.</p>
    egress = "value"  # <p>Indicates whether this is an egress rule (rule is applied to traffic leaving the subnet).</p>
}

```

---


### Host_reservation_offerings

HostReservationOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `offering_set` | Vec<String> | <p>Information about the offerings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access host_reservation_offerings outputs
host_reservation_offerings_id = host_reservation_offerings.id
host_reservation_offerings_next_token = host_reservation_offerings.next_token
host_reservation_offerings_offering_set = host_reservation_offerings.offering_set
```

---


### Flow_logs_integration_template

FlowLogsIntegrationTemplate resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `result` | String | <p>The generated CloudFormation template.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access flow_logs_integration_template outputs
flow_logs_integration_template_id = flow_logs_integration_template.id
flow_logs_integration_template_result = flow_logs_integration_template.result
```

---


### Spot_fleet_instances

SpotFleetInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `active_instances` | Vec<String> | <p>The running instances. This list is refreshed periodically and might be out of
            date.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `spot_fleet_request_id` | String | <p>The ID of the Spot Fleet request.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access spot_fleet_instances outputs
spot_fleet_instances_id = spot_fleet_instances.id
spot_fleet_instances_active_instances = spot_fleet_instances.active_instances
spot_fleet_instances_next_token = spot_fleet_instances.next_token
spot_fleet_instances_spot_fleet_request_id = spot_fleet_instances.spot_fleet_request_id
```

---


### Transit_gateway_policy_tables

TransitGatewayPolicyTables resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_policy_tables` | Vec<String> | <p>Describes the transit gateway policy tables.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_policy_tables outputs
transit_gateway_policy_tables_id = transit_gateway_policy_tables.id
transit_gateway_policy_tables_transit_gateway_policy_tables = transit_gateway_policy_tables.transit_gateway_policy_tables
transit_gateway_policy_tables_next_token = transit_gateway_policy_tables.next_token
```

---


### Ipam_pool_allocations

IpamPoolAllocations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `ipam_pool_allocations` | Vec<String> | <p>The IPAM pool allocations you want information on.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_pool_allocations outputs
ipam_pool_allocations_id = ipam_pool_allocations.id
ipam_pool_allocations_next_token = ipam_pool_allocations.next_token
ipam_pool_allocations_ipam_pool_allocations = ipam_pool_allocations.ipam_pool_allocations
```

---


### Public_ipv4_pools

PublicIpv4Pools resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `public_ipv4_pools` | Vec<String> | <p>Information about the address pools.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access public_ipv4_pools outputs
public_ipv4_pools_id = public_ipv4_pools.id
public_ipv4_pools_next_token = public_ipv4_pools.next_token
public_ipv4_pools_public_ipv4_pools = public_ipv4_pools.public_ipv4_pools
```

---


### Network_insights_access_scope_analysis_findings

NetworkInsightsAccessScopeAnalysisFindings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `analysis_status` | String | <p>The status of Network Access Scope Analysis.</p> |
| `analysis_findings` | Vec<String> | <p>The findings associated with Network Access Scope Analysis.</p> |
| `network_insights_access_scope_analysis_id` | String | <p>The ID of the Network Access Scope analysis.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_insights_access_scope_analysis_findings outputs
network_insights_access_scope_analysis_findings_id = network_insights_access_scope_analysis_findings.id
network_insights_access_scope_analysis_findings_analysis_status = network_insights_access_scope_analysis_findings.analysis_status
network_insights_access_scope_analysis_findings_analysis_findings = network_insights_access_scope_analysis_findings.analysis_findings
network_insights_access_scope_analysis_findings_network_insights_access_scope_analysis_id = network_insights_access_scope_analysis_findings.network_insights_access_scope_analysis_id
network_insights_access_scope_analysis_findings_next_token = network_insights_access_scope_analysis_findings.next_token
```

---


### Security_group_rule_descriptions_ingress

SecurityGroupRuleDescriptionsIngress resource

**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_group_rule_descriptions` | Vec<String> |  | <p>The description for the ingress security group rules. You must specify either
            a description or IP permissions.</p> |
| `group_name` | String |  | <p>[Default VPC] The name of the security group. You must specify either the
            security group ID or the security group name. For security groups in a
            nondefault VPC, you must specify the security group ID.</p> |
| `group_id` | String |  | <p>The ID of the security group. You must specify either the security group ID or the
			security group name in the request. For security groups in a nondefault VPC, you must
			specify the security group ID.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `ip_permissions` | Vec<String> |  | <p>The IP permissions for the security group rule. You must specify either IP permissions
		    or a description.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Internet_gateway

InternetGateway resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the internet gateway.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create internet_gateway
internet_gateway = provider.ec2.Internet_gateway {
}

```

---


### Coip_pool_usage

CoipPoolUsage resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `coip_address_usages` | Vec<String> | <p>Information about the address usage.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `coip_pool_id` | String | <p>The ID of the customer-owned address pool.</p> |
| `local_gateway_route_table_id` | String | <p>The ID of the local gateway route table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access coip_pool_usage outputs
coip_pool_usage_id = coip_pool_usage.id
coip_pool_usage_coip_address_usages = coip_pool_usage.coip_address_usages
coip_pool_usage_next_token = coip_pool_usage.next_token
coip_pool_usage_coip_pool_id = coip_pool_usage.coip_pool_id
coip_pool_usage_local_gateway_route_table_id = coip_pool_usage.local_gateway_route_table_id
```

---


### Reserved_instances_listing

ReservedInstancesListing resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reserved_instances_id` | String | ✅ | <p>The ID of the active Standard Reserved Instance.</p> |
| `instance_count` | i64 | ✅ | <p>The number of instances that are a part of a Reserved Instance account to be listed in the
      Reserved Instance Marketplace. This number should be less than or equal to the instance count
      associated with the Reserved Instance ID specified in this call.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier you provide to ensure idempotency of your listings. This
      helps avoid duplicate listings. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring
      Idempotency</a>.</p> |
| `price_schedules` | Vec<String> | ✅ | <p>A list specifying the price of the Standard Reserved Instance for each month remaining in
      the Reserved Instance term.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create reserved_instances_listing
reserved_instances_listing = provider.ec2.Reserved_instances_listing {
    reserved_instances_id = "value"  # <p>The ID of the active Standard Reserved Instance.</p>
    instance_count = "value"  # <p>The number of instances that are a part of a Reserved Instance account to be listed in the
      Reserved Instance Marketplace. This number should be less than or equal to the instance count
      associated with the Reserved Instance ID specified in this call.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier you provide to ensure idempotency of your listings. This
      helps avoid duplicate listings. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring
      Idempotency</a>.</p>
    price_schedules = "value"  # <p>A list specifying the price of the Standard Reserved Instance for each month remaining in
      the Reserved Instance term.</p>
}

```

---


### Transit_gateway_route

TransitGatewayRoute resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `blackhole` | bool |  | <p>Indicates whether to drop traffic that matches this route.</p> |
| `destination_cidr_block` | String | ✅ | <p>The CIDR range used for destination matches. Routing decisions are based on the
         most specific match.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `transit_gateway_attachment_id` | String |  | <p>The ID of the attachment.</p> |
| `transit_gateway_route_table_id` | String | ✅ | <p>The ID of the transit gateway route table.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_route
transit_gateway_route = provider.ec2.Transit_gateway_route {
    destination_cidr_block = "value"  # <p>The CIDR range used for destination matches. Routing decisions are based on the
         most specific match.</p>
    transit_gateway_route_table_id = "value"  # <p>The ID of the transit gateway route table.</p>
}

```

---


### Capacity_block_offerings

CapacityBlockOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_block_offerings` | Vec<String> | <p>The recommended Capacity Block offering for the dates specified.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_block_offerings outputs
capacity_block_offerings_id = capacity_block_offerings.id
capacity_block_offerings_capacity_block_offerings = capacity_block_offerings.capacity_block_offerings
capacity_block_offerings_next_token = capacity_block_offerings.next_token
```

---


### Instance_connect_endpoints

InstanceConnectEndpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `instance_connect_endpoints` | Vec<String> | <p>Information about the EC2 Instance Connect Endpoints.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_connect_endpoints outputs
instance_connect_endpoints_id = instance_connect_endpoints.id
instance_connect_endpoints_next_token = instance_connect_endpoints.next_token
instance_connect_endpoints_instance_connect_endpoints = instance_connect_endpoints.instance_connect_endpoints
```

---


### Transit_gateways

TransitGateways resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateways` | Vec<String> | <p>Information about the transit gateways.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateways outputs
transit_gateways_id = transit_gateways.id
transit_gateways_transit_gateways = transit_gateways.transit_gateways
transit_gateways_next_token = transit_gateways.next_token
```

---


### Launch_template_data

LaunchTemplateData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `launch_template_data` | String | <p>The instance data.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access launch_template_data outputs
launch_template_data_id = launch_template_data.id
launch_template_data_launch_template_data = launch_template_data.launch_template_data
```

---


### Console_screenshot

ConsoleScreenshot resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_data` | String | <p>The data that comprises the image.</p> |
| `instance_id` | String | <p>The ID of the instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access console_screenshot outputs
console_screenshot_id = console_screenshot.id
console_screenshot_image_data = console_screenshot.image_data
console_screenshot_instance_id = console_screenshot.instance_id
```

---


### Traffic_mirror_targets

TrafficMirrorTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `traffic_mirror_targets` | Vec<String> | <p>Information about one or more Traffic Mirror targets.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. The value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access traffic_mirror_targets outputs
traffic_mirror_targets_id = traffic_mirror_targets.id
traffic_mirror_targets_traffic_mirror_targets = traffic_mirror_targets.traffic_mirror_targets
traffic_mirror_targets_next_token = traffic_mirror_targets.next_token
```

---


### Volume

Volume resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency 
      of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensure 
        Idempotency</a>.</p> |
| `multi_attach_enabled` | bool |  | <p>Indicates whether to enable Amazon EBS Multi-Attach. If you enable Multi-Attach, you can attach the 
      volume to up to 16 <a href="https://docs.aws.amazon.com/ec2/latest/instancetypes/ec2-nitro-instances.html">Instances built on the Nitro System</a> in the same Availability Zone. This parameter is 
    	supported with <code>io1</code> and <code>io2</code> volumes only. For more information, 
    	see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/ebs-volumes-multi.html">
    		Amazon EBS Multi-Attach</a> in the <i>Amazon EBS User Guide</i>.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `volume_type` | String |  | <p>The volume type. This parameter can be one of the following values:</p>
         <ul>
            <li>
               <p>General Purpose SSD: <code>gp2</code> | <code>gp3</code>
               </p>
            </li>
            <li>
               <p>Provisioned IOPS SSD: <code>io1</code> | <code>io2</code>
               </p>
            </li>
            <li>
               <p>Throughput Optimized HDD: <code>st1</code>
               </p>
            </li>
            <li>
               <p>Cold HDD: <code>sc1</code>
               </p>
            </li>
            <li>
               <p>Magnetic: <code>standard</code>
               </p>
            </li>
         </ul>
         <important>
            <p>Throughput Optimized HDD (<code>st1</code>) and Cold HDD (<code>sc1</code>) volumes can't be used as boot volumes.</p>
         </important>
         <p>For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/ebs-volume-types.html">Amazon EBS volume types</a> in the
      <i>Amazon EBS User Guide</i>.</p>
         <p>Default: <code>gp2</code>
         </p> |
| `availability_zone_id` | String |  | <p>The ID of the Availability Zone in which to create the volume. For example, <code>use1-az1</code>.</p>
         <p>Either <code>AvailabilityZone</code> or <code>AvailabilityZoneId</code> must be specified,
      but not both.</p> |
| `encrypted` | bool |  | <p>Indicates whether the volume should be encrypted. 
      The effect of setting the encryption state to <code>true</code> depends on 
the volume origin (new or from a snapshot), starting encryption state, ownership, and whether encryption by default is enabled. 
      For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/work-with-ebs-encr.html#encryption-by-default">Encryption by default</a>
      in the <i>Amazon EBS User Guide</i>.</p>
         <p>Encrypted Amazon EBS volumes must be attached to instances that support Amazon EBS encryption. 
      For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/ebs-encryption-requirements.html#ebs-encryption_supported_instances">Supported
        instance types</a>.</p> |
| `size` | i64 |  | <p>The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size.
      If you specify a snapshot, the default is the snapshot size, and you can specify a volume size 
      that is equal to or larger than the snapshot size.</p>
         <p>Valid sizes:</p>
         <ul>
            <li>
               <p>gp2: <code>1 - 16,384</code> GiB</p>
            </li>
            <li>
               <p>gp3: <code>1 - 65,536</code> GiB</p>
            </li>
            <li>
               <p>io1: <code>4 - 16,384</code> GiB</p>
            </li>
            <li>
               <p>io2: <code>4 - 65,536</code> GiB</p>
            </li>
            <li>
               <p>st1 and sc1: <code>125 - 16,384</code> GiB</p>
            </li>
            <li>
               <p>standard: <code>1 - 1024</code> GiB</p>
            </li>
         </ul> |
| `kms_key_id` | String |  | <p>The identifier of the KMS key to use for Amazon EBS encryption.
      If this parameter is not specified, your KMS key for Amazon EBS is used. If <code>KmsKeyId</code> is
      specified, the encrypted state must be <code>true</code>.</p>
         <p>You can specify the KMS key using any of the following:</p>
         <ul>
            <li>
               <p>Key ID. For example, 1234abcd-12ab-34cd-56ef-1234567890ab.</p>
            </li>
            <li>
               <p>Key alias. For example, alias/ExampleAlias.</p>
            </li>
            <li>
               <p>Key ARN. For example, arn:aws:kms:us-east-1:012345678910:key/1234abcd-12ab-34cd-56ef-1234567890ab.</p>
            </li>
            <li>
               <p>Alias ARN. For example, arn:aws:kms:us-east-1:012345678910:alias/ExampleAlias.</p>
            </li>
         </ul>
         <p>Amazon Web Services authenticates the KMS key asynchronously. Therefore, if you specify an ID, alias, or ARN that is not valid, 
      the action can appear to complete, but eventually fails.</p> |
| `snapshot_id` | String |  | <p>The snapshot from which to create the volume. You must specify either a snapshot ID or a volume size.</p> |
| `volume_initialization_rate` | i64 |  | <p>Specifies the Amazon EBS Provisioned Rate for Volume Initialization (volume initialization rate), in MiB/s, at which to download 
      the snapshot blocks from Amazon S3 to the volume. This is also known as <i>volume 
        initialization</i>. Specifying a volume initialization rate ensures that the volume is 
      initialized at a predictable and consistent rate after creation.</p>
         <p>This parameter is supported only for volumes created from snapshots. Omit this parameter 
      if:</p>
         <ul>
            <li>
               <p>You want to create the volume using fast snapshot restore. You must specify a snapshot 
          that is enabled for fast snapshot restore. In this case, the volume is fully initialized at 
          creation.</p>
               <note>
                  <p>If you specify a snapshot that is enabled for fast snapshot restore and a volume initialization rate, 
            the volume will be initialized at the specified rate instead of fast snapshot restore.</p>
               </note>
            </li>
            <li>
               <p>You want to create a volume that is initialized at the default rate.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/initalize-volume.html">
      Initialize Amazon EBS volumes</a> in the <i>Amazon EC2 User Guide</i>.</p>
         <p>Valid range: 100 - 300 MiB/s</p> |
| `outpost_arn` | String |  | <p>The Amazon Resource Name (ARN) of the Outpost on which to create the volume.</p>
         <p>If you intend to use a volume with an instance running on an outpost, then you must 
         create the volume on the same outpost as the instance. You can't use a volume created 
         in an Amazon Web Services Region with an instance on an Amazon Web Services outpost, or the other way around.</p> |
| `iops` | i64 |  | <p>The number of I/O operations per second (IOPS) to provision for the volume. 
      Required for <code>io1</code> and <code>io2</code> volumes. Optional for <code>gp3</code> 
      volumes. Omit for all other volume types. </p>
         <p>Valid ranges:</p>
         <ul>
            <li>
               <p>gp3: <code>3,000 </code>(<i>default</i>)<code> - 80,000</code> IOPS</p>
            </li>
            <li>
               <p>io1: <code>100 - 64,000</code> IOPS</p>
            </li>
            <li>
               <p>io2: <code>100 - 256,000</code> IOPS</p>
            </li>
         </ul>
         <note>
            <p>
               <a href="https://docs.aws.amazon.com/ec2/latest/instancetypes/ec2-nitro-instances.html">
Instances built on the Nitro System</a> can support up to 256,000 IOPS. Other instances can support up to 32,000 
IOPS.</p>
         </note> |
| `operator` | String |  | <p>Reserved for internal use.</p> |
| `availability_zone` | String |  | <p>The ID of the Availability Zone in which to create the volume. For example, <code>us-east-1a</code>.</p>
         <p>Either <code>AvailabilityZone</code> or <code>AvailabilityZoneId</code> must be specified,
      but not both.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the volume during creation.</p> |
| `throughput` | i64 |  | <p>The throughput to provision for the volume, in MiB/s. Supported for <code>gp3</code> 
      volumes only. Omit for all other volume types.</p>
         <p>Valid Range: <code>125 - 2000</code> MiB/s</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create volume
volume = provider.ec2.Volume {
}

```

---


### Ipam

Ipam resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description for the IPAM.</p> |
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `enable_private_gua` | bool |  | <p>Enable this option to use your own GUA ranges as private IPv6 addresses. This option is disabled by default.</p> |
| `tag_specifications` | Vec<String> |  | <p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value.
    For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `metered_account` | String |  | <p>A metered account is an Amazon Web Services account that is charged for active IP addresses managed in IPAM. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/ipam-enable-cost-distro.html">Enable cost distribution</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
         <p>Possible values:</p>
         <ul>
            <li>
               <p>
                  <code>ipam-owner</code> (default): The Amazon Web Services account which owns the IPAM is charged for all active IP addresses managed in IPAM.</p>
            </li>
            <li>
               <p>
                  <code>resource-owner</code>: The Amazon Web Services account that owns the IP address is charged for the active IP address.</p>
            </li>
         </ul> |
| `operating_regions` | Vec<String> |  | <p>The operating Regions for the IPAM. Operating Regions are Amazon Web Services Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the Amazon Web Services Regions you select as operating Regions. </p>
         <p>For more information about operating Regions, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/create-ipam.html">Create an IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>.
      </p> |
| `tier` | String |  | <p>IPAM is offered in a Free Tier and an Advanced Tier. For more information about the features available in each tier and the costs associated with the tiers, see <a href="http://aws.amazon.com/vpc/pricing/">Amazon VPC pricing > IPAM tab</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ipam
ipam = provider.ec2.Ipam {
}

```

---


### Image_usage_reports

ImageUsageReports resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_usage_reports` | Vec<String> | <p>The image usage reports.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_usage_reports outputs
image_usage_reports_id = image_usage_reports.id
image_usage_reports_image_usage_reports = image_usage_reports.image_usage_reports
image_usage_reports_next_token = image_usage_reports.next_token
```

---


### Default_credit_specification

DefaultCreditSpecification resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_family_credit_specification` | String | <p>The default credit option for CPU usage of the instance family.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access default_credit_specification outputs
default_credit_specification_id = default_credit_specification.id
default_credit_specification_instance_family_credit_specification = default_credit_specification.instance_family_credit_specification
```

---


### Customer_gateway

CustomerGateway resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually
            making the request, and provides an error response. If you have the required
            permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is
                <code>UnauthorizedOperation</code>.</p> |
| `bgp_asn` | i64 |  | <p>For customer gateway devices that support BGP, specify the device's ASN. You must specify either <code>BgpAsn</code> or <code>BgpAsnExtended</code> when creating the customer gateway. If the ASN is larger than <code>2,147,483,647</code>, you must use <code>BgpAsnExtended</code>.</p>
         <p>Default: 65000</p>
         <p>Valid values: <code>1</code> to <code>2,147,483,647</code>
         </p> |
| `bgp_asn_extended` | i64 |  | <p>For customer gateway devices that support BGP, specify the device's ASN. You must specify either <code>BgpAsn</code> or <code>BgpAsnExtended</code> when creating the customer gateway. If the ASN is larger than <code>2,147,483,647</code>, you must use <code>BgpAsnExtended</code>.</p>
         <p>Valid values: <code>2,147,483,648</code> to <code>4,294,967,295</code>
         </p> |
| `type` | String | ✅ | <p>The type of VPN connection that this customer gateway supports
            (<code>ipsec.1</code>).</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the customer gateway.</p> |
| `public_ip` | String |  | <p>
            <i>This member has been deprecated.</i> The Internet-routable IP address for the customer gateway's outside interface. The
            address must be static.</p> |
| `certificate_arn` | String |  | <p>The Amazon Resource Name (ARN) for the customer gateway certificate.</p> |
| `ip_address` | String |  | <p>The IP address for the customer gateway device's outside interface. The address must be
            static. If <code>OutsideIpAddressType</code> in your VPN connection options is set to
                <code>PrivateIpv4</code>, you can use an RFC6598 or RFC1918 private IPv4 address. If
                <code>OutsideIpAddressType</code> is set to <code>Ipv6</code>, you can use an IPv6 address. </p> |
| `device_name` | String |  | <p>A name for the customer gateway device.</p>
         <p>Length Constraints: Up to 255 characters.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create customer_gateway
customer_gateway = provider.ec2.Customer_gateway {
    type = "value"  # <p>The type of VPN connection that this customer gateway supports
            (<code>ipsec.1</code>).</p>
}

```

---


### Local_gateway_route_table_virtual_interface_group_association

LocalGatewayRouteTableVirtualInterfaceGroupAssociation resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `local_gateway_virtual_interface_group_id` | String | ✅ | <p>
      The ID of the local gateway route table virtual interface group association.
      </p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `local_gateway_route_table_id` | String | ✅ | <p>
      The ID of the local gateway route table.
      </p> |
| `tag_specifications` | Vec<String> |  | <p>
      The tags assigned to the local gateway route table virtual interface group association.
      </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create local_gateway_route_table_virtual_interface_group_association
local_gateway_route_table_virtual_interface_group_association = provider.ec2.Local_gateway_route_table_virtual_interface_group_association {
    local_gateway_virtual_interface_group_id = "value"  # <p>
      The ID of the local gateway route table virtual interface group association.
      </p>
    local_gateway_route_table_id = "value"  # <p>
      The ID of the local gateway route table.
      </p>
}

```

---


### Ipam_discovered_accounts

IpamDiscoveredAccounts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipam_discovered_accounts` | Vec<String> | <p>Discovered accounts.</p> |
| `next_token` | String | <p>Specify the pagination token from a previous request to retrieve the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_discovered_accounts outputs
ipam_discovered_accounts_id = ipam_discovered_accounts.id
ipam_discovered_accounts_ipam_discovered_accounts = ipam_discovered_accounts.ipam_discovered_accounts
ipam_discovered_accounts_next_token = ipam_discovered_accounts.next_token
```

---


### Transit_gateway_route_tables

TransitGatewayRouteTables resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `transit_gateway_route_tables` | Vec<String> | <p>Information about the transit gateway route tables.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_route_tables outputs
transit_gateway_route_tables_id = transit_gateway_route_tables.id
transit_gateway_route_tables_next_token = transit_gateway_route_tables.next_token
transit_gateway_route_tables_transit_gateway_route_tables = transit_gateway_route_tables.transit_gateway_route_tables
```

---


### Security_groups_for_vpc

SecurityGroupsForVpc resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_group_for_vpcs` | Vec<String> | <p>The security group that can be used by interfaces in the VPC.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access security_groups_for_vpc outputs
security_groups_for_vpc_id = security_groups_for_vpc.id
security_groups_for_vpc_security_group_for_vpcs = security_groups_for_vpc.security_group_for_vpcs
security_groups_for_vpc_next_token = security_groups_for_vpc.next_token
```

---


### Instance_topology

InstanceTopology resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instances` | Vec<String> | <p>Information about the topology of each instance.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_topology outputs
instance_topology_id = instance_topology.id
instance_topology_instances = instance_topology.instances
instance_topology_next_token = instance_topology.next_token
```

---


### Managed_prefix_list_associations

ManagedPrefixListAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `prefix_list_associations` | Vec<String> | <p>Information about the associations.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_prefix_list_associations outputs
managed_prefix_list_associations_id = managed_prefix_list_associations.id
managed_prefix_list_associations_prefix_list_associations = managed_prefix_list_associations.prefix_list_associations
managed_prefix_list_associations_next_token = managed_prefix_list_associations.next_token
```

---


### Vpn_gateways

VpnGateways resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpn_gateways` | Vec<String> | <p>Information about one or more virtual private gateways.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpn_gateways outputs
vpn_gateways_id = vpn_gateways.id
vpn_gateways_vpn_gateways = vpn_gateways.vpn_gateways
```

---


### Ipam_external_resource_verification_token

IpamExternalResourceVerificationToken resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `tag_specifications` | Vec<String> |  | <p>Token tags.</p> |
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `ipam_id` | String | ✅ | <p>The ID of the IPAM that will create the token.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ipam_external_resource_verification_token
ipam_external_resource_verification_token = provider.ec2.Ipam_external_resource_verification_token {
    ipam_id = "value"  # <p>The ID of the IPAM that will create the token.</p>
}

```

---


### Vpc_endpoint_connection_notifications

VpcEndpointConnectionNotifications resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is
            <code>null</code> when there are no more results to return.</p> |
| `connection_notification_set` | Vec<String> | <p>The notifications.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_endpoint_connection_notifications outputs
vpc_endpoint_connection_notifications_id = vpc_endpoint_connection_notifications.id
vpc_endpoint_connection_notifications_next_token = vpc_endpoint_connection_notifications.next_token
vpc_endpoint_connection_notifications_connection_notification_set = vpc_endpoint_connection_notifications.connection_notification_set
```

---


### Vpc_block_public_access_exclusions

VpcBlockPublicAccessExclusions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |
| `vpc_block_public_access_exclusions` | Vec<String> | <p>Details related to the exclusions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_block_public_access_exclusions outputs
vpc_block_public_access_exclusions_id = vpc_block_public_access_exclusions.id
vpc_block_public_access_exclusions_next_token = vpc_block_public_access_exclusions.next_token
vpc_block_public_access_exclusions_vpc_block_public_access_exclusions = vpc_block_public_access_exclusions.vpc_block_public_access_exclusions
```

---


### Vpn_connection_device_sample_configuration

VpnConnectionDeviceSampleConfiguration resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpn_connection_device_sample_configuration` | String | <p>Sample configuration file for the specified customer gateway device.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpn_connection_device_sample_configuration outputs
vpn_connection_device_sample_configuration_id = vpn_connection_device_sample_configuration.id
vpn_connection_device_sample_configuration_vpn_connection_device_sample_configuration = vpn_connection_device_sample_configuration.vpn_connection_device_sample_configuration
```

---


### Image_references

ImageReferences resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `image_references` | Vec<String> | <p>The resources that are referencing the specified images.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_references outputs
image_references_id = image_references.id
image_references_next_token = image_references.next_token
image_references_image_references = image_references.image_references
```

---


### Local_gateway_virtual_interface_groups

LocalGatewayVirtualInterfaceGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `local_gateway_virtual_interface_groups` | Vec<String> | <p>The virtual interface groups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access local_gateway_virtual_interface_groups outputs
local_gateway_virtual_interface_groups_id = local_gateway_virtual_interface_groups.id
local_gateway_virtual_interface_groups_next_token = local_gateway_virtual_interface_groups.next_token
local_gateway_virtual_interface_groups_local_gateway_virtual_interface_groups = local_gateway_virtual_interface_groups.local_gateway_virtual_interface_groups
```

---


### Ipam_pools

IpamPools resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipam_pools` | Vec<String> | <p>Information about the IPAM pools.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_pools outputs
ipam_pools_id = ipam_pools.id
ipam_pools_ipam_pools = ipam_pools.ipam_pools
ipam_pools_next_token = ipam_pools.next_token
```

---


### Route_servers

RouteServers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `route_servers` | Vec<String> | <p>Information about the described route servers.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access route_servers outputs
route_servers_id = route_servers.id
route_servers_route_servers = route_servers.route_servers
route_servers_next_token = route_servers.next_token
```

---


### Traffic_mirror_filters

TrafficMirrorFilters resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. The value is <code>null</code> when there are no more results to return.</p> |
| `traffic_mirror_filters` | Vec<String> | <p>Information about one or more Traffic Mirror filters.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access traffic_mirror_filters outputs
traffic_mirror_filters_id = traffic_mirror_filters.id
traffic_mirror_filters_next_token = traffic_mirror_filters.next_token
traffic_mirror_filters_traffic_mirror_filters = traffic_mirror_filters.traffic_mirror_filters
```

---


### Transit_gateway

TransitGateway resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `options` | String |  | <p>The transit gateway options.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the transit gateway.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `description` | String |  | <p>A description of the transit gateway.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway
transit_gateway = provider.ec2.Transit_gateway {
}

```

---


### Instance_image_metadata

InstanceImageMetadata resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_image_metadata` | Vec<String> | <p>Information about the instance and the AMI used to launch the instance.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_image_metadata outputs
instance_image_metadata_id = instance_image_metadata.id
instance_image_metadata_instance_image_metadata = instance_image_metadata.instance_image_metadata
instance_image_metadata_next_token = instance_image_metadata.next_token
```

---


### Route_server_propagations

RouteServerPropagations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `route_server_propagations` | Vec<String> | <p>Information about the route propagations for the specified route server.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access route_server_propagations outputs
route_server_propagations_id = route_server_propagations.id
route_server_propagations_route_server_propagations = route_server_propagations.route_server_propagations
```

---


### Transit_gateway_policy_table_associations

TransitGatewayPolicyTableAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | <p>Returns details about the transit gateway policy table association.</p> |
| `next_token` | String | <p>The token for the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_policy_table_associations outputs
transit_gateway_policy_table_associations_id = transit_gateway_policy_table_associations.id
transit_gateway_policy_table_associations_associations = transit_gateway_policy_table_associations.associations
transit_gateway_policy_table_associations_next_token = transit_gateway_policy_table_associations.next_token
```

---


### Replace_root_volume_task

ReplaceRootVolumeTask resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snapshot_id` | String |  | <p>The ID of the snapshot from which to restore the replacement root volume. The 
      specified snapshot must be a snapshot that you previously created from the original 
      root volume.</p>
         <p>If you want to restore the replacement root volume to the initial launch state, 
      or if you want to restore the replacement root volume from an AMI, omit this 
      parameter.</p> |
| `delete_replaced_root_volume` | bool |  | <p>Indicates whether to automatically delete the original root volume after the root volume 
      replacement task completes. To delete the original root volume, specify <code>true</code>. 
      If you choose to keep the original root volume after the replacement task completes, you must 
      manually delete it when you no longer need it.</p> |
| `volume_initialization_rate` | i64 |  | <p>Specifies the Amazon EBS Provisioned Rate for Volume Initialization (volume initialization rate), in MiB/s, at which to download 
      the snapshot blocks from Amazon S3 to the replacement root volume. This is also known as 
      <i>volume initialization</i>. Specifying a volume initialization rate ensures that 
      the volume is initialized at a predictable and consistent rate after creation.</p>
         <p>Omit this parameter if:</p>
         <ul>
            <li>
               <p>You want to create the volume using fast snapshot restore. You must specify a snapshot 
          that is enabled for fast snapshot restore. In this case, the volume is fully initialized at 
          creation.</p>
               <note>
                  <p>If you specify a snapshot that is enabled for fast snapshot restore and a volume initialization rate, 
            the volume will be initialized at the specified rate instead of fast snapshot restore.</p>
               </note>
            </li>
            <li>
               <p>You want to create a volume that is initialized at the default rate.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/initalize-volume.html">
      Initialize Amazon EBS volumes</a> in the <i>Amazon EC2 User Guide</i>.</p>
         <p>Valid range: 100 - 300 MiB/s</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. 
      If you do not specify a client token, a randomly generated token is used for the request 
      to ensure idempotency. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `instance_id` | String | ✅ | <p>The ID of the instance for which to replace the root volume.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the root volume replacement task.</p> |
| `image_id` | String |  | <p>The ID of the AMI to use to restore the root volume. The specified AMI must have the 
      same product code, billing information, architecture type, and virtualization type as 
      that of the instance.</p>
         <p>If you want to restore the replacement volume from a specific snapshot, or if you want 
      to restore it to its launch state, omit this parameter.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create replace_root_volume_task
replace_root_volume_task = provider.ec2.Replace_root_volume_task {
    instance_id = "value"  # <p>The ID of the instance for which to replace the root volume.</p>
}

```

---


### Verified_access_groups

VerifiedAccessGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `verified_access_groups` | Vec<String> | <p>Details about the Verified Access groups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access verified_access_groups outputs
verified_access_groups_id = verified_access_groups.id
verified_access_groups_next_token = verified_access_groups.next_token
verified_access_groups_verified_access_groups = verified_access_groups.verified_access_groups
```

---


### Verified_access_instance_logging_configurations

VerifiedAccessInstanceLoggingConfigurations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logging_configurations` | Vec<String> | <p>The logging configuration for the Verified Access instances.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access verified_access_instance_logging_configurations outputs
verified_access_instance_logging_configurations_id = verified_access_instance_logging_configurations.id
verified_access_instance_logging_configurations_logging_configurations = verified_access_instance_logging_configurations.logging_configurations
verified_access_instance_logging_configurations_next_token = verified_access_instance_logging_configurations.next_token
```

---


### Fast_snapshot_restores

FastSnapshotRestores resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fast_snapshot_restores` | Vec<String> | <p>Information about the state of fast snapshot restores.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
  This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fast_snapshot_restores outputs
fast_snapshot_restores_id = fast_snapshot_restores.id
fast_snapshot_restores_fast_snapshot_restores = fast_snapshot_restores.fast_snapshot_restores
fast_snapshot_restores_next_token = fast_snapshot_restores.next_token
```

---


### Ipam_pool

IpamPool resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `allocation_default_netmask_length` | i64 |  | <p>The default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is 10.0.0.0/8 and you enter 16 here, 
         new allocations will default to 10.0.0.0/16.</p> |
| `tag_specifications` | Vec<String> |  | <p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value.
    For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p> |
| `description` | String |  | <p>A description for the IPAM pool.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `aws_service` | String |  | <p>Limits which service in Amazon Web Services that the pool can be used in. "ec2", for example, allows users to use space for Elastic IP addresses and VPCs.</p> |
| `source_ipam_pool_id` | String |  | <p>The ID of the source IPAM pool. Use this option to create a pool within an existing pool. Note that the CIDR you provision for the pool within the source pool must be available in the source pool's CIDR range.</p> |
| `allocation_resource_tags` | Vec<String> |  | <p>Tags that are required for resources that use CIDRs from this IPAM pool. Resources that do not have these tags will not be allowed to allocate space from the pool. If the resources have their tags changed after they have allocated space or if the allocation tagging requirements are changed on the pool, the resource may be marked as noncompliant.</p> |
| `auto_import` | bool |  | <p>If selected, IPAM will continuously look for resources within the CIDR range of this pool 
         and automatically import them as allocations into your IPAM. The CIDRs that will be allocated for
         these resources must not already be allocated to other resources in order for the import to succeed. IPAM will import 
         a CIDR regardless of its compliance with the pool's allocation rules, so a resource might be imported and subsequently 
         marked as noncompliant. If IPAM discovers multiple CIDRs that overlap, IPAM will import the largest CIDR only. If IPAM 
         discovers multiple CIDRs with matching CIDRs, IPAM will randomly import one of them only.
      </p>
         <p>A locale must be set on the pool for this feature to work.</p> |
| `locale` | String |  | <p>The locale for the pool should be one of the following:</p>
         <ul>
            <li>
               <p>An Amazon Web Services Region where you want this IPAM pool to be available for allocations.</p>
            </li>
            <li>
               <p>The network border group for an Amazon Web Services Local Zone where you want this IPAM pool to be available for allocations (<a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-byoip.html#byoip-zone-avail">supported Local Zones</a>). This option is only available for IPAM IPv4 pools in the public scope.</p>
            </li>
         </ul>
         <p>Possible values: Any Amazon Web Services Region or supported Amazon Web Services Local Zone. Default is <code>none</code> and means any locale.</p> |
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `allocation_max_netmask_length` | i64 |  | <p>The maximum netmask length possible for CIDR allocations in this IPAM pool to be compliant. The maximum netmask length must be 
         greater than the minimum netmask length. Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are  0 - 128.</p> |
| `allocation_min_netmask_length` | i64 |  | <p>The minimum netmask length required for CIDR allocations in this IPAM pool to be compliant. The minimum netmask length must be 
         less than the maximum netmask length. Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are  0 - 128.</p> |
| `public_ip_source` | String |  | <p>The IP address source for pools in the public scope. Only used for provisioning IP address CIDRs to pools in the public scope. Default is <code>byoip</code>. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/intro-create-ipv6-pools.html">Create IPv6 pools</a> in the <i>Amazon VPC IPAM User Guide</i>. 
         By default, you can add only one Amazon-provided IPv6 CIDR block to a top-level IPv6 pool if PublicIpSource is <code>amazon</code>. For information on increasing the default limit, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/quotas-ipam.html"> Quotas for your IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>.</p> |
| `publicly_advertisable` | bool |  | <p>Determines if the pool is publicly advertisable. The request can only contain <code>PubliclyAdvertisable</code> if <code>AddressFamily</code> is <code>ipv6</code> and <code>PublicIpSource</code> is <code>byoip</code>.</p> |
| `ipam_scope_id` | String | ✅ | <p>The ID of the scope in which you would like to create the IPAM pool.</p> |
| `source_resource` | String |  | <p>The resource used to provision CIDRs to a resource planning pool.</p> |
| `address_family` | String | ✅ | <p>The IP protocol assigned to this IPAM pool. You must choose either IPv4 or IPv6 protocol for a pool.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ipam_pool
ipam_pool = provider.ec2.Ipam_pool {
    ipam_scope_id = "value"  # <p>The ID of the scope in which you would like to create the IPAM pool.</p>
    address_family = "value"  # <p>The IP protocol assigned to this IPAM pool. You must choose either IPv4 or IPv6 protocol for a pool.</p>
}

```

---


### Capacity_block_extension_offerings

CapacityBlockExtensionOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `capacity_block_extension_offerings` | Vec<String> | <p>The recommended Capacity Block extension offerings for the dates specified.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_block_extension_offerings outputs
capacity_block_extension_offerings_id = capacity_block_extension_offerings.id
capacity_block_extension_offerings_next_token = capacity_block_extension_offerings.next_token
capacity_block_extension_offerings_capacity_block_extension_offerings = capacity_block_extension_offerings.capacity_block_extension_offerings
```

---


### Ipam_prefix_list_resolver_versions

IpamPrefixListResolverVersions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `ipam_prefix_list_resolver_versions` | Vec<String> | <p>Information about the IPAM prefix list resolver versions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_prefix_list_resolver_versions outputs
ipam_prefix_list_resolver_versions_id = ipam_prefix_list_resolver_versions.id
ipam_prefix_list_resolver_versions_next_token = ipam_prefix_list_resolver_versions.next_token
ipam_prefix_list_resolver_versions_ipam_prefix_list_resolver_versions = ipam_prefix_list_resolver_versions.ipam_prefix_list_resolver_versions
```

---


### Account_attributes

AccountAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_attributes` | Vec<String> | <p>Information about the account attributes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access account_attributes outputs
account_attributes_id = account_attributes.id
account_attributes_account_attributes = account_attributes.account_attributes
```

---


### Default_vpc

DefaultVpc resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create default_vpc
default_vpc = provider.ec2.Default_vpc {
}

```

---


### Fleet_instances

FleetInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `active_instances` | Vec<String> | <p>The running instances. This list is refreshed periodically and might be out of
         date.</p> |
| `fleet_id` | String | <p>The ID of the EC2 Fleet.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access fleet_instances outputs
fleet_instances_id = fleet_instances.id
fleet_instances_next_token = fleet_instances.next_token
fleet_instances_active_instances = fleet_instances.active_instances
fleet_instances_fleet_id = fleet_instances.fleet_id
```

---


### Spot_fleet_request_history

SpotFleetRequestHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `history_records` | Vec<String> | <p>Information about the events in the history of the Spot Fleet request.</p> |
| `last_evaluated_time` | String | <p>The last date and time for the events, in UTC format (for example,
                <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).
            All records up to this time were retrieved.</p>
         <p>If <code>nextToken</code> indicates that there are more items, this value is not
            present.</p> |
| `spot_fleet_request_id` | String | <p>The ID of the Spot Fleet request.</p> |
| `start_time` | String | <p>The starting date and time for the events, in UTC format (for example,
                <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access spot_fleet_request_history outputs
spot_fleet_request_history_id = spot_fleet_request_history.id
spot_fleet_request_history_history_records = spot_fleet_request_history.history_records
spot_fleet_request_history_last_evaluated_time = spot_fleet_request_history.last_evaluated_time
spot_fleet_request_history_spot_fleet_request_id = spot_fleet_request_history.spot_fleet_request_id
spot_fleet_request_history_start_time = spot_fleet_request_history.start_time
spot_fleet_request_history_next_token = spot_fleet_request_history.next_token
```

---


### Route_tables

RouteTables resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `route_tables` | Vec<String> | <p>Information about the route tables.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access route_tables outputs
route_tables_id = route_tables.id
route_tables_route_tables = route_tables.route_tables
route_tables_next_token = route_tables.next_token
```

---


### Image_usage_report

ImageUsageReport resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image_id` | String | ✅ | <p>The ID of the image to report on.</p> |
| `resource_types` | Vec<String> | ✅ | <p>The resource types to include in the report.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure idempotency of the request.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the report on creation. The <code>ResourceType</code> must be set to
      <code>image-usage-report</code>; any other value will cause the report creation to
      fail.</p>
         <p>To tag a report after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
			and provides an error response. If you have the required permissions, the error response is 
			<code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `account_ids` | Vec<String> |  | <p>The Amazon Web Services account IDs to include in the report. To include all accounts, omit this
      parameter.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image_usage_report
image_usage_report = provider.ec2.Image_usage_report {
    image_id = "value"  # <p>The ID of the image to report on.</p>
    resource_types = "value"  # <p>The resource types to include in the report.</p>
}

```

---


### Snapshot

Snapshot resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description for the snapshot.</p> |
| `volume_id` | String | ✅ | <p>The ID of the Amazon EBS volume.</p> |
| `outpost_arn` | String |  | <note>
            <p>Only supported for volumes on Outposts. If the source volume is not on an Outpost, 
        omit this parameter.</p>
         </note>
         <ul>
            <li>
               <p>To create the snapshot on the same Outpost as the source volume, specify the 
          ARN of that Outpost. The snapshot must be created on the same Outpost as the volume.</p>
            </li>
            <li>
               <p>To create the snapshot in the parent Region of the Outpost, omit this parameter.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/snapshots-outposts.html#create-snapshot">Create local snapshots from volumes on an Outpost</a> in the <i>Amazon EBS User Guide</i>.</p> |
| `location` | String |  | <note>
            <p>Only supported for volumes in Local Zones. If the source volume is not in a Local Zone, 
        omit this parameter.</p>
         </note>
         <ul>
            <li>
               <p>To create a local snapshot in the same Local Zone as the source volume, specify 
          <code>local</code>.</p>
            </li>
            <li>
               <p>To create a regional snapshot in the parent Region of the Local Zone, specify 
          <code>regional</code> or omit this parameter.</p>
            </li>
         </ul>
         <p>Default value: <code>regional</code>
         </p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the snapshot during creation.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create snapshot
snapshot = provider.ec2.Snapshot {
    volume_id = "value"  # <p>The ID of the Amazon EBS volume.</p>
}

```

---


### Availability_zones

AvailabilityZones resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `availability_zones` | Vec<String> | <p>Information about the Availability Zones, Local Zones, and Wavelength Zones.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access availability_zones outputs
availability_zones_id = availability_zones.id
availability_zones_availability_zones = availability_zones.availability_zones
```

---


### Network_insights_access_scopes

NetworkInsightsAccessScopes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network_insights_access_scopes` | Vec<String> | <p>The Network Access Scopes.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_insights_access_scopes outputs
network_insights_access_scopes_id = network_insights_access_scopes.id
network_insights_access_scopes_network_insights_access_scopes = network_insights_access_scopes.network_insights_access_scopes
network_insights_access_scopes_next_token = network_insights_access_scopes.next_token
```

---


### Route_server_peers

RouteServerPeers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `route_server_peers` | Vec<String> | <p>Information about the described route server peers.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access route_server_peers outputs
route_server_peers_id = route_server_peers.id
route_server_peers_next_token = route_server_peers.next_token
route_server_peers_route_server_peers = route_server_peers.route_server_peers
```

---


### Security_group_vpc_associations

SecurityGroupVpcAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |
| `security_group_vpc_associations` | Vec<String> | <p>The security group VPC associations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access security_group_vpc_associations outputs
security_group_vpc_associations_id = security_group_vpc_associations.id
security_group_vpc_associations_next_token = security_group_vpc_associations.next_token
security_group_vpc_associations_security_group_vpc_associations = security_group_vpc_associations.security_group_vpc_associations
```

---


### Ebs_default_kms_key_id

EbsDefaultKmsKeyId resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key_id` | String | <p>The Amazon Resource Name (ARN) of the default KMS key for encryption by default.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ebs_default_kms_key_id outputs
ebs_default_kms_key_id_id = ebs_default_kms_key_id.id
ebs_default_kms_key_id_kms_key_id = ebs_default_kms_key_id.kms_key_id
```

---


### Coip_pool

CoipPool resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>
      The tags to assign to the CoIP address pool.
      </p> |
| `local_gateway_route_table_id` | String | ✅ | <p>
      The ID of the local gateway route table.
      </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create coip_pool
coip_pool = provider.ec2.Coip_pool {
    local_gateway_route_table_id = "value"  # <p>
      The ID of the local gateway route table.
      </p>
}

```

---


### Network_insights_access_scope

NetworkInsightsAccessScope resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `exclude_paths` | Vec<String> |  | <p>The paths to exclude.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, 
   see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure idempotency</a>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply.</p> |
| `match_paths` | Vec<String> |  | <p>The paths to match.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create network_insights_access_scope
network_insights_access_scope = provider.ec2.Network_insights_access_scope {
    client_token = "value"  # <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, 
   see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure idempotency</a>.</p>
}

```

---


### Ipam_discovered_resource_cidrs

IpamDiscoveredResourceCidrs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipam_discovered_resource_cidrs` | Vec<String> | <p>Discovered resource CIDRs.</p> |
| `next_token` | String | <p>Specify the pagination token from a previous request to retrieve the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_discovered_resource_cidrs outputs
ipam_discovered_resource_cidrs_id = ipam_discovered_resource_cidrs.id
ipam_discovered_resource_cidrs_ipam_discovered_resource_cidrs = ipam_discovered_resource_cidrs.ipam_discovered_resource_cidrs
ipam_discovered_resource_cidrs_next_token = ipam_discovered_resource_cidrs.next_token
```

---


### Route_server_peer

RouteServerPeer resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the route server peer during creation.</p> |
| `route_server_endpoint_id` | String | ✅ | <p>The ID of the route server endpoint for which to create a peer.</p> |
| `peer_address` | String | ✅ | <p>The IPv4 address of the peer device.</p> |
| `bgp_options` | String | ✅ | <p>The BGP options for the peer, including ASN (Autonomous System Number) and BFD (Bidrectional Forwarding Detection) settings.</p> |
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create route_server_peer
route_server_peer = provider.ec2.Route_server_peer {
    route_server_endpoint_id = "value"  # <p>The ID of the route server endpoint for which to create a peer.</p>
    peer_address = "value"  # <p>The IPv4 address of the peer device.</p>
    bgp_options = "value"  # <p>The BGP options for the peer, including ASN (Autonomous System Number) and BFD (Bidrectional Forwarding Detection) settings.</p>
}

```

---


### Managed_prefix_list

ManagedPrefixList resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prefix_list_name` | String | ✅ | <p>A name for the prefix list.</p>
         <p>Constraints: Up to 255 characters in length. The name cannot start with <code>com.amazonaws</code>.</p> |
| `max_entries` | i64 | ✅ | <p>The maximum number of entries for the prefix list.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the
            request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring
                idempotency</a>.</p>
         <p>Constraints: Up to 255 UTF-8 characters in length.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `entries` | Vec<String> |  | <p>One or more entries for the prefix list.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the prefix list during creation.</p> |
| `address_family` | String | ✅ | <p>The IP address type.</p>
         <p>Valid Values: <code>IPv4</code> | <code>IPv6</code>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create managed_prefix_list
managed_prefix_list = provider.ec2.Managed_prefix_list {
    prefix_list_name = "value"  # <p>A name for the prefix list.</p>
         <p>Constraints: Up to 255 characters in length. The name cannot start with <code>com.amazonaws</code>.</p>
    max_entries = "value"  # <p>The maximum number of entries for the prefix list.</p>
    address_family = "value"  # <p>The IP address type.</p>
         <p>Valid Values: <code>IPv4</code> | <code>IPv6</code>
         </p>
}

```

---


### Capacity_manager_attributes

CapacityManagerAttributes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_export_count` | i64 | <p>
The number of active data export configurations for this account. This count includes all data exports regardless of their current delivery status.
</p> |
| `organizations_access` | bool | <p>
Indicates whether Organizations access is enabled for cross-account data aggregation.
</p> |
| `ingestion_status_message` | String | <p>
A descriptive message providing additional details about the current ingestion status. This may include error information if ingestion has 
failed or progress details during initial setup.
</p> |
| `earliest_datapoint_timestamp` | String | <p>
The timestamp of the earliest data point available in Capacity Manager, in milliseconds since epoch. This indicates how far back historical data is available for queries.
</p> |
| `ingestion_status` | String | <p>
The current data ingestion status. Initial ingestion may take several hours after enabling Capacity Manager.
</p> |
| `capacity_manager_status` | String | <p>
The current status of Capacity Manager.
</p> |
| `latest_datapoint_timestamp` | String | <p>
The timestamp of the most recent data point ingested by Capacity Manager, in milliseconds since epoch. This indicates how current your capacity data is.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_manager_attributes outputs
capacity_manager_attributes_id = capacity_manager_attributes.id
capacity_manager_attributes_data_export_count = capacity_manager_attributes.data_export_count
capacity_manager_attributes_organizations_access = capacity_manager_attributes.organizations_access
capacity_manager_attributes_ingestion_status_message = capacity_manager_attributes.ingestion_status_message
capacity_manager_attributes_earliest_datapoint_timestamp = capacity_manager_attributes.earliest_datapoint_timestamp
capacity_manager_attributes_ingestion_status = capacity_manager_attributes.ingestion_status
capacity_manager_attributes_capacity_manager_status = capacity_manager_attributes.capacity_manager_status
capacity_manager_attributes_latest_datapoint_timestamp = capacity_manager_attributes.latest_datapoint_timestamp
```

---


### Vpn_connection

VpnConnection resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String | ✅ | <p>The type of VPN connection (<code>ipsec.1</code>).</p> |
| `vpn_gateway_id` | String |  | <p>The ID of the virtual private gateway. If you specify a virtual private gateway, you
            cannot specify a transit gateway.</p> |
| `options` | String |  | <p>The options for the VPN connection.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the VPN connection.</p> |
| `customer_gateway_id` | String | ✅ | <p>The ID of the customer gateway.</p> |
| `pre_shared_key_storage` | String |  | <p>Specifies the storage mode for the pre-shared key (PSK). Valid values are <code>Standard</code>" (stored in the Site-to-Site VPN service) or <code>SecretsManager</code> (stored in Amazon Web Services Secrets Manager).</p> |
| `transit_gateway_id` | String |  | <p>The ID of the transit gateway. If you specify a transit gateway, you cannot specify a virtual private
            gateway.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually
            making the request, and provides an error response. If you have the required
            permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is
                <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpn_connection
vpn_connection = provider.ec2.Vpn_connection {
    type = "value"  # <p>The type of VPN connection (<code>ipsec.1</code>).</p>
    customer_gateway_id = "value"  # <p>The ID of the customer gateway.</p>
}

```

---


### Verified_access_instance

VerifiedAccessInstance resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive token that you provide to ensure idempotency of your
            modification request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `fips_enabled` | bool |  | <p>Enable or disable support for Federal Information Processing Standards (FIPS) on the instance.</p> |
| `cidr_endpoints_custom_sub_domain` | String |  | <p>The custom subdomain.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the Verified Access instance.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `description` | String |  | <p>A description for the Verified Access instance.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create verified_access_instance
verified_access_instance = provider.ec2.Verified_access_instance {
}

```

---


### Dhcp_options

DhcpOptions resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the DHCP option.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `dhcp_configurations` | Vec<String> | ✅ | <p>A DHCP configuration option.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |
| `dhcp_options` | Vec<String> | <p>Information about the DHCP options sets.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create dhcp_options
dhcp_options = provider.ec2.Dhcp_options {
    dhcp_configurations = "value"  # <p>A DHCP configuration option.</p>
}

# Access dhcp_options outputs
dhcp_options_id = dhcp_options.id
dhcp_options_next_token = dhcp_options.next_token
dhcp_options_dhcp_options = dhcp_options.dhcp_options
```

---


### Client_vpn_endpoints

ClientVpnEndpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_vpn_endpoints` | Vec<String> | <p>Information about the Client VPN endpoints.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access client_vpn_endpoints outputs
client_vpn_endpoints_id = client_vpn_endpoints.id
client_vpn_endpoints_client_vpn_endpoints = client_vpn_endpoints.client_vpn_endpoints
client_vpn_endpoints_next_token = client_vpn_endpoints.next_token
```

---


### Client_vpn_connections

ClientVpnConnections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connections` | Vec<String> | <p>Information about the active and terminated client connections.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access client_vpn_connections outputs
client_vpn_connections_id = client_vpn_connections.id
client_vpn_connections_connections = client_vpn_connections.connections
client_vpn_connections_next_token = client_vpn_connections.next_token
```

---


### Stale_security_groups

StaleSecurityGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stale_security_group_set` | Vec<String> | <p>Information about the stale security groups.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access stale_security_groups outputs
stale_security_groups_id = stale_security_groups.id
stale_security_groups_stale_security_group_set = stale_security_groups.stale_security_group_set
stale_security_groups_next_token = stale_security_groups.next_token
```

---


### Ipam_byoasn

IpamByoasn resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `byoasns` | Vec<String> | <p>ASN and BYOIP CIDR associations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_byoasn outputs
ipam_byoasn_id = ipam_byoasn.id
ipam_byoasn_next_token = ipam_byoasn.next_token
ipam_byoasn_byoasns = ipam_byoasn.byoasns
```

---


### Traffic_mirror_filter_rule

TrafficMirrorFilterRule resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_cidr_block` | String | ✅ | <p>The source CIDR block to assign to the Traffic Mirror rule.</p> |
| `traffic_direction` | String | ✅ | <p>The type of traffic.</p> |
| `description` | String |  | <p>The description of the Traffic Mirror rule.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure idempotency</a>.</p> |
| `tag_specifications` | Vec<String> |  | <p>Traffic Mirroring tags specifications.</p> |
| `traffic_mirror_filter_id` | String | ✅ | <p>The ID of the filter that this rule is associated with.</p> |
| `source_port_range` | String |  | <p>The source port range.</p> |
| `rule_number` | i64 | ✅ | <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given
         direction. The rules are processed in ascending order by rule number.</p> |
| `destination_port_range` | String |  | <p>The destination port range.</p> |
| `protocol` | i64 |  | <p>The protocol, for example UDP, to assign to the Traffic Mirror rule.</p>
         <p>For information about the protocol value, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a> on the  Internet Assigned Numbers Authority (IANA) website.</p> |
| `destination_cidr_block` | String | ✅ | <p>The destination CIDR block to assign to the Traffic Mirror rule.</p> |
| `rule_action` | String | ✅ | <p>The action to take on the filtered traffic.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create traffic_mirror_filter_rule
traffic_mirror_filter_rule = provider.ec2.Traffic_mirror_filter_rule {
    source_cidr_block = "value"  # <p>The source CIDR block to assign to the Traffic Mirror rule.</p>
    traffic_direction = "value"  # <p>The type of traffic.</p>
    traffic_mirror_filter_id = "value"  # <p>The ID of the filter that this rule is associated with.</p>
    rule_number = "value"  # <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given
         direction. The rules are processed in ascending order by rule number.</p>
    destination_cidr_block = "value"  # <p>The destination CIDR block to assign to the Traffic Mirror rule.</p>
    rule_action = "value"  # <p>The action to take on the filtered traffic.</p>
}

```

---


### Ipam_prefix_list_resolver_rules

IpamPrefixListResolverRules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rules` | Vec<String> | <p>The CIDR selection rules for the IPAM prefix list resolver.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_prefix_list_resolver_rules outputs
ipam_prefix_list_resolver_rules_id = ipam_prefix_list_resolver_rules.id
ipam_prefix_list_resolver_rules_rules = ipam_prefix_list_resolver_rules.rules
ipam_prefix_list_resolver_rules_next_token = ipam_prefix_list_resolver_rules.next_token
```

---


### Vpc_classic_link_dns_support

VpcClassicLinkDnsSupport resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpcs` | Vec<String> | <p>Information about the ClassicLink DNS support status of the VPCs.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_classic_link_dns_support outputs
vpc_classic_link_dns_support_id = vpc_classic_link_dns_support.id
vpc_classic_link_dns_support_vpcs = vpc_classic_link_dns_support.vpcs
vpc_classic_link_dns_support_next_token = vpc_classic_link_dns_support.next_token
```

---


### Network_acl

NetworkAcl resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the network ACL.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `vpc_id` | String | ✅ | <p>The ID of the VPC.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create network_acl
network_acl = provider.ec2.Network_acl {
    vpc_id = "value"  # <p>The ID of the VPC.</p>
}

```

---


### Transit_gateway_route_table

TransitGatewayRouteTable resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `transit_gateway_id` | String | ✅ | <p>The ID of the transit gateway.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the transit gateway route table.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_route_table
transit_gateway_route_table = provider.ec2.Transit_gateway_route_table {
    transit_gateway_id = "value"  # <p>The ID of the transit gateway.</p>
}

```

---


### Export_image_tasks

ExportImageTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to get the next page of results. This value is <code>null</code> when there are no more results
   to return.</p> |
| `export_image_tasks` | Vec<String> | <p>Information about the export image tasks.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access export_image_tasks outputs
export_image_tasks_id = export_image_tasks.id
export_image_tasks_next_token = export_image_tasks.next_token
export_image_tasks_export_image_tasks = export_image_tasks.export_image_tasks
```

---


### Internet_gateways

InternetGateways resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |
| `internet_gateways` | Vec<String> | <p>Information about the internet gateways.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access internet_gateways outputs
internet_gateways_id = internet_gateways.id
internet_gateways_next_token = internet_gateways.next_token
internet_gateways_internet_gateways = internet_gateways.internet_gateways
```

---


### Ipam_external_resource_verification_tokens

IpamExternalResourceVerificationTokens resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipam_external_resource_verification_tokens` | Vec<String> | <p>Verification tokens.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_external_resource_verification_tokens outputs
ipam_external_resource_verification_tokens_id = ipam_external_resource_verification_tokens.id
ipam_external_resource_verification_tokens_ipam_external_resource_verification_tokens = ipam_external_resource_verification_tokens.ipam_external_resource_verification_tokens
ipam_external_resource_verification_tokens_next_token = ipam_external_resource_verification_tokens.next_token
```

---


### Default_subnet

DefaultSubnet resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ipv6_native` | bool |  | <p>Indicates whether to create an IPv6 only subnet. If you already have a default subnet
            for this Availability Zone, you must delete it before you can create an IPv6 only subnet.</p> |
| `availability_zone_id` | String |  | <p>The ID of the Availability Zone.</p>
         <p>Either <code>AvailabilityZone</code> or <code>AvailabilityZoneId</code> must be specified,
            but not both.</p> |
| `availability_zone` | String |  | <p>The Availability Zone in which to create the default subnet.</p>
         <p>Either <code>AvailabilityZone</code> or <code>AvailabilityZoneId</code> must be specified,
            but not both.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create default_subnet
default_subnet = provider.ec2.Default_subnet {
}

```

---


### Vpc_endpoint_connection_notification

VpcEndpointConnectionNotification resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_id` | String |  | <p>The ID of the endpoint service.</p> |
| `vpc_endpoint_id` | String |  | <p>The ID of the endpoint.</p> |
| `connection_notification_arn` | String | ✅ | <p>The ARN of the SNS topic for the notifications.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure
                idempotency</a>.</p> |
| `connection_events` | Vec<String> | ✅ | <p>The endpoint events for which to receive notifications. Valid values are
                <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and
                <code>Reject</code>.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_endpoint_connection_notification
vpc_endpoint_connection_notification = provider.ec2.Vpc_endpoint_connection_notification {
    connection_notification_arn = "value"  # <p>The ARN of the SNS topic for the notifications.</p>
    connection_events = "value"  # <p>The endpoint events for which to receive notifications. Valid values are
                <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and
                <code>Reject</code>.</p>
}

```

---


### Elastic_gpus

ElasticGpus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `elastic_gpu_set` | Vec<String> | <p>Information about the Elastic Graphics accelerators.</p> |
| `max_results` | i64 | <p>The total number of items to return. If the total number of items available is more
            than the value specified in max-items then a Next-Token will be provided in the output
            that you can use to resume pagination.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is
                <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access elastic_gpus outputs
elastic_gpus_id = elastic_gpus.id
elastic_gpus_elastic_gpu_set = elastic_gpus.elastic_gpu_set
elastic_gpus_max_results = elastic_gpus.max_results
elastic_gpus_next_token = elastic_gpus.next_token
```

---


### Scheduled_instance_availability

ScheduledInstanceAvailability resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scheduled_instance_availability_set` | Vec<String> | <p>Information about the available Scheduled Instances.</p> |
| `next_token` | String | <p>The token required to retrieve the next set of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access scheduled_instance_availability outputs
scheduled_instance_availability_id = scheduled_instance_availability.id
scheduled_instance_availability_scheduled_instance_availability_set = scheduled_instance_availability.scheduled_instance_availability_set
scheduled_instance_availability_next_token = scheduled_instance_availability.next_token
```

---


### Transit_gateway_multicast_domains

TransitGatewayMulticastDomains resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_multicast_domains` | Vec<String> | <p>Information about the transit gateway multicast domains.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_multicast_domains outputs
transit_gateway_multicast_domains_id = transit_gateway_multicast_domains.id
transit_gateway_multicast_domains_transit_gateway_multicast_domains = transit_gateway_multicast_domains.transit_gateway_multicast_domains
transit_gateway_multicast_domains_next_token = transit_gateway_multicast_domains.next_token
```

---


### Instance_type_offerings

InstanceTypeOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `instance_type_offerings` | Vec<String> | <p>The instance types offered in the location.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_type_offerings outputs
instance_type_offerings_id = instance_type_offerings.id
instance_type_offerings_next_token = instance_type_offerings.next_token
instance_type_offerings_instance_type_offerings = instance_type_offerings.instance_type_offerings
```

---


### Addresses

Addresses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `addresses` | Vec<String> | <p>Information about the Elastic IP addresses.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access addresses outputs
addresses_id = addresses.id
addresses_addresses = addresses.addresses
```

---


### Verified_access_trust_providers

VerifiedAccessTrustProviders resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `verified_access_trust_providers` | Vec<String> | <p>Details about the Verified Access trust providers.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access verified_access_trust_providers outputs
verified_access_trust_providers_id = verified_access_trust_providers.id
verified_access_trust_providers_next_token = verified_access_trust_providers.next_token
verified_access_trust_providers_verified_access_trust_providers = verified_access_trust_providers.verified_access_trust_providers
```

---


### Route_server_endpoint

RouteServerEndpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `route_server_id` | String | ✅ | <p>The ID of the route server for which to create an endpoint.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the route server endpoint during creation.</p> |
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p> |
| `subnet_id` | String | ✅ | <p>The ID of the subnet in which to create the route server endpoint.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create route_server_endpoint
route_server_endpoint = provider.ec2.Route_server_endpoint {
    route_server_id = "value"  # <p>The ID of the route server for which to create an endpoint.</p>
    subnet_id = "value"  # <p>The ID of the subnet in which to create the route server endpoint.</p>
}

```

---


### Service_link_virtual_interfaces

ServiceLinkVirtualInterfaces resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `service_link_virtual_interfaces` | Vec<String> | <p>Describes the service link virtual interfaces.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access service_link_virtual_interfaces outputs
service_link_virtual_interfaces_id = service_link_virtual_interfaces.id
service_link_virtual_interfaces_next_token = service_link_virtual_interfaces.next_token
service_link_virtual_interfaces_service_link_virtual_interfaces = service_link_virtual_interfaces.service_link_virtual_interfaces
```

---


### Snapshot_attribute

SnapshotAttribute resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshot_id` | String | <p>The ID of the EBS snapshot.</p> |
| `product_codes` | Vec<String> | <p>The product codes.</p> |
| `create_volume_permissions` | Vec<String> | <p>The users and groups that have the permissions for creating volumes from the
      snapshot.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshot_attribute outputs
snapshot_attribute_id = snapshot_attribute.id
snapshot_attribute_snapshot_id = snapshot_attribute.snapshot_id
snapshot_attribute_product_codes = snapshot_attribute.product_codes
snapshot_attribute_create_volume_permissions = snapshot_attribute.create_volume_permissions
```

---


### Serial_console_access_status

SerialConsoleAccessStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `serial_console_access_enabled` | bool | <p>If <code>true</code>, access to the EC2 serial console of all instances is enabled for
			your account. If <code>false</code>, access to the EC2 serial console of all instances
			is disabled for your account.</p> |
| `managed_by` | String | <p>The entity that manages access to the serial console. Possible values include:</p>
         <ul>
            <li>
               <p>
                  <code>account</code> - Access is managed by the account.</p>
            </li>
            <li>
               <p>
                  <code>declarative-policy</code> - Access is managed by a declarative policy and can't
            be modified by the account.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access serial_console_access_status outputs
serial_console_access_status_id = serial_console_access_status.id
serial_console_access_status_serial_console_access_enabled = serial_console_access_status.serial_console_access_enabled
serial_console_access_status_managed_by = serial_console_access_status.managed_by
```

---


### Capacity_reservation_fleet

CapacityReservationFleet resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `allocation_strategy` | String |  | <p>The strategy used by the Capacity Reservation Fleet to determine which of the
			specified instance types to use. Currently, only the <code>prioritized</code> allocation
			strategy is supported. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/crfleet-concepts.html#allocation-strategy"> Allocation
				strategy</a> in the <i>Amazon EC2 User Guide</i>.</p>
         <p>Valid values: <code>prioritized</code>
         </p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p> |
| `tenancy` | String |  | <p>Indicates the tenancy of the Capacity Reservation Fleet. All Capacity Reservations in
			the Fleet inherit this tenancy. The Capacity Reservation Fleet can have one of the
			following tenancy settings:</p>
         <ul>
            <li>
               <p>
                  <code>default</code> - The Capacity Reservation Fleet is created on hardware
					that is shared with other Amazon Web Services accounts.</p>
            </li>
            <li>
               <p>
                  <code>dedicated</code> - The Capacity Reservations are created on single-tenant
					hardware that is dedicated to a single Amazon Web Services account.</p>
            </li>
         </ul> |
| `instance_match_criteria` | String |  | <p>Indicates the type of instance launches that the Capacity Reservation Fleet accepts.
			All Capacity Reservations in the Fleet inherit this instance matching criteria.</p>
         <p>Currently, Capacity Reservation Fleets support <code>open</code> instance matching
			criteria only. This means that instances that have matching attributes (instance type,
			platform, and Availability Zone) run in the Capacity Reservations automatically.
			Instances do not need to explicitly target a Capacity Reservation Fleet to use its
			reserved capacity.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the Capacity Reservation Fleet. The tags are automatically
			assigned to the Capacity Reservations in the Fleet.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `end_date` | String |  | <p>The date and time at which the Capacity Reservation Fleet expires. When the Capacity
			Reservation Fleet expires, its state changes to <code>expired</code> and all of the
			Capacity Reservations in the Fleet expire.</p>
         <p>The Capacity Reservation Fleet expires within an hour after the specified time. For
			example, if you specify <code>5/31/2019</code>, <code>13:30:55</code>, the Capacity
			Reservation Fleet is guaranteed to expire between <code>13:30:55</code> and
				<code>14:30:55</code> on <code>5/31/2019</code>. </p> |
| `total_target_capacity` | i64 | ✅ | <p>The total number of capacity units to be reserved by the Capacity Reservation Fleet.
			This value, together with the instance type weights that you assign to each instance
			type used by the Fleet determine the number of instances for which the Fleet reserves
			capacity. Both values are based on units that make sense for your workload. For more
			information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/crfleet-concepts.html#target-capacity">Total target
				capacity</a> in the <i>Amazon EC2 User Guide</i>.</p> |
| `instance_type_specifications` | Vec<String> | ✅ | <p>Information about the instance types for which to reserve the capacity.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create capacity_reservation_fleet
capacity_reservation_fleet = provider.ec2.Capacity_reservation_fleet {
    total_target_capacity = "value"  # <p>The total number of capacity units to be reserved by the Capacity Reservation Fleet.
			This value, together with the instance type weights that you assign to each instance
			type used by the Fleet determine the number of instances for which the Fleet reserves
			capacity. Both values are based on units that make sense for your workload. For more
			information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/crfleet-concepts.html#target-capacity">Total target
				capacity</a> in the <i>Amazon EC2 User Guide</i>.</p>
    instance_type_specifications = "value"  # <p>Information about the instance types for which to reserve the capacity.</p>
}

```

---


### Vpc_endpoint_services

VpcEndpointServices resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p> |
| `service_names` | Vec<String> | <p>The supported services.</p> |
| `service_details` | Vec<String> | <p>Information about the service.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_endpoint_services outputs
vpc_endpoint_services_id = vpc_endpoint_services.id
vpc_endpoint_services_next_token = vpc_endpoint_services.next_token
vpc_endpoint_services_service_names = vpc_endpoint_services.service_names
vpc_endpoint_services_service_details = vpc_endpoint_services.service_details
```

---


### Bundle_tasks

BundleTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bundle_tasks` | Vec<String> | <p>Information about the bundle tasks.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access bundle_tasks outputs
bundle_tasks_id = bundle_tasks.id
bundle_tasks_bundle_tasks = bundle_tasks.bundle_tasks
```

---


### Snapshot_tier_status

SnapshotTierStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshot_tier_statuses` | Vec<String> | <p>Information about the snapshot's storage tier.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
  This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access snapshot_tier_status outputs
snapshot_tier_status_id = snapshot_tier_status.id
snapshot_tier_status_snapshot_tier_statuses = snapshot_tier_status.snapshot_tier_statuses
snapshot_tier_status_next_token = snapshot_tier_status.next_token
```

---


### Reserved_instances_modifications

ReservedInstancesModifications resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reserved_instances_modifications` | Vec<String> | <p>The Reserved Instance modification information.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code>
      when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_instances_modifications outputs
reserved_instances_modifications_id = reserved_instances_modifications.id
reserved_instances_modifications_reserved_instances_modifications = reserved_instances_modifications.reserved_instances_modifications
reserved_instances_modifications_next_token = reserved_instances_modifications.next_token
```

---


### Transit_gateway_connect

TransitGatewayConnect resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `options` | String | ✅ | <p>The Connect attachment options.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the Connect attachment.</p> |
| `transport_transit_gateway_attachment_id` | String | ✅ | <p>The ID of the transit gateway attachment. You can specify a VPC attachment or Amazon Web Services Direct Connect attachment.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_connect
transit_gateway_connect = provider.ec2.Transit_gateway_connect {
    options = "value"  # <p>The Connect attachment options.</p>
    transport_transit_gateway_attachment_id = "value"  # <p>The ID of the transit gateway attachment. You can specify a VPC attachment or Amazon Web Services Direct Connect attachment.</p>
}

```

---


### Network_acls

NetworkAcls resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |
| `network_acls` | Vec<String> | <p>Information about the network ACLs.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_acls outputs
network_acls_id = network_acls.id
network_acls_next_token = network_acls.next_token
network_acls_network_acls = network_acls.network_acls
```

---


### Vpc_endpoint_service_configurations

VpcEndpointServiceConfigurations resource

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_configurations` | Vec<String> | <p>Information about the services.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_endpoint_service_configurations outputs
vpc_endpoint_service_configurations_id = vpc_endpoint_service_configurations.id
vpc_endpoint_service_configurations_service_configurations = vpc_endpoint_service_configurations.service_configurations
vpc_endpoint_service_configurations_next_token = vpc_endpoint_service_configurations.next_token
```

---


### Network_insights_analyses

NetworkInsightsAnalyses resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `network_insights_analyses` | Vec<String> | <p>Information about the network insights analyses.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_insights_analyses outputs
network_insights_analyses_id = network_insights_analyses.id
network_insights_analyses_next_token = network_insights_analyses.next_token
network_insights_analyses_network_insights_analyses = network_insights_analyses.network_insights_analyses
```

---


### Vpn_connection_route

VpnConnectionRoute resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_cidr_block` | String | ✅ | <p>The CIDR block associated with the local subnet of the customer network.</p> |
| `vpn_connection_id` | String | ✅ | <p>The ID of the VPN connection.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpn_connection_route
vpn_connection_route = provider.ec2.Vpn_connection_route {
    destination_cidr_block = "value"  # <p>The CIDR block associated with the local subnet of the customer network.</p>
    vpn_connection_id = "value"  # <p>The ID of the VPN connection.</p>
}

```

---


### Subnets

Subnets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subnets` | Vec<String> | <p>Information about the subnets.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access subnets outputs
subnets_id = subnets.id
subnets_subnets = subnets.subnets
subnets_next_token = subnets.next_token
```

---


### Security_group_rules

SecurityGroupRules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
            This value is <code>null</code> when there are no more items to return.</p> |
| `security_group_rules` | Vec<String> | <p>Information about security group rules.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access security_group_rules outputs
security_group_rules_id = security_group_rules.id
security_group_rules_next_token = security_group_rules.next_token
security_group_rules_security_group_rules = security_group_rules.security_group_rules
```

---


### Address_transfers

AddressTransfers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>Specify the pagination token from a previous request to retrieve the next page of results.</p> |
| `address_transfers` | Vec<String> | <p>The Elastic IP address transfer.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access address_transfers outputs
address_transfers_id = address_transfers.id
address_transfers_next_token = address_transfers.next_token
address_transfers_address_transfers = address_transfers.address_transfers
```

---


### Traffic_mirror_session

TrafficMirrorSession resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `virtual_network_id` | i64 |  | <p>The VXLAN ID for the Traffic Mirror session. For more information about the VXLAN
         protocol, see <a href="https://datatracker.ietf.org/doc/html/rfc7348">RFC 7348</a>. If you do
         not specify a <code>VirtualNetworkId</code>, an account-wide unique ID is chosen at
         random.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure idempotency</a>.</p> |
| `description` | String |  | <p>The description of the Traffic Mirror session.</p> |
| `traffic_mirror_filter_id` | String | ✅ | <p>The ID of the Traffic Mirror filter.</p> |
| `network_interface_id` | String | ✅ | <p>The ID of the source network interface.</p> |
| `traffic_mirror_target_id` | String | ✅ | <p>The ID of the Traffic Mirror target.</p> |
| `session_number` | i64 | ✅ | <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>
         <p>Valid values are 1-32766.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `packet_length` | i64 |  | <p>The number of bytes in each packet to mirror. These are bytes after the VXLAN header. Do
         not specify this parameter when you want to mirror the entire packet. To mirror a subset of
         the packet, set this to the length (in bytes) that you want to mirror. For example, if you
         set this value to 100, then the first 100 bytes that meet the filter criteria are copied to
         the target.</p>
         <p>If you do not want to mirror the entire packet, use the <code>PacketLength</code> parameter to specify the number of bytes in each packet to mirror.</p>
         <p>For sessions with Network Load Balancer (NLB) Traffic Mirror targets the default <code>PacketLength</code> will be set to 8500. Valid values are 1-8500. Setting a <code>PacketLength</code> greater than 8500 will result in an error response.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to a Traffic Mirror session.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create traffic_mirror_session
traffic_mirror_session = provider.ec2.Traffic_mirror_session {
    traffic_mirror_filter_id = "value"  # <p>The ID of the Traffic Mirror filter.</p>
    network_interface_id = "value"  # <p>The ID of the source network interface.</p>
    traffic_mirror_target_id = "value"  # <p>The ID of the Traffic Mirror target.</p>
    session_number = "value"  # <p>The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.</p>
         <p>Valid values are 1-32766.</p>
}

```

---


### Security_groups

SecurityGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |
| `security_groups` | Vec<String> | <p>Information about the security groups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access security_groups outputs
security_groups_id = security_groups.id
security_groups_next_token = security_groups.next_token
security_groups_security_groups = security_groups.security_groups
```

---


### Placement_groups

PlacementGroups resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `placement_groups` | Vec<String> | <p>Information about the placement groups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access placement_groups outputs
placement_groups_id = placement_groups.id
placement_groups_placement_groups = placement_groups.placement_groups
```

---


### Transit_gateway_prefix_list_reference

TransitGatewayPrefixListReference resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prefix_list_id` | String | ✅ | <p>The ID of the prefix list that is used for destination matches.</p> |
| `transit_gateway_route_table_id` | String | ✅ | <p>The ID of the transit gateway route table.</p> |
| `blackhole` | bool |  | <p>Indicates whether to drop traffic that matches this route.</p> |
| `transit_gateway_attachment_id` | String |  | <p>The ID of the attachment to which traffic is routed.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_prefix_list_reference
transit_gateway_prefix_list_reference = provider.ec2.Transit_gateway_prefix_list_reference {
    prefix_list_id = "value"  # <p>The ID of the prefix list that is used for destination matches.</p>
    transit_gateway_route_table_id = "value"  # <p>The ID of the transit gateway route table.</p>
}

```

---


### Capacity_block_status

CapacityBlockStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_block_statuses` | Vec<String> | <p>The availability of capacity for a Capacity Block.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_block_status outputs
capacity_block_status_id = capacity_block_status.id
capacity_block_status_capacity_block_statuses = capacity_block_status.capacity_block_statuses
capacity_block_status_next_token = capacity_block_status.next_token
```

---


### Restore_image_task

RestoreImageTask resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the AMI and snapshots on restoration. You can tag the AMI, the
      snapshots, or both.</p>
         <ul>
            <li>
               <p>To tag the AMI, the value for <code>ResourceType</code> must be
          <code>image</code>.</p>
            </li>
            <li>
               <p>To tag the snapshots, the value for <code>ResourceType</code> must be
          <code>snapshot</code>. The same tag is applied to all of the snapshots that are
          created.</p>
            </li>
         </ul> |
| `bucket` | String | ✅ | <p>The name of the Amazon S3 bucket that contains the stored AMI object.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
			and provides an error response. If you have the required permissions, the error response is 
			<code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `object_key` | String | ✅ | <p>The name of the stored AMI object in the bucket.</p> |
| `name` | String |  | <p>The name for the restored AMI. The name must be unique for AMIs in the Region for this
      account. If you do not provide a name, the new AMI gets the same name as the original
      AMI.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create restore_image_task
restore_image_task = provider.ec2.Restore_image_task {
    bucket = "value"  # <p>The name of the Amazon S3 bucket that contains the stored AMI object.</p>
    object_key = "value"  # <p>The name of the stored AMI object in the bucket.</p>
}

```

---


### Placement_group

PlacementGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_name` | String |  | <p>A name for the placement group. Must be unique within the scope of your account for
            the Region.</p>
         <p>Constraints: Up to 255 ASCII characters</p> |
| `strategy` | String |  | <p>The placement strategy.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the operation, without actually making the 
  request, and provides an error response. If you have the required permissions, the error response is 
  <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `spread_level` | String |  | <p>Determines how placement groups spread instances. </p>
         <ul>
            <li>
               <p>Host – You can use <code>host</code> only with Outpost placement
                    groups.</p>
            </li>
            <li>
               <p>Rack – No usage restrictions.</p>
            </li>
         </ul> |
| `partition_count` | i64 |  | <p>The number of partitions. Valid only when <b>Strategy</b> is
            set to <code>partition</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the new placement group.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create placement_group
placement_group = provider.ec2.Placement_group {
}

```

---


### Local_gateway_route_table

LocalGatewayRouteTable resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `local_gateway_id` | String | ✅ | <p>
      The ID of the local gateway. 
      </p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>
      The tags assigned to the local gateway route table.
      </p> |
| `mode` | String |  | <p>
      The mode of the local gateway route table.
      </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create local_gateway_route_table
local_gateway_route_table = provider.ec2.Local_gateway_route_table {
    local_gateway_id = "value"  # <p>
      The ID of the local gateway. 
      </p>
}

```

---


### Security_group_references

SecurityGroupReferences resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_group_reference_set` | Vec<String> | <p>Information about the VPCs with the referencing security groups.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access security_group_references outputs
security_group_references_id = security_group_references.id
security_group_references_security_group_reference_set = security_group_references.security_group_reference_set
```

---


### Volumes

Volumes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `volumes` | Vec<String> | <p>Information about the volumes.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
  This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access volumes outputs
volumes_id = volumes.id
volumes_volumes = volumes.volumes
volumes_next_token = volumes.next_token
```

---


### Local_gateway_virtual_interface

LocalGatewayVirtualInterface resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `peer_bgp_asn_extended` | i64 |  | <p>The extended 32-bit ASN of the BGP peer for use with larger ASN values.</p> |
| `vlan` | i64 | ✅ | <p>The virtual local area network (VLAN) used for the local gateway virtual interface.</p> |
| `peer_bgp_asn` | i64 |  | <p>The Autonomous System Number (ASN) of the Border Gateway Protocol (BGP) peer.</p> |
| `local_gateway_virtual_interface_group_id` | String | ✅ | <p>The ID of the local gateway virtual interface group.</p> |
| `outpost_lag_id` | String | ✅ | <p>References the Link Aggregation Group (LAG) that connects the Outpost to on-premises network devices.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to a resource when the local gateway virtual interface is being created. </p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `peer_address` | String | ✅ | <p>The peer IP address for the local gateway virtual interface. Only IPv4 is
         supported.</p> |
| `local_address` | String | ✅ | <p>The IP address assigned to the local gateway virtual interface on the Outpost side. Only IPv4 is supported.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create local_gateway_virtual_interface
local_gateway_virtual_interface = provider.ec2.Local_gateway_virtual_interface {
    vlan = "value"  # <p>The virtual local area network (VLAN) used for the local gateway virtual interface.</p>
    local_gateway_virtual_interface_group_id = "value"  # <p>The ID of the local gateway virtual interface group.</p>
    outpost_lag_id = "value"  # <p>References the Link Aggregation Group (LAG) that connects the Outpost to on-premises network devices.</p>
    peer_address = "value"  # <p>The peer IP address for the local gateway virtual interface. Only IPv4 is
         supported.</p>
    local_address = "value"  # <p>The IP address assigned to the local gateway virtual interface on the Outpost side. Only IPv4 is supported.</p>
}

```

---


### Vpn_tunnel_replacement_status

VpnTunnelReplacementStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpn_gateway_id` | String | <p>The ID of the virtual private gateway.</p> |
| `customer_gateway_id` | String | <p>The ID of the customer gateway.</p> |
| `maintenance_details` | String | <p>Get details of pending tunnel endpoint maintenance.</p> |
| `vpn_connection_id` | String | <p>The ID of the Site-to-Site VPN connection. </p> |
| `transit_gateway_id` | String | <p>The ID of the transit gateway associated with the VPN connection.</p> |
| `vpn_tunnel_outside_ip_address` | String | <p>The external IP address of the VPN tunnel.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpn_tunnel_replacement_status outputs
vpn_tunnel_replacement_status_id = vpn_tunnel_replacement_status.id
vpn_tunnel_replacement_status_vpn_gateway_id = vpn_tunnel_replacement_status.vpn_gateway_id
vpn_tunnel_replacement_status_customer_gateway_id = vpn_tunnel_replacement_status.customer_gateway_id
vpn_tunnel_replacement_status_maintenance_details = vpn_tunnel_replacement_status.maintenance_details
vpn_tunnel_replacement_status_vpn_connection_id = vpn_tunnel_replacement_status.vpn_connection_id
vpn_tunnel_replacement_status_transit_gateway_id = vpn_tunnel_replacement_status.transit_gateway_id
vpn_tunnel_replacement_status_vpn_tunnel_outside_ip_address = vpn_tunnel_replacement_status.vpn_tunnel_outside_ip_address
```

---


### Active_vpn_tunnel_status

ActiveVpnTunnelStatus resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `active_vpn_tunnel_status` | String | <p>Information about the current security configuration of the VPN tunnel.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access active_vpn_tunnel_status outputs
active_vpn_tunnel_status_id = active_vpn_tunnel_status.id
active_vpn_tunnel_status_active_vpn_tunnel_status = active_vpn_tunnel_status.active_vpn_tunnel_status
```

---


### Ipam_scopes

IpamScopes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `ipam_scopes` | Vec<String> | <p>The scopes you want information on.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_scopes outputs
ipam_scopes_id = ipam_scopes.id
ipam_scopes_next_token = ipam_scopes.next_token
ipam_scopes_ipam_scopes = ipam_scopes.ipam_scopes
```

---


### Verified_access_group

VerifiedAccessGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `sse_specification` | String |  | <p>The options for server side encryption.</p> |
| `verified_access_instance_id` | String | ✅ | <p>The ID of the Verified Access instance.</p> |
| `policy_document` | String |  | <p>The Verified Access policy document.</p> |
| `description` | String |  | <p>A description for the Verified Access group.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the Verified Access group.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive token that you provide to ensure idempotency of your
            modification request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create verified_access_group
verified_access_group = provider.ec2.Verified_access_group {
    verified_access_instance_id = "value"  # <p>The ID of the Verified Access instance.</p>
}

```

---


### Client_vpn_authorization_rules

ClientVpnAuthorizationRules resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `authorization_rules` | Vec<String> | <p>Information about the authorization rules.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access client_vpn_authorization_rules outputs
client_vpn_authorization_rules_id = client_vpn_authorization_rules.id
client_vpn_authorization_rules_next_token = client_vpn_authorization_rules.next_token
client_vpn_authorization_rules_authorization_rules = client_vpn_authorization_rules.authorization_rules
```

---


### Transit_gateway_route_table_propagations

TransitGatewayRouteTablePropagations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `transit_gateway_route_table_propagations` | Vec<String> | <p>Information about the route table propagations.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_route_table_propagations outputs
transit_gateway_route_table_propagations_id = transit_gateway_route_table_propagations.id
transit_gateway_route_table_propagations_next_token = transit_gateway_route_table_propagations.next_token
transit_gateway_route_table_propagations_transit_gateway_route_table_propagations = transit_gateway_route_table_propagations.transit_gateway_route_table_propagations
```

---


### Network_insights_analysis

NetworkInsightsAnalysis resource

**Operations**: ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

```

---


### Password_data

PasswordData resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `password_data` | String | <p>The password of the instance. Returns an empty string if the password is not
            available.</p> |
| `timestamp` | String | <p>The time the data was last updated.</p> |
| `instance_id` | String | <p>The ID of the Windows instance.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access password_data outputs
password_data_id = password_data.id
password_data_password_data = password_data.password_data
password_data_timestamp = password_data.timestamp
password_data_instance_id = password_data.instance_id
```

---


### Host_reservations

HostReservations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `host_reservation_set` | Vec<String> | <p>Details about the reservation's configuration.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access host_reservations outputs
host_reservations_id = host_reservations.id
host_reservations_next_token = host_reservations.next_token
host_reservations_host_reservation_set = host_reservations.host_reservation_set
```

---


### Hosts

Hosts resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `hosts` | Vec<String> | <p>Information about the Dedicated Hosts.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access hosts outputs
hosts_id = hosts.id
hosts_next_token = hosts.next_token
hosts_hosts = hosts.hosts
```

---


### Vpn_connection_device_types

VpnConnectionDeviceTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpn_connection_device_types` | Vec<String> | <p>List of customer gateway devices that have a sample configuration file available for
            use.</p> |
| `next_token` | String | <p>The <code>NextToken</code> value to include in a future
                <code>GetVpnConnectionDeviceTypes</code> request. When the results of a
                <code>GetVpnConnectionDeviceTypes</code> request exceed <code>MaxResults</code>,
            this value can be used to retrieve the next page of results. This value is null when
            there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpn_connection_device_types outputs
vpn_connection_device_types_id = vpn_connection_device_types.id
vpn_connection_device_types_vpn_connection_device_types = vpn_connection_device_types.vpn_connection_device_types
vpn_connection_device_types_next_token = vpn_connection_device_types.next_token
```

---


### Network_interface_permissions

NetworkInterfacePermissions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is
                <code>null</code> when there are no more items to return.</p> |
| `network_interface_permissions` | Vec<String> | <p>The network interface permissions.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_interface_permissions outputs
network_interface_permissions_id = network_interface_permissions.id
network_interface_permissions_next_token = network_interface_permissions.next_token
network_interface_permissions_network_interface_permissions = network_interface_permissions.network_interface_permissions
```

---


### Ipam_address_history

IpamAddressHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `history_records` | Vec<String> | <p>A historical record for a CIDR within an IPAM scope. If the CIDR is associated with an EC2 instance, you will see an object in the response for the instance and one for the network interface.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_address_history outputs
ipam_address_history_id = ipam_address_history.id
ipam_address_history_next_token = ipam_address_history.next_token
ipam_address_history_history_records = ipam_address_history.history_records
```

---


### Vpc_endpoint

VpcEndpoint resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure
                idempotency</a>.</p> |
| `resource_configuration_arn` | String |  | <p>The Amazon Resource Name (ARN) of a resource configuration that will be associated with
         the VPC endpoint of type resource.</p> |
| `route_table_ids` | Vec<String> |  | <p>(Gateway endpoint) The route table IDs.</p> |
| `security_group_ids` | Vec<String> |  | <p>(Interface endpoint) The IDs of the security groups to associate with the
            endpoint network interfaces. If this parameter is not specified, we use the default 
            security group for the VPC.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `vpc_endpoint_type` | String |  | <p>The type of endpoint.</p>
         <p>Default: Gateway</p> |
| `subnet_ids` | Vec<String> |  | <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to create endpoint
            network interfaces. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p> |
| `policy_document` | String |  | <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the
            service. The policy must be in valid JSON format. If this parameter is not specified, we
            attach a default policy that allows full access to the service.</p> |
| `subnet_configurations` | Vec<String> |  | <p>The subnet configurations for the endpoint.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to associate with the endpoint.</p> |
| `dns_options` | String |  | <p>The DNS options for the endpoint.</p> |
| `vpc_id` | String | ✅ | <p>The ID of the VPC.</p> |
| `service_name` | String |  | <p>The name of the endpoint service.</p> |
| `service_region` | String |  | <p>The Region where the service is hosted. The default is the current Region.</p> |
| `private_dns_enabled` | bool |  | <p>(Interface endpoint) Indicates whether to associate a private hosted zone with the
            specified VPC. The private hosted zone contains a record set for the default public DNS
            name for the service for the Region (for example,
                <code>kinesis.us-east-1.amazonaws.com</code>), which resolves to the private IP
            addresses of the endpoint network interfaces in the VPC. This enables you to make
            requests to the default public DNS name for the service instead of the public DNS names
            that are automatically generated by the VPC endpoint service.</p>
         <p>To use a private hosted zone, you must set the following VPC attributes to
            <code>true</code>: <code>enableDnsHostnames</code> and
            <code>enableDnsSupport</code>. Use <a>ModifyVpcAttribute</a> to set the VPC
            attributes.</p> |
| `service_network_arn` | String |  | <p>The Amazon Resource Name (ARN) of a service network that will be associated with the VPC
         endpoint of type service-network.</p> |
| `ip_address_type` | String |  | <p>The IP address type for the endpoint.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create vpc_endpoint
vpc_endpoint = provider.ec2.Vpc_endpoint {
    vpc_id = "value"  # <p>The ID of the VPC.</p>
}

```

---


### Ipam_prefix_list_resolver

IpamPrefixListResolver resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ipam_id` | String | ✅ | <p>The ID of the IPAM that will serve as the source of the IP address database for CIDR selection. The IPAM must be in the Advanced tier to use this feature.</p> |
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `description` | String |  | <p>A description for the IPAM prefix list resolver to help you identify its purpose and configuration.</p> |
| `address_family` | String | ✅ | <p>The address family for the IPAM prefix list resolver. Valid values are <code>ipv4</code> and <code>ipv6</code>. You must create separate resolvers for IPv4 and IPv6 CIDRs as they cannot be mixed in the same resolver.</p> |
| `rules` | Vec<String> |  | <p>The CIDR selection rules for the resolver.</p>
         <p>CIDR selection rules define the business logic for selecting CIDRs from IPAM. If a CIDR matches any of the rules, it will be included. If a rule has multiple conditions, the CIDR has to match every condition of that rule. You can create a prefix list resolver without any CIDR selection rules, but it will generate empty versions (containing no CIDRs) until you add rules.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the IPAM prefix list resolver during creation. Tags help you organize and manage your Amazon Web Services resources.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ipam_prefix_list_resolver
ipam_prefix_list_resolver = provider.ec2.Ipam_prefix_list_resolver {
    ipam_id = "value"  # <p>The ID of the IPAM that will serve as the source of the IP address database for CIDR selection. The IPAM must be in the Advanced tier to use this feature.</p>
    address_family = "value"  # <p>The address family for the IPAM prefix list resolver. Valid values are <code>ipv4</code> and <code>ipv6</code>. You must create separate resolvers for IPv4 and IPv6 CIDRs as they cannot be mixed in the same resolver.</p>
}

```

---


### Aggregate_id_format

AggregateIdFormat resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `use_long_ids_aggregated` | bool | <p>Indicates whether all resource types in the Region are configured to use longer IDs.
            This value is only <code>true</code> if all users are configured to use longer IDs for
            all resources types in the Region.</p> |
| `statuses` | Vec<String> | <p>Information about each resource's ID format.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access aggregate_id_format outputs
aggregate_id_format_id = aggregate_id_format.id
aggregate_id_format_use_long_ids_aggregated = aggregate_id_format.use_long_ids_aggregated
aggregate_id_format_statuses = aggregate_id_format.statuses
```

---


### Transit_gateway_route_table_announcements

TransitGatewayRouteTableAnnouncements resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token for the next page of results.</p> |
| `transit_gateway_route_table_announcements` | Vec<String> | <p>Describes the transit gateway route table announcement.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_route_table_announcements outputs
transit_gateway_route_table_announcements_id = transit_gateway_route_table_announcements.id
transit_gateway_route_table_announcements_next_token = transit_gateway_route_table_announcements.next_token
transit_gateway_route_table_announcements_transit_gateway_route_table_announcements = transit_gateway_route_table_announcements.transit_gateway_route_table_announcements
```

---


### Flow_logs

FlowLogs resource

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `resource_ids` | Vec<String> | ✅ | <p>The IDs of the resources to monitor. For example, if the resource type is
                <code>VPC</code>, specify the IDs of the VPCs.</p>
         <p>Constraints: Maximum of 25 for transit gateway resource types. Maximum of 1000 for the
            other resource types.</p> |
| `resource_type` | String | ✅ | <p>The type of resource to monitor.</p> |
| `traffic_type` | String |  | <p>The type of traffic to monitor (accepted traffic, rejected traffic, or all traffic).
            This parameter is not supported for transit gateway resource types. It is required for
            the other resource types.</p> |
| `log_destination_type` | String |  | <p>The type of destination for the flow log data.</p>
         <p>Default: <code>cloud-watch-logs</code>
         </p> |
| `log_destination` | String |  | <p>The destination for the flow log data. The meaning of this parameter depends on the destination type.</p>
         <ul>
            <li>
               <p>If the destination type is <code>cloud-watch-logs</code>, specify the ARN of a CloudWatch Logs log group. For example:</p>
               <p>arn:aws:logs:<i>region</i>:<i>account_id</i>:log-group:<i>my_group</i>
               </p>
               <p>Alternatively, use the <code>LogGroupName</code> parameter.</p>
            </li>
            <li>
               <p>If the destination type is <code>s3</code>, specify the ARN of an S3 bucket. For example:</p>
               <p>arn:aws:s3:::<i>my_bucket</i>/<i>my_subfolder</i>/</p>
               <p>The subfolder is optional. Note that you can't use <code>AWSLogs</code> as a subfolder name.</p>
            </li>
            <li>
               <p>If the destination type is <code>kinesis-data-firehose</code>, specify the ARN of a Kinesis Data Firehose delivery stream. For example:</p>
               <p>arn:aws:firehose:<i>region</i>:<i>account_id</i>:deliverystream:<i>my_stream</i>
               </p>
            </li>
         </ul> |
| `log_format` | String |  | <p>The fields to include in the flow log record. List the fields in the order in which
            they should appear. If you omit this parameter, the flow log is created using the
            default format. If you specify this parameter, you must include at least one
            field. For more information about the available fields, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/flow-log-records.html">Flow log records</a> 
            in the <i>Amazon VPC User Guide</i> or <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-flow-logs.html#flow-log-records">Transit Gateway Flow Log
                    records</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
         <p>Specify the fields using the <code>${field-id}</code> format, separated by spaces.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
            request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure
                idempotency</a>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the flow logs.</p> |
| `max_aggregation_interval` | i64 |  | <p>The maximum interval of time during which a flow of packets is captured and aggregated into a flow log record. 
            The possible values are 60 seconds (1 minute) or 600 seconds (10 minutes).
            This parameter must be 60 seconds for transit gateway resource types.</p>
         <p>When a network interface is attached to a <a href="https://docs.aws.amazon.com/ec2/latest/instancetypes/ec2-nitro-instances.html">Nitro-based
                instance</a>, the aggregation interval is always 60 seconds or less, regardless
            of the value that you specify.</p>
         <p>Default: 600</p> |
| `destination_options` | String |  | <p>The destination options.</p> |
| `deliver_logs_permission_arn` | String |  | <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs to the log destination.</p>
         <p>This parameter is required if the destination type is <code>cloud-watch-logs</code>,
            or if the destination type is <code>kinesis-data-firehose</code> and the delivery stream
            and the resources to monitor are in different accounts.</p> |
| `log_group_name` | String |  | <p>The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your flow logs.</p>
         <p>This parameter is valid only if the destination type is <code>cloud-watch-logs</code>.</p> |
| `deliver_cross_account_role` | String |  | <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `flow_logs` | Vec<String> | <p>Information about the flow logs.</p> |
| `next_token` | String | <p>The token to request the next page of items. This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create flow_logs
flow_logs = provider.ec2.Flow_logs {
    resource_ids = "value"  # <p>The IDs of the resources to monitor. For example, if the resource type is
                <code>VPC</code>, specify the IDs of the VPCs.</p>
         <p>Constraints: Maximum of 25 for transit gateway resource types. Maximum of 1000 for the
            other resource types.</p>
    resource_type = "value"  # <p>The type of resource to monitor.</p>
}

# Access flow_logs outputs
flow_logs_id = flow_logs.id
flow_logs_flow_logs = flow_logs.flow_logs
flow_logs_next_token = flow_logs.next_token
```

---


### Network_insights_paths

NetworkInsightsPaths resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network_insights_paths` | Vec<String> | <p>Information about the paths.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_insights_paths outputs
network_insights_paths_id = network_insights_paths.id
network_insights_paths_network_insights_paths = network_insights_paths.network_insights_paths
network_insights_paths_next_token = network_insights_paths.next_token
```

---


### Image_block_public_access_state

ImageBlockPublicAccessState resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_by` | String | <p>The entity that manages the state for block public access for AMIs. Possible values
            include:</p>
         <ul>
            <li>
               <p>
                  <code>account</code> -  The state is managed by the account.</p>
            </li>
            <li>
               <p>
                  <code>declarative-policy</code> - The state is managed by a declarative policy and
            can't be modified by the account.</p>
            </li>
         </ul> |
| `image_block_public_access_state` | String | <p>The current state of block public access for AMIs at the account level in the specified
      Amazon Web Services Region.</p>
         <p>Possible values:</p>
         <ul>
            <li>
               <p>
                  <code>block-new-sharing</code> - Any attempt to publicly share your AMIs in the
          specified Region is blocked.</p>
            </li>
            <li>
               <p>
                  <code>unblocked</code> - Your AMIs in the specified Region can be publicly
          shared.</p>
            </li>
         </ul> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access image_block_public_access_state outputs
image_block_public_access_state_id = image_block_public_access_state.id
image_block_public_access_state_managed_by = image_block_public_access_state.managed_by
image_block_public_access_state_image_block_public_access_state = image_block_public_access_state.image_block_public_access_state
```

---


### Transit_gateway_policy_table_entries

TransitGatewayPolicyTableEntries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_policy_table_entries` | Vec<String> | <p>The entries for the transit gateway policy table.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_policy_table_entries outputs
transit_gateway_policy_table_entries_id = transit_gateway_policy_table_entries.id
transit_gateway_policy_table_entries_transit_gateway_policy_table_entries = transit_gateway_policy_table_entries.transit_gateway_policy_table_entries
```

---


### Reserved_instances_offerings

ReservedInstancesOfferings resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code>
      when there are no more results to return.</p> |
| `reserved_instances_offerings` | Vec<String> | <p>A list of Reserved Instances offerings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access reserved_instances_offerings outputs
reserved_instances_offerings_id = reserved_instances_offerings.id
reserved_instances_offerings_next_token = reserved_instances_offerings.next_token
reserved_instances_offerings_reserved_instances_offerings = reserved_instances_offerings.reserved_instances_offerings
```

---


### Transit_gateway_route_table_announcement

TransitGatewayRouteTableAnnouncement resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transit_gateway_route_table_id` | String | ✅ | <p>The ID of the transit gateway route table.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags specifications applied to the transit gateway route table announcement.</p> |
| `peering_attachment_id` | String | ✅ | <p>The ID of the peering attachment.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_route_table_announcement
transit_gateway_route_table_announcement = provider.ec2.Transit_gateway_route_table_announcement {
    transit_gateway_route_table_id = "value"  # <p>The ID of the transit gateway route table.</p>
    peering_attachment_id = "value"  # <p>The ID of the peering attachment.</p>
}

```

---


### Ipam_prefix_list_resolvers

IpamPrefixListResolvers resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipam_prefix_list_resolvers` | Vec<String> | <p>Information about the IPAM prefix list resolvers.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_prefix_list_resolvers outputs
ipam_prefix_list_resolvers_id = ipam_prefix_list_resolvers.id
ipam_prefix_list_resolvers_ipam_prefix_list_resolvers = ipam_prefix_list_resolvers.ipam_prefix_list_resolvers
ipam_prefix_list_resolvers_next_token = ipam_prefix_list_resolvers.next_token
```

---


### Verified_access_endpoint_targets

VerifiedAccessEndpointTargets resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `verified_access_endpoint_targets` | Vec<String> | <p>The Verified Access targets.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access verified_access_endpoint_targets outputs
verified_access_endpoint_targets_id = verified_access_endpoint_targets.id
verified_access_endpoint_targets_verified_access_endpoint_targets = verified_access_endpoint_targets.verified_access_endpoint_targets
verified_access_endpoint_targets_next_token = verified_access_endpoint_targets.next_token
```

---


### Instance_types_from_instance_requirements

InstanceTypesFromInstanceRequirements resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instance_types` | Vec<String> | <p>The instance types with the specified instance attributes.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_types_from_instance_requirements outputs
instance_types_from_instance_requirements_id = instance_types_from_instance_requirements.id
instance_types_from_instance_requirements_instance_types = instance_types_from_instance_requirements.instance_types
instance_types_from_instance_requirements_next_token = instance_types_from_instance_requirements.next_token
```

---


### Carrier_gateways

CarrierGateways resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `carrier_gateways` | Vec<String> | <p>Information about the carrier gateway.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access carrier_gateways outputs
carrier_gateways_id = carrier_gateways.id
carrier_gateways_carrier_gateways = carrier_gateways.carrier_gateways
carrier_gateways_next_token = carrier_gateways.next_token
```

---


### Transit_gateway_vpc_attachments

TransitGatewayVpcAttachments resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transit_gateway_vpc_attachments` | Vec<String> | <p>Information about the VPC attachments.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_vpc_attachments outputs
transit_gateway_vpc_attachments_id = transit_gateway_vpc_attachments.id
transit_gateway_vpc_attachments_transit_gateway_vpc_attachments = transit_gateway_vpc_attachments.transit_gateway_vpc_attachments
transit_gateway_vpc_attachments_next_token = transit_gateway_vpc_attachments.next_token
```

---


### Snapshots

Snapshots resource

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_specification` | String | ✅ | <p>The instance to specify which volumes should be included in the snapshots.</p> |
| `outpost_arn` | String |  | <note>
            <p>Only supported for instances on Outposts. If the source instance is not on an Outpost, 
        omit this parameter.</p>
         </note>
         <ul>
            <li>
               <p>To create the snapshots on the same Outpost as the source instance, specify the 
          ARN of that Outpost. The snapshots must be created on the same Outpost as the instance.</p>
            </li>
            <li>
               <p>To create the snapshots in the parent Region of the Outpost, omit this parameter.</p>
            </li>
         </ul>
         <p>For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/snapshots-outposts.html#create-snapshot">
      Create local snapshots from volumes on an Outpost</a> in the <i>Amazon EBS User Guide</i>.</p> |
| `description` | String |  | <p> A description propagated to every snapshot specified by the instance.</p> |
| `tag_specifications` | Vec<String> |  | <p>Tags to apply to every snapshot specified by the instance.</p> |
| `copy_tags_from_source` | String |  | <p>Copies the tags from the specified volume to corresponding snapshot.</p> |
| `location` | String |  | <note>
            <p>Only supported for instances in Local Zones. If the source instance is not in a Local Zone, 
        omit this parameter.</p>
         </note>
         <ul>
            <li>
               <p>To create local snapshots in the same Local Zone as the source instance, specify 
          <code>local</code>.</p>
            </li>
            <li>
               <p>To create regional snapshots in the parent Region of the Local Zone, specify 
          <code>regional</code> or omit this parameter.</p>
            </li>
         </ul>
         <p>Default value: <code>regional</code>
         </p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
  This value is <code>null</code> when there are no more items to return.</p> |
| `snapshots` | Vec<String> | <p>Information about the snapshots.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create snapshots
snapshots = provider.ec2.Snapshots {
    instance_specification = "value"  # <p>The instance to specify which volumes should be included in the snapshots.</p>
}

# Access snapshots outputs
snapshots_id = snapshots.id
snapshots_next_token = snapshots.next_token
snapshots_snapshots = snapshots.snapshots
```

---


### Local_gateway_virtual_interfaces

LocalGatewayVirtualInterfaces resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `local_gateway_virtual_interfaces` | Vec<String> | <p>Information about the virtual interfaces.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access local_gateway_virtual_interfaces outputs
local_gateway_virtual_interfaces_id = local_gateway_virtual_interfaces.id
local_gateway_virtual_interfaces_next_token = local_gateway_virtual_interfaces.next_token
local_gateway_virtual_interfaces_local_gateway_virtual_interfaces = local_gateway_virtual_interfaces.local_gateway_virtual_interfaces
```

---


### Ipam_resource_discovery_associations

IpamResourceDiscoveryAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipam_resource_discovery_associations` | Vec<String> | <p>The resource discovery associations.</p> |
| `next_token` | String | <p>Specify the pagination token from a previous request to retrieve the next page of results.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_resource_discovery_associations outputs
ipam_resource_discovery_associations_id = ipam_resource_discovery_associations.id
ipam_resource_discovery_associations_ipam_resource_discovery_associations = ipam_resource_discovery_associations.ipam_resource_discovery_associations
ipam_resource_discovery_associations_next_token = ipam_resource_discovery_associations.next_token
```

---


### Nat_gateways

NatGateways resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |
| `nat_gateways` | Vec<String> | <p>Information about the NAT gateways.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access nat_gateways outputs
nat_gateways_id = nat_gateways.id
nat_gateways_next_token = nat_gateways.next_token
nat_gateways_nat_gateways = nat_gateways.nat_gateways
```

---


### Images

Images resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `images` | Vec<String> | <p>Information about the images.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access images outputs
images_id = images.id
images_images = images.images
images_next_token = images.next_token
```

---


### Image

Image resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String | ✅ | <p>The ID of the instance.</p> |
| `description` | String |  | <p>A description for the new image.</p> |
| `no_reboot` | bool |  | <p>Indicates whether or not the instance should be automatically rebooted before creating the
      image. Specify one of the following values:</p>
         <ul>
            <li>
               <p>
                  <code>true</code> - The instance is not rebooted before creating the image. This
          creates crash-consistent snapshots that include only the data that has been written to the
          volumes at the time the snapshots are created. Buffered data and data in memory that has
          not yet been written to the volumes is not included in the snapshots.</p>
            </li>
            <li>
               <p>
                  <code>false</code> - The instance is rebooted before creating the image. This ensures
          that all buffered data and data in memory is written to the volumes before the snapshots
          are created.</p>
            </li>
         </ul>
         <p>Default: <code>false</code>
         </p> |
| `name` | String | ✅ | <p>A name for the new image.</p>
         <p>Constraints: 3-128 alphanumeric characters, parentheses (()), square brackets ([]), spaces
      ( ), periods (.), slashes (/), dashes (-), single quotes ('), at-signs (@), or
      underscores(_)</p> |
| `block_device_mappings` | Vec<String> |  | <p>The block device mappings.</p>
         <p>When using the CreateImage action:</p>
         <ul>
            <li>
               <p>You can't change the volume size using the VolumeSize parameter. If you want a
          different volume size, you must first change the volume size of the source
          instance.</p>
            </li>
            <li>
               <p>You can't modify the encryption status of existing volumes or snapshots. To create an
          AMI with volumes or snapshots that have a different encryption status (for example, where
          the source volume and snapshots are unencrypted, and you want to create an AMI with
          encrypted volumes or snapshots), copy the image instead.</p>
            </li>
            <li>
               <p>The only option that can be changed for existing mappings or snapshots is
            <code>DeleteOnTermination</code>.</p>
            </li>
         </ul> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
			and provides an error response. If you have the required permissions, the error response is 
			<code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the AMI and snapshots on creation. You can tag the AMI, the
      snapshots, or both.</p>
         <ul>
            <li>
               <p>To tag the AMI, the value for <code>ResourceType</code> must be
          <code>image</code>.</p>
            </li>
            <li>
               <p>To tag the snapshots that are created of the root volume and of other Amazon EBS volumes
          that are attached to the instance, the value for <code>ResourceType</code> must be
            <code>snapshot</code>. The same tag is applied to all of the snapshots that are
          created.</p>
            </li>
         </ul>
         <p>If you specify other values for <code>ResourceType</code>, the request fails.</p>
         <p>To tag an AMI or snapshot after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p> |
| `snapshot_location` | String |  | <note>
            <p>Only supported for instances in Local Zones. If the source instance is not in a Local
        Zone, omit this parameter.</p>
         </note>
         <p>The Amazon S3 location where the snapshots will be stored.</p>
         <ul>
            <li>
               <p>To create local snapshots in the same Local Zone as the source instance, specify
          <code>local</code>.</p>
            </li>
            <li>
               <p>To create regional snapshots in the parent Region of the Local Zone, specify
          <code>regional</code> or omit this parameter.</p>
            </li>
         </ul>
         <p>Default: <code>regional</code>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create image
image = provider.ec2.Image {
    instance_id = "value"  # <p>The ID of the instance.</p>
    name = "value"  # <p>A name for the new image.</p>
         <p>Constraints: 3-128 alphanumeric characters, parentheses (()), square brackets ([]), spaces
      ( ), periods (.), slashes (/), dashes (-), single quotes ('), at-signs (@), or
      underscores(_)</p>
}

```

---


### Nat_gateway

NatGateway resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `allocation_id` | String |  | <p>[Public NAT gateways only] The allocation ID of an Elastic IP address to associate 
          with the NAT gateway. You cannot specify an Elastic IP address with a private NAT gateway.
          If the Elastic IP address is associated with another resource, you must first disassociate it.</p> |
| `secondary_private_ip_address_count` | i64 |  | <p>[Private NAT gateway only] The number of secondary private IPv4 addresses you want to assign to the NAT gateway. 
            For more information about secondary addresses, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/nat-gateway-working-with.html">Create a NAT gateway</a> 
            in the <i>Amazon VPC User Guide</i>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the NAT gateway.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
			request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p>
         <p>Constraint: Maximum 64 ASCII characters.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `subnet_id` | String | ✅ | <p>The ID of the subnet in which to create the NAT gateway.</p> |
| `secondary_private_ip_addresses` | Vec<String> |  | <p>Secondary private IPv4 addresses. For more information about secondary addresses, see 
            <a href="https://docs.aws.amazon.com/vpc/latest/userguide/nat-gateway-working-with.html">Create a NAT gateway</a> in the <i>Amazon VPC User Guide</i>.</p> |
| `connectivity_type` | String |  | <p>Indicates whether the NAT gateway supports public or private connectivity. 
          The default is public connectivity.</p> |
| `private_ip_address` | String |  | <p>The private IPv4 address to assign to the NAT gateway. If you don't provide an address, a private IPv4 address will be automatically assigned.</p> |
| `secondary_allocation_ids` | Vec<String> |  | <p>Secondary EIP allocation IDs. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/nat-gateway-working-with.html">Create a NAT gateway</a> 
            in the <i>Amazon VPC User Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create nat_gateway
nat_gateway = provider.ec2.Nat_gateway {
    subnet_id = "value"  # <p>The ID of the subnet in which to create the NAT gateway.</p>
}

```

---


### Route_server_endpoints

RouteServerEndpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `route_server_endpoints` | Vec<String> | <p>Information about the described route server endpoints.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access route_server_endpoints outputs
route_server_endpoints_id = route_server_endpoints.id
route_server_endpoints_route_server_endpoints = route_server_endpoints.route_server_endpoints
route_server_endpoints_next_token = route_server_endpoints.next_token
```

---


### Route_server

RouteServer resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `amazon_side_asn` | i64 | ✅ | <p>The private Autonomous System Number (ASN) for the Amazon side of the BGP session. Valid values are from 1 to 4294967295. We recommend using a private ASN in the 64512–65534 (16-bit ASN) or 4200000000–4294967294 (32-bit ASN) range.</p> |
| `persist_routes_duration` | i64 |  | <p>The number of minutes a route server will wait after BGP is re-established to unpersist the routes in the FIB and RIB. Value must be in the range of 1-5. Required if PersistRoutes is <code>enabled</code>.</p>
         <p>If you set the duration to 1 minute, then when your network appliance re-establishes BGP with route server, it has 1 minute to relearn it's adjacent network and advertise those routes to route server before route server resumes normal functionality. In most cases, 1 minute is probably sufficient. If, however, you have concerns that your BGP network may not be capable of fully re-establishing and re-learning everything in 1 minute, you can increase the duration up to 5 minutes.</p> |
| `sns_notifications_enabled` | bool |  | <p>Indicates whether SNS notifications should be enabled for route server events. Enabling SNS notifications persists BGP status changes to an SNS topic provisioned by Amazon Web Services.</p> |
| `persist_routes` | String |  | <p>Indicates whether routes should be persisted after all BGP sessions are terminated.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier to ensure idempotency of the request.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the route server during creation.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create route_server
route_server = provider.ec2.Route_server {
    amazon_side_asn = "value"  # <p>The private Autonomous System Number (ASN) for the Amazon side of the BGP session. Valid values are from 1 to 4294967295. We recommend using a private ASN in the 64512–65534 (16-bit ASN) or 4200000000–4294967294 (32-bit ASN) range.</p>
}

```

---


### Addresses_attribute

AddressesAttribute resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `addresses` | Vec<String> | <p>Information about the IP addresses.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access addresses_attribute outputs
addresses_attribute_id = addresses_attribute.id
addresses_attribute_next_token = addresses_attribute.next_token
addresses_attribute_addresses = addresses_attribute.addresses
```

---


### Host_reservation_purchase_preview

HostReservationPurchasePreview resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `purchase` | Vec<String> | <p>The purchase information of the Dedicated Host reservation and the Dedicated Hosts
            associated with it.</p> |
| `currency_code` | String | <p>The currency in which the <code>totalUpfrontPrice</code> and
                <code>totalHourlyPrice</code> amounts are specified. At this time, the only
            supported currency is <code>USD</code>.</p> |
| `total_upfront_price` | String | <p>The potential total upfront price. This is billed immediately.</p> |
| `total_hourly_price` | String | <p>The potential total hourly price of the reservation per hour.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access host_reservation_purchase_preview outputs
host_reservation_purchase_preview_id = host_reservation_purchase_preview.id
host_reservation_purchase_preview_purchase = host_reservation_purchase_preview.purchase
host_reservation_purchase_preview_currency_code = host_reservation_purchase_preview.currency_code
host_reservation_purchase_preview_total_upfront_price = host_reservation_purchase_preview.total_upfront_price
host_reservation_purchase_preview_total_hourly_price = host_reservation_purchase_preview.total_hourly_price
```

---


### Instance_types

InstanceTypes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `instance_types` | Vec<String> | <p>The instance type.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_types outputs
instance_types_id = instance_types.id
instance_types_next_token = instance_types.next_token
instance_types_instance_types = instance_types.instance_types
```

---


### Route_server_routing_database

RouteServerRoutingDatabase resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `routes` | Vec<String> | <p>The collection of routes in the route server's routing database.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `are_routes_persisted` | bool | <p>Indicates whether routes are being persisted in the routing database.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access route_server_routing_database outputs
route_server_routing_database_id = route_server_routing_database.id
route_server_routing_database_routes = route_server_routing_database.routes
route_server_routing_database_next_token = route_server_routing_database.next_token
route_server_routing_database_are_routes_persisted = route_server_routing_database.are_routes_persisted
```

---


### Trunk_interface_associations

TrunkInterfaceAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `interface_associations` | Vec<String> | <p>Information about the trunk associations.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access trunk_interface_associations outputs
trunk_interface_associations_id = trunk_interface_associations.id
trunk_interface_associations_interface_associations = trunk_interface_associations.interface_associations
trunk_interface_associations_next_token = trunk_interface_associations.next_token
```

---


### Verified_access_endpoints

VerifiedAccessEndpoints resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `verified_access_endpoints` | Vec<String> | <p>Details about the Verified Access endpoints.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access verified_access_endpoints outputs
verified_access_endpoints_id = verified_access_endpoints.id
verified_access_endpoints_verified_access_endpoints = verified_access_endpoints.verified_access_endpoints
verified_access_endpoints_next_token = verified_access_endpoints.next_token
```

---


### Verified_access_trust_provider

VerifiedAccessTrustProvider resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `oidc_options` | String |  | <p>The options for a OpenID Connect-compatible user-identity trust provider. This parameter
         is required when the provider type is <code>user</code>.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `device_options` | String |  | <p>The options for a device-based trust provider. This parameter is required when the
         provider type is <code>device</code>.</p> |
| `description` | String |  | <p>A description for the Verified Access trust provider.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the Verified Access trust provider.</p> |
| `client_token` | String |  | <p>A unique, case-sensitive token that you provide to ensure idempotency of your
            modification request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `sse_specification` | String |  | <p>The options for server side encryption.</p> |
| `native_application_oidc_options` | String |  | <p>The OpenID Connect (OIDC) options.</p> |
| `trust_provider_type` | String | ✅ | <p>The type of trust provider.</p> |
| `user_trust_provider_type` | String |  | <p>The type of user-based trust provider. This parameter is required when the provider type
         is <code>user</code>.</p> |
| `device_trust_provider_type` | String |  | <p>The type of device-based trust provider. This parameter is required when the provider
         type is <code>device</code>.</p> |
| `policy_reference_name` | String | ✅ | <p>The identifier to be used when working with policy rules.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create verified_access_trust_provider
verified_access_trust_provider = provider.ec2.Verified_access_trust_provider {
    trust_provider_type = "value"  # <p>The type of trust provider.</p>
    policy_reference_name = "value"  # <p>The identifier to be used when working with policy rules.</p>
}

```

---


### Spot_price_history

SpotPriceHistory resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is
            an empty string (<code>""</code>) or <code>null</code> when there are no more items to return.</p> |
| `spot_price_history` | Vec<String> | <p>The historical Spot prices.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access spot_price_history outputs
spot_price_history_id = spot_price_history.id
spot_price_history_next_token = spot_price_history.next_token
spot_price_history_spot_price_history = spot_price_history.spot_price_history
```

---


### Vpc_attribute

VpcAttribute resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_network_address_usage_metrics` | String | <p>Indicates whether Network Address Usage metrics are enabled for your VPC.</p> |
| `enable_dns_hostnames` | String | <p>Indicates whether the instances launched in the VPC get DNS hostnames.
				If this attribute is <code>true</code>, instances in the VPC get DNS hostnames;
				otherwise, they do not.</p> |
| `vpc_id` | String | <p>The ID of the VPC.</p> |
| `enable_dns_support` | String | <p>Indicates whether DNS resolution is enabled for
				the VPC. If this attribute is <code>true</code>, the Amazon DNS server
				resolves DNS hostnames for your instances to their corresponding
				IP addresses; otherwise, it does not.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpc_attribute outputs
vpc_attribute_id = vpc_attribute.id
vpc_attribute_enable_network_address_usage_metrics = vpc_attribute.enable_network_address_usage_metrics
vpc_attribute_enable_dns_hostnames = vpc_attribute.enable_dns_hostnames
vpc_attribute_vpc_id = vpc_attribute.vpc_id
vpc_attribute_enable_dns_support = vpc_attribute.enable_dns_support
```

---


### Spot_instance_requests

SpotInstanceRequests resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spot_instance_requests` | Vec<String> | <p>The Spot Instance requests.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access spot_instance_requests outputs
spot_instance_requests_id = spot_instance_requests.id
spot_instance_requests_spot_instance_requests = spot_instance_requests.spot_instance_requests
spot_instance_requests_next_token = spot_instance_requests.next_token
```

---


### Instance_metadata_defaults

InstanceMetadataDefaults resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_level` | String | <p>The account-level default IMDS settings.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access instance_metadata_defaults outputs
instance_metadata_defaults_id = instance_metadata_defaults.id
instance_metadata_defaults_account_level = instance_metadata_defaults.account_level
```

---


### Ipam_prefix_list_resolver_version_entries

IpamPrefixListResolverVersionEntries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `entries` | Vec<String> | <p>The CIDR entries for the specified resolver version.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_prefix_list_resolver_version_entries outputs
ipam_prefix_list_resolver_version_entries_id = ipam_prefix_list_resolver_version_entries.id
ipam_prefix_list_resolver_version_entries_next_token = ipam_prefix_list_resolver_version_entries.next_token
ipam_prefix_list_resolver_version_entries_entries = ipam_prefix_list_resolver_version_entries.entries
```

---


### Network_insights_path

NetworkInsightsPath resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_ip` | String |  | <p>The IP address of the source.</p> |
| `destination_ip` | String |  | <p>The IP address of the destination.</p> |
| `destination_port` | i64 |  | <p>The destination port.</p> |
| `destination` | String |  | <p>The ID or ARN of the destination. If the resource is in another account, you must specify an ARN.</p> |
| `protocol` | String | ✅ | <p>The protocol.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to add to the path.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `client_token` | String | ✅ | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, 
   see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure idempotency</a>.</p> |
| `filter_at_source` | String |  | <p>Scopes the analysis to network paths that match specific filters at the source. If you specify
          this parameter, you can't specify the parameters for the source IP address or the destination port.</p> |
| `source` | String | ✅ | <p>The ID or ARN of the source. If the resource is in another account, you must specify an ARN.</p> |
| `filter_at_destination` | String |  | <p>Scopes the analysis to network paths that match specific filters at the destination. If you specify
          this parameter, you can't specify the parameter for the destination IP address.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create network_insights_path
network_insights_path = provider.ec2.Network_insights_path {
    protocol = "value"  # <p>The protocol.</p>
    client_token = "value"  # <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, 
   see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure idempotency</a>.</p>
    source = "value"  # <p>The ID or ARN of the source. If the resource is in another account, you must specify an ARN.</p>
}

```

---


### Ipam_scope

IpamScope resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_token` | String |  | <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `ipam_id` | String | ✅ | <p>The ID of the IPAM for which you're creating this scope.</p> |
| `description` | String |  | <p>A description for the scope you're creating.</p> |
| `tag_specifications` | Vec<String> |  | <p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value.
    For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p> |
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create ipam_scope
ipam_scope = provider.ec2.Ipam_scope {
    ipam_id = "value"  # <p>The ID of the IPAM for which you're creating this scope.</p>
}

```

---


### Identity_id_format

IdentityIdFormat resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `statuses` | Vec<String> | <p>Information about the ID format for the resources.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access identity_id_format outputs
identity_id_format_id = identity_id_format.id
identity_id_format_statuses = identity_id_format.statuses
```

---


### Managed_prefix_list_entries

ManagedPrefixListEntries resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entries` | Vec<String> | <p>Information about the prefix list entries.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access managed_prefix_list_entries outputs
managed_prefix_list_entries_id = managed_prefix_list_entries.id
managed_prefix_list_entries_entries = managed_prefix_list_entries.entries
managed_prefix_list_entries_next_token = managed_prefix_list_entries.next_token
```

---


### Transit_gateway_multicast_domain_associations

TransitGatewayMulticastDomainAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `multicast_domain_associations` | Vec<String> | <p>Information about the multicast domain associations.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_multicast_domain_associations outputs
transit_gateway_multicast_domain_associations_id = transit_gateway_multicast_domain_associations.id
transit_gateway_multicast_domain_associations_multicast_domain_associations = transit_gateway_multicast_domain_associations.multicast_domain_associations
transit_gateway_multicast_domain_associations_next_token = transit_gateway_multicast_domain_associations.next_token
```

---


### Spot_placement_scores

SpotPlacementScores resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spot_placement_scores` | Vec<String> | <p>The Spot placement score for the top 10 Regions or Availability Zones, scored on a scale
         from 1 to 10. Each score  reflects how likely it is that each Region or Availability Zone
         will succeed at fulfilling the specified target capacity  <i>at the time of the Spot
            placement score request</i>. A score of <code>10</code> means that your Spot
         capacity request is highly likely to succeed in that Region or Availability Zone. </p>
         <p>If you request a Spot placement score for Regions, a high score assumes that your fleet
         request will be configured to use all Availability Zones and the
         <code>capacity-optimized</code> allocation strategy. If you request a Spot placement
         score for Availability Zones, a high score assumes that your fleet request will be
         configured to use a single Availability Zone and the <code>capacity-optimized</code>
         allocation strategy.</p>
         <p>Different  Regions or Availability Zones might return the same score.</p>
         <note>
            <p>The Spot placement score serves as a recommendation only. No score guarantees that your
            Spot request will be fully or partially fulfilled.</p>
         </note> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access spot_placement_scores outputs
spot_placement_scores_id = spot_placement_scores.id
spot_placement_scores_spot_placement_scores = spot_placement_scores.spot_placement_scores
spot_placement_scores_next_token = spot_placement_scores.next_token
```

---


### Transit_gateway_prefix_list_references

TransitGatewayPrefixListReferences resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `transit_gateway_prefix_list_references` | Vec<String> | <p>Information about the prefix list references.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_prefix_list_references outputs
transit_gateway_prefix_list_references_id = transit_gateway_prefix_list_references.id
transit_gateway_prefix_list_references_next_token = transit_gateway_prefix_list_references.next_token
transit_gateway_prefix_list_references_transit_gateway_prefix_list_references = transit_gateway_prefix_list_references.transit_gateway_prefix_list_references
```

---


### Client_vpn_routes

ClientVpnRoutes resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `routes` | Vec<String> | <p>Information about the Client VPN endpoint routes.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access client_vpn_routes outputs
client_vpn_routes_id = client_vpn_routes.id
client_vpn_routes_next_token = client_vpn_routes.next_token
client_vpn_routes_routes = client_vpn_routes.routes
```

---


### Ipv6_pools

Ipv6Pools resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipv6_pools` | Vec<String> | <p>Information about the IPv6 address pools.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipv6_pools outputs
ipv6_pools_id = ipv6_pools.id
ipv6_pools_ipv6_pools = ipv6_pools.ipv6_pools
ipv6_pools_next_token = ipv6_pools.next_token
```

---


### Transit_gateway_route_table_associations

TransitGatewayRouteTableAssociations resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | <p>Information about the associations.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access transit_gateway_route_table_associations outputs
transit_gateway_route_table_associations_id = transit_gateway_route_table_associations.id
transit_gateway_route_table_associations_associations = transit_gateway_route_table_associations.associations
transit_gateway_route_table_associations_next_token = transit_gateway_route_table_associations.next_token
```

---


### Instance_export_task

InstanceExportTask resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | <p>A description for the conversion task or the resource being exported. The maximum length is 255 characters.</p> |
| `export_to_s3_task` | String | ✅ | <p>The format and location for an export instance task.</p> |
| `target_environment` | String | ✅ | <p>The target virtualization environment.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the export instance task during creation.</p> |
| `instance_id` | String | ✅ | <p>The ID of the instance.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance_export_task
instance_export_task = provider.ec2.Instance_export_task {
    export_to_s3_task = "value"  # <p>The format and location for an export instance task.</p>
    target_environment = "value"  # <p>The target virtualization environment.</p>
    instance_id = "value"  # <p>The ID of the instance.</p>
}

```

---


### Traffic_mirror_filter

TrafficMirrorFilter resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `description` | String |  | <p>The description of the Traffic Mirror filter.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">How to ensure idempotency</a>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to a Traffic Mirror filter.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create traffic_mirror_filter
traffic_mirror_filter = provider.ec2.Traffic_mirror_filter {
}

```

---


### Transit_gateway_vpc_attachment

TransitGatewayVpcAttachment resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transit_gateway_id` | String | ✅ | <p>The ID of the transit gateway.</p> |
| `subnet_ids` | Vec<String> | ✅ | <p>The IDs of one or more subnets. You can specify only one subnet per Availability Zone. 
         You must specify at least one subnet, but we recommend that you specify two subnets for better availability.
         The transit gateway uses one IP address from each specified subnet.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the VPC attachment.</p> |
| `vpc_id` | String | ✅ | <p>The ID of the VPC.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `options` | String |  | <p>The VPC attachment options.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create transit_gateway_vpc_attachment
transit_gateway_vpc_attachment = provider.ec2.Transit_gateway_vpc_attachment {
    transit_gateway_id = "value"  # <p>The ID of the transit gateway.</p>
    subnet_ids = "value"  # <p>The IDs of one or more subnets. You can specify only one subnet per Availability Zone. 
         You must specify at least one subnet, but we recommend that you specify two subnets for better availability.
         The transit gateway uses one IP address from each specified subnet.</p>
    vpc_id = "value"  # <p>The ID of the VPC.</p>
}

```

---


### Instance_connect_endpoint

InstanceConnectEndpoint resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
            and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
            Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `preserve_client_ip` | bool |  | <p>Indicates whether the client IP address is preserved as the source. The following are the possible values.</p>
         <ul>
            <li>
               <p>
                  <code>true</code> - Use the client IP address as the source.</p>
            </li>
            <li>
               <p>
                  <code>false</code> - Use the network interface IP address as the source.</p>
            </li>
         </ul>
         <note>
            <p>
               <code>PreserveClientIp</code> is only supported on IPv4 EC2 Instance Connect
                Endpoints. To use <code>PreserveClientIp</code>, the value for
                    <code>IpAddressType</code> must be <code>ipv4</code>.</p>
         </note>
         <p>Default: <code>false</code>
         </p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the EC2 Instance Connect Endpoint during creation.</p> |
| `security_group_ids` | Vec<String> |  | <p>One or more security groups to associate with the endpoint. If you don't specify a security group, 
            the default security group for your VPC will be associated with the endpoint.</p> |
| `ip_address_type` | String |  | <p>The IP address type of the endpoint.</p>
         <p>If no value is specified, the default value is determined by the IP address type of
            the subnet:</p>
         <ul>
            <li>
               <p>
                  <code>dualstack</code> - If the subnet has both IPv4 and IPv6 CIDRs</p>
            </li>
            <li>
               <p>
                  <code>ipv4</code> - If the subnet has only IPv4 CIDRs</p>
            </li>
            <li>
               <p>
                  <code>ipv6</code> - If the subnet has only IPv6 CIDRs</p>
            </li>
         </ul>
         <note>
            <p>
               <code>PreserveClientIp</code> is only supported on IPv4 EC2 Instance Connect
                Endpoints. To use <code>PreserveClientIp</code>, the value for
                <code>IpAddressType</code> must be <code>ipv4</code>.</p>
         </note> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p> |
| `subnet_id` | String | ✅ | <p>The ID of the subnet in which to create the EC2 Instance Connect Endpoint.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create instance_connect_endpoint
instance_connect_endpoint = provider.ec2.Instance_connect_endpoint {
    subnet_id = "value"  # <p>The ID of the subnet in which to create the EC2 Instance Connect Endpoint.</p>
}

```

---


### Capacity_reservation_by_splitting

CapacityReservationBySplitting resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p> |
| `source_capacity_reservation_id` | String | ✅ | <p> The ID of the Capacity Reservation from which you want to split the capacity. </p> |
| `instance_count` | i64 | ✅ | <p> The number of instances to split from the source Capacity Reservation. </p> |
| `tag_specifications` | Vec<String> |  | <p> The tags to apply to the new Capacity Reservation. </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create capacity_reservation_by_splitting
capacity_reservation_by_splitting = provider.ec2.Capacity_reservation_by_splitting {
    source_capacity_reservation_id = "value"  # <p> The ID of the Capacity Reservation from which you want to split the capacity. </p>
    instance_count = "value"  # <p> The number of instances to split from the source Capacity Reservation. </p>
}

```

---


### Local_gateways

LocalGateways resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `local_gateways` | Vec<String> | <p>Information about the local gateways.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access local_gateways outputs
local_gateways_id = local_gateways.id
local_gateways_next_token = local_gateways.next_token
local_gateways_local_gateways = local_gateways.local_gateways
```

---


### Locked_snapshots

LockedSnapshots resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `snapshots` | Vec<String> | <p>Information about the snapshots.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
  This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access locked_snapshots outputs
locked_snapshots_id = locked_snapshots.id
locked_snapshots_snapshots = locked_snapshots.snapshots
locked_snapshots_next_token = locked_snapshots.next_token
```

---


### Vpn_connections

VpnConnections resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vpn_connections` | Vec<String> | <p>Information about one or more VPN connections.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access vpn_connections outputs
vpn_connections_id = vpn_connections.id
vpn_connections_vpn_connections = vpn_connections.vpn_connections
```

---


### Fpga_image

FpgaImage resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | <p>A name for the AFI.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. 
      	For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring Idempotency</a>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the FPGA image during creation.</p> |
| `input_storage_location` | String | ✅ | <p>The location of the encrypted design checkpoint in Amazon S3. The input must be a tarball.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `description` | String |  | <p>A description for the AFI.</p> |
| `logs_storage_location` | String |  | <p>The location in Amazon S3 for the output logs.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create fpga_image
fpga_image = provider.ec2.Fpga_image {
    input_storage_location = "value"  # <p>The location of the encrypted design checkpoint in Amazon S3. The input must be a tarball.</p>
}

```

---


### Network_interface_attribute

NetworkInterfaceAttribute resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | <p>The description of the network interface.</p> |
| `source_dest_check` | String | <p>Indicates whether source/destination checking is enabled.</p> |
| `network_interface_id` | String | <p>The ID of the network interface.</p> |
| `attachment` | String | <p>The attachment (if any) of the network interface.</p> |
| `groups` | Vec<String> | <p>The security groups associated with the network interface.</p> |
| `associate_public_ip_address` | bool | <p>Indicates whether to assign a public IPv4 address to a network interface. This option
            can be enabled for any network interface but will only apply to the primary network
            interface (eth0).</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access network_interface_attribute outputs
network_interface_attribute_id = network_interface_attribute.id
network_interface_attribute_description = network_interface_attribute.description
network_interface_attribute_source_dest_check = network_interface_attribute.source_dest_check
network_interface_attribute_network_interface_id = network_interface_attribute.network_interface_id
network_interface_attribute_attachment = network_interface_attribute.attachment
network_interface_attribute_groups = network_interface_attribute.groups
network_interface_attribute_associate_public_ip_address = network_interface_attribute.associate_public_ip_address
```

---


### Ipam_pool_cidrs

IpamPoolCidrs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |
| `ipam_pool_cidrs` | Vec<String> | <p>Information about the CIDRs provisioned to an IPAM pool.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipam_pool_cidrs outputs
ipam_pool_cidrs_id = ipam_pool_cidrs.id
ipam_pool_cidrs_next_token = ipam_pool_cidrs.next_token
ipam_pool_cidrs_ipam_pool_cidrs = ipam_pool_cidrs.ipam_pool_cidrs
```

---


### Ipams

Ipams resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ipams` | Vec<String> | <p>Information about the IPAMs.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access ipams outputs
ipams_id = ipams.id
ipams_ipams = ipams.ipams
ipams_next_token = ipams.next_token
```

---


### Public_ipv4_pool

PublicIpv4Pool resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_specifications` | Vec<String> |  | <p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value.
    For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p> |
| `dry_run` | bool |  | <p>A check for whether you have the required permissions for the action without actually making the request 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `network_border_group` | String |  | <p>The Availability Zone (AZ) or Local Zone (LZ) network border group that the resource that the IP address is assigned to is in. Defaults to an AZ network border group. For more information on available Local Zones, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-byoip.html#byoip-zone-avail">Local Zone availability</a> in the <i>Amazon EC2 User Guide</i>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create public_ipv4_pool
public_ipv4_pool = provider.ec2.Public_ipv4_pool {
}

```

---


### Byoip_cidrs

ByoipCidrs resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `byoip_cidrs` | Vec<String> | <p>Information about your address ranges.</p> |
| `next_token` | String | <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access byoip_cidrs outputs
byoip_cidrs_id = byoip_cidrs.id
byoip_cidrs_byoip_cidrs = byoip_cidrs.byoip_cidrs
byoip_cidrs_next_token = byoip_cidrs.next_token
```

---


### Capacity_reservation_topology

CapacityReservationTopology resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |
| `capacity_reservations` | Vec<String> | <p>Information about the topology of each Capacity Reservation.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_reservation_topology outputs
capacity_reservation_topology_id = capacity_reservation_topology.id
capacity_reservation_topology_next_token = capacity_reservation_topology.next_token
capacity_reservation_topology_capacity_reservations = capacity_reservation_topology.capacity_reservations
```

---


### Classic_link_instances

ClassicLinkInstances resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p> |
| `instances` | Vec<String> | <p>Information about one or more linked EC2-Classic instances.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access classic_link_instances outputs
classic_link_instances_id = classic_link_instances.id
classic_link_instances_next_token = classic_link_instances.next_token
classic_link_instances_instances = classic_link_instances.instances
```

---


### Local_gateway_virtual_interface_group

LocalGatewayVirtualInterfaceGroup resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `local_bgp_asn` | i64 |  | <p>The Autonomous System Number(ASN) for the local Border Gateway Protocol (BGP).</p> |
| `local_bgp_asn_extended` | i64 |  | <p>The extended 32-bit ASN for the local BGP configuration.</p> |
| `local_gateway_id` | String | ✅ | <p>The ID of the local gateway.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the local gateway virtual interface group when the resource is
         being created.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create local_gateway_virtual_interface_group
local_gateway_virtual_interface_group = provider.ec2.Local_gateway_virtual_interface_group {
    local_gateway_id = "value"  # <p>The ID of the local gateway.</p>
}

```

---


### Route

Route resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instance_id` | String |  | <p>The ID of a NAT instance in your VPC. The operation fails if you specify an instance ID unless exactly one network interface is attached.</p> |
| `core_network_arn` | String |  | <p>The Amazon Resource Name (ARN) of the core network.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `odb_network_arn` | String |  | <p>The Amazon Resource Name (ARN) of the ODB network.</p> |
| `network_interface_id` | String |  | <p>The ID of a network interface.</p> |
| `nat_gateway_id` | String |  | <p>[IPv4 traffic only] The ID of a NAT gateway.</p> |
| `gateway_id` | String |  | <p>The ID of an internet gateway or virtual private gateway attached to your
			VPC.</p> |
| `vpc_endpoint_id` | String |  | <p>The ID of a VPC endpoint. Supported for Gateway Load Balancer endpoints only.</p> |
| `destination_ipv6_cidr_block` | String |  | <p>The IPv6 CIDR block used for the destination match. Routing decisions are based on the most specific match.</p> |
| `local_gateway_id` | String |  | <p>The ID of the local gateway.</p> |
| `carrier_gateway_id` | String |  | <p>The ID of the carrier gateway.</p>
         <p>You can only use this option when the VPC contains a subnet which is associated with a Wavelength Zone.</p> |
| `vpc_peering_connection_id` | String |  | <p>The ID of a VPC peering connection.</p> |
| `egress_only_internet_gateway_id` | String |  | <p>[IPv6 traffic only] The ID of an egress-only internet gateway.</p> |
| `transit_gateway_id` | String |  | <p>The ID of a transit gateway.</p> |
| `route_table_id` | String | ✅ | <p>The ID of the route table for the route.</p> |
| `destination_prefix_list_id` | String |  | <p>The ID of a prefix list used for the destination match.</p> |
| `destination_cidr_block` | String |  | <p>The IPv4 CIDR address block used for the destination match. Routing decisions are based on the most specific match. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create route
route = provider.ec2.Route {
    route_table_id = "value"  # <p>The ID of the route table for the route.</p>
}

```

---


### Capacity_manager_metric_dimensions

CapacityManagerMetricDimensions resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_token` | String | <p>
The token to use to retrieve the next page of results. This value is null when there are no more results to return.
</p> |
| `metric_dimension_results` | Vec<String> | <p>
The available dimension combinations that have data within the specified time range and filters.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_manager_metric_dimensions outputs
capacity_manager_metric_dimensions_id = capacity_manager_metric_dimensions.id
capacity_manager_metric_dimensions_next_token = capacity_manager_metric_dimensions.next_token
capacity_manager_metric_dimensions_metric_dimension_results = capacity_manager_metric_dimensions.metric_dimension_results
```

---


### Coip_cidr

CoipCidr resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cidr` | String | ✅ | <p>
      A customer-owned IP address range to create.
      </p> |
| `coip_pool_id` | String | ✅ | <p>
         The ID of the address pool.
      </p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create coip_cidr
coip_cidr = provider.ec2.Coip_cidr {
    cidr = "value"  # <p>
      A customer-owned IP address range to create.
      </p>
    coip_pool_id = "value"  # <p>
         The ID of the address pool.
      </p>
}

```

---


### Egress_only_internet_gateway

EgressOnlyInternetGateway resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, 
   and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. 
   Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `vpc_id` | String | ✅ | <p>The ID of the VPC for which to create the egress-only internet gateway.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to assign to the egress-only internet gateway.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the
			request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create egress_only_internet_gateway
egress_only_internet_gateway = provider.ec2.Egress_only_internet_gateway {
    vpc_id = "value"  # <p>The ID of the VPC for which to create the egress-only internet gateway.</p>
}

```

---


### Capacity_reservation

CapacityReservation resource

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ebs_optimized` | bool |  | <p>Indicates whether the Capacity Reservation supports EBS-optimized instances. This
			optimization provides dedicated throughput to Amazon EBS and an optimized configuration
			stack to provide optimal I/O performance. This optimization isn't available with all
			instance types. Additional usage charges apply when using an EBS- optimized
			instance.</p> |
| `instance_platform` | String | ✅ | <p>The type of operating system for which to reserve capacity.</p> |
| `ephemeral_storage` | bool |  | <p>
            <i>Deprecated.</i>
         </p> |
| `start_date` | String |  | <note>
            <p>Required for future-dated Capacity Reservations only. To create a Capacity
				Reservation for immediate use, omit this parameter. </p>
         </note>
         <p>The date and time at which the future-dated Capacity Reservation should become
			available for use, in the ISO8601 format in the UTC time zone
				(<code>YYYY-MM-DDThh:mm:ss.sssZ</code>).</p>
         <p>You can request a future-dated Capacity Reservation between 5 and 120 days in
			advance.</p> |
| `end_date_type` | String |  | <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can
			have one of the following end types:</p>
         <ul>
            <li>
               <p>
                  <code>unlimited</code> - The Capacity Reservation remains active until you
					explicitly cancel it. Do not provide an <code>EndDate</code> if the
						<code>EndDateType</code> is <code>unlimited</code>.</p>
            </li>
            <li>
               <p>
                  <code>limited</code> - The Capacity Reservation expires automatically at a
					specified date and time. You must provide an <code>EndDate</code> value if the
						<code>EndDateType</code> value is <code>limited</code>.</p>
            </li>
         </ul> |
| `placement_group_arn` | String |  | <note>
            <p>Not supported for future-dated Capacity Reservations.</p>
         </note>
         <p>The Amazon Resource Name (ARN) of the cluster placement group in which to create the
			Capacity Reservation. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/cr-cpg.html"> Capacity Reservations for cluster
				placement groups</a> in the <i>Amazon EC2 User Guide</i>.</p> |
| `availability_zone` | String |  | <p>The Availability Zone in which to create the Capacity Reservation.</p> |
| `availability_zone_id` | String |  | <p>The ID of the Availability Zone in which to create the Capacity Reservation.</p> |
| `end_date` | String |  | <p>The date and time at which the Capacity Reservation expires. When a Capacity
			Reservation expires, the reserved capacity is released and you can no longer launch
			instances into it. The Capacity Reservation's state changes to <code>expired</code> when
			it reaches its end date and time.</p>
         <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is
				<code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is
				<code>unlimited</code>.</p>
         <p>If the <code>EndDateType</code> is <code>limited</code>, the Capacity Reservation is
			cancelled within an hour from the specified time. For example, if you specify 5/31/2019,
			13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on
			5/31/2019.</p>
         <p>If you are requesting a future-dated Capacity Reservation, you can't specify an end
			date and time that is within the commitment duration.</p> |
| `commitment_duration` | i64 |  | <note>
            <p>Required for future-dated Capacity Reservations only. To create a Capacity
				Reservation for immediate use, omit this parameter. </p>
         </note>
         <p>Specify a commitment duration, in seconds, for the future-dated Capacity
			Reservation.</p>
         <p>The commitment duration is a minimum duration for which you commit to having the
			future-dated Capacity Reservation in the <code>active</code> state in your account after
			it has been delivered.</p>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/cr-concepts.html#cr-commitment-duration"> Commitment
				duration</a>.</p> |
| `instance_type` | String | ✅ | <p>The instance type for which to reserve capacity.</p>
         <note>
            <p>You can request future-dated Capacity Reservations for instance types in the C, M,
				R, I, T, and G instance families only.</p>
         </note>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the
				<i>Amazon EC2 User Guide</i>.</p> |
| `tenancy` | String |  | <p>Indicates the tenancy of the Capacity Reservation. A Capacity Reservation can have one
			of the following tenancy settings:</p>
         <ul>
            <li>
               <p>
                  <code>default</code> - The Capacity Reservation is created on hardware that is
					shared with other Amazon Web Services accounts.</p>
            </li>
            <li>
               <p>
                  <code>dedicated</code> - The Capacity Reservation is created on single-tenant
					hardware that is dedicated to a single Amazon Web Services account.</p>
            </li>
         </ul> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the Capacity Reservation during launch.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |
| `delivery_preference` | String |  | <note>
            <p>Required for future-dated Capacity Reservations only. To create a Capacity
				Reservation for immediate use, omit this parameter. </p>
         </note>
         <p>Indicates that the requested capacity will be delivered in addition to any running
			instances or reserved capacity that you have in your account at the requested date and
			time.</p>
         <p>The only supported value is <code>incremental</code>.</p> |
| `instance_count` | i64 | ✅ | <p>The number of instances for which to reserve capacity.</p>
         <note>
            <p>You can request future-dated Capacity Reservations for an instance count with a
				minimum of 64 vCPUs. For example, if you request a future-dated Capacity
				Reservation for <code>m5.xlarge</code> instances, you must request at least 25
				instances (<i>16 * m5.xlarge = 64 vCPUs</i>).</p>
         </note>
         <p>Valid range: 1 - 1000</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p> |
| `outpost_arn` | String |  | <note>
            <p>Not supported for future-dated Capacity Reservations.</p>
         </note>
         <p>The Amazon Resource Name (ARN) of the Outpost on which to create the Capacity
			Reservation.</p> |
| `instance_match_criteria` | String |  | <p>Indicates the type of instance launches that the Capacity Reservation accepts. The
			options include:</p>
         <ul>
            <li>
               <p>
                  <code>open</code> - The Capacity Reservation automatically matches all instances
					that have matching attributes (instance type, platform, and Availability Zone).
					Instances that have matching attributes run in the Capacity Reservation
					automatically without specifying any additional parameters.</p>
            </li>
            <li>
               <p>
                  <code>targeted</code> - The Capacity Reservation only accepts instances that
					have matching attributes (instance type, platform, and Availability Zone), and
					explicitly target the Capacity Reservation. This ensures that only permitted
					instances can use the reserved capacity. </p>
            </li>
         </ul>
         <note>
            <p>If you are requesting a future-dated Capacity Reservation, you must specify
					<code>targeted</code>.</p>
         </note>
         <p>Default: <code>open</code>
         </p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create capacity_reservation
capacity_reservation = provider.ec2.Capacity_reservation {
    instance_platform = "value"  # <p>The type of operating system for which to reserve capacity.</p>
    instance_type = "value"  # <p>The instance type for which to reserve capacity.</p>
         <note>
            <p>You can request future-dated Capacity Reservations for instance types in the C, M,
				R, I, T, and G instance families only.</p>
         </note>
         <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the
				<i>Amazon EC2 User Guide</i>.</p>
    instance_count = "value"  # <p>The number of instances for which to reserve capacity.</p>
         <note>
            <p>You can request future-dated Capacity Reservations for an instance count with a
				minimum of 64 vCPUs. For example, if you request a future-dated Capacity
				Reservation for <code>m5.xlarge</code> instances, you must request at least 25
				instances (<i>16 * m5.xlarge = 64 vCPUs</i>).</p>
         </note>
         <p>Valid range: 1 - 1000</p>
}

```

---


### Client_vpn_route

ClientVpnRoute resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_cidr_block` | String | ✅ | <p>The IPv4 address range, in CIDR notation, of the route destination. For example:</p>
         <ul>
            <li>
               <p>To add a route for Internet access, enter <code>0.0.0.0/0</code>
               </p>
            </li>
            <li>
               <p>To add a route for a peered VPC, enter the peered VPC's IPv4 CIDR range</p>
            </li>
            <li>
               <p>To add a route for an on-premises network, enter the Amazon Web Services Site-to-Site VPN connection's IPv4 CIDR range</p>
            </li>
            <li>
               <p>To add a route for the local network, enter the client CIDR range</p>
            </li>
         </ul> |
| `target_vpc_subnet_id` | String | ✅ | <p>The ID of the subnet through which you want to route traffic. The specified subnet must be
			an existing target network of the Client VPN endpoint.</p>
         <p>Alternatively, if you're adding a route for the local network, specify <code>local</code>.</p> |
| `description` | String |  | <p>A brief description of the route.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. 
For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `client_vpn_endpoint_id` | String | ✅ | <p>The ID of the Client VPN endpoint to which to add the route.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create client_vpn_route
client_vpn_route = provider.ec2.Client_vpn_route {
    destination_cidr_block = "value"  # <p>The IPv4 address range, in CIDR notation, of the route destination. For example:</p>
         <ul>
            <li>
               <p>To add a route for Internet access, enter <code>0.0.0.0/0</code>
               </p>
            </li>
            <li>
               <p>To add a route for a peered VPC, enter the peered VPC's IPv4 CIDR range</p>
            </li>
            <li>
               <p>To add a route for an on-premises network, enter the Amazon Web Services Site-to-Site VPN connection's IPv4 CIDR range</p>
            </li>
            <li>
               <p>To add a route for the local network, enter the client CIDR range</p>
            </li>
         </ul>
    target_vpc_subnet_id = "value"  # <p>The ID of the subnet through which you want to route traffic. The specified subnet must be
			an existing target network of the Client VPN endpoint.</p>
         <p>Alternatively, if you're adding a route for the local network, specify <code>local</code>.</p>
    client_vpn_endpoint_id = "value"  # <p>The ID of the Client VPN endpoint to which to add the route.</p>
}

```

---


### Network_interface

NetworkInterface resource

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connection_tracking_specification` | String |  | <p>A connection tracking specification for the network interface.</p> |
| `ipv6_address_count` | i64 |  | <p>The number of IPv6 addresses to assign to a network interface. Amazon EC2
            automatically selects the IPv6 addresses from the subnet range.</p>
         <p>You can't specify a count of IPv6 addresses using this parameter if you've specified
            one of the following: specific IPv6 addresses, specific IPv6 prefixes, or a count of
            IPv6 prefixes.</p>
         <p>If your subnet has the <code>AssignIpv6AddressOnCreation</code> attribute set, you can
            override that setting by specifying 0 as the IPv6 address count.</p> |
| `ipv6_prefix_count` | i64 |  | <p>The number of IPv6 prefixes that Amazon Web Services automatically assigns to the
            network interface.</p>
         <p>You can't specify a count of IPv6 prefixes if you've specified one of the following:
            specific IPv6 prefixes, specific IPv6 addresses, or a count of IPv6 addresses.</p> |
| `client_token` | String |  | <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/ec2/latest/devguide/ec2-api-idempotency.html">Ensuring idempotency</a>.</p> |
| `ipv4_prefixes` | Vec<String> |  | <p>The IPv4 prefixes assigned to the network interface.</p>
         <p>You can't specify IPv4 prefixes if you've specified one of the following: a count of
            IPv4 prefixes, specific private IPv4 addresses, or a count of private IPv4
            addresses.</p> |
| `subnet_id` | String | ✅ | <p>The ID of the subnet to associate with the network interface.</p> |
| `tag_specifications` | Vec<String> |  | <p>The tags to apply to the new network interface.</p> |
| `interface_type` | String |  | <p>The type of network interface. The default is <code>interface</code>.</p>
         <p>If you specify <code>efa-only</code>, do not assign any IP addresses to the network
            interface. EFA-only network interfaces do not support IP addresses.</p>
         <p>The only supported values are <code>interface</code>, <code>efa</code>,
                <code>efa-only</code>, and <code>trunk</code>.</p> |
| `dry_run` | bool |  | <p>Checks whether you have the required permissions for the action, without actually
            making the request, and provides an error response. If you have the required
            permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is
                <code>UnauthorizedOperation</code>.</p> |
| `operator` | String |  | <p>Reserved for internal use.</p> |
| `description` | String |  | <p>A description for the network interface.</p> |
| `groups` | Vec<String> |  | <p>The IDs of the security groups.</p> |
| `private_ip_address` | String |  | <p>The primary private IPv4 address of the network interface. If you don't specify an
            IPv4 address, Amazon EC2 selects one for you from the subnet's IPv4 CIDR range. If you
            specify an IP address, you cannot indicate any IP addresses specified in
                <code>privateIpAddresses</code> as primary (only one IP address can be designated as
            primary).</p> |
| `enable_primary_ipv6` | bool |  | <p>If you’re creating a network interface in a dual-stack or IPv6-only subnet, you have
            the option to assign a primary IPv6 IP address. A primary IPv6 address is an IPv6 GUA
            address associated with an ENI that you have enabled to use a primary IPv6 address. Use
            this option if the instance that this ENI will be attached to relies on its IPv6 address
            not changing. Amazon Web Services will automatically assign an IPv6 address associated
            with the ENI attached to your instance to be the primary IPv6 address. Once you enable
            an IPv6 GUA address to be a primary IPv6, you cannot disable it. When you enable an IPv6
            GUA address to be a primary IPv6, the first IPv6 GUA will be made the primary IPv6
            address until the instance is terminated or the network interface is detached. If you
            have multiple IPv6 addresses associated with an ENI attached to your instance and you
            enable a primary IPv6 address, the first IPv6 GUA address associated with the ENI
            becomes the primary IPv6 address.</p> |
| `secondary_private_ip_address_count` | i64 |  | <p>The number of secondary private IPv4 addresses to assign to a network interface. When
            you specify a number of secondary IPv4 addresses, Amazon EC2 selects these IP addresses
            within the subnet's IPv4 CIDR range. You can't specify this option and specify more than
            one private IP address using <code>privateIpAddresses</code>.</p>
         <p>You can't specify a count of private IPv4 addresses if you've specified one of the
            following: specific private IPv4 addresses, specific IPv4 prefixes, or a count of IPv4
            prefixes.</p> |
| `ipv6_addresses` | Vec<String> |  | <p>The IPv6 addresses from the IPv6 CIDR block range of your subnet.</p>
         <p>You can't specify IPv6 addresses using this parameter if you've specified one of the
            following: a count of IPv6 addresses, specific IPv6 prefixes, or a count of IPv6
            prefixes.</p> |
| `ipv6_prefixes` | Vec<String> |  | <p>The IPv6 prefixes assigned to the network interface.</p>
         <p>You can't specify IPv6 prefixes if you've specified one of the following: a count of
            IPv6 prefixes, specific IPv6 addresses, or a count of IPv6 addresses.</p> |
| `ipv4_prefix_count` | i64 |  | <p>The number of IPv4 prefixes that Amazon Web Services automatically assigns to the
            network interface.</p>
         <p>You can't specify a count of IPv4 prefixes if you've specified one of the following:
            specific IPv4 prefixes, specific private IPv4 addresses, or a count of private IPv4
            addresses.</p> |
| `private_ip_addresses` | Vec<String> |  | <p>The private IPv4 addresses.</p>
         <p>You can't specify private IPv4 addresses if you've specified one of the following: a
            count of private IPv4 addresses, specific IPv4 prefixes, or a count of IPv4
            prefixes.</p> |



#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create network_interface
network_interface = provider.ec2.Network_interface {
    subnet_id = "value"  # <p>The ID of the subnet to associate with the network interface.</p>
}

```

---


### Capacity_manager_data_exports

CapacityManagerDataExports resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capacity_manager_data_exports` | Vec<String> | <p>
Information about the data export configurations, including export settings, delivery status, and recent activity.
</p> |
| `next_token` | String | <p>
The token to use to retrieve the next page of results. This value is null when there are no more results to return.
</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access capacity_manager_data_exports outputs
capacity_manager_data_exports_id = capacity_manager_data_exports.id
capacity_manager_data_exports_capacity_manager_data_exports = capacity_manager_data_exports.capacity_manager_data_exports
capacity_manager_data_exports_next_token = capacity_manager_data_exports.next_token
```

---


### Store_image_tasks

StoreImageTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `store_image_task_results` | Vec<String> | <p>The information about the AMI store tasks.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there
         are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access store_image_tasks outputs
store_image_tasks_id = store_image_tasks.id
store_image_tasks_store_image_task_results = store_image_tasks.store_image_task_results
store_image_tasks_next_token = store_image_tasks.next_token
```

---


### Replace_root_volume_tasks

ReplaceRootVolumeTasks resource

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replace_root_volume_tasks` | Vec<String> | <p>Information about the root volume replacement task.</p> |
| `next_token` | String | <p>The token to include in another request to get the next page of items. 
  This value is <code>null</code> when there are no more items to return.</p> |


#### Usage Example

```kcl
# main.k
import aws

# Initialize provider
provider = aws.AwsProvider {
    region = "us-east-1"
}

# Access replace_root_volume_tasks outputs
replace_root_volume_tasks_id = replace_root_volume_tasks.id
replace_root_volume_tasks_replace_root_volume_tasks = replace_root_volume_tasks.replace_root_volume_tasks
replace_root_volume_tasks_next_token = replace_root_volume_tasks.next_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import aws

provider = aws.AwsProvider {
    region = "us-east-1"
}

# Create multiple vpcs resources
vpcs_0 = provider.ec2.Vpcs {
}
vpcs_1 = provider.ec2.Vpcs {
}
vpcs_2 = provider.ec2.Vpcs {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    vpcs = provider.ec2.Vpcs {
    }
```

---

## Related Documentation

- [AWS Ec2 Documentation](https://docs.aws.amazon.com/ec2/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
