//! Codestar_notifications_2019_10_15 Service
//!
//! Auto-generated service module for codestar_notifications_2019_10_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codestar_notifications_2019_10_15
pub struct Codestar_notifications_2019_10_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Codestar_notifications_2019_10_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get notification_rule resource handler
    pub fn notification_rule(&self) -> resources::Notification_rule<'_> {
        resources::Notification_rule::new(self.provider)
    }
    /// Get target resource handler
    pub fn target(&self) -> resources::Target<'_> {
        resources::Target::new(self.provider)
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
