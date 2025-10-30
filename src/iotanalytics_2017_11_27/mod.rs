//! Iotanalytics_2017_11_27 Service
//!
//! Auto-generated service module for iotanalytics_2017_11_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iotanalytics_2017_11_27
pub struct Iotanalytics_2017_11_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iotanalytics_2017_11_27Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get pipeline resource handler
    pub fn pipeline(&self) -> resources::Pipeline<'_> {
        resources::Pipeline::new(self.provider)
    }
    /// Get datastore resource handler
    pub fn datastore(&self) -> resources::Datastore<'_> {
        resources::Datastore::new(self.provider)
    }
    /// Get logging_options resource handler
    pub fn logging_options(&self) -> resources::Logging_options<'_> {
        resources::Logging_options::new(self.provider)
    }
    /// Get dataset_content resource handler
    pub fn dataset_content(&self) -> resources::Dataset_content<'_> {
        resources::Dataset_content::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
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
