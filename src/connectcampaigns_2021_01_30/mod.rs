//! Connectcampaigns_2021_01_30 Service
//!
//! Auto-generated service module for connectcampaigns_2021_01_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for connectcampaigns_2021_01_30
pub struct Connectcampaigns_2021_01_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Connectcampaigns_2021_01_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get campaign_state resource handler
    pub fn campaign_state(&self) -> resources::Campaign_state<'_> {
        resources::Campaign_state::new(self.provider)
    }
    /// Get campaign_name resource handler
    pub fn campaign_name(&self) -> resources::Campaign_name<'_> {
        resources::Campaign_name::new(self.provider)
    }
    /// Get connect_instance_config resource handler
    pub fn connect_instance_config(&self) -> resources::Connect_instance_config<'_> {
        resources::Connect_instance_config::new(self.provider)
    }
    /// Get campaign_state_batch resource handler
    pub fn campaign_state_batch(&self) -> resources::Campaign_state_batch<'_> {
        resources::Campaign_state_batch::new(self.provider)
    }
    /// Get instance_onboarding_job_status resource handler
    pub fn instance_onboarding_job_status(&self) -> resources::Instance_onboarding_job_status<'_> {
        resources::Instance_onboarding_job_status::new(self.provider)
    }
    /// Get campaign_outbound_call_config resource handler
    pub fn campaign_outbound_call_config(&self) -> resources::Campaign_outbound_call_config<'_> {
        resources::Campaign_outbound_call_config::new(self.provider)
    }
    /// Get campaign resource handler
    pub fn campaign(&self) -> resources::Campaign<'_> {
        resources::Campaign::new(self.provider)
    }
    /// Get dial_request_batch resource handler
    pub fn dial_request_batch(&self) -> resources::Dial_request_batch<'_> {
        resources::Dial_request_batch::new(self.provider)
    }
    /// Get instance_onboarding_job resource handler
    pub fn instance_onboarding_job(&self) -> resources::Instance_onboarding_job<'_> {
        resources::Instance_onboarding_job::new(self.provider)
    }
    /// Get campaign_dialer_config resource handler
    pub fn campaign_dialer_config(&self) -> resources::Campaign_dialer_config<'_> {
        resources::Campaign_dialer_config::new(self.provider)
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
