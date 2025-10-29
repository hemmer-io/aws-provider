//! Dax Service
//!
//! Auto-generated service module for dax

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for dax
pub struct DaxService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> DaxService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get parameters resource handler
    pub fn parameters(&self) -> resources::Parameters<'_> {
        resources::Parameters::new(self.provider)
    }
    /// Get subnet_groups resource handler
    pub fn subnet_groups(&self) -> resources::Subnet_groups<'_> {
        resources::Subnet_groups::new(self.provider)
    }
    /// Get parameter_group resource handler
    pub fn parameter_group(&self) -> resources::Parameter_group<'_> {
        resources::Parameter_group::new(self.provider)
    }
    /// Get subnet_group resource handler
    pub fn subnet_group(&self) -> resources::Subnet_group<'_> {
        resources::Subnet_group::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get cluster resource handler
    pub fn cluster(&self) -> resources::Cluster<'_> {
        resources::Cluster::new(self.provider)
    }
    /// Get parameter_groups resource handler
    pub fn parameter_groups(&self) -> resources::Parameter_groups<'_> {
        resources::Parameter_groups::new(self.provider)
    }
    /// Get default_parameters resource handler
    pub fn default_parameters(&self) -> resources::Default_parameters<'_> {
        resources::Default_parameters::new(self.provider)
    }
    /// Get clusters resource handler
    pub fn clusters(&self) -> resources::Clusters<'_> {
        resources::Clusters::new(self.provider)
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
