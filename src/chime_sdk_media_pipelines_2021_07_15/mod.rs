//! Chime_sdk_media_pipelines_2021_07_15 Service
//!
//! Auto-generated service module for chime_sdk_media_pipelines_2021_07_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for chime_sdk_media_pipelines_2021_07_15
pub struct Chime_sdk_media_pipelines_2021_07_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Chime_sdk_media_pipelines_2021_07_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get media_capture_pipeline resource handler
    pub fn media_capture_pipeline(&self) -> resources::Media_capture_pipeline<'_> {
        resources::Media_capture_pipeline::new(self.provider)
    }
    /// Get speaker_search_task resource handler
    pub fn speaker_search_task(&self) -> resources::Speaker_search_task<'_> {
        resources::Speaker_search_task::new(self.provider)
    }
    /// Get media_pipeline_kinesis_video_stream_pool resource handler
    pub fn media_pipeline_kinesis_video_stream_pool(&self) -> resources::Media_pipeline_kinesis_video_stream_pool<'_> {
        resources::Media_pipeline_kinesis_video_stream_pool::new(self.provider)
    }
    /// Get media_stream_pipeline resource handler
    pub fn media_stream_pipeline(&self) -> resources::Media_stream_pipeline<'_> {
        resources::Media_stream_pipeline::new(self.provider)
    }
    /// Get media_insights_pipeline_configuration resource handler
    pub fn media_insights_pipeline_configuration(&self) -> resources::Media_insights_pipeline_configuration<'_> {
        resources::Media_insights_pipeline_configuration::new(self.provider)
    }
    /// Get media_insights_pipeline resource handler
    pub fn media_insights_pipeline(&self) -> resources::Media_insights_pipeline<'_> {
        resources::Media_insights_pipeline::new(self.provider)
    }
    /// Get media_concatenation_pipeline resource handler
    pub fn media_concatenation_pipeline(&self) -> resources::Media_concatenation_pipeline<'_> {
        resources::Media_concatenation_pipeline::new(self.provider)
    }
    /// Get media_pipeline resource handler
    pub fn media_pipeline(&self) -> resources::Media_pipeline<'_> {
        resources::Media_pipeline::new(self.provider)
    }
    /// Get voice_tone_analysis_task resource handler
    pub fn voice_tone_analysis_task(&self) -> resources::Voice_tone_analysis_task<'_> {
        resources::Voice_tone_analysis_task::new(self.provider)
    }
    /// Get media_insights_pipeline_status resource handler
    pub fn media_insights_pipeline_status(&self) -> resources::Media_insights_pipeline_status<'_> {
        resources::Media_insights_pipeline_status::new(self.provider)
    }
    /// Get media_live_connector_pipeline resource handler
    pub fn media_live_connector_pipeline(&self) -> resources::Media_live_connector_pipeline<'_> {
        resources::Media_live_connector_pipeline::new(self.provider)
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
