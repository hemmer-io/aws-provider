//! Batch_2016_08_10 Service
//!
//! Auto-generated service module for batch_2016_08_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for batch_2016_08_10
pub struct Batch_2016_08_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Batch_2016_08_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get service_environment resource handler
    pub fn service_environment(&self) -> resources::Service_environment<'_> {
        resources::Service_environment::new(self.provider)
    }
    /// Get service_job resource handler
    pub fn service_job(&self) -> resources::Service_job<'_> {
        resources::Service_job::new(self.provider)
    }
    /// Get job_queue_snapshot resource handler
    pub fn job_queue_snapshot(&self) -> resources::Job_queue_snapshot<'_> {
        resources::Job_queue_snapshot::new(self.provider)
    }
    /// Get jobs resource handler
    pub fn jobs(&self) -> resources::Jobs<'_> {
        resources::Jobs::new(self.provider)
    }
    /// Get scheduling_policies resource handler
    pub fn scheduling_policies(&self) -> resources::Scheduling_policies<'_> {
        resources::Scheduling_policies::new(self.provider)
    }
    /// Get consumable_resource resource handler
    pub fn consumable_resource(&self) -> resources::Consumable_resource<'_> {
        resources::Consumable_resource::new(self.provider)
    }
    /// Get compute_environment resource handler
    pub fn compute_environment(&self) -> resources::Compute_environment<'_> {
        resources::Compute_environment::new(self.provider)
    }
    /// Get job_definitions resource handler
    pub fn job_definitions(&self) -> resources::Job_definitions<'_> {
        resources::Job_definitions::new(self.provider)
    }
    /// Get job_queues resource handler
    pub fn job_queues(&self) -> resources::Job_queues<'_> {
        resources::Job_queues::new(self.provider)
    }
    /// Get job_queue resource handler
    pub fn job_queue(&self) -> resources::Job_queue<'_> {
        resources::Job_queue::new(self.provider)
    }
    /// Get compute_environments resource handler
    pub fn compute_environments(&self) -> resources::Compute_environments<'_> {
        resources::Compute_environments::new(self.provider)
    }
    /// Get service_environments resource handler
    pub fn service_environments(&self) -> resources::Service_environments<'_> {
        resources::Service_environments::new(self.provider)
    }
    /// Get scheduling_policy resource handler
    pub fn scheduling_policy(&self) -> resources::Scheduling_policy<'_> {
        resources::Scheduling_policy::new(self.provider)
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
