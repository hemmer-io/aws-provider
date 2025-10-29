//! Connectparticipant Service
//!
//! Auto-generated service module for connectparticipant

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for connectparticipant
pub struct ConnectparticipantService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ConnectparticipantService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get participant_connection resource handler
    pub fn participant_connection(&self) -> resources::Participant_connection<'_> {
        resources::Participant_connection::new(self.provider)
    }
    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
    }
    /// Get authentication_url resource handler
    pub fn authentication_url(&self) -> resources::Authentication_url<'_> {
        resources::Authentication_url::new(self.provider)
    }
    /// Get transcript resource handler
    pub fn transcript(&self) -> resources::Transcript<'_> {
        resources::Transcript::new(self.provider)
    }
    /// Get view resource handler
    pub fn view(&self) -> resources::View<'_> {
        resources::View::new(self.provider)
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
