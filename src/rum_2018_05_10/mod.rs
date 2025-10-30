//! Rum_2018_05_10 Service
//!
//! Auto-generated service module for rum_2018_05_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rum_2018_05_10
pub struct Rum_2018_05_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Rum_2018_05_10Service<'a> {
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
