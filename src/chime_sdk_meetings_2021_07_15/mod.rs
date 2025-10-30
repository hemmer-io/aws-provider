//! Chime_sdk_meetings_2021_07_15 Service
//!
//! Auto-generated service module for chime_sdk_meetings_2021_07_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chime_sdk_meetings_2021_07_15
pub struct Chime_sdk_meetings_2021_07_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Chime_sdk_meetings_2021_07_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get meeting resource handler
    pub fn meeting(&self) -> resources::Meeting<'_> {
        resources::Meeting::new(self.provider)
    }
    /// Get attendee_capabilities resource handler
    pub fn attendee_capabilities(&self) -> resources::Attendee_capabilities<'_> {
        resources::Attendee_capabilities::new(self.provider)
    }
    /// Get meeting_with_attendees resource handler
    pub fn meeting_with_attendees(&self) -> resources::Meeting_with_attendees<'_> {
        resources::Meeting_with_attendees::new(self.provider)
    }
    /// Get attendee resource handler
    pub fn attendee(&self) -> resources::Attendee<'_> {
        resources::Attendee::new(self.provider)
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
