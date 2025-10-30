//! Ivschat_2020_07_14 Service
//!
//! Auto-generated service module for ivschat_2020_07_14

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ivschat_2020_07_14
pub struct Ivschat_2020_07_14Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ivschat_2020_07_14Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get logging_configuration resource handler
    pub fn logging_configuration(&self) -> resources::Logging_configuration<'_> {
        resources::Logging_configuration::new(self.provider)
    }
    /// Get room resource handler
    pub fn room(&self) -> resources::Room<'_> {
        resources::Room::new(self.provider)
    }
    /// Get message resource handler
    pub fn message(&self) -> resources::Message<'_> {
        resources::Message::new(self.provider)
    }
    /// Get chat_token resource handler
    pub fn chat_token(&self) -> resources::Chat_token<'_> {
        resources::Chat_token::new(self.provider)
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
