//! Iot_data_plane_2015_05_28 Service
//!
//! Auto-generated service module for iot_data_plane_2015_05_28

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iot_data_plane_2015_05_28
pub struct Iot_data_plane_2015_05_28Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iot_data_plane_2015_05_28Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get thing_shadow resource handler
    pub fn thing_shadow(&self) -> resources::Thing_shadow<'_> {
        resources::Thing_shadow::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get retained_message resource handler
    pub fn retained_message(&self) -> resources::Retained_message<'_> {
        resources::Retained_message::new(self.provider)
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
