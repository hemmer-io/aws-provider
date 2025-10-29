//! Amplify Service
//!
//! Auto-generated service module for amplify

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for amplify
pub struct AmplifyService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> AmplifyService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get webhook resource handler
    pub fn webhook(&self) -> resources::Webhook<'_> {
        resources::Webhook::new(self.provider)
    }
    /// Get domain_association resource handler
    pub fn domain_association(&self) -> resources::Domain_association<'_> {
        resources::Domain_association::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get artifact_url resource handler
    pub fn artifact_url(&self) -> resources::Artifact_url<'_> {
        resources::Artifact_url::new(self.provider)
    }
    /// Get backend_environment resource handler
    pub fn backend_environment(&self) -> resources::Backend_environment<'_> {
        resources::Backend_environment::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get branch resource handler
    pub fn branch(&self) -> resources::Branch<'_> {
        resources::Branch::new(self.provider)
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
