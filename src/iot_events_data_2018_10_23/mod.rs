//! Iot_events_data_2018_10_23 Service
//!
//! Auto-generated service module for iot_events_data_2018_10_23

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iot_events_data_2018_10_23
pub struct Iot_events_data_2018_10_23Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iot_events_data_2018_10_23Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get detector resource handler
    pub fn detector(&self) -> resources::Detector<'_> {
        resources::Detector::new(self.provider)
    }
    /// Get alarm resource handler
    pub fn alarm(&self) -> resources::Alarm<'_> {
        resources::Alarm::new(self.provider)
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
