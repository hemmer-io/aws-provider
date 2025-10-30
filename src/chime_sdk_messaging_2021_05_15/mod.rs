//! Chime_sdk_messaging_2021_05_15 Service
//!
//! Auto-generated service module for chime_sdk_messaging_2021_05_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chime_sdk_messaging_2021_05_15
pub struct Chime_sdk_messaging_2021_05_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Chime_sdk_messaging_2021_05_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get channel_membership_for_app_instance_user resource handler
    pub fn channel_membership_for_app_instance_user(&self) -> resources::Channel_membership_for_app_instance_user<'_> {
        resources::Channel_membership_for_app_instance_user::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
    }
    /// Get channel_flow resource handler
    pub fn channel_flow(&self) -> resources::Channel_flow<'_> {
        resources::Channel_flow::new(self.provider)
    }
    /// Get channel_membership resource handler
    pub fn channel_membership(&self) -> resources::Channel_membership<'_> {
        resources::Channel_membership::new(self.provider)
    }
    /// Get channel_message resource handler
    pub fn channel_message(&self) -> resources::Channel_message<'_> {
        resources::Channel_message::new(self.provider)
    }
    /// Get channel_moderated_by_app_instance_user resource handler
    pub fn channel_moderated_by_app_instance_user(&self) -> resources::Channel_moderated_by_app_instance_user<'_> {
        resources::Channel_moderated_by_app_instance_user::new(self.provider)
    }
    /// Get channel_expiration_settings resource handler
    pub fn channel_expiration_settings(&self) -> resources::Channel_expiration_settings<'_> {
        resources::Channel_expiration_settings::new(self.provider)
    }
    /// Get messaging_streaming_configurations resource handler
    pub fn messaging_streaming_configurations(&self) -> resources::Messaging_streaming_configurations<'_> {
        resources::Messaging_streaming_configurations::new(self.provider)
    }
    /// Get channel_ban resource handler
    pub fn channel_ban(&self) -> resources::Channel_ban<'_> {
        resources::Channel_ban::new(self.provider)
    }
    /// Get channel_moderator resource handler
    pub fn channel_moderator(&self) -> resources::Channel_moderator<'_> {
        resources::Channel_moderator::new(self.provider)
    }
    /// Get channel_read_marker resource handler
    pub fn channel_read_marker(&self) -> resources::Channel_read_marker<'_> {
        resources::Channel_read_marker::new(self.provider)
    }
    /// Get channel_membership_preferences resource handler
    pub fn channel_membership_preferences(&self) -> resources::Channel_membership_preferences<'_> {
        resources::Channel_membership_preferences::new(self.provider)
    }
    /// Get messaging_session_endpoint resource handler
    pub fn messaging_session_endpoint(&self) -> resources::Messaging_session_endpoint<'_> {
        resources::Messaging_session_endpoint::new(self.provider)
    }
    /// Get channel_message_status resource handler
    pub fn channel_message_status(&self) -> resources::Channel_message_status<'_> {
        resources::Channel_message_status::new(self.provider)
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
