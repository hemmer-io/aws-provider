//! Transcribe_2017_10_26 Service
//!
//! Auto-generated service module for transcribe_2017_10_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for transcribe_2017_10_26
pub struct Transcribe_2017_10_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Transcribe_2017_10_26Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get language_model resource handler
    pub fn language_model(&self) -> resources::Language_model<'_> {
        resources::Language_model::new(self.provider)
    }
    /// Get vocabulary resource handler
    pub fn vocabulary(&self) -> resources::Vocabulary<'_> {
        resources::Vocabulary::new(self.provider)
    }
    /// Get medical_scribe_job resource handler
    pub fn medical_scribe_job(&self) -> resources::Medical_scribe_job<'_> {
        resources::Medical_scribe_job::new(self.provider)
    }
    /// Get call_analytics_category resource handler
    pub fn call_analytics_category(&self) -> resources::Call_analytics_category<'_> {
        resources::Call_analytics_category::new(self.provider)
    }
    /// Get vocabulary_filter resource handler
    pub fn vocabulary_filter(&self) -> resources::Vocabulary_filter<'_> {
        resources::Vocabulary_filter::new(self.provider)
    }
    /// Get call_analytics_job resource handler
    pub fn call_analytics_job(&self) -> resources::Call_analytics_job<'_> {
        resources::Call_analytics_job::new(self.provider)
    }
    /// Get medical_transcription_job resource handler
    pub fn medical_transcription_job(&self) -> resources::Medical_transcription_job<'_> {
        resources::Medical_transcription_job::new(self.provider)
    }
    /// Get medical_vocabulary resource handler
    pub fn medical_vocabulary(&self) -> resources::Medical_vocabulary<'_> {
        resources::Medical_vocabulary::new(self.provider)
    }
    /// Get transcription_job resource handler
    pub fn transcription_job(&self) -> resources::Transcription_job<'_> {
        resources::Transcription_job::new(self.provider)
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
