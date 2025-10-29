//! Lightsail Service
//!
//! Auto-generated service module for lightsail

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for lightsail
pub struct LightsailService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> LightsailService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
    }
    /// Get load_balancer_tls_policies resource handler
    pub fn load_balancer_tls_policies(&self) -> resources::Load_balancer_tls_policies<'_> {
        resources::Load_balancer_tls_policies::new(self.provider)
    }
    /// Get guisession_access_details resource handler
    pub fn guisession_access_details(&self) -> resources::Guisession_access_details<'_> {
        resources::Guisession_access_details::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get bucket_metric_data resource handler
    pub fn bucket_metric_data(&self) -> resources::Bucket_metric_data<'_> {
        resources::Bucket_metric_data::new(self.provider)
    }
    /// Get instance_port_states resource handler
    pub fn instance_port_states(&self) -> resources::Instance_port_states<'_> {
        resources::Instance_port_states::new(self.provider)
    }
    /// Get operations resource handler
    pub fn operations(&self) -> resources::Operations<'_> {
        resources::Operations::new(self.provider)
    }
    /// Get instance_metadata_options resource handler
    pub fn instance_metadata_options(&self) -> resources::Instance_metadata_options<'_> {
        resources::Instance_metadata_options::new(self.provider)
    }
    /// Get buckets resource handler
    pub fn buckets(&self) -> resources::Buckets<'_> {
        resources::Buckets::new(self.provider)
    }
    /// Get key_pair resource handler
    pub fn key_pair(&self) -> resources::Key_pair<'_> {
        resources::Key_pair::new(self.provider)
    }
    /// Get relational_database_snapshot resource handler
    pub fn relational_database_snapshot(&self) -> resources::Relational_database_snapshot<'_> {
        resources::Relational_database_snapshot::new(self.provider)
    }
    /// Get static_ips resource handler
    pub fn static_ips(&self) -> resources::Static_ips<'_> {
        resources::Static_ips::new(self.provider)
    }
    /// Get bucket resource handler
    pub fn bucket(&self) -> resources::Bucket<'_> {
        resources::Bucket::new(self.provider)
    }
    /// Get contact_method resource handler
    pub fn contact_method(&self) -> resources::Contact_method<'_> {
        resources::Contact_method::new(self.provider)
    }
    /// Get disk_from_snapshot resource handler
    pub fn disk_from_snapshot(&self) -> resources::Disk_from_snapshot<'_> {
        resources::Disk_from_snapshot::new(self.provider)
    }
    /// Get container_service_powers resource handler
    pub fn container_service_powers(&self) -> resources::Container_service_powers<'_> {
        resources::Container_service_powers::new(self.provider)
    }
    /// Get container_service_deployment resource handler
    pub fn container_service_deployment(&self) -> resources::Container_service_deployment<'_> {
        resources::Container_service_deployment::new(self.provider)
    }
    /// Get bucket_access_keys resource handler
    pub fn bucket_access_keys(&self) -> resources::Bucket_access_keys<'_> {
        resources::Bucket_access_keys::new(self.provider)
    }
    /// Get operations_for_resource resource handler
    pub fn operations_for_resource(&self) -> resources::Operations_for_resource<'_> {
        resources::Operations_for_resource::new(self.provider)
    }
    /// Get container_apimetadata resource handler
    pub fn container_apimetadata(&self) -> resources::Container_apimetadata<'_> {
        resources::Container_apimetadata::new(self.provider)
    }
    /// Get relational_database_metric_data resource handler
    pub fn relational_database_metric_data(&self) -> resources::Relational_database_metric_data<'_> {
        resources::Relational_database_metric_data::new(self.provider)
    }
    /// Get certificate resource handler
    pub fn certificate(&self) -> resources::Certificate<'_> {
        resources::Certificate::new(self.provider)
    }
    /// Get relational_databases resource handler
    pub fn relational_databases(&self) -> resources::Relational_databases<'_> {
        resources::Relational_databases::new(self.provider)
    }
    /// Get known_host_keys resource handler
    pub fn known_host_keys(&self) -> resources::Known_host_keys<'_> {
        resources::Known_host_keys::new(self.provider)
    }
    /// Get load_balancer resource handler
    pub fn load_balancer(&self) -> resources::Load_balancer<'_> {
        resources::Load_balancer::new(self.provider)
    }
    /// Get load_balancers resource handler
    pub fn load_balancers(&self) -> resources::Load_balancers<'_> {
        resources::Load_balancers::new(self.provider)
    }
    /// Get relational_database_bundles resource handler
    pub fn relational_database_bundles(&self) -> resources::Relational_database_bundles<'_> {
        resources::Relational_database_bundles::new(self.provider)
    }
    /// Get load_balancer_attribute resource handler
    pub fn load_balancer_attribute(&self) -> resources::Load_balancer_attribute<'_> {
        resources::Load_balancer_attribute::new(self.provider)
    }
    /// Get distributions resource handler
    pub fn distributions(&self) -> resources::Distributions<'_> {
        resources::Distributions::new(self.provider)
    }
    /// Get instance_public_ports resource handler
    pub fn instance_public_ports(&self) -> resources::Instance_public_ports<'_> {
        resources::Instance_public_ports::new(self.provider)
    }
    /// Get instance_snapshot resource handler
    pub fn instance_snapshot(&self) -> resources::Instance_snapshot<'_> {
        resources::Instance_snapshot::new(self.provider)
    }
    /// Get blueprints resource handler
    pub fn blueprints(&self) -> resources::Blueprints<'_> {
        resources::Blueprints::new(self.provider)
    }
    /// Get disk resource handler
    pub fn disk(&self) -> resources::Disk<'_> {
        resources::Disk::new(self.provider)
    }
    /// Get relational_database_master_user_password resource handler
    pub fn relational_database_master_user_password(&self) -> resources::Relational_database_master_user_password<'_> {
        resources::Relational_database_master_user_password::new(self.provider)
    }
    /// Get instances_from_snapshot resource handler
    pub fn instances_from_snapshot(&self) -> resources::Instances_from_snapshot<'_> {
        resources::Instances_from_snapshot::new(self.provider)
    }
    /// Get instance_metric_data resource handler
    pub fn instance_metric_data(&self) -> resources::Instance_metric_data<'_> {
        resources::Instance_metric_data::new(self.provider)
    }
    /// Get container_images resource handler
    pub fn container_images(&self) -> resources::Container_images<'_> {
        resources::Container_images::new(self.provider)
    }
    /// Get disk_snapshots resource handler
    pub fn disk_snapshots(&self) -> resources::Disk_snapshots<'_> {
        resources::Disk_snapshots::new(self.provider)
    }
    /// Get load_balancer_tls_certificates resource handler
    pub fn load_balancer_tls_certificates(&self) -> resources::Load_balancer_tls_certificates<'_> {
        resources::Load_balancer_tls_certificates::new(self.provider)
    }
    /// Get active_names resource handler
    pub fn active_names(&self) -> resources::Active_names<'_> {
        resources::Active_names::new(self.provider)
    }
    /// Get cloud_formation_stack resource handler
    pub fn cloud_formation_stack(&self) -> resources::Cloud_formation_stack<'_> {
        resources::Cloud_formation_stack::new(self.provider)
    }
    /// Get container_service_registry_login resource handler
    pub fn container_service_registry_login(&self) -> resources::Container_service_registry_login<'_> {
        resources::Container_service_registry_login::new(self.provider)
    }
    /// Get relational_database_log_streams resource handler
    pub fn relational_database_log_streams(&self) -> resources::Relational_database_log_streams<'_> {
        resources::Relational_database_log_streams::new(self.provider)
    }
    /// Get relational_database_snapshots resource handler
    pub fn relational_database_snapshots(&self) -> resources::Relational_database_snapshots<'_> {
        resources::Relational_database_snapshots::new(self.provider)
    }
    /// Get domain_entry resource handler
    pub fn domain_entry(&self) -> resources::Domain_entry<'_> {
        resources::Domain_entry::new(self.provider)
    }
    /// Get alarm resource handler
    pub fn alarm(&self) -> resources::Alarm<'_> {
        resources::Alarm::new(self.provider)
    }
    /// Get container_log resource handler
    pub fn container_log(&self) -> resources::Container_log<'_> {
        resources::Container_log::new(self.provider)
    }
    /// Get instance resource handler
    pub fn instance(&self) -> resources::Instance<'_> {
        resources::Instance::new(self.provider)
    }
    /// Get distribution_bundle resource handler
    pub fn distribution_bundle(&self) -> resources::Distribution_bundle<'_> {
        resources::Distribution_bundle::new(self.provider)
    }
    /// Get disk_snapshot resource handler
    pub fn disk_snapshot(&self) -> resources::Disk_snapshot<'_> {
        resources::Disk_snapshot::new(self.provider)
    }
    /// Get container_service_deployments resource handler
    pub fn container_service_deployments(&self) -> resources::Container_service_deployments<'_> {
        resources::Container_service_deployments::new(self.provider)
    }
    /// Get disks resource handler
    pub fn disks(&self) -> resources::Disks<'_> {
        resources::Disks::new(self.provider)
    }
    /// Get instance_state resource handler
    pub fn instance_state(&self) -> resources::Instance_state<'_> {
        resources::Instance_state::new(self.provider)
    }
    /// Get bucket_bundle resource handler
    pub fn bucket_bundle(&self) -> resources::Bucket_bundle<'_> {
        resources::Bucket_bundle::new(self.provider)
    }
    /// Get export_snapshot_records resource handler
    pub fn export_snapshot_records(&self) -> resources::Export_snapshot_records<'_> {
        resources::Export_snapshot_records::new(self.provider)
    }
    /// Get alarms resource handler
    pub fn alarms(&self) -> resources::Alarms<'_> {
        resources::Alarms::new(self.provider)
    }
    /// Get bundles resource handler
    pub fn bundles(&self) -> resources::Bundles<'_> {
        resources::Bundles::new(self.provider)
    }
    /// Get instance_access_details resource handler
    pub fn instance_access_details(&self) -> resources::Instance_access_details<'_> {
        resources::Instance_access_details::new(self.provider)
    }
    /// Get domains resource handler
    pub fn domains(&self) -> resources::Domains<'_> {
        resources::Domains::new(self.provider)
    }
    /// Get relational_database_from_snapshot resource handler
    pub fn relational_database_from_snapshot(&self) -> resources::Relational_database_from_snapshot<'_> {
        resources::Relational_database_from_snapshot::new(self.provider)
    }
    /// Get certificates resource handler
    pub fn certificates(&self) -> resources::Certificates<'_> {
        resources::Certificates::new(self.provider)
    }
    /// Get auto_snapshots resource handler
    pub fn auto_snapshots(&self) -> resources::Auto_snapshots<'_> {
        resources::Auto_snapshots::new(self.provider)
    }
    /// Get container_service_metric_data resource handler
    pub fn container_service_metric_data(&self) -> resources::Container_service_metric_data<'_> {
        resources::Container_service_metric_data::new(self.provider)
    }
    /// Get distribution_latest_cache_reset resource handler
    pub fn distribution_latest_cache_reset(&self) -> resources::Distribution_latest_cache_reset<'_> {
        resources::Distribution_latest_cache_reset::new(self.provider)
    }
    /// Get container_services resource handler
    pub fn container_services(&self) -> resources::Container_services<'_> {
        resources::Container_services::new(self.provider)
    }
    /// Get key_pairs resource handler
    pub fn key_pairs(&self) -> resources::Key_pairs<'_> {
        resources::Key_pairs::new(self.provider)
    }
    /// Get distribution_bundles resource handler
    pub fn distribution_bundles(&self) -> resources::Distribution_bundles<'_> {
        resources::Distribution_bundles::new(self.provider)
    }
    /// Get auto_snapshot resource handler
    pub fn auto_snapshot(&self) -> resources::Auto_snapshot<'_> {
        resources::Auto_snapshot::new(self.provider)
    }
    /// Get instances resource handler
    pub fn instances(&self) -> resources::Instances<'_> {
        resources::Instances::new(self.provider)
    }
    /// Get regions resource handler
    pub fn regions(&self) -> resources::Regions<'_> {
        resources::Regions::new(self.provider)
    }
    /// Get instance_snapshots resource handler
    pub fn instance_snapshots(&self) -> resources::Instance_snapshots<'_> {
        resources::Instance_snapshots::new(self.provider)
    }
    /// Get setup_history resource handler
    pub fn setup_history(&self) -> resources::Setup_history<'_> {
        resources::Setup_history::new(self.provider)
    }
    /// Get relational_database_blueprints resource handler
    pub fn relational_database_blueprints(&self) -> resources::Relational_database_blueprints<'_> {
        resources::Relational_database_blueprints::new(self.provider)
    }
    /// Get load_balancer_tls_certificate resource handler
    pub fn load_balancer_tls_certificate(&self) -> resources::Load_balancer_tls_certificate<'_> {
        resources::Load_balancer_tls_certificate::new(self.provider)
    }
    /// Get bucket_bundles resource handler
    pub fn bucket_bundles(&self) -> resources::Bucket_bundles<'_> {
        resources::Bucket_bundles::new(self.provider)
    }
    /// Get container_image resource handler
    pub fn container_image(&self) -> resources::Container_image<'_> {
        resources::Container_image::new(self.provider)
    }
    /// Get cloud_formation_stack_records resource handler
    pub fn cloud_formation_stack_records(&self) -> resources::Cloud_formation_stack_records<'_> {
        resources::Cloud_formation_stack_records::new(self.provider)
    }
    /// Get cost_estimate resource handler
    pub fn cost_estimate(&self) -> resources::Cost_estimate<'_> {
        resources::Cost_estimate::new(self.provider)
    }
    /// Get relational_database_events resource handler
    pub fn relational_database_events(&self) -> resources::Relational_database_events<'_> {
        resources::Relational_database_events::new(self.provider)
    }
    /// Get relational_database_log_events resource handler
    pub fn relational_database_log_events(&self) -> resources::Relational_database_log_events<'_> {
        resources::Relational_database_log_events::new(self.provider)
    }
    /// Get relational_database_parameters resource handler
    pub fn relational_database_parameters(&self) -> resources::Relational_database_parameters<'_> {
        resources::Relational_database_parameters::new(self.provider)
    }
    /// Get static_ip resource handler
    pub fn static_ip(&self) -> resources::Static_ip<'_> {
        resources::Static_ip::new(self.provider)
    }
    /// Get load_balancer_metric_data resource handler
    pub fn load_balancer_metric_data(&self) -> resources::Load_balancer_metric_data<'_> {
        resources::Load_balancer_metric_data::new(self.provider)
    }
    /// Get distribution resource handler
    pub fn distribution(&self) -> resources::Distribution<'_> {
        resources::Distribution::new(self.provider)
    }
    /// Get contact_methods resource handler
    pub fn contact_methods(&self) -> resources::Contact_methods<'_> {
        resources::Contact_methods::new(self.provider)
    }
    /// Get container_service resource handler
    pub fn container_service(&self) -> resources::Container_service<'_> {
        resources::Container_service::new(self.provider)
    }
    /// Get distribution_metric_data resource handler
    pub fn distribution_metric_data(&self) -> resources::Distribution_metric_data<'_> {
        resources::Distribution_metric_data::new(self.provider)
    }
    /// Get relational_database resource handler
    pub fn relational_database(&self) -> resources::Relational_database<'_> {
        resources::Relational_database::new(self.provider)
    }
    /// Get bucket_access_key resource handler
    pub fn bucket_access_key(&self) -> resources::Bucket_access_key<'_> {
        resources::Bucket_access_key::new(self.provider)
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
