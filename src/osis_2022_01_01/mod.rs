//! Osis_2022_01_01 Service
//!
//! Auto-generated service module for osis_2022_01_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for osis_2022_01_01
pub struct Osis_2022_01_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Osis_2022_01_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get pipeline_blueprint resource handler
    pub fn pipeline_blueprint(&self) -> resources::Pipeline_blueprint<'_> {
        resources::Pipeline_blueprint::new(self.provider)
    }
    /// Get pipeline_change_progress resource handler
    pub fn pipeline_change_progress(&self) -> resources::Pipeline_change_progress<'_> {
        resources::Pipeline_change_progress::new(self.provider)
    }
    /// Get pipeline_endpoint resource handler
    pub fn pipeline_endpoint(&self) -> resources::Pipeline_endpoint<'_> {
        resources::Pipeline_endpoint::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
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
