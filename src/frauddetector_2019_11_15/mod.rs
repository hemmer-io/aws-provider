//! Frauddetector_2019_11_15 Service
//!
//! Auto-generated service module for frauddetector_2019_11_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for frauddetector_2019_11_15
pub struct Frauddetector_2019_11_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Frauddetector_2019_11_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get events_by_event_type resource handler
    pub fn events_by_event_type(&self) -> resources::Events_by_event_type<'_> {
        resources::Events_by_event_type::new(self.provider)
    }
    /// Get model_version resource handler
    pub fn model_version(&self) -> resources::Model_version<'_> {
        resources::Model_version::new(self.provider)
    }
    /// Get label resource handler
    pub fn label(&self) -> resources::Label<'_> {
        resources::Label::new(self.provider)
    }
    /// Get external_model resource handler
    pub fn external_model(&self) -> resources::External_model<'_> {
        resources::External_model::new(self.provider)
    }
    /// Get labels resource handler
    pub fn labels(&self) -> resources::Labels<'_> {
        resources::Labels::new(self.provider)
    }
    /// Get delete_events_by_event_type_status resource handler
    pub fn delete_events_by_event_type_status(&self) -> resources::Delete_events_by_event_type_status<'_> {
        resources::Delete_events_by_event_type_status::new(self.provider)
    }
    /// Get rule resource handler
    pub fn rule(&self) -> resources::Rule<'_> {
        resources::Rule::new(self.provider)
    }
    /// Get lists_metadata resource handler
    pub fn lists_metadata(&self) -> resources::Lists_metadata<'_> {
        resources::Lists_metadata::new(self.provider)
    }
    /// Get model_version_status resource handler
    pub fn model_version_status(&self) -> resources::Model_version_status<'_> {
        resources::Model_version_status::new(self.provider)
    }
    /// Get batch_prediction_job resource handler
    pub fn batch_prediction_job(&self) -> resources::Batch_prediction_job<'_> {
        resources::Batch_prediction_job::new(self.provider)
    }
    /// Get outcome resource handler
    pub fn outcome(&self) -> resources::Outcome<'_> {
        resources::Outcome::new(self.provider)
    }
    /// Get event_types resource handler
    pub fn event_types(&self) -> resources::Event_types<'_> {
        resources::Event_types::new(self.provider)
    }
    /// Get batch_import_job resource handler
    pub fn batch_import_job(&self) -> resources::Batch_import_job<'_> {
        resources::Batch_import_job::new(self.provider)
    }
    /// Get entity_types resource handler
    pub fn entity_types(&self) -> resources::Entity_types<'_> {
        resources::Entity_types::new(self.provider)
    }
    /// Get entity_type resource handler
    pub fn entity_type(&self) -> resources::Entity_type<'_> {
        resources::Entity_type::new(self.provider)
    }
    /// Get rules resource handler
    pub fn rules(&self) -> resources::Rules<'_> {
        resources::Rules::new(self.provider)
    }
    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
    }
    /// Get batch_prediction_jobs resource handler
    pub fn batch_prediction_jobs(&self) -> resources::Batch_prediction_jobs<'_> {
        resources::Batch_prediction_jobs::new(self.provider)
    }
    /// Get detectors resource handler
    pub fn detectors(&self) -> resources::Detectors<'_> {
        resources::Detectors::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
    }
    /// Get detector_version_metadata resource handler
    pub fn detector_version_metadata(&self) -> resources::Detector_version_metadata<'_> {
        resources::Detector_version_metadata::new(self.provider)
    }
    /// Get list_elements resource handler
    pub fn list_elements(&self) -> resources::List_elements<'_> {
        resources::List_elements::new(self.provider)
    }
    /// Get outcomes resource handler
    pub fn outcomes(&self) -> resources::Outcomes<'_> {
        resources::Outcomes::new(self.provider)
    }
    /// Get detector resource handler
    pub fn detector(&self) -> resources::Detector<'_> {
        resources::Detector::new(self.provider)
    }
    /// Get external_models resource handler
    pub fn external_models(&self) -> resources::External_models<'_> {
        resources::External_models::new(self.provider)
    }
    /// Get detector_version resource handler
    pub fn detector_version(&self) -> resources::Detector_version<'_> {
        resources::Detector_version::new(self.provider)
    }
    /// Get models resource handler
    pub fn models(&self) -> resources::Models<'_> {
        resources::Models::new(self.provider)
    }
    /// Get rule_version resource handler
    pub fn rule_version(&self) -> resources::Rule_version<'_> {
        resources::Rule_version::new(self.provider)
    }
    /// Get batch_import_jobs resource handler
    pub fn batch_import_jobs(&self) -> resources::Batch_import_jobs<'_> {
        resources::Batch_import_jobs::new(self.provider)
    }
    /// Get model_versions resource handler
    pub fn model_versions(&self) -> resources::Model_versions<'_> {
        resources::Model_versions::new(self.provider)
    }
    /// Get event_prediction resource handler
    pub fn event_prediction(&self) -> resources::Event_prediction<'_> {
        resources::Event_prediction::new(self.provider)
    }
    /// Get kms_encryption_key resource handler
    pub fn kms_encryption_key(&self) -> resources::Kms_encryption_key<'_> {
        resources::Kms_encryption_key::new(self.provider)
    }
    /// Get variables resource handler
    pub fn variables(&self) -> resources::Variables<'_> {
        resources::Variables::new(self.provider)
    }
    /// Get detector_version_status resource handler
    pub fn detector_version_status(&self) -> resources::Detector_version_status<'_> {
        resources::Detector_version_status::new(self.provider)
    }
    /// Get event_label resource handler
    pub fn event_label(&self) -> resources::Event_label<'_> {
        resources::Event_label::new(self.provider)
    }
    /// Get list resource handler
    pub fn list(&self) -> resources::List<'_> {
        resources::List::new(self.provider)
    }
    /// Get variable resource handler
    pub fn variable(&self) -> resources::Variable<'_> {
        resources::Variable::new(self.provider)
    }
    /// Get event_type resource handler
    pub fn event_type(&self) -> resources::Event_type<'_> {
        resources::Event_type::new(self.provider)
    }
    /// Get event_prediction_metadata resource handler
    pub fn event_prediction_metadata(&self) -> resources::Event_prediction_metadata<'_> {
        resources::Event_prediction_metadata::new(self.provider)
    }
    /// Get rule_metadata resource handler
    pub fn rule_metadata(&self) -> resources::Rule_metadata<'_> {
        resources::Rule_metadata::new(self.provider)
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
