//! Data_pipeline_2012_10_29 Service
//!
//! Auto-generated service module for data_pipeline_2012_10_29

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for data_pipeline_2012_10_29
pub struct Data_pipeline_2012_10_29Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Data_pipeline_2012_10_29Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get pipeline resource handler
    pub fn pipeline(&self) -> resources::Pipeline<'_> {
        resources::Pipeline::new(self.provider)
    }
    /// Get pipeline_definition resource handler
    pub fn pipeline_definition(&self) -> resources::Pipeline_definition<'_> {
        resources::Pipeline_definition::new(self.provider)
    }
    /// Get objects resource handler
    pub fn objects(&self) -> resources::Objects<'_> {
        resources::Objects::new(self.provider)
    }
    /// Get pipelines resource handler
    pub fn pipelines(&self) -> resources::Pipelines<'_> {
        resources::Pipelines::new(self.provider)
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
