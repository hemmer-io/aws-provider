//! Support_app_2021_08_20 Service
//!
//! Auto-generated service module for support_app_2021_08_20

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for support_app_2021_08_20
pub struct Support_app_2021_08_20Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Support_app_2021_08_20Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get slack_channel_configuration resource handler
    pub fn slack_channel_configuration(&self) -> resources::Slack_channel_configuration<'_> {
        resources::Slack_channel_configuration::new(self.provider)
    }
    /// Get account_alias resource handler
    pub fn account_alias(&self) -> resources::Account_alias<'_> {
        resources::Account_alias::new(self.provider)
    }
    /// Get slack_workspace_configuration resource handler
    pub fn slack_workspace_configuration(&self) -> resources::Slack_workspace_configuration<'_> {
        resources::Slack_workspace_configuration::new(self.provider)
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
