//! Connectcampaignsv2 Service
//!
//! Auto-generated service module for connectcampaignsv2

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for connectcampaignsv2
pub struct Connectcampaignsv2Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Connectcampaignsv2Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get instance_onboarding_job_status resource handler
    pub fn instance_onboarding_job_status(&self) -> resources::Instance_onboarding_job_status<'_> {
        resources::Instance_onboarding_job_status::new(self.provider)
    }
    /// Get connect_instance_integration resource handler
    pub fn connect_instance_integration(&self) -> resources::Connect_instance_integration<'_> {
        resources::Connect_instance_integration::new(self.provider)
    }
    /// Get campaign_schedule resource handler
    pub fn campaign_schedule(&self) -> resources::Campaign_schedule<'_> {
        resources::Campaign_schedule::new(self.provider)
    }
    /// Get connect_instance_config resource handler
    pub fn connect_instance_config(&self) -> resources::Connect_instance_config<'_> {
        resources::Connect_instance_config::new(self.provider)
    }
    /// Get campaign_channel_subtype_config resource handler
    pub fn campaign_channel_subtype_config(&self) -> resources::Campaign_channel_subtype_config<'_> {
        resources::Campaign_channel_subtype_config::new(self.provider)
    }
    /// Get campaign_source resource handler
    pub fn campaign_source(&self) -> resources::Campaign_source<'_> {
        resources::Campaign_source::new(self.provider)
    }
    /// Get outbound_request_batch resource handler
    pub fn outbound_request_batch(&self) -> resources::Outbound_request_batch<'_> {
        resources::Outbound_request_batch::new(self.provider)
    }
    /// Get campaign_communication_time resource handler
    pub fn campaign_communication_time(&self) -> resources::Campaign_communication_time<'_> {
        resources::Campaign_communication_time::new(self.provider)
    }
    /// Get instance_onboarding_job resource handler
    pub fn instance_onboarding_job(&self) -> resources::Instance_onboarding_job<'_> {
        resources::Instance_onboarding_job::new(self.provider)
    }
    /// Get instance_communication_limits resource handler
    pub fn instance_communication_limits(&self) -> resources::Instance_communication_limits<'_> {
        resources::Instance_communication_limits::new(self.provider)
    }
    /// Get campaign_communication_limits resource handler
    pub fn campaign_communication_limits(&self) -> resources::Campaign_communication_limits<'_> {
        resources::Campaign_communication_limits::new(self.provider)
    }
    /// Get campaign resource handler
    pub fn campaign(&self) -> resources::Campaign<'_> {
        resources::Campaign::new(self.provider)
    }
    /// Get campaign_state resource handler
    pub fn campaign_state(&self) -> resources::Campaign_state<'_> {
        resources::Campaign_state::new(self.provider)
    }
    /// Get campaign_state_batch resource handler
    pub fn campaign_state_batch(&self) -> resources::Campaign_state_batch<'_> {
        resources::Campaign_state_batch::new(self.provider)
    }
    /// Get profile_outbound_request_batch resource handler
    pub fn profile_outbound_request_batch(&self) -> resources::Profile_outbound_request_batch<'_> {
        resources::Profile_outbound_request_batch::new(self.provider)
    }
    /// Get campaign_flow_association resource handler
    pub fn campaign_flow_association(&self) -> resources::Campaign_flow_association<'_> {
        resources::Campaign_flow_association::new(self.provider)
    }
    /// Get campaign_name resource handler
    pub fn campaign_name(&self) -> resources::Campaign_name<'_> {
        resources::Campaign_name::new(self.provider)
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
