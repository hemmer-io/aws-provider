//! Ec2_2016_11_15 Service
//!
//! Auto-generated service module for ec2_2016_11_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ec2_2016_11_15
pub struct Ec2_2016_11_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ec2_2016_11_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get coip_pool resource handler
    pub fn coip_pool(&self) -> resources::Coip_pool<'_> {
        resources::Coip_pool::new(self.provider)
    }
    /// Get subnet resource handler
    pub fn subnet(&self) -> resources::Subnet<'_> {
        resources::Subnet::new(self.provider)
    }
    /// Get capacity_reservations resource handler
    pub fn capacity_reservations(&self) -> resources::Capacity_reservations<'_> {
        resources::Capacity_reservations::new(self.provider)
    }
    /// Get fleet_history resource handler
    pub fn fleet_history(&self) -> resources::Fleet_history<'_> {
        resources::Fleet_history::new(self.provider)
    }
    /// Get scheduled_instance_availability resource handler
    pub fn scheduled_instance_availability(&self) -> resources::Scheduled_instance_availability<'_> {
        resources::Scheduled_instance_availability::new(self.provider)
    }
    /// Get carrier_gateway resource handler
    pub fn carrier_gateway(&self) -> resources::Carrier_gateway<'_> {
        resources::Carrier_gateway::new(self.provider)
    }
    /// Get route resource handler
    pub fn route(&self) -> resources::Route<'_> {
        resources::Route::new(self.provider)
    }
    /// Get transit_gateway_connects resource handler
    pub fn transit_gateway_connects(&self) -> resources::Transit_gateway_connects<'_> {
        resources::Transit_gateway_connects::new(self.provider)
    }
    /// Get transit_gateway_vpc_attachment resource handler
    pub fn transit_gateway_vpc_attachment(&self) -> resources::Transit_gateway_vpc_attachment<'_> {
        resources::Transit_gateway_vpc_attachment::new(self.provider)
    }
    /// Get ipam_address_history resource handler
    pub fn ipam_address_history(&self) -> resources::Ipam_address_history<'_> {
        resources::Ipam_address_history::new(self.provider)
    }
    /// Get ipam_pool_cidrs resource handler
    pub fn ipam_pool_cidrs(&self) -> resources::Ipam_pool_cidrs<'_> {
        resources::Ipam_pool_cidrs::new(self.provider)
    }
    /// Get transit_gateway_multicast_domain resource handler
    pub fn transit_gateway_multicast_domain(&self) -> resources::Transit_gateway_multicast_domain<'_> {
        resources::Transit_gateway_multicast_domain::new(self.provider)
    }
    /// Get regions resource handler
    pub fn regions(&self) -> resources::Regions<'_> {
        resources::Regions::new(self.provider)
    }
    /// Get network_insights_access_scopes resource handler
    pub fn network_insights_access_scopes(&self) -> resources::Network_insights_access_scopes<'_> {
        resources::Network_insights_access_scopes::new(self.provider)
    }
    /// Get transit_gateway_route_table_associations resource handler
    pub fn transit_gateway_route_table_associations(&self) -> resources::Transit_gateway_route_table_associations<'_> {
        resources::Transit_gateway_route_table_associations::new(self.provider)
    }
    /// Get fast_launch_images resource handler
    pub fn fast_launch_images(&self) -> resources::Fast_launch_images<'_> {
        resources::Fast_launch_images::new(self.provider)
    }
    /// Get conversion_tasks resource handler
    pub fn conversion_tasks(&self) -> resources::Conversion_tasks<'_> {
        resources::Conversion_tasks::new(self.provider)
    }
    /// Get transit_gateways resource handler
    pub fn transit_gateways(&self) -> resources::Transit_gateways<'_> {
        resources::Transit_gateways::new(self.provider)
    }
    /// Get outpost_lags resource handler
    pub fn outpost_lags(&self) -> resources::Outpost_lags<'_> {
        resources::Outpost_lags::new(self.provider)
    }
    /// Get managed_prefix_list resource handler
    pub fn managed_prefix_list(&self) -> resources::Managed_prefix_list<'_> {
        resources::Managed_prefix_list::new(self.provider)
    }
    /// Get import_image_tasks resource handler
    pub fn import_image_tasks(&self) -> resources::Import_image_tasks<'_> {
        resources::Import_image_tasks::new(self.provider)
    }
    /// Get instance_topology resource handler
    pub fn instance_topology(&self) -> resources::Instance_topology<'_> {
        resources::Instance_topology::new(self.provider)
    }
    /// Get fpga_image resource handler
    pub fn fpga_image(&self) -> resources::Fpga_image<'_> {
        resources::Fpga_image::new(self.provider)
    }
    /// Get associated_enclave_certificate_iam_roles resource handler
    pub fn associated_enclave_certificate_iam_roles(&self) -> resources::Associated_enclave_certificate_iam_roles<'_> {
        resources::Associated_enclave_certificate_iam_roles::new(self.provider)
    }
    /// Get capacity_reservation_usage resource handler
    pub fn capacity_reservation_usage(&self) -> resources::Capacity_reservation_usage<'_> {
        resources::Capacity_reservation_usage::new(self.provider)
    }
    /// Get console_output resource handler
    pub fn console_output(&self) -> resources::Console_output<'_> {
        resources::Console_output::new(self.provider)
    }
    /// Get ipam_resource_discovery_associations resource handler
    pub fn ipam_resource_discovery_associations(&self) -> resources::Ipam_resource_discovery_associations<'_> {
        resources::Ipam_resource_discovery_associations::new(self.provider)
    }
    /// Get route_servers resource handler
    pub fn route_servers(&self) -> resources::Route_servers<'_> {
        resources::Route_servers::new(self.provider)
    }
    /// Get service_link_virtual_interfaces resource handler
    pub fn service_link_virtual_interfaces(&self) -> resources::Service_link_virtual_interfaces<'_> {
        resources::Service_link_virtual_interfaces::new(self.provider)
    }
    /// Get ipam_discovered_public_addresses resource handler
    pub fn ipam_discovered_public_addresses(&self) -> resources::Ipam_discovered_public_addresses<'_> {
        resources::Ipam_discovered_public_addresses::new(self.provider)
    }
    /// Get trunk_interface_associations resource handler
    pub fn trunk_interface_associations(&self) -> resources::Trunk_interface_associations<'_> {
        resources::Trunk_interface_associations::new(self.provider)
    }
    /// Get vpn_connection_device_sample_configuration resource handler
    pub fn vpn_connection_device_sample_configuration(&self) -> resources::Vpn_connection_device_sample_configuration<'_> {
        resources::Vpn_connection_device_sample_configuration::new(self.provider)
    }
    /// Get transit_gateway_route_table resource handler
    pub fn transit_gateway_route_table(&self) -> resources::Transit_gateway_route_table<'_> {
        resources::Transit_gateway_route_table::new(self.provider)
    }
    /// Get launch_template_version resource handler
    pub fn launch_template_version(&self) -> resources::Launch_template_version<'_> {
        resources::Launch_template_version::new(self.provider)
    }
    /// Get vpc_endpoints resource handler
    pub fn vpc_endpoints(&self) -> resources::Vpc_endpoints<'_> {
        resources::Vpc_endpoints::new(self.provider)
    }
    /// Get instance_types resource handler
    pub fn instance_types(&self) -> resources::Instance_types<'_> {
        resources::Instance_types::new(self.provider)
    }
    /// Get network_interfaces resource handler
    pub fn network_interfaces(&self) -> resources::Network_interfaces<'_> {
        resources::Network_interfaces::new(self.provider)
    }
    /// Get scheduled_instances resource handler
    pub fn scheduled_instances(&self) -> resources::Scheduled_instances<'_> {
        resources::Scheduled_instances::new(self.provider)
    }
    /// Get security_group_references resource handler
    pub fn security_group_references(&self) -> resources::Security_group_references<'_> {
        resources::Security_group_references::new(self.provider)
    }
    /// Get vpc_endpoint_connections resource handler
    pub fn vpc_endpoint_connections(&self) -> resources::Vpc_endpoint_connections<'_> {
        resources::Vpc_endpoint_connections::new(self.provider)
    }
    /// Get traffic_mirror_filter resource handler
    pub fn traffic_mirror_filter(&self) -> resources::Traffic_mirror_filter<'_> {
        resources::Traffic_mirror_filter::new(self.provider)
    }
    /// Get iam_instance_profile_associations resource handler
    pub fn iam_instance_profile_associations(&self) -> resources::Iam_instance_profile_associations<'_> {
        resources::Iam_instance_profile_associations::new(self.provider)
    }
    /// Get locked_snapshots resource handler
    pub fn locked_snapshots(&self) -> resources::Locked_snapshots<'_> {
        resources::Locked_snapshots::new(self.provider)
    }
    /// Get instance_image_metadata resource handler
    pub fn instance_image_metadata(&self) -> resources::Instance_image_metadata<'_> {
        resources::Instance_image_metadata::new(self.provider)
    }
    /// Get export_tasks resource handler
    pub fn export_tasks(&self) -> resources::Export_tasks<'_> {
        resources::Export_tasks::new(self.provider)
    }
    /// Get transit_gateway_multicast_domains resource handler
    pub fn transit_gateway_multicast_domains(&self) -> resources::Transit_gateway_multicast_domains<'_> {
        resources::Transit_gateway_multicast_domains::new(self.provider)
    }
    /// Get vpn_gateways resource handler
    pub fn vpn_gateways(&self) -> resources::Vpn_gateways<'_> {
        resources::Vpn_gateways::new(self.provider)
    }
    /// Get client_vpn_endpoint resource handler
    pub fn client_vpn_endpoint(&self) -> resources::Client_vpn_endpoint<'_> {
        resources::Client_vpn_endpoint::new(self.provider)
    }
    /// Get store_image_tasks resource handler
    pub fn store_image_tasks(&self) -> resources::Store_image_tasks<'_> {
        resources::Store_image_tasks::new(self.provider)
    }
    /// Get snapshot resource handler
    pub fn snapshot(&self) -> resources::Snapshot<'_> {
        resources::Snapshot::new(self.provider)
    }
    /// Get ipam_resource_discoveries resource handler
    pub fn ipam_resource_discoveries(&self) -> resources::Ipam_resource_discoveries<'_> {
        resources::Ipam_resource_discoveries::new(self.provider)
    }
    /// Get instance_types_from_instance_requirements resource handler
    pub fn instance_types_from_instance_requirements(&self) -> resources::Instance_types_from_instance_requirements<'_> {
        resources::Instance_types_from_instance_requirements::new(self.provider)
    }
    /// Get ipam_discovered_accounts resource handler
    pub fn ipam_discovered_accounts(&self) -> resources::Ipam_discovered_accounts<'_> {
        resources::Ipam_discovered_accounts::new(self.provider)
    }
    /// Get volume_status resource handler
    pub fn volume_status(&self) -> resources::Volume_status<'_> {
        resources::Volume_status::new(self.provider)
    }
    /// Get verified_access_group resource handler
    pub fn verified_access_group(&self) -> resources::Verified_access_group<'_> {
        resources::Verified_access_group::new(self.provider)
    }
    /// Get vpc_block_public_access_exclusions resource handler
    pub fn vpc_block_public_access_exclusions(&self) -> resources::Vpc_block_public_access_exclusions<'_> {
        resources::Vpc_block_public_access_exclusions::new(self.provider)
    }
    /// Get spot_instance_requests resource handler
    pub fn spot_instance_requests(&self) -> resources::Spot_instance_requests<'_> {
        resources::Spot_instance_requests::new(self.provider)
    }
    /// Get capacity_manager_data_exports resource handler
    pub fn capacity_manager_data_exports(&self) -> resources::Capacity_manager_data_exports<'_> {
        resources::Capacity_manager_data_exports::new(self.provider)
    }
    /// Get vpc_endpoint_service_configuration resource handler
    pub fn vpc_endpoint_service_configuration(&self) -> resources::Vpc_endpoint_service_configuration<'_> {
        resources::Vpc_endpoint_service_configuration::new(self.provider)
    }
    /// Get spot_price_history resource handler
    pub fn spot_price_history(&self) -> resources::Spot_price_history<'_> {
        resources::Spot_price_history::new(self.provider)
    }
    /// Get host_reservations resource handler
    pub fn host_reservations(&self) -> resources::Host_reservations<'_> {
        resources::Host_reservations::new(self.provider)
    }
    /// Get verified_access_groups resource handler
    pub fn verified_access_groups(&self) -> resources::Verified_access_groups<'_> {
        resources::Verified_access_groups::new(self.provider)
    }
    /// Get security_group_rule_descriptions_egress resource handler
    pub fn security_group_rule_descriptions_egress(&self) -> resources::Security_group_rule_descriptions_egress<'_> {
        resources::Security_group_rule_descriptions_egress::new(self.provider)
    }
    /// Get vpc_peering_connections resource handler
    pub fn vpc_peering_connections(&self) -> resources::Vpc_peering_connections<'_> {
        resources::Vpc_peering_connections::new(self.provider)
    }
    /// Get client_vpn_routes resource handler
    pub fn client_vpn_routes(&self) -> resources::Client_vpn_routes<'_> {
        resources::Client_vpn_routes::new(self.provider)
    }
    /// Get vpc_block_public_access_exclusion resource handler
    pub fn vpc_block_public_access_exclusion(&self) -> resources::Vpc_block_public_access_exclusion<'_> {
        resources::Vpc_block_public_access_exclusion::new(self.provider)
    }
    /// Get security_group_vpc_associations resource handler
    pub fn security_group_vpc_associations(&self) -> resources::Security_group_vpc_associations<'_> {
        resources::Security_group_vpc_associations::new(self.provider)
    }
    /// Get key_pairs resource handler
    pub fn key_pairs(&self) -> resources::Key_pairs<'_> {
        resources::Key_pairs::new(self.provider)
    }
    /// Get capacity_block_status resource handler
    pub fn capacity_block_status(&self) -> resources::Capacity_block_status<'_> {
        resources::Capacity_block_status::new(self.provider)
    }
    /// Get fleet resource handler
    pub fn fleet(&self) -> resources::Fleet<'_> {
        resources::Fleet::new(self.provider)
    }
    /// Get local_gateways resource handler
    pub fn local_gateways(&self) -> resources::Local_gateways<'_> {
        resources::Local_gateways::new(self.provider)
    }
    /// Get subnets resource handler
    pub fn subnets(&self) -> resources::Subnets<'_> {
        resources::Subnets::new(self.provider)
    }
    /// Get client_vpn_connections resource handler
    pub fn client_vpn_connections(&self) -> resources::Client_vpn_connections<'_> {
        resources::Client_vpn_connections::new(self.provider)
    }
    /// Get ipam_resource_cidrs resource handler
    pub fn ipam_resource_cidrs(&self) -> resources::Ipam_resource_cidrs<'_> {
        resources::Ipam_resource_cidrs::new(self.provider)
    }
    /// Get import_snapshot_tasks resource handler
    pub fn import_snapshot_tasks(&self) -> resources::Import_snapshot_tasks<'_> {
        resources::Import_snapshot_tasks::new(self.provider)
    }
    /// Get image_usage_report resource handler
    pub fn image_usage_report(&self) -> resources::Image_usage_report<'_> {
        resources::Image_usage_report::new(self.provider)
    }
    /// Get route_server_peer resource handler
    pub fn route_server_peer(&self) -> resources::Route_server_peer<'_> {
        resources::Route_server_peer::new(self.provider)
    }
    /// Get volume resource handler
    pub fn volume(&self) -> resources::Volume<'_> {
        resources::Volume::new(self.provider)
    }
    /// Get network_insights_access_scope_analysis resource handler
    pub fn network_insights_access_scope_analysis(&self) -> resources::Network_insights_access_scope_analysis<'_> {
        resources::Network_insights_access_scope_analysis::new(self.provider)
    }
    /// Get ipam_scope resource handler
    pub fn ipam_scope(&self) -> resources::Ipam_scope<'_> {
        resources::Ipam_scope::new(self.provider)
    }
    /// Get network_acl_entry resource handler
    pub fn network_acl_entry(&self) -> resources::Network_acl_entry<'_> {
        resources::Network_acl_entry::new(self.provider)
    }
    /// Get capacity_manager_attributes resource handler
    pub fn capacity_manager_attributes(&self) -> resources::Capacity_manager_attributes<'_> {
        resources::Capacity_manager_attributes::new(self.provider)
    }
    /// Get capacity_manager_metric_dimensions resource handler
    pub fn capacity_manager_metric_dimensions(&self) -> resources::Capacity_manager_metric_dimensions<'_> {
        resources::Capacity_manager_metric_dimensions::new(self.provider)
    }
    /// Get reserved_instances_exchange_quote resource handler
    pub fn reserved_instances_exchange_quote(&self) -> resources::Reserved_instances_exchange_quote<'_> {
        resources::Reserved_instances_exchange_quote::new(self.provider)
    }
    /// Get verified_access_endpoint_targets resource handler
    pub fn verified_access_endpoint_targets(&self) -> resources::Verified_access_endpoint_targets<'_> {
        resources::Verified_access_endpoint_targets::new(self.provider)
    }
    /// Get transit_gateway_route_tables resource handler
    pub fn transit_gateway_route_tables(&self) -> resources::Transit_gateway_route_tables<'_> {
        resources::Transit_gateway_route_tables::new(self.provider)
    }
    /// Get console_screenshot resource handler
    pub fn console_screenshot(&self) -> resources::Console_screenshot<'_> {
        resources::Console_screenshot::new(self.provider)
    }
    /// Get network_insights_analysis resource handler
    pub fn network_insights_analysis(&self) -> resources::Network_insights_analysis<'_> {
        resources::Network_insights_analysis::new(self.provider)
    }
    /// Get mac_modification_tasks resource handler
    pub fn mac_modification_tasks(&self) -> resources::Mac_modification_tasks<'_> {
        resources::Mac_modification_tasks::new(self.provider)
    }
    /// Get byoip_cidrs resource handler
    pub fn byoip_cidrs(&self) -> resources::Byoip_cidrs<'_> {
        resources::Byoip_cidrs::new(self.provider)
    }
    /// Get coip_pool_usage resource handler
    pub fn coip_pool_usage(&self) -> resources::Coip_pool_usage<'_> {
        resources::Coip_pool_usage::new(self.provider)
    }
    /// Get local_gateway_route_table_vpc_association resource handler
    pub fn local_gateway_route_table_vpc_association(&self) -> resources::Local_gateway_route_table_vpc_association<'_> {
        resources::Local_gateway_route_table_vpc_association::new(self.provider)
    }
    /// Get instance_attribute resource handler
    pub fn instance_attribute(&self) -> resources::Instance_attribute<'_> {
        resources::Instance_attribute::new(self.provider)
    }
    /// Get capacity_reservation_billing_requests resource handler
    pub fn capacity_reservation_billing_requests(&self) -> resources::Capacity_reservation_billing_requests<'_> {
        resources::Capacity_reservation_billing_requests::new(self.provider)
    }
    /// Get capacity_manager_metric_data resource handler
    pub fn capacity_manager_metric_data(&self) -> resources::Capacity_manager_metric_data<'_> {
        resources::Capacity_manager_metric_data::new(self.provider)
    }
    /// Get verified_access_endpoints resource handler
    pub fn verified_access_endpoints(&self) -> resources::Verified_access_endpoints<'_> {
        resources::Verified_access_endpoints::new(self.provider)
    }
    /// Get local_gateway_route_table_virtual_interface_group_association resource handler
    pub fn local_gateway_route_table_virtual_interface_group_association(&self) -> resources::Local_gateway_route_table_virtual_interface_group_association<'_> {
        resources::Local_gateway_route_table_virtual_interface_group_association::new(self.provider)
    }
    /// Get flow_logs_integration_template resource handler
    pub fn flow_logs_integration_template(&self) -> resources::Flow_logs_integration_template<'_> {
        resources::Flow_logs_integration_template::new(self.provider)
    }
    /// Get local_gateway_virtual_interface_groups resource handler
    pub fn local_gateway_virtual_interface_groups(&self) -> resources::Local_gateway_virtual_interface_groups<'_> {
        resources::Local_gateway_virtual_interface_groups::new(self.provider)
    }
    /// Get aws_network_performance_metric_subscriptions resource handler
    pub fn aws_network_performance_metric_subscriptions(&self) -> resources::Aws_network_performance_metric_subscriptions<'_> {
        resources::Aws_network_performance_metric_subscriptions::new(self.provider)
    }
    /// Get verified_access_trust_providers resource handler
    pub fn verified_access_trust_providers(&self) -> resources::Verified_access_trust_providers<'_> {
        resources::Verified_access_trust_providers::new(self.provider)
    }
    /// Get transit_gateway_route_table_propagations resource handler
    pub fn transit_gateway_route_table_propagations(&self) -> resources::Transit_gateway_route_table_propagations<'_> {
        resources::Transit_gateway_route_table_propagations::new(self.provider)
    }
    /// Get spot_fleet_request_history resource handler
    pub fn spot_fleet_request_history(&self) -> resources::Spot_fleet_request_history<'_> {
        resources::Spot_fleet_request_history::new(self.provider)
    }
    /// Get store_image_task resource handler
    pub fn store_image_task(&self) -> resources::Store_image_task<'_> {
        resources::Store_image_task::new(self.provider)
    }
    /// Get identity_id_format resource handler
    pub fn identity_id_format(&self) -> resources::Identity_id_format<'_> {
        resources::Identity_id_format::new(self.provider)
    }
    /// Get vpc_peering_connection resource handler
    pub fn vpc_peering_connection(&self) -> resources::Vpc_peering_connection<'_> {
        resources::Vpc_peering_connection::new(self.provider)
    }
    /// Get vpn_tunnel_replacement_status resource handler
    pub fn vpn_tunnel_replacement_status(&self) -> resources::Vpn_tunnel_replacement_status<'_> {
        resources::Vpn_tunnel_replacement_status::new(self.provider)
    }
    /// Get capacity_reservation_topology resource handler
    pub fn capacity_reservation_topology(&self) -> resources::Capacity_reservation_topology<'_> {
        resources::Capacity_reservation_topology::new(self.provider)
    }
    /// Get traffic_mirror_session resource handler
    pub fn traffic_mirror_session(&self) -> resources::Traffic_mirror_session<'_> {
        resources::Traffic_mirror_session::new(self.provider)
    }
    /// Get transit_gateway_policy_table_associations resource handler
    pub fn transit_gateway_policy_table_associations(&self) -> resources::Transit_gateway_policy_table_associations<'_> {
        resources::Transit_gateway_policy_table_associations::new(self.provider)
    }
    /// Get traffic_mirror_filters resource handler
    pub fn traffic_mirror_filters(&self) -> resources::Traffic_mirror_filters<'_> {
        resources::Traffic_mirror_filters::new(self.provider)
    }
    /// Get vpc_endpoint_associations resource handler
    pub fn vpc_endpoint_associations(&self) -> resources::Vpc_endpoint_associations<'_> {
        resources::Vpc_endpoint_associations::new(self.provider)
    }
    /// Get coip_pools resource handler
    pub fn coip_pools(&self) -> resources::Coip_pools<'_> {
        resources::Coip_pools::new(self.provider)
    }
    /// Get network_insights_access_scope_analyses resource handler
    pub fn network_insights_access_scope_analyses(&self) -> resources::Network_insights_access_scope_analyses<'_> {
        resources::Network_insights_access_scope_analyses::new(self.provider)
    }
    /// Get vpc_endpoint_service_configurations resource handler
    pub fn vpc_endpoint_service_configurations(&self) -> resources::Vpc_endpoint_service_configurations<'_> {
        resources::Vpc_endpoint_service_configurations::new(self.provider)
    }
    /// Get client_vpn_target_networks resource handler
    pub fn client_vpn_target_networks(&self) -> resources::Client_vpn_target_networks<'_> {
        resources::Client_vpn_target_networks::new(self.provider)
    }
    /// Get capacity_reservation_by_splitting resource handler
    pub fn capacity_reservation_by_splitting(&self) -> resources::Capacity_reservation_by_splitting<'_> {
        resources::Capacity_reservation_by_splitting::new(self.provider)
    }
    /// Get network_insights_access_scope resource handler
    pub fn network_insights_access_scope(&self) -> resources::Network_insights_access_scope<'_> {
        resources::Network_insights_access_scope::new(self.provider)
    }
    /// Get vpn_connections resource handler
    pub fn vpn_connections(&self) -> resources::Vpn_connections<'_> {
        resources::Vpn_connections::new(self.provider)
    }
    /// Get instance_connect_endpoint resource handler
    pub fn instance_connect_endpoint(&self) -> resources::Instance_connect_endpoint<'_> {
        resources::Instance_connect_endpoint::new(self.provider)
    }
    /// Get address_transfers resource handler
    pub fn address_transfers(&self) -> resources::Address_transfers<'_> {
        resources::Address_transfers::new(self.provider)
    }
    /// Get snapshot_attribute resource handler
    pub fn snapshot_attribute(&self) -> resources::Snapshot_attribute<'_> {
        resources::Snapshot_attribute::new(self.provider)
    }
    /// Get instance_metadata_defaults resource handler
    pub fn instance_metadata_defaults(&self) -> resources::Instance_metadata_defaults<'_> {
        resources::Instance_metadata_defaults::new(self.provider)
    }
    /// Get network_insights_path resource handler
    pub fn network_insights_path(&self) -> resources::Network_insights_path<'_> {
        resources::Network_insights_path::new(self.provider)
    }
    /// Get client_vpn_route resource handler
    pub fn client_vpn_route(&self) -> resources::Client_vpn_route<'_> {
        resources::Client_vpn_route::new(self.provider)
    }
    /// Get network_interface_permission resource handler
    pub fn network_interface_permission(&self) -> resources::Network_interface_permission<'_> {
        resources::Network_interface_permission::new(self.provider)
    }
    /// Get network_insights_paths resource handler
    pub fn network_insights_paths(&self) -> resources::Network_insights_paths<'_> {
        resources::Network_insights_paths::new(self.provider)
    }
    /// Get declarative_policies_report_summary resource handler
    pub fn declarative_policies_report_summary(&self) -> resources::Declarative_policies_report_summary<'_> {
        resources::Declarative_policies_report_summary::new(self.provider)
    }
    /// Get nat_gateways resource handler
    pub fn nat_gateways(&self) -> resources::Nat_gateways<'_> {
        resources::Nat_gateways::new(self.provider)
    }
    /// Get traffic_mirror_targets resource handler
    pub fn traffic_mirror_targets(&self) -> resources::Traffic_mirror_targets<'_> {
        resources::Traffic_mirror_targets::new(self.provider)
    }
    /// Get ipam_scopes resource handler
    pub fn ipam_scopes(&self) -> resources::Ipam_scopes<'_> {
        resources::Ipam_scopes::new(self.provider)
    }
    /// Get spot_placement_scores resource handler
    pub fn spot_placement_scores(&self) -> resources::Spot_placement_scores<'_> {
        resources::Spot_placement_scores::new(self.provider)
    }
    /// Get transit_gateway_prefix_list_references resource handler
    pub fn transit_gateway_prefix_list_references(&self) -> resources::Transit_gateway_prefix_list_references<'_> {
        resources::Transit_gateway_prefix_list_references::new(self.provider)
    }
    /// Get fpga_image_attribute resource handler
    pub fn fpga_image_attribute(&self) -> resources::Fpga_image_attribute<'_> {
        resources::Fpga_image_attribute::new(self.provider)
    }
    /// Get groups_for_capacity_reservation resource handler
    pub fn groups_for_capacity_reservation(&self) -> resources::Groups_for_capacity_reservation<'_> {
        resources::Groups_for_capacity_reservation::new(self.provider)
    }
    /// Get local_gateway_virtual_interface_group resource handler
    pub fn local_gateway_virtual_interface_group(&self) -> resources::Local_gateway_virtual_interface_group<'_> {
        resources::Local_gateway_virtual_interface_group::new(self.provider)
    }
    /// Get capacity_blocks resource handler
    pub fn capacity_blocks(&self) -> resources::Capacity_blocks<'_> {
        resources::Capacity_blocks::new(self.provider)
    }
    /// Get launch_template_versions resource handler
    pub fn launch_template_versions(&self) -> resources::Launch_template_versions<'_> {
        resources::Launch_template_versions::new(self.provider)
    }
    /// Get vpn_gateway resource handler
    pub fn vpn_gateway(&self) -> resources::Vpn_gateway<'_> {
        resources::Vpn_gateway::new(self.provider)
    }
    /// Get capacity_reservation_fleet resource handler
    pub fn capacity_reservation_fleet(&self) -> resources::Capacity_reservation_fleet<'_> {
        resources::Capacity_reservation_fleet::new(self.provider)
    }
    /// Get id_format resource handler
    pub fn id_format(&self) -> resources::Id_format<'_> {
        resources::Id_format::new(self.provider)
    }
    /// Get capacity_block_extension_history resource handler
    pub fn capacity_block_extension_history(&self) -> resources::Capacity_block_extension_history<'_> {
        resources::Capacity_block_extension_history::new(self.provider)
    }
    /// Get ipam_byoasn resource handler
    pub fn ipam_byoasn(&self) -> resources::Ipam_byoasn<'_> {
        resources::Ipam_byoasn::new(self.provider)
    }
    /// Get traffic_mirror_target resource handler
    pub fn traffic_mirror_target(&self) -> resources::Traffic_mirror_target<'_> {
        resources::Traffic_mirror_target::new(self.provider)
    }
    /// Get capacity_block_offerings resource handler
    pub fn capacity_block_offerings(&self) -> resources::Capacity_block_offerings<'_> {
        resources::Capacity_block_offerings::new(self.provider)
    }
    /// Get dhcp_options resource handler
    pub fn dhcp_options(&self) -> resources::Dhcp_options<'_> {
        resources::Dhcp_options::new(self.provider)
    }
    /// Get route_server resource handler
    pub fn route_server(&self) -> resources::Route_server<'_> {
        resources::Route_server::new(self.provider)
    }
    /// Get route_server_endpoint resource handler
    pub fn route_server_endpoint(&self) -> resources::Route_server_endpoint<'_> {
        resources::Route_server_endpoint::new(self.provider)
    }
    /// Get route_table resource handler
    pub fn route_table(&self) -> resources::Route_table<'_> {
        resources::Route_table::new(self.provider)
    }
    /// Get snapshots resource handler
    pub fn snapshots(&self) -> resources::Snapshots<'_> {
        resources::Snapshots::new(self.provider)
    }
    /// Get transit_gateway_connect resource handler
    pub fn transit_gateway_connect(&self) -> resources::Transit_gateway_connect<'_> {
        resources::Transit_gateway_connect::new(self.provider)
    }
    /// Get transit_gateway_route resource handler
    pub fn transit_gateway_route(&self) -> resources::Transit_gateway_route<'_> {
        resources::Transit_gateway_route::new(self.provider)
    }
    /// Get vpc_block_public_access_options resource handler
    pub fn vpc_block_public_access_options(&self) -> resources::Vpc_block_public_access_options<'_> {
        resources::Vpc_block_public_access_options::new(self.provider)
    }
    /// Get security_groups_for_vpc resource handler
    pub fn security_groups_for_vpc(&self) -> resources::Security_groups_for_vpc<'_> {
        resources::Security_groups_for_vpc::new(self.provider)
    }
    /// Get network_insights_analyses resource handler
    pub fn network_insights_analyses(&self) -> resources::Network_insights_analyses<'_> {
        resources::Network_insights_analyses::new(self.provider)
    }
    /// Get capacity_manager_data_export resource handler
    pub fn capacity_manager_data_export(&self) -> resources::Capacity_manager_data_export<'_> {
        resources::Capacity_manager_data_export::new(self.provider)
    }
    /// Get principal_id_format resource handler
    pub fn principal_id_format(&self) -> resources::Principal_id_format<'_> {
        resources::Principal_id_format::new(self.provider)
    }
    /// Get instance_type_offerings resource handler
    pub fn instance_type_offerings(&self) -> resources::Instance_type_offerings<'_> {
        resources::Instance_type_offerings::new(self.provider)
    }
    /// Get image_references resource handler
    pub fn image_references(&self) -> resources::Image_references<'_> {
        resources::Image_references::new(self.provider)
    }
    /// Get active_vpn_tunnel_status resource handler
    pub fn active_vpn_tunnel_status(&self) -> resources::Active_vpn_tunnel_status<'_> {
        resources::Active_vpn_tunnel_status::new(self.provider)
    }
    /// Get network_interface resource handler
    pub fn network_interface(&self) -> resources::Network_interface<'_> {
        resources::Network_interface::new(self.provider)
    }
    /// Get coip_cidr resource handler
    pub fn coip_cidr(&self) -> resources::Coip_cidr<'_> {
        resources::Coip_cidr::new(self.provider)
    }
    /// Get client_vpn_authorization_rules resource handler
    pub fn client_vpn_authorization_rules(&self) -> resources::Client_vpn_authorization_rules<'_> {
        resources::Client_vpn_authorization_rules::new(self.provider)
    }
    /// Get image_usage_report_entries resource handler
    pub fn image_usage_report_entries(&self) -> resources::Image_usage_report_entries<'_> {
        resources::Image_usage_report_entries::new(self.provider)
    }
    /// Get placement_group resource handler
    pub fn placement_group(&self) -> resources::Placement_group<'_> {
        resources::Placement_group::new(self.provider)
    }
    /// Get network_acl resource handler
    pub fn network_acl(&self) -> resources::Network_acl<'_> {
        resources::Network_acl::new(self.provider)
    }
    /// Get vpc_classic_link resource handler
    pub fn vpc_classic_link(&self) -> resources::Vpc_classic_link<'_> {
        resources::Vpc_classic_link::new(self.provider)
    }
    /// Get route_server_associations resource handler
    pub fn route_server_associations(&self) -> resources::Route_server_associations<'_> {
        resources::Route_server_associations::new(self.provider)
    }
    /// Get volume_attribute resource handler
    pub fn volume_attribute(&self) -> resources::Volume_attribute<'_> {
        resources::Volume_attribute::new(self.provider)
    }
    /// Get addresses resource handler
    pub fn addresses(&self) -> resources::Addresses<'_> {
        resources::Addresses::new(self.provider)
    }
    /// Get vpc_endpoint_service_permissions resource handler
    pub fn vpc_endpoint_service_permissions(&self) -> resources::Vpc_endpoint_service_permissions<'_> {
        resources::Vpc_endpoint_service_permissions::new(self.provider)
    }
    /// Get allowed_images_settings resource handler
    pub fn allowed_images_settings(&self) -> resources::Allowed_images_settings<'_> {
        resources::Allowed_images_settings::new(self.provider)
    }
    /// Get subnet_cidr_reservation resource handler
    pub fn subnet_cidr_reservation(&self) -> resources::Subnet_cidr_reservation<'_> {
        resources::Subnet_cidr_reservation::new(self.provider)
    }
    /// Get managed_prefix_lists resource handler
    pub fn managed_prefix_lists(&self) -> resources::Managed_prefix_lists<'_> {
        resources::Managed_prefix_lists::new(self.provider)
    }
    /// Get host_reservation_offerings resource handler
    pub fn host_reservation_offerings(&self) -> resources::Host_reservation_offerings<'_> {
        resources::Host_reservation_offerings::new(self.provider)
    }
    /// Get ebs_encryption_by_default resource handler
    pub fn ebs_encryption_by_default(&self) -> resources::Ebs_encryption_by_default<'_> {
        resources::Ebs_encryption_by_default::new(self.provider)
    }
    /// Get fpga_images resource handler
    pub fn fpga_images(&self) -> resources::Fpga_images<'_> {
        resources::Fpga_images::new(self.provider)
    }
    /// Get transit_gateway_peering_attachment resource handler
    pub fn transit_gateway_peering_attachment(&self) -> resources::Transit_gateway_peering_attachment<'_> {
        resources::Transit_gateway_peering_attachment::new(self.provider)
    }
    /// Get verified_access_instances resource handler
    pub fn verified_access_instances(&self) -> resources::Verified_access_instances<'_> {
        resources::Verified_access_instances::new(self.provider)
    }
    /// Get vpc_endpoint_connection_notifications resource handler
    pub fn vpc_endpoint_connection_notifications(&self) -> resources::Vpc_endpoint_connection_notifications<'_> {
        resources::Vpc_endpoint_connection_notifications::new(self.provider)
    }
    /// Get ipam_discovered_resource_cidrs resource handler
    pub fn ipam_discovered_resource_cidrs(&self) -> resources::Ipam_discovered_resource_cidrs<'_> {
        resources::Ipam_discovered_resource_cidrs::new(self.provider)
    }
    /// Get capacity_block_extension_offerings resource handler
    pub fn capacity_block_extension_offerings(&self) -> resources::Capacity_block_extension_offerings<'_> {
        resources::Capacity_block_extension_offerings::new(self.provider)
    }
    /// Get image_attribute resource handler
    pub fn image_attribute(&self) -> resources::Image_attribute<'_> {
        resources::Image_attribute::new(self.provider)
    }
    /// Get transit_gateway_vpc_attachments resource handler
    pub fn transit_gateway_vpc_attachments(&self) -> resources::Transit_gateway_vpc_attachments<'_> {
        resources::Transit_gateway_vpc_attachments::new(self.provider)
    }
    /// Get ipam_pool_allocations resource handler
    pub fn ipam_pool_allocations(&self) -> resources::Ipam_pool_allocations<'_> {
        resources::Ipam_pool_allocations::new(self.provider)
    }
    /// Get image_usage_reports resource handler
    pub fn image_usage_reports(&self) -> resources::Image_usage_reports<'_> {
        resources::Image_usage_reports::new(self.provider)
    }
    /// Get aws_network_performance_data resource handler
    pub fn aws_network_performance_data(&self) -> resources::Aws_network_performance_data<'_> {
        resources::Aws_network_performance_data::new(self.provider)
    }
    /// Get volumes resource handler
    pub fn volumes(&self) -> resources::Volumes<'_> {
        resources::Volumes::new(self.provider)
    }
    /// Get serial_console_access_status resource handler
    pub fn serial_console_access_status(&self) -> resources::Serial_console_access_status<'_> {
        resources::Serial_console_access_status::new(self.provider)
    }
    /// Get security_group resource handler
    pub fn security_group(&self) -> resources::Security_group<'_> {
        resources::Security_group::new(self.provider)
    }
    /// Get elastic_gpus resource handler
    pub fn elastic_gpus(&self) -> resources::Elastic_gpus<'_> {
        resources::Elastic_gpus::new(self.provider)
    }
    /// Get launch_templates resource handler
    pub fn launch_templates(&self) -> resources::Launch_templates<'_> {
        resources::Launch_templates::new(self.provider)
    }
    /// Get mac_hosts resource handler
    pub fn mac_hosts(&self) -> resources::Mac_hosts<'_> {
        resources::Mac_hosts::new(self.provider)
    }
    /// Get public_ipv4_pool resource handler
    pub fn public_ipv4_pool(&self) -> resources::Public_ipv4_pool<'_> {
        resources::Public_ipv4_pool::new(self.provider)
    }
    /// Get managed_prefix_list_associations resource handler
    pub fn managed_prefix_list_associations(&self) -> resources::Managed_prefix_list_associations<'_> {
        resources::Managed_prefix_list_associations::new(self.provider)
    }
    /// Get verified_access_trust_provider resource handler
    pub fn verified_access_trust_provider(&self) -> resources::Verified_access_trust_provider<'_> {
        resources::Verified_access_trust_provider::new(self.provider)
    }
    /// Get classic_link_instances resource handler
    pub fn classic_link_instances(&self) -> resources::Classic_link_instances<'_> {
        resources::Classic_link_instances::new(self.provider)
    }
    /// Get egress_only_internet_gateway resource handler
    pub fn egress_only_internet_gateway(&self) -> resources::Egress_only_internet_gateway<'_> {
        resources::Egress_only_internet_gateway::new(self.provider)
    }
    /// Get transit_gateway_connect_peer resource handler
    pub fn transit_gateway_connect_peer(&self) -> resources::Transit_gateway_connect_peer<'_> {
        resources::Transit_gateway_connect_peer::new(self.provider)
    }
    /// Get queued_reserved_instances resource handler
    pub fn queued_reserved_instances(&self) -> resources::Queued_reserved_instances<'_> {
        resources::Queued_reserved_instances::new(self.provider)
    }
    /// Get vpc resource handler
    pub fn vpc(&self) -> resources::Vpc<'_> {
        resources::Vpc::new(self.provider)
    }
    /// Get spot_fleet_requests resource handler
    pub fn spot_fleet_requests(&self) -> resources::Spot_fleet_requests<'_> {
        resources::Spot_fleet_requests::new(self.provider)
    }
    /// Get vpcs resource handler
    pub fn vpcs(&self) -> resources::Vpcs<'_> {
        resources::Vpcs::new(self.provider)
    }
    /// Get transit_gateway_policy_table_entries resource handler
    pub fn transit_gateway_policy_table_entries(&self) -> resources::Transit_gateway_policy_table_entries<'_> {
        resources::Transit_gateway_policy_table_entries::new(self.provider)
    }
    /// Get verified_access_endpoint resource handler
    pub fn verified_access_endpoint(&self) -> resources::Verified_access_endpoint<'_> {
        resources::Verified_access_endpoint::new(self.provider)
    }
    /// Get addresses_attribute resource handler
    pub fn addresses_attribute(&self) -> resources::Addresses_attribute<'_> {
        resources::Addresses_attribute::new(self.provider)
    }
    /// Get nat_gateway resource handler
    pub fn nat_gateway(&self) -> resources::Nat_gateway<'_> {
        resources::Nat_gateway::new(self.provider)
    }
    /// Get ipam_resource_discovery resource handler
    pub fn ipam_resource_discovery(&self) -> resources::Ipam_resource_discovery<'_> {
        resources::Ipam_resource_discovery::new(self.provider)
    }
    /// Get verified_access_instance resource handler
    pub fn verified_access_instance(&self) -> resources::Verified_access_instance<'_> {
        resources::Verified_access_instance::new(self.provider)
    }
    /// Get security_groups resource handler
    pub fn security_groups(&self) -> resources::Security_groups<'_> {
        resources::Security_groups::new(self.provider)
    }
    /// Get route_server_routing_database resource handler
    pub fn route_server_routing_database(&self) -> resources::Route_server_routing_database<'_> {
        resources::Route_server_routing_database::new(self.provider)
    }
    /// Get traffic_mirror_filter_rules resource handler
    pub fn traffic_mirror_filter_rules(&self) -> resources::Traffic_mirror_filter_rules<'_> {
        resources::Traffic_mirror_filter_rules::new(self.provider)
    }
    /// Get password_data resource handler
    pub fn password_data(&self) -> resources::Password_data<'_> {
        resources::Password_data::new(self.provider)
    }
    /// Get snapshot_block_public_access_state resource handler
    pub fn snapshot_block_public_access_state(&self) -> resources::Snapshot_block_public_access_state<'_> {
        resources::Snapshot_block_public_access_state::new(self.provider)
    }
    /// Get transit_gateway_attachment_propagations resource handler
    pub fn transit_gateway_attachment_propagations(&self) -> resources::Transit_gateway_attachment_propagations<'_> {
        resources::Transit_gateway_attachment_propagations::new(self.provider)
    }
    /// Get moving_addresses resource handler
    pub fn moving_addresses(&self) -> resources::Moving_addresses<'_> {
        resources::Moving_addresses::new(self.provider)
    }
    /// Get restore_image_task resource handler
    pub fn restore_image_task(&self) -> resources::Restore_image_task<'_> {
        resources::Restore_image_task::new(self.provider)
    }
    /// Get security_group_rule_descriptions_ingress resource handler
    pub fn security_group_rule_descriptions_ingress(&self) -> resources::Security_group_rule_descriptions_ingress<'_> {
        resources::Security_group_rule_descriptions_ingress::new(self.provider)
    }
    /// Get spot_fleet_instances resource handler
    pub fn spot_fleet_instances(&self) -> resources::Spot_fleet_instances<'_> {
        resources::Spot_fleet_instances::new(self.provider)
    }
    /// Get placement_groups resource handler
    pub fn placement_groups(&self) -> resources::Placement_groups<'_> {
        resources::Placement_groups::new(self.provider)
    }
    /// Get snapshot_tier_status resource handler
    pub fn snapshot_tier_status(&self) -> resources::Snapshot_tier_status<'_> {
        resources::Snapshot_tier_status::new(self.provider)
    }
    /// Get account_attributes resource handler
    pub fn account_attributes(&self) -> resources::Account_attributes<'_> {
        resources::Account_attributes::new(self.provider)
    }
    /// Get internet_gateway resource handler
    pub fn internet_gateway(&self) -> resources::Internet_gateway<'_> {
        resources::Internet_gateway::new(self.provider)
    }
    /// Get customer_gateway resource handler
    pub fn customer_gateway(&self) -> resources::Customer_gateway<'_> {
        resources::Customer_gateway::new(self.provider)
    }
    /// Get launch_template resource handler
    pub fn launch_template(&self) -> resources::Launch_template<'_> {
        resources::Launch_template::new(self.provider)
    }
    /// Get network_interface_attribute resource handler
    pub fn network_interface_attribute(&self) -> resources::Network_interface_attribute<'_> {
        resources::Network_interface_attribute::new(self.provider)
    }
    /// Get local_gateway_virtual_interface resource handler
    pub fn local_gateway_virtual_interface(&self) -> resources::Local_gateway_virtual_interface<'_> {
        resources::Local_gateway_virtual_interface::new(self.provider)
    }
    /// Get transit_gateway_connect_peers resource handler
    pub fn transit_gateway_connect_peers(&self) -> resources::Transit_gateway_connect_peers<'_> {
        resources::Transit_gateway_connect_peers::new(self.provider)
    }
    /// Get vpc_endpoint_connection_notification resource handler
    pub fn vpc_endpoint_connection_notification(&self) -> resources::Vpc_endpoint_connection_notification<'_> {
        resources::Vpc_endpoint_connection_notification::new(self.provider)
    }
    /// Get traffic_mirror_sessions resource handler
    pub fn traffic_mirror_sessions(&self) -> resources::Traffic_mirror_sessions<'_> {
        resources::Traffic_mirror_sessions::new(self.provider)
    }
    /// Get associated_ipv6_pool_cidrs resource handler
    pub fn associated_ipv6_pool_cidrs(&self) -> resources::Associated_ipv6_pool_cidrs<'_> {
        resources::Associated_ipv6_pool_cidrs::new(self.provider)
    }
    /// Get network_insights_access_scope_content resource handler
    pub fn network_insights_access_scope_content(&self) -> resources::Network_insights_access_scope_content<'_> {
        resources::Network_insights_access_scope_content::new(self.provider)
    }
    /// Get instance_credit_specifications resource handler
    pub fn instance_credit_specifications(&self) -> resources::Instance_credit_specifications<'_> {
        resources::Instance_credit_specifications::new(self.provider)
    }
    /// Get images resource handler
    pub fn images(&self) -> resources::Images<'_> {
        resources::Images::new(self.provider)
    }
    /// Get instance_tpm_ek_pub resource handler
    pub fn instance_tpm_ek_pub(&self) -> resources::Instance_tpm_ek_pub<'_> {
        resources::Instance_tpm_ek_pub::new(self.provider)
    }
    /// Get ipam_external_resource_verification_token resource handler
    pub fn ipam_external_resource_verification_token(&self) -> resources::Ipam_external_resource_verification_token<'_> {
        resources::Ipam_external_resource_verification_token::new(self.provider)
    }
    /// Get vpn_connection resource handler
    pub fn vpn_connection(&self) -> resources::Vpn_connection<'_> {
        resources::Vpn_connection::new(self.provider)
    }
    /// Get default_vpc resource handler
    pub fn default_vpc(&self) -> resources::Default_vpc<'_> {
        resources::Default_vpc::new(self.provider)
    }
    /// Get traffic_mirror_filter_rule resource handler
    pub fn traffic_mirror_filter_rule(&self) -> resources::Traffic_mirror_filter_rule<'_> {
        resources::Traffic_mirror_filter_rule::new(self.provider)
    }
    /// Get verified_access_instance_logging_configurations resource handler
    pub fn verified_access_instance_logging_configurations(&self) -> resources::Verified_access_instance_logging_configurations<'_> {
        resources::Verified_access_instance_logging_configurations::new(self.provider)
    }
    /// Get vpn_connection_route resource handler
    pub fn vpn_connection_route(&self) -> resources::Vpn_connection_route<'_> {
        resources::Vpn_connection_route::new(self.provider)
    }
    /// Get mac_system_integrity_protection_modification_task resource handler
    pub fn mac_system_integrity_protection_modification_task(&self) -> resources::Mac_system_integrity_protection_modification_task<'_> {
        resources::Mac_system_integrity_protection_modification_task::new(self.provider)
    }
    /// Get capacity_reservation_fleets resource handler
    pub fn capacity_reservation_fleets(&self) -> resources::Capacity_reservation_fleets<'_> {
        resources::Capacity_reservation_fleets::new(self.provider)
    }
    /// Get ipv6_pools resource handler
    pub fn ipv6_pools(&self) -> resources::Ipv6_pools<'_> {
        resources::Ipv6_pools::new(self.provider)
    }
    /// Get local_gateway_route_tables resource handler
    pub fn local_gateway_route_tables(&self) -> resources::Local_gateway_route_tables<'_> {
        resources::Local_gateway_route_tables::new(self.provider)
    }
    /// Get capacity_reservation resource handler
    pub fn capacity_reservation(&self) -> resources::Capacity_reservation<'_> {
        resources::Capacity_reservation::new(self.provider)
    }
    /// Get volumes_modifications resource handler
    pub fn volumes_modifications(&self) -> resources::Volumes_modifications<'_> {
        resources::Volumes_modifications::new(self.provider)
    }
    /// Get local_gateway_route resource handler
    pub fn local_gateway_route(&self) -> resources::Local_gateway_route<'_> {
        resources::Local_gateway_route::new(self.provider)
    }
    /// Get transit_gateway_route_table_announcement resource handler
    pub fn transit_gateway_route_table_announcement(&self) -> resources::Transit_gateway_route_table_announcement<'_> {
        resources::Transit_gateway_route_table_announcement::new(self.provider)
    }
    /// Get subnet_cidr_reservations resource handler
    pub fn subnet_cidr_reservations(&self) -> resources::Subnet_cidr_reservations<'_> {
        resources::Subnet_cidr_reservations::new(self.provider)
    }
    /// Get network_insights_access_scope_analysis_findings resource handler
    pub fn network_insights_access_scope_analysis_findings(&self) -> resources::Network_insights_access_scope_analysis_findings<'_> {
        resources::Network_insights_access_scope_analysis_findings::new(self.provider)
    }
    /// Get capacity_manager_organizations_access resource handler
    pub fn capacity_manager_organizations_access(&self) -> resources::Capacity_manager_organizations_access<'_> {
        resources::Capacity_manager_organizations_access::new(self.provider)
    }
    /// Get prefix_lists resource handler
    pub fn prefix_lists(&self) -> resources::Prefix_lists<'_> {
        resources::Prefix_lists::new(self.provider)
    }
    /// Get local_gateway_virtual_interfaces resource handler
    pub fn local_gateway_virtual_interfaces(&self) -> resources::Local_gateway_virtual_interfaces<'_> {
        resources::Local_gateway_virtual_interfaces::new(self.provider)
    }
    /// Get ipam_pools resource handler
    pub fn ipam_pools(&self) -> resources::Ipam_pools<'_> {
        resources::Ipam_pools::new(self.provider)
    }
    /// Get reserved_instances_offerings resource handler
    pub fn reserved_instances_offerings(&self) -> resources::Reserved_instances_offerings<'_> {
        resources::Reserved_instances_offerings::new(self.provider)
    }
    /// Get spot_datafeed_subscription resource handler
    pub fn spot_datafeed_subscription(&self) -> resources::Spot_datafeed_subscription<'_> {
        resources::Spot_datafeed_subscription::new(self.provider)
    }
    /// Get internet_gateways resource handler
    pub fn internet_gateways(&self) -> resources::Internet_gateways<'_> {
        resources::Internet_gateways::new(self.provider)
    }
    /// Get ebs_default_kms_key_id resource handler
    pub fn ebs_default_kms_key_id(&self) -> resources::Ebs_default_kms_key_id<'_> {
        resources::Ebs_default_kms_key_id::new(self.provider)
    }
    /// Get egress_only_internet_gateways resource handler
    pub fn egress_only_internet_gateways(&self) -> resources::Egress_only_internet_gateways<'_> {
        resources::Egress_only_internet_gateways::new(self.provider)
    }
    /// Get transit_gateway_policy_table resource handler
    pub fn transit_gateway_policy_table(&self) -> resources::Transit_gateway_policy_table<'_> {
        resources::Transit_gateway_policy_table::new(self.provider)
    }
    /// Get local_gateway_route_table resource handler
    pub fn local_gateway_route_table(&self) -> resources::Local_gateway_route_table<'_> {
        resources::Local_gateway_route_table::new(self.provider)
    }
    /// Get instance_uefi_data resource handler
    pub fn instance_uefi_data(&self) -> resources::Instance_uefi_data<'_> {
        resources::Instance_uefi_data::new(self.provider)
    }
    /// Get carrier_gateways resource handler
    pub fn carrier_gateways(&self) -> resources::Carrier_gateways<'_> {
        resources::Carrier_gateways::new(self.provider)
    }
    /// Get instance_event_windows resource handler
    pub fn instance_event_windows(&self) -> resources::Instance_event_windows<'_> {
        resources::Instance_event_windows::new(self.provider)
    }
    /// Get instances resource handler
    pub fn instances(&self) -> resources::Instances<'_> {
        resources::Instances::new(self.provider)
    }
    /// Get delegate_mac_volume_ownership_task resource handler
    pub fn delegate_mac_volume_ownership_task(&self) -> resources::Delegate_mac_volume_ownership_task<'_> {
        resources::Delegate_mac_volume_ownership_task::new(self.provider)
    }
    /// Get transit_gateway_policy_tables resource handler
    pub fn transit_gateway_policy_tables(&self) -> resources::Transit_gateway_policy_tables<'_> {
        resources::Transit_gateway_policy_tables::new(self.provider)
    }
    /// Get ipam resource handler
    pub fn ipam(&self) -> resources::Ipam<'_> {
        resources::Ipam::new(self.provider)
    }
    /// Get route_server_endpoints resource handler
    pub fn route_server_endpoints(&self) -> resources::Route_server_endpoints<'_> {
        resources::Route_server_endpoints::new(self.provider)
    }
    /// Get stale_security_groups resource handler
    pub fn stale_security_groups(&self) -> resources::Stale_security_groups<'_> {
        resources::Stale_security_groups::new(self.provider)
    }
    /// Get local_gateway_route_table_virtual_interface_group_associations resource handler
    pub fn local_gateway_route_table_virtual_interface_group_associations(&self) -> resources::Local_gateway_route_table_virtual_interface_group_associations<'_> {
        resources::Local_gateway_route_table_virtual_interface_group_associations::new(self.provider)
    }
    /// Get availability_zones resource handler
    pub fn availability_zones(&self) -> resources::Availability_zones<'_> {
        resources::Availability_zones::new(self.provider)
    }
    /// Get replace_root_volume_tasks resource handler
    pub fn replace_root_volume_tasks(&self) -> resources::Replace_root_volume_tasks<'_> {
        resources::Replace_root_volume_tasks::new(self.provider)
    }
    /// Get vpc_endpoint_services resource handler
    pub fn vpc_endpoint_services(&self) -> resources::Vpc_endpoint_services<'_> {
        resources::Vpc_endpoint_services::new(self.provider)
    }
    /// Get fleet_instances resource handler
    pub fn fleet_instances(&self) -> resources::Fleet_instances<'_> {
        resources::Fleet_instances::new(self.provider)
    }
    /// Get route_server_peers resource handler
    pub fn route_server_peers(&self) -> resources::Route_server_peers<'_> {
        resources::Route_server_peers::new(self.provider)
    }
    /// Get launch_template_data resource handler
    pub fn launch_template_data(&self) -> resources::Launch_template_data<'_> {
        resources::Launch_template_data::new(self.provider)
    }
    /// Get managed_prefix_list_entries resource handler
    pub fn managed_prefix_list_entries(&self) -> resources::Managed_prefix_list_entries<'_> {
        resources::Managed_prefix_list_entries::new(self.provider)
    }
    /// Get bundle_tasks resource handler
    pub fn bundle_tasks(&self) -> resources::Bundle_tasks<'_> {
        resources::Bundle_tasks::new(self.provider)
    }
    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
    }
    /// Get fleets resource handler
    pub fn fleets(&self) -> resources::Fleets<'_> {
        resources::Fleets::new(self.provider)
    }
    /// Get reserved_instances resource handler
    pub fn reserved_instances(&self) -> resources::Reserved_instances<'_> {
        resources::Reserved_instances::new(self.provider)
    }
    /// Get transit_gateway_attachments resource handler
    pub fn transit_gateway_attachments(&self) -> resources::Transit_gateway_attachments<'_> {
        resources::Transit_gateway_attachments::new(self.provider)
    }
    /// Get declarative_policies_reports resource handler
    pub fn declarative_policies_reports(&self) -> resources::Declarative_policies_reports<'_> {
        resources::Declarative_policies_reports::new(self.provider)
    }
    /// Get ipams resource handler
    pub fn ipams(&self) -> resources::Ipams<'_> {
        resources::Ipams::new(self.provider)
    }
    /// Get transit_gateway_peering_attachments resource handler
    pub fn transit_gateway_peering_attachments(&self) -> resources::Transit_gateway_peering_attachments<'_> {
        resources::Transit_gateway_peering_attachments::new(self.provider)
    }
    /// Get reserved_instances_listing resource handler
    pub fn reserved_instances_listing(&self) -> resources::Reserved_instances_listing<'_> {
        resources::Reserved_instances_listing::new(self.provider)
    }
    /// Get vpc_attribute resource handler
    pub fn vpc_attribute(&self) -> resources::Vpc_attribute<'_> {
        resources::Vpc_attribute::new(self.provider)
    }
    /// Get customer_gateways resource handler
    pub fn customer_gateways(&self) -> resources::Customer_gateways<'_> {
        resources::Customer_gateways::new(self.provider)
    }
    /// Get reserved_instances_listings resource handler
    pub fn reserved_instances_listings(&self) -> resources::Reserved_instances_listings<'_> {
        resources::Reserved_instances_listings::new(self.provider)
    }
    /// Get local_gateway_route_table_vpc_associations resource handler
    pub fn local_gateway_route_table_vpc_associations(&self) -> resources::Local_gateway_route_table_vpc_associations<'_> {
        resources::Local_gateway_route_table_vpc_associations::new(self.provider)
    }
    /// Get host_reservation_purchase_preview resource handler
    pub fn host_reservation_purchase_preview(&self) -> resources::Host_reservation_purchase_preview<'_> {
        resources::Host_reservation_purchase_preview::new(self.provider)
    }
    /// Get aggregate_id_format resource handler
    pub fn aggregate_id_format(&self) -> resources::Aggregate_id_format<'_> {
        resources::Aggregate_id_format::new(self.provider)
    }
    /// Get network_interface_permissions resource handler
    pub fn network_interface_permissions(&self) -> resources::Network_interface_permissions<'_> {
        resources::Network_interface_permissions::new(self.provider)
    }
    /// Get ipam_external_resource_verification_tokens resource handler
    pub fn ipam_external_resource_verification_tokens(&self) -> resources::Ipam_external_resource_verification_tokens<'_> {
        resources::Ipam_external_resource_verification_tokens::new(self.provider)
    }
    /// Get image_block_public_access_state resource handler
    pub fn image_block_public_access_state(&self) -> resources::Image_block_public_access_state<'_> {
        resources::Image_block_public_access_state::new(self.provider)
    }
    /// Get route_server_propagations resource handler
    pub fn route_server_propagations(&self) -> resources::Route_server_propagations<'_> {
        resources::Route_server_propagations::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get transit_gateway resource handler
    pub fn transit_gateway(&self) -> resources::Transit_gateway<'_> {
        resources::Transit_gateway::new(self.provider)
    }
    /// Get hosts resource handler
    pub fn hosts(&self) -> resources::Hosts<'_> {
        resources::Hosts::new(self.provider)
    }
    /// Get network_acls resource handler
    pub fn network_acls(&self) -> resources::Network_acls<'_> {
        resources::Network_acls::new(self.provider)
    }
    /// Get default_subnet resource handler
    pub fn default_subnet(&self) -> resources::Default_subnet<'_> {
        resources::Default_subnet::new(self.provider)
    }
    /// Get instance_export_task resource handler
    pub fn instance_export_task(&self) -> resources::Instance_export_task<'_> {
        resources::Instance_export_task::new(self.provider)
    }
    /// Get transit_gateway_multicast_domain_associations resource handler
    pub fn transit_gateway_multicast_domain_associations(&self) -> resources::Transit_gateway_multicast_domain_associations<'_> {
        resources::Transit_gateway_multicast_domain_associations::new(self.provider)
    }
    /// Get transit_gateway_route_table_announcements resource handler
    pub fn transit_gateway_route_table_announcements(&self) -> resources::Transit_gateway_route_table_announcements<'_> {
        resources::Transit_gateway_route_table_announcements::new(self.provider)
    }
    /// Get ipam_pool resource handler
    pub fn ipam_pool(&self) -> resources::Ipam_pool<'_> {
        resources::Ipam_pool::new(self.provider)
    }
    /// Get replace_root_volume_task resource handler
    pub fn replace_root_volume_task(&self) -> resources::Replace_root_volume_task<'_> {
        resources::Replace_root_volume_task::new(self.provider)
    }
    /// Get instance_event_notification_attributes resource handler
    pub fn instance_event_notification_attributes(&self) -> resources::Instance_event_notification_attributes<'_> {
        resources::Instance_event_notification_attributes::new(self.provider)
    }
    /// Get vpc_endpoint resource handler
    pub fn vpc_endpoint(&self) -> resources::Vpc_endpoint<'_> {
        resources::Vpc_endpoint::new(self.provider)
    }
    /// Get export_image_tasks resource handler
    pub fn export_image_tasks(&self) -> resources::Export_image_tasks<'_> {
        resources::Export_image_tasks::new(self.provider)
    }
    /// Get client_vpn_endpoints resource handler
    pub fn client_vpn_endpoints(&self) -> resources::Client_vpn_endpoints<'_> {
        resources::Client_vpn_endpoints::new(self.provider)
    }
    /// Get fast_snapshot_restores resource handler
    pub fn fast_snapshot_restores(&self) -> resources::Fast_snapshot_restores<'_> {
        resources::Fast_snapshot_restores::new(self.provider)
    }
    /// Get reserved_instances_modifications resource handler
    pub fn reserved_instances_modifications(&self) -> resources::Reserved_instances_modifications<'_> {
        resources::Reserved_instances_modifications::new(self.provider)
    }
    /// Get security_group_rules resource handler
    pub fn security_group_rules(&self) -> resources::Security_group_rules<'_> {
        resources::Security_group_rules::new(self.provider)
    }
    /// Get default_credit_specification resource handler
    pub fn default_credit_specification(&self) -> resources::Default_credit_specification<'_> {
        resources::Default_credit_specification::new(self.provider)
    }
    /// Get vpn_connection_device_types resource handler
    pub fn vpn_connection_device_types(&self) -> resources::Vpn_connection_device_types<'_> {
        resources::Vpn_connection_device_types::new(self.provider)
    }
    /// Get route_tables resource handler
    pub fn route_tables(&self) -> resources::Route_tables<'_> {
        resources::Route_tables::new(self.provider)
    }
    /// Get vpc_classic_link_dns_support resource handler
    pub fn vpc_classic_link_dns_support(&self) -> resources::Vpc_classic_link_dns_support<'_> {
        resources::Vpc_classic_link_dns_support::new(self.provider)
    }
    /// Get flow_logs resource handler
    pub fn flow_logs(&self) -> resources::Flow_logs<'_> {
        resources::Flow_logs::new(self.provider)
    }
    /// Get verified_access_group_policy resource handler
    pub fn verified_access_group_policy(&self) -> resources::Verified_access_group_policy<'_> {
        resources::Verified_access_group_policy::new(self.provider)
    }
    /// Get transit_gateway_prefix_list_reference resource handler
    pub fn transit_gateway_prefix_list_reference(&self) -> resources::Transit_gateway_prefix_list_reference<'_> {
        resources::Transit_gateway_prefix_list_reference::new(self.provider)
    }
    /// Get instance_status resource handler
    pub fn instance_status(&self) -> resources::Instance_status<'_> {
        resources::Instance_status::new(self.provider)
    }
    /// Get verified_access_endpoint_policy resource handler
    pub fn verified_access_endpoint_policy(&self) -> resources::Verified_access_endpoint_policy<'_> {
        resources::Verified_access_endpoint_policy::new(self.provider)
    }
    /// Get instance_event_window resource handler
    pub fn instance_event_window(&self) -> resources::Instance_event_window<'_> {
        resources::Instance_event_window::new(self.provider)
    }
    /// Get key_pair resource handler
    pub fn key_pair(&self) -> resources::Key_pair<'_> {
        resources::Key_pair::new(self.provider)
    }
    /// Get public_ipv4_pools resource handler
    pub fn public_ipv4_pools(&self) -> resources::Public_ipv4_pools<'_> {
        resources::Public_ipv4_pools::new(self.provider)
    }
    /// Get instance_connect_endpoints resource handler
    pub fn instance_connect_endpoints(&self) -> resources::Instance_connect_endpoints<'_> {
        resources::Instance_connect_endpoints::new(self.provider)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
