//! Macie2 Service
//!
//! Auto-generated service module for macie2

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for macie2
pub struct Macie2Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Macie2Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get member resource handler
    pub fn member(&self) -> resources::Member<'_> {
        resources::Member::new(self.provider)
    }
    /// Get sensitive_data_occurrences_availability resource handler
    pub fn sensitive_data_occurrences_availability(&self) -> resources::Sensitive_data_occurrences_availability<'_> {
        resources::Sensitive_data_occurrences_availability::new(self.provider)
    }
    /// Get resource_profile_detections resource handler
    pub fn resource_profile_detections(&self) -> resources::Resource_profile_detections<'_> {
        resources::Resource_profile_detections::new(self.provider)
    }
    /// Get classification_scope resource handler
    pub fn classification_scope(&self) -> resources::Classification_scope<'_> {
        resources::Classification_scope::new(self.provider)
    }
    /// Get automated_discovery_configuration resource handler
    pub fn automated_discovery_configuration(&self) -> resources::Automated_discovery_configuration<'_> {
        resources::Automated_discovery_configuration::new(self.provider)
    }
    /// Get findings_publication_configuration resource handler
    pub fn findings_publication_configuration(&self) -> resources::Findings_publication_configuration<'_> {
        resources::Findings_publication_configuration::new(self.provider)
    }
    /// Get custom_data_identifier resource handler
    pub fn custom_data_identifier(&self) -> resources::Custom_data_identifier<'_> {
        resources::Custom_data_identifier::new(self.provider)
    }
    /// Get classification_export_configuration resource handler
    pub fn classification_export_configuration(&self) -> resources::Classification_export_configuration<'_> {
        resources::Classification_export_configuration::new(self.provider)
    }
    /// Get master_account resource handler
    pub fn master_account(&self) -> resources::Master_account<'_> {
        resources::Master_account::new(self.provider)
    }
    /// Get allow_list resource handler
    pub fn allow_list(&self) -> resources::Allow_list<'_> {
        resources::Allow_list::new(self.provider)
    }
    /// Get macie_session resource handler
    pub fn macie_session(&self) -> resources::Macie_session<'_> {
        resources::Macie_session::new(self.provider)
    }
    /// Get member_session resource handler
    pub fn member_session(&self) -> resources::Member_session<'_> {
        resources::Member_session::new(self.provider)
    }
    /// Get sample_findings resource handler
    pub fn sample_findings(&self) -> resources::Sample_findings<'_> {
        resources::Sample_findings::new(self.provider)
    }
    /// Get usage_statistics resource handler
    pub fn usage_statistics(&self) -> resources::Usage_statistics<'_> {
        resources::Usage_statistics::new(self.provider)
    }
    /// Get bucket_statistics resource handler
    pub fn bucket_statistics(&self) -> resources::Bucket_statistics<'_> {
        resources::Bucket_statistics::new(self.provider)
    }
    /// Get invitations_count resource handler
    pub fn invitations_count(&self) -> resources::Invitations_count<'_> {
        resources::Invitations_count::new(self.provider)
    }
    /// Get administrator_account resource handler
    pub fn administrator_account(&self) -> resources::Administrator_account<'_> {
        resources::Administrator_account::new(self.provider)
    }
    /// Get usage_totals resource handler
    pub fn usage_totals(&self) -> resources::Usage_totals<'_> {
        resources::Usage_totals::new(self.provider)
    }
    /// Get reveal_configuration resource handler
    pub fn reveal_configuration(&self) -> resources::Reveal_configuration<'_> {
        resources::Reveal_configuration::new(self.provider)
    }
    /// Get buckets resource handler
    pub fn buckets(&self) -> resources::Buckets<'_> {
        resources::Buckets::new(self.provider)
    }
    /// Get findings resource handler
    pub fn findings(&self) -> resources::Findings<'_> {
        resources::Findings::new(self.provider)
    }
    /// Get sensitive_data_occurrences resource handler
    pub fn sensitive_data_occurrences(&self) -> resources::Sensitive_data_occurrences<'_> {
        resources::Sensitive_data_occurrences::new(self.provider)
    }
    /// Get invitations resource handler
    pub fn invitations(&self) -> resources::Invitations<'_> {
        resources::Invitations::new(self.provider)
    }
    /// Get finding_statistics resource handler
    pub fn finding_statistics(&self) -> resources::Finding_statistics<'_> {
        resources::Finding_statistics::new(self.provider)
    }
    /// Get sensitivity_inspection_template resource handler
    pub fn sensitivity_inspection_template(&self) -> resources::Sensitivity_inspection_template<'_> {
        resources::Sensitivity_inspection_template::new(self.provider)
    }
    /// Get classification_job resource handler
    pub fn classification_job(&self) -> resources::Classification_job<'_> {
        resources::Classification_job::new(self.provider)
    }
    /// Get findings_filter resource handler
    pub fn findings_filter(&self) -> resources::Findings_filter<'_> {
        resources::Findings_filter::new(self.provider)
    }
    /// Get organization_configuration resource handler
    pub fn organization_configuration(&self) -> resources::Organization_configuration<'_> {
        resources::Organization_configuration::new(self.provider)
    }
    /// Get resource_profile resource handler
    pub fn resource_profile(&self) -> resources::Resource_profile<'_> {
        resources::Resource_profile::new(self.provider)
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
