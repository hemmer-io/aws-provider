//! Rum Service
//!
//! Auto-generated service module for rum

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rum
pub struct RumService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> RumService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get rum_events resource handler
    pub fn rum_events(&self) -> resources::Rum_events<'_> {
        resources::Rum_events::new(self.provider)
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
