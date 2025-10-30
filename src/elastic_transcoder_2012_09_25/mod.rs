//! Elastic_transcoder_2012_09_25 Service
//!
//! Auto-generated service module for elastic_transcoder_2012_09_25

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for elastic_transcoder_2012_09_25
pub struct Elastic_transcoder_2012_09_25Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Elastic_transcoder_2012_09_25Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get preset resource handler
    pub fn preset(&self) -> resources::Preset<'_> {
        resources::Preset::new(self.provider)
    }
    /// Get pipeline resource handler
    pub fn pipeline(&self) -> resources::Pipeline<'_> {
        resources::Pipeline::new(self.provider)
    }
    /// Get pipeline_notifications resource handler
    pub fn pipeline_notifications(&self) -> resources::Pipeline_notifications<'_> {
        resources::Pipeline_notifications::new(self.provider)
    }
    /// Get pipeline_status resource handler
    pub fn pipeline_status(&self) -> resources::Pipeline_status<'_> {
        resources::Pipeline_status::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
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
