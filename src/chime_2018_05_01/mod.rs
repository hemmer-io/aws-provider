//! Chime_2018_05_01 Service
//!
//! Auto-generated service module for chime_2018_05_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chime_2018_05_01
pub struct Chime_2018_05_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Chime_2018_05_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get phone_number_settings resource handler
    pub fn phone_number_settings(&self) -> resources::Phone_number_settings<'_> {
        resources::Phone_number_settings::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get bot resource handler
    pub fn bot(&self) -> resources::Bot<'_> {
        resources::Bot::new(self.provider)
    }
    /// Get meeting_dial_out resource handler
    pub fn meeting_dial_out(&self) -> resources::Meeting_dial_out<'_> {
        resources::Meeting_dial_out::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get phone_number resource handler
    pub fn phone_number(&self) -> resources::Phone_number<'_> {
        resources::Phone_number::new(self.provider)
    }
    /// Get account_settings resource handler
    pub fn account_settings(&self) -> resources::Account_settings<'_> {
        resources::Account_settings::new(self.provider)
    }
    /// Get room_membership resource handler
    pub fn room_membership(&self) -> resources::Room_membership<'_> {
        resources::Room_membership::new(self.provider)
    }
    /// Get retention_settings resource handler
    pub fn retention_settings(&self) -> resources::Retention_settings<'_> {
        resources::Retention_settings::new(self.provider)
    }
    /// Get room resource handler
    pub fn room(&self) -> resources::Room<'_> {
        resources::Room::new(self.provider)
    }
    /// Get phone_number_order resource handler
    pub fn phone_number_order(&self) -> resources::Phone_number_order<'_> {
        resources::Phone_number_order::new(self.provider)
    }
    /// Get events_configuration resource handler
    pub fn events_configuration(&self) -> resources::Events_configuration<'_> {
        resources::Events_configuration::new(self.provider)
    }
    /// Get global_settings resource handler
    pub fn global_settings(&self) -> resources::Global_settings<'_> {
        resources::Global_settings::new(self.provider)
    }
    /// Get user_settings resource handler
    pub fn user_settings(&self) -> resources::User_settings<'_> {
        resources::User_settings::new(self.provider)
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
