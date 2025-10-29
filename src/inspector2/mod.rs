//! Inspector2 Service
//!
//! Auto-generated service module for inspector2

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for inspector2
pub struct Inspector2Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Inspector2Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get configuration resource handler
    pub fn configuration(&self) -> resources::Configuration<'_> {
        resources::Configuration::new(self.provider)
    }
    /// Get filter resource handler
    pub fn filter(&self) -> resources::Filter<'_> {
        resources::Filter::new(self.provider)
    }
    /// Get org_ec2_deep_inspection_configuration resource handler
    pub fn org_ec2_deep_inspection_configuration(&self) -> resources::Org_ec2_deep_inspection_configuration<'_> {
        resources::Org_ec2_deep_inspection_configuration::new(self.provider)
    }
    /// Get findings_report_status resource handler
    pub fn findings_report_status(&self) -> resources::Findings_report_status<'_> {
        resources::Findings_report_status::new(self.provider)
    }
    /// Get cis_scan_result_details resource handler
    pub fn cis_scan_result_details(&self) -> resources::Cis_scan_result_details<'_> {
        resources::Cis_scan_result_details::new(self.provider)
    }
    /// Get code_security_scan_configuration resource handler
    pub fn code_security_scan_configuration(&self) -> resources::Code_security_scan_configuration<'_> {
        resources::Code_security_scan_configuration::new(self.provider)
    }
    /// Get encryption_key resource handler
    pub fn encryption_key(&self) -> resources::Encryption_key<'_> {
        resources::Encryption_key::new(self.provider)
    }
    /// Get sbom_export resource handler
    pub fn sbom_export(&self) -> resources::Sbom_export<'_> {
        resources::Sbom_export::new(self.provider)
    }
    /// Get clusters_for_image resource handler
    pub fn clusters_for_image(&self) -> resources::Clusters_for_image<'_> {
        resources::Clusters_for_image::new(self.provider)
    }
    /// Get findings_report resource handler
    pub fn findings_report(&self) -> resources::Findings_report<'_> {
        resources::Findings_report::new(self.provider)
    }
    /// Get cis_scan_report resource handler
    pub fn cis_scan_report(&self) -> resources::Cis_scan_report<'_> {
        resources::Cis_scan_report::new(self.provider)
    }
    /// Get ec2_deep_inspection_configuration resource handler
    pub fn ec2_deep_inspection_configuration(&self) -> resources::Ec2_deep_inspection_configuration<'_> {
        resources::Ec2_deep_inspection_configuration::new(self.provider)
    }
    /// Get code_security_scan resource handler
    pub fn code_security_scan(&self) -> resources::Code_security_scan<'_> {
        resources::Code_security_scan::new(self.provider)
    }
    /// Get member resource handler
    pub fn member(&self) -> resources::Member<'_> {
        resources::Member::new(self.provider)
    }
    /// Get delegated_admin_account resource handler
    pub fn delegated_admin_account(&self) -> resources::Delegated_admin_account<'_> {
        resources::Delegated_admin_account::new(self.provider)
    }
    /// Get organization_configuration resource handler
    pub fn organization_configuration(&self) -> resources::Organization_configuration<'_> {
        resources::Organization_configuration::new(self.provider)
    }
    /// Get cis_scan_configuration resource handler
    pub fn cis_scan_configuration(&self) -> resources::Cis_scan_configuration<'_> {
        resources::Cis_scan_configuration::new(self.provider)
    }
    /// Get code_security_integration resource handler
    pub fn code_security_integration(&self) -> resources::Code_security_integration<'_> {
        resources::Code_security_integration::new(self.provider)
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
