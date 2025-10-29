//! Swf Service
//!
//! Auto-generated service module for swf

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for swf
pub struct SwfService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> SwfService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get workflow_type resource handler
    pub fn workflow_type(&self) -> resources::Workflow_type<'_> {
        resources::Workflow_type::new(self.provider)
    }
    /// Get activity_type resource handler
    pub fn activity_type(&self) -> resources::Activity_type<'_> {
        resources::Activity_type::new(self.provider)
    }
    /// Get workflow_execution resource handler
    pub fn workflow_execution(&self) -> resources::Workflow_execution<'_> {
        resources::Workflow_execution::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
    }
    /// Get workflow_execution_history resource handler
    pub fn workflow_execution_history(&self) -> resources::Workflow_execution_history<'_> {
        resources::Workflow_execution_history::new(self.provider)
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
