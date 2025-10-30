//! Iotsecuretunneling_2018_10_05 Service
//!
//! Auto-generated service module for iotsecuretunneling_2018_10_05

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iotsecuretunneling_2018_10_05
pub struct Iotsecuretunneling_2018_10_05Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iotsecuretunneling_2018_10_05Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get tunnel resource handler
    pub fn tunnel(&self) -> resources::Tunnel<'_> {
        resources::Tunnel::new(self.provider)
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
