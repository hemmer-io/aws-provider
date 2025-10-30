//! Pinpoint_2016_12_01 Service
//!
//! Auto-generated service module for pinpoint_2016_12_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pinpoint_2016_12_01
pub struct Pinpoint_2016_12_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Pinpoint_2016_12_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get campaign_date_range_kpi resource handler
    pub fn campaign_date_range_kpi(&self) -> resources::Campaign_date_range_kpi<'_> {
        resources::Campaign_date_range_kpi::new(self.provider)
    }
    /// Get journey resource handler
    pub fn journey(&self) -> resources::Journey<'_> {
        resources::Journey::new(self.provider)
    }
    /// Get apns_voip_channel resource handler
    pub fn apns_voip_channel(&self) -> resources::Apns_voip_channel<'_> {
        resources::Apns_voip_channel::new(self.provider)
    }
    /// Get push_template resource handler
    pub fn push_template(&self) -> resources::Push_template<'_> {
        resources::Push_template::new(self.provider)
    }
    /// Get email_channel resource handler
    pub fn email_channel(&self) -> resources::Email_channel<'_> {
        resources::Email_channel::new(self.provider)
    }
    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
    }
    /// Get apns_sandbox_channel resource handler
    pub fn apns_sandbox_channel(&self) -> resources::Apns_sandbox_channel<'_> {
        resources::Apns_sandbox_channel::new(self.provider)
    }
    /// Get campaign_version resource handler
    pub fn campaign_version(&self) -> resources::Campaign_version<'_> {
        resources::Campaign_version::new(self.provider)
    }
    /// Get segments resource handler
    pub fn segments(&self) -> resources::Segments<'_> {
        resources::Segments::new(self.provider)
    }
    /// Get campaign_versions resource handler
    pub fn campaign_versions(&self) -> resources::Campaign_versions<'_> {
        resources::Campaign_versions::new(self.provider)
    }
    /// Get voice_template resource handler
    pub fn voice_template(&self) -> resources::Voice_template<'_> {
        resources::Voice_template::new(self.provider)
    }
    /// Get apns_channel resource handler
    pub fn apns_channel(&self) -> resources::Apns_channel<'_> {
        resources::Apns_channel::new(self.provider)
    }
    /// Get campaigns resource handler
    pub fn campaigns(&self) -> resources::Campaigns<'_> {
        resources::Campaigns::new(self.provider)
    }
    /// Get export_jobs resource handler
    pub fn export_jobs(&self) -> resources::Export_jobs<'_> {
        resources::Export_jobs::new(self.provider)
    }
    /// Get sms_template resource handler
    pub fn sms_template(&self) -> resources::Sms_template<'_> {
        resources::Sms_template::new(self.provider)
    }
    /// Get in_app_messages resource handler
    pub fn in_app_messages(&self) -> resources::In_app_messages<'_> {
        resources::In_app_messages::new(self.provider)
    }
    /// Get segment_export_jobs resource handler
    pub fn segment_export_jobs(&self) -> resources::Segment_export_jobs<'_> {
        resources::Segment_export_jobs::new(self.provider)
    }
    /// Get endpoints_batch resource handler
    pub fn endpoints_batch(&self) -> resources::Endpoints_batch<'_> {
        resources::Endpoints_batch::new(self.provider)
    }
    /// Get channels resource handler
    pub fn channels(&self) -> resources::Channels<'_> {
        resources::Channels::new(self.provider)
    }
    /// Get journey_state resource handler
    pub fn journey_state(&self) -> resources::Journey_state<'_> {
        resources::Journey_state::new(self.provider)
    }
    /// Get baidu_channel resource handler
    pub fn baidu_channel(&self) -> resources::Baidu_channel<'_> {
        resources::Baidu_channel::new(self.provider)
    }
    /// Get journey_date_range_kpi resource handler
    pub fn journey_date_range_kpi(&self) -> resources::Journey_date_range_kpi<'_> {
        resources::Journey_date_range_kpi::new(self.provider)
    }
    /// Get application_settings resource handler
    pub fn application_settings(&self) -> resources::Application_settings<'_> {
        resources::Application_settings::new(self.provider)
    }
    /// Get apps resource handler
    pub fn apps(&self) -> resources::Apps<'_> {
        resources::Apps::new(self.provider)
    }
    /// Get import_job resource handler
    pub fn import_job(&self) -> resources::Import_job<'_> {
        resources::Import_job::new(self.provider)
    }
    /// Get in_app_template resource handler
    pub fn in_app_template(&self) -> resources::In_app_template<'_> {
        resources::In_app_template::new(self.provider)
    }
    /// Get journey_runs resource handler
    pub fn journey_runs(&self) -> resources::Journey_runs<'_> {
        resources::Journey_runs::new(self.provider)
    }
    /// Get segment resource handler
    pub fn segment(&self) -> resources::Segment<'_> {
        resources::Segment::new(self.provider)
    }
    /// Get voice_channel resource handler
    pub fn voice_channel(&self) -> resources::Voice_channel<'_> {
        resources::Voice_channel::new(self.provider)
    }
    /// Get apns_voip_sandbox_channel resource handler
    pub fn apns_voip_sandbox_channel(&self) -> resources::Apns_voip_sandbox_channel<'_> {
        resources::Apns_voip_sandbox_channel::new(self.provider)
    }
    /// Get campaign resource handler
    pub fn campaign(&self) -> resources::Campaign<'_> {
        resources::Campaign::new(self.provider)
    }
    /// Get email_template resource handler
    pub fn email_template(&self) -> resources::Email_template<'_> {
        resources::Email_template::new(self.provider)
    }
    /// Get recommender_configurations resource handler
    pub fn recommender_configurations(&self) -> resources::Recommender_configurations<'_> {
        resources::Recommender_configurations::new(self.provider)
    }
    /// Get template_active_version resource handler
    pub fn template_active_version(&self) -> resources::Template_active_version<'_> {
        resources::Template_active_version::new(self.provider)
    }
    /// Get export_job resource handler
    pub fn export_job(&self) -> resources::Export_job<'_> {
        resources::Export_job::new(self.provider)
    }
    /// Get import_jobs resource handler
    pub fn import_jobs(&self) -> resources::Import_jobs<'_> {
        resources::Import_jobs::new(self.provider)
    }
    /// Get journey_run_execution_activity_metrics resource handler
    pub fn journey_run_execution_activity_metrics(&self) -> resources::Journey_run_execution_activity_metrics<'_> {
        resources::Journey_run_execution_activity_metrics::new(self.provider)
    }
    /// Get event_stream resource handler
    pub fn event_stream(&self) -> resources::Event_stream<'_> {
        resources::Event_stream::new(self.provider)
    }
    /// Get gcm_channel resource handler
    pub fn gcm_channel(&self) -> resources::Gcm_channel<'_> {
        resources::Gcm_channel::new(self.provider)
    }
    /// Get events resource handler
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(self.provider)
    }
    /// Get sms_channel resource handler
    pub fn sms_channel(&self) -> resources::Sms_channel<'_> {
        resources::Sms_channel::new(self.provider)
    }
    /// Get user_endpoints resource handler
    pub fn user_endpoints(&self) -> resources::User_endpoints<'_> {
        resources::User_endpoints::new(self.provider)
    }
    /// Get application_date_range_kpi resource handler
    pub fn application_date_range_kpi(&self) -> resources::Application_date_range_kpi<'_> {
        resources::Application_date_range_kpi::new(self.provider)
    }
    /// Get campaign_activities resource handler
    pub fn campaign_activities(&self) -> resources::Campaign_activities<'_> {
        resources::Campaign_activities::new(self.provider)
    }
    /// Get journey_execution_activity_metrics resource handler
    pub fn journey_execution_activity_metrics(&self) -> resources::Journey_execution_activity_metrics<'_> {
        resources::Journey_execution_activity_metrics::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
    }
    /// Get journey_run_execution_metrics resource handler
    pub fn journey_run_execution_metrics(&self) -> resources::Journey_run_execution_metrics<'_> {
        resources::Journey_run_execution_metrics::new(self.provider)
    }
    /// Get adm_channel resource handler
    pub fn adm_channel(&self) -> resources::Adm_channel<'_> {
        resources::Adm_channel::new(self.provider)
    }
    /// Get recommender_configuration resource handler
    pub fn recommender_configuration(&self) -> resources::Recommender_configuration<'_> {
        resources::Recommender_configuration::new(self.provider)
    }
    /// Get journey_execution_metrics resource handler
    pub fn journey_execution_metrics(&self) -> resources::Journey_execution_metrics<'_> {
        resources::Journey_execution_metrics::new(self.provider)
    }
    /// Get segment_versions resource handler
    pub fn segment_versions(&self) -> resources::Segment_versions<'_> {
        resources::Segment_versions::new(self.provider)
    }
    /// Get segment_import_jobs resource handler
    pub fn segment_import_jobs(&self) -> resources::Segment_import_jobs<'_> {
        resources::Segment_import_jobs::new(self.provider)
    }
    /// Get segment_version resource handler
    pub fn segment_version(&self) -> resources::Segment_version<'_> {
        resources::Segment_version::new(self.provider)
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
