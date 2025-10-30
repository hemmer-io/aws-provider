//! Kinesis_analytics Service
//!
//! Auto-generated service module for kinesis_analytics

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kinesis_analytics
pub struct Kinesis_analyticsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Kinesis_analyticsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get application_snapshot resource handler
    pub fn application_snapshot(&self) -> resources::Application_snapshot<'_> {
        resources::Application_snapshot::new(self.provider)
    }
    /// Get application_presigned_url resource handler
    pub fn application_presigned_url(&self) -> resources::Application_presigned_url<'_> {
        resources::Application_presigned_url::new(self.provider)
    }
    /// Get application_output resource handler
    pub fn application_output(&self) -> resources::Application_output<'_> {
        resources::Application_output::new(self.provider)
    }
    /// Get application_operation resource handler
    pub fn application_operation(&self) -> resources::Application_operation<'_> {
        resources::Application_operation::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get application_version resource handler
    pub fn application_version(&self) -> resources::Application_version<'_> {
        resources::Application_version::new(self.provider)
    }
    /// Get application_input_processing_configuration resource handler
    pub fn application_input_processing_configuration(&self) -> resources::Application_input_processing_configuration<'_> {
        resources::Application_input_processing_configuration::new(self.provider)
    }
    /// Get application_cloud_watch_logging_option resource handler
    pub fn application_cloud_watch_logging_option(&self) -> resources::Application_cloud_watch_logging_option<'_> {
        resources::Application_cloud_watch_logging_option::new(self.provider)
    }
    /// Get application_reference_data_source resource handler
    pub fn application_reference_data_source(&self) -> resources::Application_reference_data_source<'_> {
        resources::Application_reference_data_source::new(self.provider)
    }
    /// Get application_maintenance_configuration resource handler
    pub fn application_maintenance_configuration(&self) -> resources::Application_maintenance_configuration<'_> {
        resources::Application_maintenance_configuration::new(self.provider)
    }
    /// Get application_vpc_configuration resource handler
    pub fn application_vpc_configuration(&self) -> resources::Application_vpc_configuration<'_> {
        resources::Application_vpc_configuration::new(self.provider)
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
