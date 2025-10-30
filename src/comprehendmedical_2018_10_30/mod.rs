//! Comprehendmedical_2018_10_30 Service
//!
//! Auto-generated service module for comprehendmedical_2018_10_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for comprehendmedical_2018_10_30
pub struct Comprehendmedical_2018_10_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Comprehendmedical_2018_10_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get entities_detection_v2_job resource handler
    pub fn entities_detection_v2_job(&self) -> resources::Entities_detection_v2_job<'_> {
        resources::Entities_detection_v2_job::new(self.provider)
    }
    /// Get phi_detection_job resource handler
    pub fn phi_detection_job(&self) -> resources::Phi_detection_job<'_> {
        resources::Phi_detection_job::new(self.provider)
    }
    /// Get rx_norm_inference_job resource handler
    pub fn rx_norm_inference_job(&self) -> resources::Rx_norm_inference_job<'_> {
        resources::Rx_norm_inference_job::new(self.provider)
    }
    /// Get snomedct_inference_job resource handler
    pub fn snomedct_inference_job(&self) -> resources::Snomedct_inference_job<'_> {
        resources::Snomedct_inference_job::new(self.provider)
    }
    /// Get icd10_cm_inference_job resource handler
    pub fn icd10_cm_inference_job(&self) -> resources::Icd10_cm_inference_job<'_> {
        resources::Icd10_cm_inference_job::new(self.provider)
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
