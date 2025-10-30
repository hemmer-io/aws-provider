//! Workspaces_thin_client_2023_08_22 Service
//!
//! Auto-generated service module for workspaces_thin_client_2023_08_22

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workspaces_thin_client_2023_08_22
pub struct Workspaces_thin_client_2023_08_22Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Workspaces_thin_client_2023_08_22Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get device resource handler
    pub fn device(&self) -> resources::Device<'_> {
        resources::Device::new(self.provider)
    }
    /// Get software_set resource handler
    pub fn software_set(&self) -> resources::Software_set<'_> {
        resources::Software_set::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
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
