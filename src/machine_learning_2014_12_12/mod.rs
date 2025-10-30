//! Machine_learning_2014_12_12 Service
//!
//! Auto-generated service module for machine_learning_2014_12_12

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for machine_learning_2014_12_12
pub struct Machine_learning_2014_12_12Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Machine_learning_2014_12_12Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get batch_predictions resource handler
    pub fn batch_predictions(&self) -> resources::Batch_predictions<'_> {
        resources::Batch_predictions::new(self.provider)
    }
    /// Get ml_model resource handler
    pub fn ml_model(&self) -> resources::Ml_model<'_> {
        resources::Ml_model::new(self.provider)
    }
    /// Get data_source resource handler
    pub fn data_source(&self) -> resources::Data_source<'_> {
        resources::Data_source::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get ml_models resource handler
    pub fn ml_models(&self) -> resources::Ml_models<'_> {
        resources::Ml_models::new(self.provider)
    }
    /// Get data_source_from_rds resource handler
    pub fn data_source_from_rds(&self) -> resources::Data_source_from_rds<'_> {
        resources::Data_source_from_rds::new(self.provider)
    }
    /// Get batch_prediction resource handler
    pub fn batch_prediction(&self) -> resources::Batch_prediction<'_> {
        resources::Batch_prediction::new(self.provider)
    }
    /// Get data_source_from_s3 resource handler
    pub fn data_source_from_s3(&self) -> resources::Data_source_from_s3<'_> {
        resources::Data_source_from_s3::new(self.provider)
    }
    /// Get realtime_endpoint resource handler
    pub fn realtime_endpoint(&self) -> resources::Realtime_endpoint<'_> {
        resources::Realtime_endpoint::new(self.provider)
    }
    /// Get data_sources resource handler
    pub fn data_sources(&self) -> resources::Data_sources<'_> {
        resources::Data_sources::new(self.provider)
    }
    /// Get evaluations resource handler
    pub fn evaluations(&self) -> resources::Evaluations<'_> {
        resources::Evaluations::new(self.provider)
    }
    /// Get evaluation resource handler
    pub fn evaluation(&self) -> resources::Evaluation<'_> {
        resources::Evaluation::new(self.provider)
    }
    /// Get data_source_from_redshift resource handler
    pub fn data_source_from_redshift(&self) -> resources::Data_source_from_redshift<'_> {
        resources::Data_source_from_redshift::new(self.provider)
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
