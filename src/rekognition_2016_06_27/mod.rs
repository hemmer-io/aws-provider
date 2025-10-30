//! Rekognition_2016_06_27 Service
//!
//! Auto-generated service module for rekognition_2016_06_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rekognition_2016_06_27
pub struct Rekognition_2016_06_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Rekognition_2016_06_27Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get celebrity_info resource handler
    pub fn celebrity_info(&self) -> resources::Celebrity_info<'_> {
        resources::Celebrity_info::new(self.provider)
    }
    /// Get content_moderation resource handler
    pub fn content_moderation(&self) -> resources::Content_moderation<'_> {
        resources::Content_moderation::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get face_liveness_session resource handler
    pub fn face_liveness_session(&self) -> resources::Face_liveness_session<'_> {
        resources::Face_liveness_session::new(self.provider)
    }
    /// Get label_detection resource handler
    pub fn label_detection(&self) -> resources::Label_detection<'_> {
        resources::Label_detection::new(self.provider)
    }
    /// Get collection resource handler
    pub fn collection(&self) -> resources::Collection<'_> {
        resources::Collection::new(self.provider)
    }
    /// Get project_version resource handler
    pub fn project_version(&self) -> resources::Project_version<'_> {
        resources::Project_version::new(self.provider)
    }
    /// Get projects resource handler
    pub fn projects(&self) -> resources::Projects<'_> {
        resources::Projects::new(self.provider)
    }
    /// Get segment_detection resource handler
    pub fn segment_detection(&self) -> resources::Segment_detection<'_> {
        resources::Segment_detection::new(self.provider)
    }
    /// Get face_liveness_session_results resource handler
    pub fn face_liveness_session_results(&self) -> resources::Face_liveness_session_results<'_> {
        resources::Face_liveness_session_results::new(self.provider)
    }
    /// Get project_versions resource handler
    pub fn project_versions(&self) -> resources::Project_versions<'_> {
        resources::Project_versions::new(self.provider)
    }
    /// Get project_policy resource handler
    pub fn project_policy(&self) -> resources::Project_policy<'_> {
        resources::Project_policy::new(self.provider)
    }
    /// Get face_search resource handler
    pub fn face_search(&self) -> resources::Face_search<'_> {
        resources::Face_search::new(self.provider)
    }
    /// Get media_analysis_job resource handler
    pub fn media_analysis_job(&self) -> resources::Media_analysis_job<'_> {
        resources::Media_analysis_job::new(self.provider)
    }
    /// Get stream_processor resource handler
    pub fn stream_processor(&self) -> resources::Stream_processor<'_> {
        resources::Stream_processor::new(self.provider)
    }
    /// Get text_detection resource handler
    pub fn text_detection(&self) -> resources::Text_detection<'_> {
        resources::Text_detection::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get celebrity_recognition resource handler
    pub fn celebrity_recognition(&self) -> resources::Celebrity_recognition<'_> {
        resources::Celebrity_recognition::new(self.provider)
    }
    /// Get face_detection resource handler
    pub fn face_detection(&self) -> resources::Face_detection<'_> {
        resources::Face_detection::new(self.provider)
    }
    /// Get dataset_entries resource handler
    pub fn dataset_entries(&self) -> resources::Dataset_entries<'_> {
        resources::Dataset_entries::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get person_tracking resource handler
    pub fn person_tracking(&self) -> resources::Person_tracking<'_> {
        resources::Person_tracking::new(self.provider)
    }
    /// Get faces resource handler
    pub fn faces(&self) -> resources::Faces<'_> {
        resources::Faces::new(self.provider)
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
