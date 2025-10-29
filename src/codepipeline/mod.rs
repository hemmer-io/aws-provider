//! Codepipeline Service
//!
//! Auto-generated service module for codepipeline

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codepipeline
pub struct CodepipelineService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CodepipelineService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get approval_result resource handler
    pub fn approval_result(&self) -> resources::Approval_result<'_> {
        resources::Approval_result::new(self.provider)
    }
    /// Get pipeline resource handler
    pub fn pipeline(&self) -> resources::Pipeline<'_> {
        resources::Pipeline::new(self.provider)
    }
    /// Get pipeline_state resource handler
    pub fn pipeline_state(&self) -> resources::Pipeline_state<'_> {
        resources::Pipeline_state::new(self.provider)
    }
    /// Get job_details resource handler
    pub fn job_details(&self) -> resources::Job_details<'_> {
        resources::Job_details::new(self.provider)
    }
    /// Get job_success_result resource handler
    pub fn job_success_result(&self) -> resources::Job_success_result<'_> {
        resources::Job_success_result::new(self.provider)
    }
    /// Get webhook resource handler
    pub fn webhook(&self) -> resources::Webhook<'_> {
        resources::Webhook::new(self.provider)
    }
    /// Get custom_action_type resource handler
    pub fn custom_action_type(&self) -> resources::Custom_action_type<'_> {
        resources::Custom_action_type::new(self.provider)
    }
    /// Get action_revision resource handler
    pub fn action_revision(&self) -> resources::Action_revision<'_> {
        resources::Action_revision::new(self.provider)
    }
    /// Get action_type resource handler
    pub fn action_type(&self) -> resources::Action_type<'_> {
        resources::Action_type::new(self.provider)
    }
    /// Get pipeline_execution resource handler
    pub fn pipeline_execution(&self) -> resources::Pipeline_execution<'_> {
        resources::Pipeline_execution::new(self.provider)
    }
    /// Get third_party_job_details resource handler
    pub fn third_party_job_details(&self) -> resources::Third_party_job_details<'_> {
        resources::Third_party_job_details::new(self.provider)
    }
    /// Get third_party_job_success_result resource handler
    pub fn third_party_job_success_result(&self) -> resources::Third_party_job_success_result<'_> {
        resources::Third_party_job_success_result::new(self.provider)
    }
    /// Get job_failure_result resource handler
    pub fn job_failure_result(&self) -> resources::Job_failure_result<'_> {
        resources::Job_failure_result::new(self.provider)
    }
    /// Get third_party_job_failure_result resource handler
    pub fn third_party_job_failure_result(&self) -> resources::Third_party_job_failure_result<'_> {
        resources::Third_party_job_failure_result::new(self.provider)
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
