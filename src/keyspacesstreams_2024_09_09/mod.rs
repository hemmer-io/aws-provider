//! Keyspacesstreams_2024_09_09 Service
//!
//! Auto-generated service module for keyspacesstreams_2024_09_09

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for keyspacesstreams_2024_09_09
pub struct Keyspacesstreams_2024_09_09Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Keyspacesstreams_2024_09_09Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get stream resource handler
    pub fn stream(&self) -> resources::Stream<'_> {
        resources::Stream::new(self.provider)
    }
    /// Get shard_iterator resource handler
    pub fn shard_iterator(&self) -> resources::Shard_iterator<'_> {
        resources::Shard_iterator::new(self.provider)
    }
    /// Get records resource handler
    pub fn records(&self) -> resources::Records<'_> {
        resources::Records::new(self.provider)
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
