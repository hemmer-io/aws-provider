//! Sagemaker_edge_2020_09_23 Service
//!
//! Auto-generated service module for sagemaker_edge_2020_09_23

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sagemaker_edge_2020_09_23
pub struct Sagemaker_edge_2020_09_23Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sagemaker_edge_2020_09_23Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get device_registration resource handler
    pub fn device_registration(&self) -> resources::Device_registration<'_> {
        resources::Device_registration::new(self.provider)
    }
    /// Get deployments resource handler
    pub fn deployments(&self) -> resources::Deployments<'_> {
        resources::Deployments::new(self.provider)
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
