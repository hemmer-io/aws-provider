//! Schemas Service
//!
//! Auto-generated service module for schemas

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for schemas
pub struct SchemasService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> SchemasService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get schema_version resource handler
    pub fn schema_version(&self) -> resources::Schema_version<'_> {
        resources::Schema_version::new(self.provider)
    }
    /// Get code_binding resource handler
    pub fn code_binding(&self) -> resources::Code_binding<'_> {
        resources::Code_binding::new(self.provider)
    }
    /// Get schema resource handler
    pub fn schema(&self) -> resources::Schema<'_> {
        resources::Schema::new(self.provider)
    }
    /// Get discovered_schema resource handler
    pub fn discovered_schema(&self) -> resources::Discovered_schema<'_> {
        resources::Discovered_schema::new(self.provider)
    }
    /// Get discoverer resource handler
    pub fn discoverer(&self) -> resources::Discoverer<'_> {
        resources::Discoverer::new(self.provider)
    }
    /// Get registry resource handler
    pub fn registry(&self) -> resources::Registry<'_> {
        resources::Registry::new(self.provider)
    }
    /// Get code_binding_source resource handler
    pub fn code_binding_source(&self) -> resources::Code_binding_source<'_> {
        resources::Code_binding_source::new(self.provider)
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
