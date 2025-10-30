//! Chime_sdk Service
//!
//! Auto-generated service module for chime_sdk

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chime_sdk
pub struct Chime_sdkService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Chime_sdkService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get voice_connector_streaming_configuration resource handler
    pub fn voice_connector_streaming_configuration(&self) -> resources::Voice_connector_streaming_configuration<'_> {
        resources::Voice_connector_streaming_configuration::new(self.provider)
    }
    /// Get voice_tone_analysis_task resource handler
    pub fn voice_tone_analysis_task(&self) -> resources::Voice_tone_analysis_task<'_> {
        resources::Voice_tone_analysis_task::new(self.provider)
    }
    /// Get voice_connector resource handler
    pub fn voice_connector(&self) -> resources::Voice_connector<'_> {
        resources::Voice_connector::new(self.provider)
    }
    /// Get sip_media_application_alexa_skill_configuration resource handler
    pub fn sip_media_application_alexa_skill_configuration(&self) -> resources::Sip_media_application_alexa_skill_configuration<'_> {
        resources::Sip_media_application_alexa_skill_configuration::new(self.provider)
    }
    /// Get voice_connector_emergency_calling_configuration resource handler
    pub fn voice_connector_emergency_calling_configuration(&self) -> resources::Voice_connector_emergency_calling_configuration<'_> {
        resources::Voice_connector_emergency_calling_configuration::new(self.provider)
    }
    /// Get speaker_search_task resource handler
    pub fn speaker_search_task(&self) -> resources::Speaker_search_task<'_> {
        resources::Speaker_search_task::new(self.provider)
    }
    /// Get sip_media_application_call resource handler
    pub fn sip_media_application_call(&self) -> resources::Sip_media_application_call<'_> {
        resources::Sip_media_application_call::new(self.provider)
    }
    /// Get voice_profile_domain resource handler
    pub fn voice_profile_domain(&self) -> resources::Voice_profile_domain<'_> {
        resources::Voice_profile_domain::new(self.provider)
    }
    /// Get sip_rule resource handler
    pub fn sip_rule(&self) -> resources::Sip_rule<'_> {
        resources::Sip_rule::new(self.provider)
    }
    /// Get voice_connector_termination_credentials resource handler
    pub fn voice_connector_termination_credentials(&self) -> resources::Voice_connector_termination_credentials<'_> {
        resources::Voice_connector_termination_credentials::new(self.provider)
    }
    /// Get voice_connector_termination_health resource handler
    pub fn voice_connector_termination_health(&self) -> resources::Voice_connector_termination_health<'_> {
        resources::Voice_connector_termination_health::new(self.provider)
    }
    /// Get phone_number resource handler
    pub fn phone_number(&self) -> resources::Phone_number<'_> {
        resources::Phone_number::new(self.provider)
    }
    /// Get voice_profile resource handler
    pub fn voice_profile(&self) -> resources::Voice_profile<'_> {
        resources::Voice_profile::new(self.provider)
    }
    /// Get voice_connector_termination resource handler
    pub fn voice_connector_termination(&self) -> resources::Voice_connector_termination<'_> {
        resources::Voice_connector_termination::new(self.provider)
    }
    /// Get global_settings resource handler
    pub fn global_settings(&self) -> resources::Global_settings<'_> {
        resources::Global_settings::new(self.provider)
    }
    /// Get sip_media_application_logging_configuration resource handler
    pub fn sip_media_application_logging_configuration(&self) -> resources::Sip_media_application_logging_configuration<'_> {
        resources::Sip_media_application_logging_configuration::new(self.provider)
    }
    /// Get voice_connector_origination resource handler
    pub fn voice_connector_origination(&self) -> resources::Voice_connector_origination<'_> {
        resources::Voice_connector_origination::new(self.provider)
    }
    /// Get voice_connector_logging_configuration resource handler
    pub fn voice_connector_logging_configuration(&self) -> resources::Voice_connector_logging_configuration<'_> {
        resources::Voice_connector_logging_configuration::new(self.provider)
    }
    /// Get voice_connector_external_systems_configuration resource handler
    pub fn voice_connector_external_systems_configuration(&self) -> resources::Voice_connector_external_systems_configuration<'_> {
        resources::Voice_connector_external_systems_configuration::new(self.provider)
    }
    /// Get phone_number_order resource handler
    pub fn phone_number_order(&self) -> resources::Phone_number_order<'_> {
        resources::Phone_number_order::new(self.provider)
    }
    /// Get proxy_session resource handler
    pub fn proxy_session(&self) -> resources::Proxy_session<'_> {
        resources::Proxy_session::new(self.provider)
    }
    /// Get phone_number_settings resource handler
    pub fn phone_number_settings(&self) -> resources::Phone_number_settings<'_> {
        resources::Phone_number_settings::new(self.provider)
    }
    /// Get voice_connector_proxy resource handler
    pub fn voice_connector_proxy(&self) -> resources::Voice_connector_proxy<'_> {
        resources::Voice_connector_proxy::new(self.provider)
    }
    /// Get sip_media_application resource handler
    pub fn sip_media_application(&self) -> resources::Sip_media_application<'_> {
        resources::Sip_media_application::new(self.provider)
    }
    /// Get voice_connector_group resource handler
    pub fn voice_connector_group(&self) -> resources::Voice_connector_group<'_> {
        resources::Voice_connector_group::new(self.provider)
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
