//! Repostspace Service
//!
//! Auto-generated service module for repostspace

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for repostspace
pub struct RepostspaceService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> RepostspaceService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
    }
    /// Get space resource handler
    pub fn space(&self) -> resources::Space<'_> {
        resources::Space::new(self.provider)
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
