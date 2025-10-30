//! Personalize_2018_05_22 Service
//!
//! Auto-generated service module for personalize_2018_05_22

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for personalize_2018_05_22
pub struct Personalize_2018_05_22Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Personalize_2018_05_22Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get dataset_export_job resource handler
    pub fn dataset_export_job(&self) -> resources::Dataset_export_job<'_> {
        resources::Dataset_export_job::new(self.provider)
    }
    /// Get filter resource handler
    pub fn filter(&self) -> resources::Filter<'_> {
        resources::Filter::new(self.provider)
    }
    /// Get campaign resource handler
    pub fn campaign(&self) -> resources::Campaign<'_> {
        resources::Campaign::new(self.provider)
    }
    /// Get metric_attribution resource handler
    pub fn metric_attribution(&self) -> resources::Metric_attribution<'_> {
        resources::Metric_attribution::new(self.provider)
    }
    /// Get data_deletion_job resource handler
    pub fn data_deletion_job(&self) -> resources::Data_deletion_job<'_> {
        resources::Data_deletion_job::new(self.provider)
    }
    /// Get algorithm resource handler
    pub fn algorithm(&self) -> resources::Algorithm<'_> {
        resources::Algorithm::new(self.provider)
    }
    /// Get recommender resource handler
    pub fn recommender(&self) -> resources::Recommender<'_> {
        resources::Recommender::new(self.provider)
    }
    /// Get batch_segment_job resource handler
    pub fn batch_segment_job(&self) -> resources::Batch_segment_job<'_> {
        resources::Batch_segment_job::new(self.provider)
    }
    /// Get batch_inference_job resource handler
    pub fn batch_inference_job(&self) -> resources::Batch_inference_job<'_> {
        resources::Batch_inference_job::new(self.provider)
    }
    /// Get solution_version resource handler
    pub fn solution_version(&self) -> resources::Solution_version<'_> {
        resources::Solution_version::new(self.provider)
    }
    /// Get recipe resource handler
    pub fn recipe(&self) -> resources::Recipe<'_> {
        resources::Recipe::new(self.provider)
    }
    /// Get feature_transformation resource handler
    pub fn feature_transformation(&self) -> resources::Feature_transformation<'_> {
        resources::Feature_transformation::new(self.provider)
    }
    /// Get dataset_import_job resource handler
    pub fn dataset_import_job(&self) -> resources::Dataset_import_job<'_> {
        resources::Dataset_import_job::new(self.provider)
    }
    /// Get event_tracker resource handler
    pub fn event_tracker(&self) -> resources::Event_tracker<'_> {
        resources::Event_tracker::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get dataset_group resource handler
    pub fn dataset_group(&self) -> resources::Dataset_group<'_> {
        resources::Dataset_group::new(self.provider)
    }
    /// Get schema resource handler
    pub fn schema(&self) -> resources::Schema<'_> {
        resources::Schema::new(self.provider)
    }
    /// Get solution resource handler
    pub fn solution(&self) -> resources::Solution<'_> {
        resources::Solution::new(self.provider)
    }
    /// Get solution_metrics resource handler
    pub fn solution_metrics(&self) -> resources::Solution_metrics<'_> {
        resources::Solution_metrics::new(self.provider)
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
