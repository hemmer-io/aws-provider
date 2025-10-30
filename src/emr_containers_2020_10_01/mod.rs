//! Emr_containers_2020_10_01 Service
//!
//! Auto-generated service module for emr_containers_2020_10_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for emr_containers_2020_10_01
pub struct Emr_containers_2020_10_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Emr_containers_2020_10_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get job_template resource handler
    pub fn job_template(&self) -> resources::Job_template<'_> {
        resources::Job_template::new(self.provider)
    }
    /// Get security_configuration resource handler
    pub fn security_configuration(&self) -> resources::Security_configuration<'_> {
        resources::Security_configuration::new(self.provider)
    }
    /// Get managed_endpoint resource handler
    pub fn managed_endpoint(&self) -> resources::Managed_endpoint<'_> {
        resources::Managed_endpoint::new(self.provider)
    }
    /// Get job_run resource handler
    pub fn job_run(&self) -> resources::Job_run<'_> {
        resources::Job_run::new(self.provider)
    }
    /// Get managed_endpoint_session_credentials resource handler
    pub fn managed_endpoint_session_credentials(&self) -> resources::Managed_endpoint_session_credentials<'_> {
        resources::Managed_endpoint_session_credentials::new(self.provider)
    }
    /// Get virtual_cluster resource handler
    pub fn virtual_cluster(&self) -> resources::Virtual_cluster<'_> {
        resources::Virtual_cluster::new(self.provider)
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
