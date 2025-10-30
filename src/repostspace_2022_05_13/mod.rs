//! Repostspace_2022_05_13 Service
//!
//! Auto-generated service module for repostspace_2022_05_13

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for repostspace_2022_05_13
pub struct Repostspace_2022_05_13Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Repostspace_2022_05_13Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get space resource handler
    pub fn space(&self) -> resources::Space<'_> {
        resources::Space::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
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
