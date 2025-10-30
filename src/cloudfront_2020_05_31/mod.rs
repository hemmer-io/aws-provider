//! Cloudfront_2020_05_31 Service
//!
//! Auto-generated service module for cloudfront_2020_05_31

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudfront_2020_05_31
pub struct Cloudfront_2020_05_31Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudfront_2020_05_31Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get cache_policy resource handler
    pub fn cache_policy(&self) -> resources::Cache_policy<'_> {
        resources::Cache_policy::new(self.provider)
    }
    /// Get connection_group resource handler
    pub fn connection_group(&self) -> resources::Connection_group<'_> {
        resources::Connection_group::new(self.provider)
    }
    /// Get continuous_deployment_policy resource handler
    pub fn continuous_deployment_policy(&self) -> resources::Continuous_deployment_policy<'_> {
        resources::Continuous_deployment_policy::new(self.provider)
    }
    /// Get field_level_encryption_config resource handler
    pub fn field_level_encryption_config(&self) -> resources::Field_level_encryption_config<'_> {
        resources::Field_level_encryption_config::new(self.provider)
    }
    /// Get invalidation_for_distribution_tenant resource handler
    pub fn invalidation_for_distribution_tenant(&self) -> resources::Invalidation_for_distribution_tenant<'_> {
        resources::Invalidation_for_distribution_tenant::new(self.provider)
    }
    /// Get distribution resource handler
    pub fn distribution(&self) -> resources::Distribution<'_> {
        resources::Distribution::new(self.provider)
    }
    /// Get invalidation resource handler
    pub fn invalidation(&self) -> resources::Invalidation<'_> {
        resources::Invalidation::new(self.provider)
    }
    /// Get distribution_with_tags resource handler
    pub fn distribution_with_tags(&self) -> resources::Distribution_with_tags<'_> {
        resources::Distribution_with_tags::new(self.provider)
    }
    /// Get origin_request_policy resource handler
    pub fn origin_request_policy(&self) -> resources::Origin_request_policy<'_> {
        resources::Origin_request_policy::new(self.provider)
    }
    /// Get connection_group_by_routing_endpoint resource handler
    pub fn connection_group_by_routing_endpoint(&self) -> resources::Connection_group_by_routing_endpoint<'_> {
        resources::Connection_group_by_routing_endpoint::new(self.provider)
    }
    /// Get cloud_front_origin_access_identity_config resource handler
    pub fn cloud_front_origin_access_identity_config(&self) -> resources::Cloud_front_origin_access_identity_config<'_> {
        resources::Cloud_front_origin_access_identity_config::new(self.provider)
    }
    /// Get origin_access_control_config resource handler
    pub fn origin_access_control_config(&self) -> resources::Origin_access_control_config<'_> {
        resources::Origin_access_control_config::new(self.provider)
    }
    /// Get streaming_distribution_with_tags resource handler
    pub fn streaming_distribution_with_tags(&self) -> resources::Streaming_distribution_with_tags<'_> {
        resources::Streaming_distribution_with_tags::new(self.provider)
    }
    /// Get streaming_distribution resource handler
    pub fn streaming_distribution(&self) -> resources::Streaming_distribution<'_> {
        resources::Streaming_distribution::new(self.provider)
    }
    /// Get continuous_deployment_policy_config resource handler
    pub fn continuous_deployment_policy_config(&self) -> resources::Continuous_deployment_policy_config<'_> {
        resources::Continuous_deployment_policy_config::new(self.provider)
    }
    /// Get distribution_tenant_by_domain resource handler
    pub fn distribution_tenant_by_domain(&self) -> resources::Distribution_tenant_by_domain<'_> {
        resources::Distribution_tenant_by_domain::new(self.provider)
    }
    /// Get domain_association resource handler
    pub fn domain_association(&self) -> resources::Domain_association<'_> {
        resources::Domain_association::new(self.provider)
    }
    /// Get distribution_tenant resource handler
    pub fn distribution_tenant(&self) -> resources::Distribution_tenant<'_> {
        resources::Distribution_tenant::new(self.provider)
    }
    /// Get cloud_front_origin_access_identity resource handler
    pub fn cloud_front_origin_access_identity(&self) -> resources::Cloud_front_origin_access_identity<'_> {
        resources::Cloud_front_origin_access_identity::new(self.provider)
    }
    /// Get key_value_store resource handler
    pub fn key_value_store(&self) -> resources::Key_value_store<'_> {
        resources::Key_value_store::new(self.provider)
    }
    /// Get vpc_origin resource handler
    pub fn vpc_origin(&self) -> resources::Vpc_origin<'_> {
        resources::Vpc_origin::new(self.provider)
    }
    /// Get response_headers_policy_config resource handler
    pub fn response_headers_policy_config(&self) -> resources::Response_headers_policy_config<'_> {
        resources::Response_headers_policy_config::new(self.provider)
    }
    /// Get managed_certificate_details resource handler
    pub fn managed_certificate_details(&self) -> resources::Managed_certificate_details<'_> {
        resources::Managed_certificate_details::new(self.provider)
    }
    /// Get field_level_encryption resource handler
    pub fn field_level_encryption(&self) -> resources::Field_level_encryption<'_> {
        resources::Field_level_encryption::new(self.provider)
    }
    /// Get public_key_config resource handler
    pub fn public_key_config(&self) -> resources::Public_key_config<'_> {
        resources::Public_key_config::new(self.provider)
    }
    /// Get streaming_distribution_config resource handler
    pub fn streaming_distribution_config(&self) -> resources::Streaming_distribution_config<'_> {
        resources::Streaming_distribution_config::new(self.provider)
    }
    /// Get key_group resource handler
    pub fn key_group(&self) -> resources::Key_group<'_> {
        resources::Key_group::new(self.provider)
    }
    /// Get cache_policy_config resource handler
    pub fn cache_policy_config(&self) -> resources::Cache_policy_config<'_> {
        resources::Cache_policy_config::new(self.provider)
    }
    /// Get field_level_encryption_profile_config resource handler
    pub fn field_level_encryption_profile_config(&self) -> resources::Field_level_encryption_profile_config<'_> {
        resources::Field_level_encryption_profile_config::new(self.provider)
    }
    /// Get public_key resource handler
    pub fn public_key(&self) -> resources::Public_key<'_> {
        resources::Public_key::new(self.provider)
    }
    /// Get origin_access_control resource handler
    pub fn origin_access_control(&self) -> resources::Origin_access_control<'_> {
        resources::Origin_access_control::new(self.provider)
    }
    /// Get response_headers_policy resource handler
    pub fn response_headers_policy(&self) -> resources::Response_headers_policy<'_> {
        resources::Response_headers_policy::new(self.provider)
    }
    /// Get origin_request_policy_config resource handler
    pub fn origin_request_policy_config(&self) -> resources::Origin_request_policy_config<'_> {
        resources::Origin_request_policy_config::new(self.provider)
    }
    /// Get anycast_ip_list resource handler
    pub fn anycast_ip_list(&self) -> resources::Anycast_ip_list<'_> {
        resources::Anycast_ip_list::new(self.provider)
    }
    /// Get monitoring_subscription resource handler
    pub fn monitoring_subscription(&self) -> resources::Monitoring_subscription<'_> {
        resources::Monitoring_subscription::new(self.provider)
    }
    /// Get field_level_encryption_profile resource handler
    pub fn field_level_encryption_profile(&self) -> resources::Field_level_encryption_profile<'_> {
        resources::Field_level_encryption_profile::new(self.provider)
    }
    /// Get function resource handler
    pub fn function(&self) -> resources::Function<'_> {
        resources::Function::new(self.provider)
    }
    /// Get realtime_log_config resource handler
    pub fn realtime_log_config(&self) -> resources::Realtime_log_config<'_> {
        resources::Realtime_log_config::new(self.provider)
    }
    /// Get key_group_config resource handler
    pub fn key_group_config(&self) -> resources::Key_group_config<'_> {
        resources::Key_group_config::new(self.provider)
    }
    /// Get distribution_config resource handler
    pub fn distribution_config(&self) -> resources::Distribution_config<'_> {
        resources::Distribution_config::new(self.provider)
    }
    /// Get distribution_with_staging_config resource handler
    pub fn distribution_with_staging_config(&self) -> resources::Distribution_with_staging_config<'_> {
        resources::Distribution_with_staging_config::new(self.provider)
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
