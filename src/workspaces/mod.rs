//! Workspaces Service
//!
//! Auto-generated service module for workspaces

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workspaces
pub struct WorkspacesService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> WorkspacesService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get session resource handler
    pub fn session(&self) -> resources::Session<'_> {
        resources::Session::new(self.provider)
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
