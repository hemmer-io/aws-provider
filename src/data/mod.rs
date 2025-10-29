//! Data Service
//!
//! Auto-generated service module for data

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for data
pub struct DataService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> DataService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get pipelines resource handler
    pub fn pipelines(&self) -> resources::Pipelines<'_> {
        resources::Pipelines::new(self.provider)
    }
    /// Get pipeline_definition resource handler
    pub fn pipeline_definition(&self) -> resources::Pipeline_definition<'_> {
        resources::Pipeline_definition::new(self.provider)
    }
    /// Get objects resource handler
    pub fn objects(&self) -> resources::Objects<'_> {
        resources::Objects::new(self.provider)
    }
    /// Get pipeline resource handler
    pub fn pipeline(&self) -> resources::Pipeline<'_> {
        resources::Pipeline::new(self.provider)
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
