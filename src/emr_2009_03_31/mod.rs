//! Emr_2009_03_31 Service
//!
//! Auto-generated service module for emr_2009_03_31

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for emr_2009_03_31
pub struct Emr_2009_03_31Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Emr_2009_03_31Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get job_flows resource handler
    pub fn job_flows(&self) -> resources::Job_flows<'_> {
        resources::Job_flows::new(self.provider)
    }
    /// Get security_configuration resource handler
    pub fn security_configuration(&self) -> resources::Security_configuration<'_> {
        resources::Security_configuration::new(self.provider)
    }
    /// Get release_label resource handler
    pub fn release_label(&self) -> resources::Release_label<'_> {
        resources::Release_label::new(self.provider)
    }
    /// Get studio resource handler
    pub fn studio(&self) -> resources::Studio<'_> {
        resources::Studio::new(self.provider)
    }
    /// Get persistent_app_ui resource handler
    pub fn persistent_app_ui(&self) -> resources::Persistent_app_ui<'_> {
        resources::Persistent_app_ui::new(self.provider)
    }
    /// Get auto_termination_policy resource handler
    pub fn auto_termination_policy(&self) -> resources::Auto_termination_policy<'_> {
        resources::Auto_termination_policy::new(self.provider)
    }
    /// Get persistent_app_ui_presigned_url resource handler
    pub fn persistent_app_ui_presigned_url(&self) -> resources::Persistent_app_ui_presigned_url<'_> {
        resources::Persistent_app_ui_presigned_url::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get cluster_session_credentials resource handler
    pub fn cluster_session_credentials(&self) -> resources::Cluster_session_credentials<'_> {
        resources::Cluster_session_credentials::new(self.provider)
    }
    /// Get notebook_execution resource handler
    pub fn notebook_execution(&self) -> resources::Notebook_execution<'_> {
        resources::Notebook_execution::new(self.provider)
    }
    /// Get on_cluster_app_ui_presigned_url resource handler
    pub fn on_cluster_app_ui_presigned_url(&self) -> resources::On_cluster_app_ui_presigned_url<'_> {
        resources::On_cluster_app_ui_presigned_url::new(self.provider)
    }
    /// Get studio_session_mapping resource handler
    pub fn studio_session_mapping(&self) -> resources::Studio_session_mapping<'_> {
        resources::Studio_session_mapping::new(self.provider)
    }
    /// Get step resource handler
    pub fn step(&self) -> resources::Step<'_> {
        resources::Step::new(self.provider)
    }
    /// Get managed_scaling_policy resource handler
    pub fn managed_scaling_policy(&self) -> resources::Managed_scaling_policy<'_> {
        resources::Managed_scaling_policy::new(self.provider)
    }
    /// Get auto_scaling_policy resource handler
    pub fn auto_scaling_policy(&self) -> resources::Auto_scaling_policy<'_> {
        resources::Auto_scaling_policy::new(self.provider)
    }
    /// Get block_public_access_configuration resource handler
    pub fn block_public_access_configuration(&self) -> resources::Block_public_access_configuration<'_> {
        resources::Block_public_access_configuration::new(self.provider)
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
