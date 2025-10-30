//! Iot_jobs_data_plane_2017_09_29 Service
//!
//! Auto-generated service module for iot_jobs_data_plane_2017_09_29

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iot_jobs_data_plane_2017_09_29
pub struct Iot_jobs_data_plane_2017_09_29Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Iot_jobs_data_plane_2017_09_29Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get pending_job_executions resource handler
    pub fn pending_job_executions(&self) -> resources::Pending_job_executions<'_> {
        resources::Pending_job_executions::new(self.provider)
    }
    /// Get job_execution resource handler
    pub fn job_execution(&self) -> resources::Job_execution<'_> {
        resources::Job_execution::new(self.provider)
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
