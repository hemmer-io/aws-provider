//! Workspaces_instances_2022_07_26 Service
//!
//! Auto-generated service module for workspaces_instances_2022_07_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workspaces_instances_2022_07_26
pub struct Workspaces_instances_2022_07_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Workspaces_instances_2022_07_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get volume resource handler
    pub fn volume(&self) -> resources::Volume<'_> {
        resources::Volume::new(self.provider)
    }
    /// Get workspace_instance resource handler
    pub fn workspace_instance(&self) -> resources::Workspace_instance<'_> {
        resources::Workspace_instance::new(self.provider)
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
