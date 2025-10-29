//! Machine Service
//!
//! Auto-generated service module for machine

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for machine
pub struct MachineService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MachineService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get realtime_endpoint resource handler
    pub fn realtime_endpoint(&self) -> resources::Realtime_endpoint<'_> {
        resources::Realtime_endpoint::new(self.provider)
    }
    /// Get mlmodel resource handler
    pub fn mlmodel(&self) -> resources::Mlmodel<'_> {
        resources::Mlmodel::new(self.provider)
    }
    /// Get batch_prediction resource handler
    pub fn batch_prediction(&self) -> resources::Batch_prediction<'_> {
        resources::Batch_prediction::new(self.provider)
    }
    /// Get mlmodels resource handler
    pub fn mlmodels(&self) -> resources::Mlmodels<'_> {
        resources::Mlmodels::new(self.provider)
    }
    /// Get data_source_from_rds resource handler
    pub fn data_source_from_rds(&self) -> resources::Data_source_from_rds<'_> {
        resources::Data_source_from_rds::new(self.provider)
    }
    /// Get data_source resource handler
    pub fn data_source(&self) -> resources::Data_source<'_> {
        resources::Data_source::new(self.provider)
    }
    /// Get data_source_from_redshift resource handler
    pub fn data_source_from_redshift(&self) -> resources::Data_source_from_redshift<'_> {
        resources::Data_source_from_redshift::new(self.provider)
    }
    /// Get data_source_from_s3 resource handler
    pub fn data_source_from_s3(&self) -> resources::Data_source_from_s3<'_> {
        resources::Data_source_from_s3::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get data_sources resource handler
    pub fn data_sources(&self) -> resources::Data_sources<'_> {
        resources::Data_sources::new(self.provider)
    }
    /// Get evaluations resource handler
    pub fn evaluations(&self) -> resources::Evaluations<'_> {
        resources::Evaluations::new(self.provider)
    }
    /// Get batch_predictions resource handler
    pub fn batch_predictions(&self) -> resources::Batch_predictions<'_> {
        resources::Batch_predictions::new(self.provider)
    }
    /// Get evaluation resource handler
    pub fn evaluation(&self) -> resources::Evaluation<'_> {
        resources::Evaluation::new(self.provider)
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
