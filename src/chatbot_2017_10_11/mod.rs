//! Chatbot_2017_10_11 Service
//!
//! Auto-generated service module for chatbot_2017_10_11

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chatbot_2017_10_11
pub struct Chatbot_2017_10_11Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Chatbot_2017_10_11Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get chime_webhook_configurations resource handler
    pub fn chime_webhook_configurations(&self) -> resources::Chime_webhook_configurations<'_> {
        resources::Chime_webhook_configurations::new(self.provider)
    }
    /// Get microsoft_teams_configured_team resource handler
    pub fn microsoft_teams_configured_team(&self) -> resources::Microsoft_teams_configured_team<'_> {
        resources::Microsoft_teams_configured_team::new(self.provider)
    }
    /// Get slack_workspace_authorization resource handler
    pub fn slack_workspace_authorization(&self) -> resources::Slack_workspace_authorization<'_> {
        resources::Slack_workspace_authorization::new(self.provider)
    }
    /// Get slack_user_identity resource handler
    pub fn slack_user_identity(&self) -> resources::Slack_user_identity<'_> {
        resources::Slack_user_identity::new(self.provider)
    }
    /// Get slack_channel_configuration resource handler
    pub fn slack_channel_configuration(&self) -> resources::Slack_channel_configuration<'_> {
        resources::Slack_channel_configuration::new(self.provider)
    }
    /// Get slack_user_identities resource handler
    pub fn slack_user_identities(&self) -> resources::Slack_user_identities<'_> {
        resources::Slack_user_identities::new(self.provider)
    }
    /// Get slack_workspaces resource handler
    pub fn slack_workspaces(&self) -> resources::Slack_workspaces<'_> {
        resources::Slack_workspaces::new(self.provider)
    }
    /// Get account_preferences resource handler
    pub fn account_preferences(&self) -> resources::Account_preferences<'_> {
        resources::Account_preferences::new(self.provider)
    }
    /// Get microsoft_teams_user_identity resource handler
    pub fn microsoft_teams_user_identity(&self) -> resources::Microsoft_teams_user_identity<'_> {
        resources::Microsoft_teams_user_identity::new(self.provider)
    }
    /// Get slack_channel_configurations resource handler
    pub fn slack_channel_configurations(&self) -> resources::Slack_channel_configurations<'_> {
        resources::Slack_channel_configurations::new(self.provider)
    }
    /// Get microsoft_teams_channel_configuration resource handler
    pub fn microsoft_teams_channel_configuration(&self) -> resources::Microsoft_teams_channel_configuration<'_> {
        resources::Microsoft_teams_channel_configuration::new(self.provider)
    }
    /// Get chime_webhook_configuration resource handler
    pub fn chime_webhook_configuration(&self) -> resources::Chime_webhook_configuration<'_> {
        resources::Chime_webhook_configuration::new(self.provider)
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
