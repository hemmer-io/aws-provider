//! Mediaconvert_2017_08_29 Service
//!
//! Auto-generated service module for mediaconvert_2017_08_29

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mediaconvert_2017_08_29
pub struct Mediaconvert_2017_08_29Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Mediaconvert_2017_08_29Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get endpoints resource handler
    pub fn endpoints(&self) -> resources::Endpoints<'_> {
        resources::Endpoints::new(self.provider)
    }
    /// Get queue resource handler
    pub fn queue(&self) -> resources::Queue<'_> {
        resources::Queue::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
    }
    /// Get resource_share resource handler
    pub fn resource_share(&self) -> resources::Resource_share<'_> {
        resources::Resource_share::new(self.provider)
    }
    /// Get jobs_query_results resource handler
    pub fn jobs_query_results(&self) -> resources::Jobs_query_results<'_> {
        resources::Jobs_query_results::new(self.provider)
    }
    /// Get preset resource handler
    pub fn preset(&self) -> resources::Preset<'_> {
        resources::Preset::new(self.provider)
    }
    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
    }
    /// Get job_template resource handler
    pub fn job_template(&self) -> resources::Job_template<'_> {
        resources::Job_template::new(self.provider)
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
