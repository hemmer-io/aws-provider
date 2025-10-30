//! Detective_2018_10_26 Service
//!
//! Auto-generated service module for detective_2018_10_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for detective_2018_10_26
pub struct Detective_2018_10_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Detective_2018_10_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get members resource handler
    pub fn members(&self) -> resources::Members<'_> {
        resources::Members::new(self.provider)
    }
    /// Get organization_configuration resource handler
    pub fn organization_configuration(&self) -> resources::Organization_configuration<'_> {
        resources::Organization_configuration::new(self.provider)
    }
    /// Get investigation_state resource handler
    pub fn investigation_state(&self) -> resources::Investigation_state<'_> {
        resources::Investigation_state::new(self.provider)
    }
    /// Get graph resource handler
    pub fn graph(&self) -> resources::Graph<'_> {
        resources::Graph::new(self.provider)
    }
    /// Get investigation resource handler
    pub fn investigation(&self) -> resources::Investigation<'_> {
        resources::Investigation::new(self.provider)
    }
    /// Get datasource_packages resource handler
    pub fn datasource_packages(&self) -> resources::Datasource_packages<'_> {
        resources::Datasource_packages::new(self.provider)
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
