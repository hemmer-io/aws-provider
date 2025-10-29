//! Guardduty Service
//!
//! Auto-generated service module for guardduty

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for guardduty
pub struct GuarddutyService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> GuarddutyService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get threat_intel_set resource handler
    pub fn threat_intel_set(&self) -> resources::Threat_intel_set<'_> {
        resources::Threat_intel_set::new(self.provider)
    }
    /// Get members resource handler
    pub fn members(&self) -> resources::Members<'_> {
        resources::Members::new(self.provider)
    }
    /// Get master_account resource handler
    pub fn master_account(&self) -> resources::Master_account<'_> {
        resources::Master_account::new(self.provider)
    }
    /// Get filter resource handler
    pub fn filter(&self) -> resources::Filter<'_> {
        resources::Filter::new(self.provider)
    }
    /// Get threat_entity_set resource handler
    pub fn threat_entity_set(&self) -> resources::Threat_entity_set<'_> {
        resources::Threat_entity_set::new(self.provider)
    }
    /// Get publishing_destination resource handler
    pub fn publishing_destination(&self) -> resources::Publishing_destination<'_> {
        resources::Publishing_destination::new(self.provider)
    }
    /// Get malware_scans resource handler
    pub fn malware_scans(&self) -> resources::Malware_scans<'_> {
        resources::Malware_scans::new(self.provider)
    }
    /// Get findings resource handler
    pub fn findings(&self) -> resources::Findings<'_> {
        resources::Findings::new(self.provider)
    }
    /// Get findings_feedback resource handler
    pub fn findings_feedback(&self) -> resources::Findings_feedback<'_> {
        resources::Findings_feedback::new(self.provider)
    }
    /// Get malware_protection_plan resource handler
    pub fn malware_protection_plan(&self) -> resources::Malware_protection_plan<'_> {
        resources::Malware_protection_plan::new(self.provider)
    }
    /// Get invitations_count resource handler
    pub fn invitations_count(&self) -> resources::Invitations_count<'_> {
        resources::Invitations_count::new(self.provider)
    }
    /// Get remaining_free_trial_days resource handler
    pub fn remaining_free_trial_days(&self) -> resources::Remaining_free_trial_days<'_> {
        resources::Remaining_free_trial_days::new(self.provider)
    }
    /// Get malware_scan_settings resource handler
    pub fn malware_scan_settings(&self) -> resources::Malware_scan_settings<'_> {
        resources::Malware_scan_settings::new(self.provider)
    }
    /// Get trusted_entity_set resource handler
    pub fn trusted_entity_set(&self) -> resources::Trusted_entity_set<'_> {
        resources::Trusted_entity_set::new(self.provider)
    }
    /// Get findings_statistics resource handler
    pub fn findings_statistics(&self) -> resources::Findings_statistics<'_> {
        resources::Findings_statistics::new(self.provider)
    }
    /// Get sample_findings resource handler
    pub fn sample_findings(&self) -> resources::Sample_findings<'_> {
        resources::Sample_findings::new(self.provider)
    }
    /// Get organization_statistics resource handler
    pub fn organization_statistics(&self) -> resources::Organization_statistics<'_> {
        resources::Organization_statistics::new(self.provider)
    }
    /// Get ipset resource handler
    pub fn ipset(&self) -> resources::Ipset<'_> {
        resources::Ipset::new(self.provider)
    }
    /// Get coverage_statistics resource handler
    pub fn coverage_statistics(&self) -> resources::Coverage_statistics<'_> {
        resources::Coverage_statistics::new(self.provider)
    }
    /// Get member_detectors resource handler
    pub fn member_detectors(&self) -> resources::Member_detectors<'_> {
        resources::Member_detectors::new(self.provider)
    }
    /// Get detector resource handler
    pub fn detector(&self) -> resources::Detector<'_> {
        resources::Detector::new(self.provider)
    }
    /// Get usage_statistics resource handler
    pub fn usage_statistics(&self) -> resources::Usage_statistics<'_> {
        resources::Usage_statistics::new(self.provider)
    }
    /// Get organization_configuration resource handler
    pub fn organization_configuration(&self) -> resources::Organization_configuration<'_> {
        resources::Organization_configuration::new(self.provider)
    }
    /// Get administrator_account resource handler
    pub fn administrator_account(&self) -> resources::Administrator_account<'_> {
        resources::Administrator_account::new(self.provider)
    }
    /// Get invitations resource handler
    pub fn invitations(&self) -> resources::Invitations<'_> {
        resources::Invitations::new(self.provider)
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
