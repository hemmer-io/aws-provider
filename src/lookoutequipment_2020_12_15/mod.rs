//! Lookoutequipment_2020_12_15 Service
//!
//! Auto-generated service module for lookoutequipment_2020_12_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for lookoutequipment_2020_12_15
pub struct Lookoutequipment_2020_12_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Lookoutequipment_2020_12_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get retraining_scheduler resource handler
    pub fn retraining_scheduler(&self) -> resources::Retraining_scheduler<'_> {
        resources::Retraining_scheduler::new(self.provider)
    }
    /// Get data_ingestion_job resource handler
    pub fn data_ingestion_job(&self) -> resources::Data_ingestion_job<'_> {
        resources::Data_ingestion_job::new(self.provider)
    }
    /// Get model_version resource handler
    pub fn model_version(&self) -> resources::Model_version<'_> {
        resources::Model_version::new(self.provider)
    }
    /// Get label_group resource handler
    pub fn label_group(&self) -> resources::Label_group<'_> {
        resources::Label_group::new(self.provider)
    }
    /// Get label resource handler
    pub fn label(&self) -> resources::Label<'_> {
        resources::Label::new(self.provider)
    }
    /// Get inference_scheduler resource handler
    pub fn inference_scheduler(&self) -> resources::Inference_scheduler<'_> {
        resources::Inference_scheduler::new(self.provider)
    }
    /// Get active_model_version resource handler
    pub fn active_model_version(&self) -> resources::Active_model_version<'_> {
        resources::Active_model_version::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
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
