//! Migrationhub Service
//!
//! Auto-generated service module for migrationhub

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for migrationhub
pub struct MigrationhubService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MigrationhubService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get home_region_control resource handler
    pub fn home_region_control(&self) -> resources::Home_region_control<'_> {
        resources::Home_region_control::new(self.provider)
    }
    /// Get home_region_controls resource handler
    pub fn home_region_controls(&self) -> resources::Home_region_controls<'_> {
        resources::Home_region_controls::new(self.provider)
    }
    /// Get home_region resource handler
    pub fn home_region(&self) -> resources::Home_region<'_> {
        resources::Home_region::new(self.provider)
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
