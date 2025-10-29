//! Iotsecuretunneling Service
//!
//! Auto-generated service module for iotsecuretunneling

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iotsecuretunneling
pub struct IotsecuretunnelingService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> IotsecuretunnelingService<'a> {
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
