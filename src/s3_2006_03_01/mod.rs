//! S3_2006_03_01 Service
//!
//! Auto-generated service module for s3_2006_03_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for s3_2006_03_01
pub struct S3_2006_03_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> S3_2006_03_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get bucket_policy resource handler
    pub fn bucket_policy(&self) -> resources::Bucket_policy<'_> {
        resources::Bucket_policy::new(self.provider)
    }
    /// Get bucket_metadata_table_configuration resource handler
    pub fn bucket_metadata_table_configuration(&self) -> resources::Bucket_metadata_table_configuration<'_> {
        resources::Bucket_metadata_table_configuration::new(self.provider)
    }
    /// Get multipart_upload resource handler
    pub fn multipart_upload(&self) -> resources::Multipart_upload<'_> {
        resources::Multipart_upload::new(self.provider)
    }
    /// Get bucket_accelerate_configuration resource handler
    pub fn bucket_accelerate_configuration(&self) -> resources::Bucket_accelerate_configuration<'_> {
        resources::Bucket_accelerate_configuration::new(self.provider)
    }
    /// Get object resource handler
    pub fn object(&self) -> resources::Object<'_> {
        resources::Object::new(self.provider)
    }
    /// Get bucket_ownership_controls resource handler
    pub fn bucket_ownership_controls(&self) -> resources::Bucket_ownership_controls<'_> {
        resources::Bucket_ownership_controls::new(self.provider)
    }
    /// Get bucket_metadata_configuration resource handler
    pub fn bucket_metadata_configuration(&self) -> resources::Bucket_metadata_configuration<'_> {
        resources::Bucket_metadata_configuration::new(self.provider)
    }
    /// Get bucket_analytics_configuration resource handler
    pub fn bucket_analytics_configuration(&self) -> resources::Bucket_analytics_configuration<'_> {
        resources::Bucket_analytics_configuration::new(self.provider)
    }
    /// Get bucket_replication resource handler
    pub fn bucket_replication(&self) -> resources::Bucket_replication<'_> {
        resources::Bucket_replication::new(self.provider)
    }
    /// Get bucket_acl resource handler
    pub fn bucket_acl(&self) -> resources::Bucket_acl<'_> {
        resources::Bucket_acl::new(self.provider)
    }
    /// Get bucket_metadata_journal_table_configuration resource handler
    pub fn bucket_metadata_journal_table_configuration(&self) -> resources::Bucket_metadata_journal_table_configuration<'_> {
        resources::Bucket_metadata_journal_table_configuration::new(self.provider)
    }
    /// Get bucket_tagging resource handler
    pub fn bucket_tagging(&self) -> resources::Bucket_tagging<'_> {
        resources::Bucket_tagging::new(self.provider)
    }
    /// Get bucket_policy_status resource handler
    pub fn bucket_policy_status(&self) -> resources::Bucket_policy_status<'_> {
        resources::Bucket_policy_status::new(self.provider)
    }
    /// Get bucket resource handler
    pub fn bucket(&self) -> resources::Bucket<'_> {
        resources::Bucket::new(self.provider)
    }
    /// Get object_attributes resource handler
    pub fn object_attributes(&self) -> resources::Object_attributes<'_> {
        resources::Object_attributes::new(self.provider)
    }
    /// Get bucket_location resource handler
    pub fn bucket_location(&self) -> resources::Bucket_location<'_> {
        resources::Bucket_location::new(self.provider)
    }
    /// Get object_retention resource handler
    pub fn object_retention(&self) -> resources::Object_retention<'_> {
        resources::Object_retention::new(self.provider)
    }
    /// Get bucket_lifecycle resource handler
    pub fn bucket_lifecycle(&self) -> resources::Bucket_lifecycle<'_> {
        resources::Bucket_lifecycle::new(self.provider)
    }
    /// Get bucket_request_payment resource handler
    pub fn bucket_request_payment(&self) -> resources::Bucket_request_payment<'_> {
        resources::Bucket_request_payment::new(self.provider)
    }
    /// Get bucket_website resource handler
    pub fn bucket_website(&self) -> resources::Bucket_website<'_> {
        resources::Bucket_website::new(self.provider)
    }
    /// Get objects resource handler
    pub fn objects(&self) -> resources::Objects<'_> {
        resources::Objects::new(self.provider)
    }
    /// Get object_legal_hold resource handler
    pub fn object_legal_hold(&self) -> resources::Object_legal_hold<'_> {
        resources::Object_legal_hold::new(self.provider)
    }
    /// Get bucket_metadata_inventory_table_configuration resource handler
    pub fn bucket_metadata_inventory_table_configuration(&self) -> resources::Bucket_metadata_inventory_table_configuration<'_> {
        resources::Bucket_metadata_inventory_table_configuration::new(self.provider)
    }
    /// Get session resource handler
    pub fn session(&self) -> resources::Session<'_> {
        resources::Session::new(self.provider)
    }
    /// Get bucket_metrics_configuration resource handler
    pub fn bucket_metrics_configuration(&self) -> resources::Bucket_metrics_configuration<'_> {
        resources::Bucket_metrics_configuration::new(self.provider)
    }
    /// Get object_acl resource handler
    pub fn object_acl(&self) -> resources::Object_acl<'_> {
        resources::Object_acl::new(self.provider)
    }
    /// Get object_tagging resource handler
    pub fn object_tagging(&self) -> resources::Object_tagging<'_> {
        resources::Object_tagging::new(self.provider)
    }
    /// Get bucket_cors resource handler
    pub fn bucket_cors(&self) -> resources::Bucket_cors<'_> {
        resources::Bucket_cors::new(self.provider)
    }
    /// Get bucket_versioning resource handler
    pub fn bucket_versioning(&self) -> resources::Bucket_versioning<'_> {
        resources::Bucket_versioning::new(self.provider)
    }
    /// Get bucket_intelligent_tiering_configuration resource handler
    pub fn bucket_intelligent_tiering_configuration(&self) -> resources::Bucket_intelligent_tiering_configuration<'_> {
        resources::Bucket_intelligent_tiering_configuration::new(self.provider)
    }
    /// Get public_access_block resource handler
    pub fn public_access_block(&self) -> resources::Public_access_block<'_> {
        resources::Public_access_block::new(self.provider)
    }
    /// Get bucket_encryption resource handler
    pub fn bucket_encryption(&self) -> resources::Bucket_encryption<'_> {
        resources::Bucket_encryption::new(self.provider)
    }
    /// Get bucket_notification_configuration resource handler
    pub fn bucket_notification_configuration(&self) -> resources::Bucket_notification_configuration<'_> {
        resources::Bucket_notification_configuration::new(self.provider)
    }
    /// Get object_lock_configuration resource handler
    pub fn object_lock_configuration(&self) -> resources::Object_lock_configuration<'_> {
        resources::Object_lock_configuration::new(self.provider)
    }
    /// Get bucket_inventory_configuration resource handler
    pub fn bucket_inventory_configuration(&self) -> resources::Bucket_inventory_configuration<'_> {
        resources::Bucket_inventory_configuration::new(self.provider)
    }
    /// Get object_torrent resource handler
    pub fn object_torrent(&self) -> resources::Object_torrent<'_> {
        resources::Object_torrent::new(self.provider)
    }
    /// Get bucket_lifecycle_configuration resource handler
    pub fn bucket_lifecycle_configuration(&self) -> resources::Bucket_lifecycle_configuration<'_> {
        resources::Bucket_lifecycle_configuration::new(self.provider)
    }
    /// Get bucket_logging resource handler
    pub fn bucket_logging(&self) -> resources::Bucket_logging<'_> {
        resources::Bucket_logging::new(self.provider)
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
