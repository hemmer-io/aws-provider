//! Workspaces_web_2020_07_08 Service
//!
//! Auto-generated service module for workspaces_web_2020_07_08

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for workspaces_web_2020_07_08
pub struct Workspaces_web_2020_07_08Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Workspaces_web_2020_07_08Service<'a> {
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
