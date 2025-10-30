//! S3_control_2018_08_20 Service
//!
//! Auto-generated service module for s3_control_2018_08_20

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for s3_control_2018_08_20
pub struct S3_control_2018_08_20Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> S3_control_2018_08_20Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get access_grants_instance resource handler
    pub fn access_grants_instance(&self) -> resources::Access_grants_instance<'_> {
        resources::Access_grants_instance::new(self.provider)
    }
    /// Get access_grants_location resource handler
    pub fn access_grants_location(&self) -> resources::Access_grants_location<'_> {
        resources::Access_grants_location::new(self.provider)
    }
    /// Get storage_lens_group resource handler
    pub fn storage_lens_group(&self) -> resources::Storage_lens_group<'_> {
        resources::Storage_lens_group::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get multi_region_access_point resource handler
    pub fn multi_region_access_point(&self) -> resources::Multi_region_access_point<'_> {
        resources::Multi_region_access_point::new(self.provider)
    }
    /// Get access_grants_instance_resource_policy resource handler
    pub fn access_grants_instance_resource_policy(&self) -> resources::Access_grants_instance_resource_policy<'_> {
        resources::Access_grants_instance_resource_policy::new(self.provider)
    }
    /// Get storage_lens_configuration resource handler
    pub fn storage_lens_configuration(&self) -> resources::Storage_lens_configuration<'_> {
        resources::Storage_lens_configuration::new(self.provider)
    }
    /// Get access_point_policy resource handler
    pub fn access_point_policy(&self) -> resources::Access_point_policy<'_> {
        resources::Access_point_policy::new(self.provider)
    }
    /// Get bucket_policy resource handler
    pub fn bucket_policy(&self) -> resources::Bucket_policy<'_> {
        resources::Bucket_policy::new(self.provider)
    }
    /// Get bucket_tagging resource handler
    pub fn bucket_tagging(&self) -> resources::Bucket_tagging<'_> {
        resources::Bucket_tagging::new(self.provider)
    }
    /// Get job_tagging resource handler
    pub fn job_tagging(&self) -> resources::Job_tagging<'_> {
        resources::Job_tagging::new(self.provider)
    }
    /// Get access_point resource handler
    pub fn access_point(&self) -> resources::Access_point<'_> {
        resources::Access_point::new(self.provider)
    }
    /// Get bucket resource handler
    pub fn bucket(&self) -> resources::Bucket<'_> {
        resources::Bucket::new(self.provider)
    }
    /// Get multi_region_access_point_policy resource handler
    pub fn multi_region_access_point_policy(&self) -> resources::Multi_region_access_point_policy<'_> {
        resources::Multi_region_access_point_policy::new(self.provider)
    }
    /// Get multi_region_access_point_operation resource handler
    pub fn multi_region_access_point_operation(&self) -> resources::Multi_region_access_point_operation<'_> {
        resources::Multi_region_access_point_operation::new(self.provider)
    }
    /// Get job_priority resource handler
    pub fn job_priority(&self) -> resources::Job_priority<'_> {
        resources::Job_priority::new(self.provider)
    }
    /// Get access_point_policy_status resource handler
    pub fn access_point_policy_status(&self) -> resources::Access_point_policy_status<'_> {
        resources::Access_point_policy_status::new(self.provider)
    }
    /// Get job_status resource handler
    pub fn job_status(&self) -> resources::Job_status<'_> {
        resources::Job_status::new(self.provider)
    }
    /// Get access_point_policy_for_object_lambda resource handler
    pub fn access_point_policy_for_object_lambda(&self) -> resources::Access_point_policy_for_object_lambda<'_> {
        resources::Access_point_policy_for_object_lambda::new(self.provider)
    }
    /// Get bucket_replication resource handler
    pub fn bucket_replication(&self) -> resources::Bucket_replication<'_> {
        resources::Bucket_replication::new(self.provider)
    }
    /// Get access_point_configuration_for_object_lambda resource handler
    pub fn access_point_configuration_for_object_lambda(&self) -> resources::Access_point_configuration_for_object_lambda<'_> {
        resources::Access_point_configuration_for_object_lambda::new(self.provider)
    }
    /// Get multi_region_access_point_policy_status resource handler
    pub fn multi_region_access_point_policy_status(&self) -> resources::Multi_region_access_point_policy_status<'_> {
        resources::Multi_region_access_point_policy_status::new(self.provider)
    }
    /// Get multi_region_access_point_routes resource handler
    pub fn multi_region_access_point_routes(&self) -> resources::Multi_region_access_point_routes<'_> {
        resources::Multi_region_access_point_routes::new(self.provider)
    }
    /// Get access_point_for_object_lambda resource handler
    pub fn access_point_for_object_lambda(&self) -> resources::Access_point_for_object_lambda<'_> {
        resources::Access_point_for_object_lambda::new(self.provider)
    }
    /// Get bucket_lifecycle_configuration resource handler
    pub fn bucket_lifecycle_configuration(&self) -> resources::Bucket_lifecycle_configuration<'_> {
        resources::Bucket_lifecycle_configuration::new(self.provider)
    }
    /// Get access_grants_instance_for_prefix resource handler
    pub fn access_grants_instance_for_prefix(&self) -> resources::Access_grants_instance_for_prefix<'_> {
        resources::Access_grants_instance_for_prefix::new(self.provider)
    }
    /// Get data_access resource handler
    pub fn data_access(&self) -> resources::Data_access<'_> {
        resources::Data_access::new(self.provider)
    }
    /// Get access_point_scope resource handler
    pub fn access_point_scope(&self) -> resources::Access_point_scope<'_> {
        resources::Access_point_scope::new(self.provider)
    }
    /// Get storage_lens_configuration_tagging resource handler
    pub fn storage_lens_configuration_tagging(&self) -> resources::Storage_lens_configuration_tagging<'_> {
        resources::Storage_lens_configuration_tagging::new(self.provider)
    }
    /// Get access_point_policy_status_for_object_lambda resource handler
    pub fn access_point_policy_status_for_object_lambda(&self) -> resources::Access_point_policy_status_for_object_lambda<'_> {
        resources::Access_point_policy_status_for_object_lambda::new(self.provider)
    }
    /// Get public_access_block resource handler
    pub fn public_access_block(&self) -> resources::Public_access_block<'_> {
        resources::Public_access_block::new(self.provider)
    }
    /// Get bucket_versioning resource handler
    pub fn bucket_versioning(&self) -> resources::Bucket_versioning<'_> {
        resources::Bucket_versioning::new(self.provider)
    }
    /// Get access_grant resource handler
    pub fn access_grant(&self) -> resources::Access_grant<'_> {
        resources::Access_grant::new(self.provider)
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
