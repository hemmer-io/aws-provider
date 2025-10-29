//! Rolesanywhere Service
//!
//! Auto-generated service module for rolesanywhere

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rolesanywhere
pub struct RolesanywhereService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> RolesanywhereService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get notification_settings resource handler
    pub fn notification_settings(&self) -> resources::Notification_settings<'_> {
        resources::Notification_settings::new(self.provider)
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
