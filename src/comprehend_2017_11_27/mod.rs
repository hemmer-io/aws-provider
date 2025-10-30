//! Comprehend_2017_11_27 Service
//!
//! Auto-generated service module for comprehend_2017_11_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for comprehend_2017_11_27
pub struct Comprehend_2017_11_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Comprehend_2017_11_27Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get sentiment_detection_job resource handler
    pub fn sentiment_detection_job(&self) -> resources::Sentiment_detection_job<'_> {
        resources::Sentiment_detection_job::new(self.provider)
    }
    /// Get key_phrases_detection_job resource handler
    pub fn key_phrases_detection_job(&self) -> resources::Key_phrases_detection_job<'_> {
        resources::Key_phrases_detection_job::new(self.provider)
    }
    /// Get flywheel_iteration resource handler
    pub fn flywheel_iteration(&self) -> resources::Flywheel_iteration<'_> {
        resources::Flywheel_iteration::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get dominant_language_detection_job resource handler
    pub fn dominant_language_detection_job(&self) -> resources::Dominant_language_detection_job<'_> {
        resources::Dominant_language_detection_job::new(self.provider)
    }
    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
    }
    /// Get targeted_sentiment_detection_job resource handler
    pub fn targeted_sentiment_detection_job(&self) -> resources::Targeted_sentiment_detection_job<'_> {
        resources::Targeted_sentiment_detection_job::new(self.provider)
    }
    /// Get pii_entities_detection_job resource handler
    pub fn pii_entities_detection_job(&self) -> resources::Pii_entities_detection_job<'_> {
        resources::Pii_entities_detection_job::new(self.provider)
    }
    /// Get document_classification_job resource handler
    pub fn document_classification_job(&self) -> resources::Document_classification_job<'_> {
        resources::Document_classification_job::new(self.provider)
    }
    /// Get entities_detection_job resource handler
    pub fn entities_detection_job(&self) -> resources::Entities_detection_job<'_> {
        resources::Entities_detection_job::new(self.provider)
    }
    /// Get entity_recognizer resource handler
    pub fn entity_recognizer(&self) -> resources::Entity_recognizer<'_> {
        resources::Entity_recognizer::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get events_detection_job resource handler
    pub fn events_detection_job(&self) -> resources::Events_detection_job<'_> {
        resources::Events_detection_job::new(self.provider)
    }
    /// Get flywheel resource handler
    pub fn flywheel(&self) -> resources::Flywheel<'_> {
        resources::Flywheel::new(self.provider)
    }
    /// Get document_classifier resource handler
    pub fn document_classifier(&self) -> resources::Document_classifier<'_> {
        resources::Document_classifier::new(self.provider)
    }
    /// Get topics_detection_job resource handler
    pub fn topics_detection_job(&self) -> resources::Topics_detection_job<'_> {
        resources::Topics_detection_job::new(self.provider)
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
