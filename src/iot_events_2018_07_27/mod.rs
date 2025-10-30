//! Iot_events_2018_07_27 Service
//!
//! Auto-generated service module for iot_events_2018_07_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iot_events_2018_07_27
pub struct Iot_events_2018_07_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iot_events_2018_07_27Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get detector_model_analysis_results resource handler
    pub fn detector_model_analysis_results(&self) -> resources::Detector_model_analysis_results<'_> {
        resources::Detector_model_analysis_results::new(self.provider)
    }
    /// Get logging_options resource handler
    pub fn logging_options(&self) -> resources::Logging_options<'_> {
        resources::Logging_options::new(self.provider)
    }
    /// Get input resource handler
    pub fn input(&self) -> resources::Input<'_> {
        resources::Input::new(self.provider)
    }
    /// Get detector_model_analysis resource handler
    pub fn detector_model_analysis(&self) -> resources::Detector_model_analysis<'_> {
        resources::Detector_model_analysis::new(self.provider)
    }
    /// Get detector_model resource handler
    pub fn detector_model(&self) -> resources::Detector_model<'_> {
        resources::Detector_model::new(self.provider)
    }
    /// Get alarm_model resource handler
    pub fn alarm_model(&self) -> resources::Alarm_model<'_> {
        resources::Alarm_model::new(self.provider)
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
