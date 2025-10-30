//! Auditmanager_2017_07_25 Service
//!
//! Auto-generated service module for auditmanager_2017_07_25

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for auditmanager_2017_07_25
pub struct Auditmanager_2017_07_25Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Auditmanager_2017_07_25Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get assessment_status resource handler
    pub fn assessment_status(&self) -> resources::Assessment_status<'_> {
        resources::Assessment_status::new(self.provider)
    }
    /// Get evidence_folders_by_assessment_control resource handler
    pub fn evidence_folders_by_assessment_control(&self) -> resources::Evidence_folders_by_assessment_control<'_> {
        resources::Evidence_folders_by_assessment_control::new(self.provider)
    }
    /// Get insights_by_assessment resource handler
    pub fn insights_by_assessment(&self) -> resources::Insights_by_assessment<'_> {
        resources::Insights_by_assessment::new(self.provider)
    }
    /// Get assessment_framework_share resource handler
    pub fn assessment_framework_share(&self) -> resources::Assessment_framework_share<'_> {
        resources::Assessment_framework_share::new(self.provider)
    }
    /// Get assessment resource handler
    pub fn assessment(&self) -> resources::Assessment<'_> {
        resources::Assessment::new(self.provider)
    }
    /// Get assessment_control resource handler
    pub fn assessment_control(&self) -> resources::Assessment_control<'_> {
        resources::Assessment_control::new(self.provider)
    }
    /// Get evidence_file_upload_url resource handler
    pub fn evidence_file_upload_url(&self) -> resources::Evidence_file_upload_url<'_> {
        resources::Evidence_file_upload_url::new(self.provider)
    }
    /// Get insights resource handler
    pub fn insights(&self) -> resources::Insights<'_> {
        resources::Insights::new(self.provider)
    }
    /// Get delegations resource handler
    pub fn delegations(&self) -> resources::Delegations<'_> {
        resources::Delegations::new(self.provider)
    }
    /// Get assessment_control_set_status resource handler
    pub fn assessment_control_set_status(&self) -> resources::Assessment_control_set_status<'_> {
        resources::Assessment_control_set_status::new(self.provider)
    }
    /// Get assessment_report resource handler
    pub fn assessment_report(&self) -> resources::Assessment_report<'_> {
        resources::Assessment_report::new(self.provider)
    }
    /// Get account_status resource handler
    pub fn account_status(&self) -> resources::Account_status<'_> {
        resources::Account_status::new(self.provider)
    }
    /// Get evidence resource handler
    pub fn evidence(&self) -> resources::Evidence<'_> {
        resources::Evidence::new(self.provider)
    }
    /// Get control resource handler
    pub fn control(&self) -> resources::Control<'_> {
        resources::Control::new(self.provider)
    }
    /// Get evidence_by_evidence_folder resource handler
    pub fn evidence_by_evidence_folder(&self) -> resources::Evidence_by_evidence_folder<'_> {
        resources::Evidence_by_evidence_folder::new(self.provider)
    }
    /// Get evidence_folder resource handler
    pub fn evidence_folder(&self) -> resources::Evidence_folder<'_> {
        resources::Evidence_folder::new(self.provider)
    }
    /// Get organization_admin_account resource handler
    pub fn organization_admin_account(&self) -> resources::Organization_admin_account<'_> {
        resources::Organization_admin_account::new(self.provider)
    }
    /// Get services_in_scope resource handler
    pub fn services_in_scope(&self) -> resources::Services_in_scope<'_> {
        resources::Services_in_scope::new(self.provider)
    }
    /// Get evidence_folders_by_assessment resource handler
    pub fn evidence_folders_by_assessment(&self) -> resources::Evidence_folders_by_assessment<'_> {
        resources::Evidence_folders_by_assessment::new(self.provider)
    }
    /// Get assessment_framework resource handler
    pub fn assessment_framework(&self) -> resources::Assessment_framework<'_> {
        resources::Assessment_framework::new(self.provider)
    }
    /// Get settings resource handler
    pub fn settings(&self) -> resources::Settings<'_> {
        resources::Settings::new(self.provider)
    }
    /// Get change_logs resource handler
    pub fn change_logs(&self) -> resources::Change_logs<'_> {
        resources::Change_logs::new(self.provider)
    }
    /// Get assessment_report_url resource handler
    pub fn assessment_report_url(&self) -> resources::Assessment_report_url<'_> {
        resources::Assessment_report_url::new(self.provider)
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
