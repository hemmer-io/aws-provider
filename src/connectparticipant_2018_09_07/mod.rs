//! Connectparticipant_2018_09_07 Service
//!
//! Auto-generated service module for connectparticipant_2018_09_07

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for connectparticipant_2018_09_07
pub struct Connectparticipant_2018_09_07Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Connectparticipant_2018_09_07Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get view resource handler
    pub fn view(&self) -> resources::View<'_> {
        resources::View::new(self.provider)
    }
    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
    }
    /// Get participant_connection resource handler
    pub fn participant_connection(&self) -> resources::Participant_connection<'_> {
        resources::Participant_connection::new(self.provider)
    }
    /// Get authentication_url resource handler
    pub fn authentication_url(&self) -> resources::Authentication_url<'_> {
        resources::Authentication_url::new(self.provider)
    }
    /// Get transcript resource handler
    pub fn transcript(&self) -> resources::Transcript<'_> {
        resources::Transcript::new(self.provider)
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
