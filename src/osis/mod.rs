//! Osis Service
//!
//! Auto-generated service module for osis

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for osis
pub struct OsisService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> OsisService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get pipeline resource handler
    pub fn pipeline(&self) -> resources::Pipeline<'_> {
        resources::Pipeline::new(self.provider)
    }
    /// Get pipeline_change_progress resource handler
    pub fn pipeline_change_progress(&self) -> resources::Pipeline_change_progress<'_> {
        resources::Pipeline_change_progress::new(self.provider)
    }
    /// Get pipeline_blueprint resource handler
    pub fn pipeline_blueprint(&self) -> resources::Pipeline_blueprint<'_> {
        resources::Pipeline_blueprint::new(self.provider)
    }
    /// Get pipeline_endpoint resource handler
    pub fn pipeline_endpoint(&self) -> resources::Pipeline_endpoint<'_> {
        resources::Pipeline_endpoint::new(self.provider)
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
