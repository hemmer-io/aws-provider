//! Kinesis_analytics_2015_08_14 Service
//!
//! Auto-generated service module for kinesis_analytics_2015_08_14

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kinesis_analytics_2015_08_14
pub struct Kinesis_analytics_2015_08_14Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Kinesis_analytics_2015_08_14Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get application_cloud_watch_logging_option resource handler
    pub fn application_cloud_watch_logging_option(&self) -> resources::Application_cloud_watch_logging_option<'_> {
        resources::Application_cloud_watch_logging_option::new(self.provider)
    }
    /// Get application_output resource handler
    pub fn application_output(&self) -> resources::Application_output<'_> {
        resources::Application_output::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get application_reference_data_source resource handler
    pub fn application_reference_data_source(&self) -> resources::Application_reference_data_source<'_> {
        resources::Application_reference_data_source::new(self.provider)
    }
    /// Get application_input_processing_configuration resource handler
    pub fn application_input_processing_configuration(&self) -> resources::Application_input_processing_configuration<'_> {
        resources::Application_input_processing_configuration::new(self.provider)
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
