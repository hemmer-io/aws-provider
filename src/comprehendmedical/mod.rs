//! Comprehendmedical Service
//!
//! Auto-generated service module for comprehendmedical

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for comprehendmedical
pub struct ComprehendmedicalService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ComprehendmedicalService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get rx_norm_inference_job resource handler
    pub fn rx_norm_inference_job(&self) -> resources::Rx_norm_inference_job<'_> {
        resources::Rx_norm_inference_job::new(self.provider)
    }
    /// Get entities_detection_v2_job resource handler
    pub fn entities_detection_v2_job(&self) -> resources::Entities_detection_v2_job<'_> {
        resources::Entities_detection_v2_job::new(self.provider)
    }
    /// Get snomedctinference_job resource handler
    pub fn snomedctinference_job(&self) -> resources::Snomedctinference_job<'_> {
        resources::Snomedctinference_job::new(self.provider)
    }
    /// Get phidetection_job resource handler
    pub fn phidetection_job(&self) -> resources::Phidetection_job<'_> {
        resources::Phidetection_job::new(self.provider)
    }
    /// Get icd10_cminference_job resource handler
    pub fn icd10_cminference_job(&self) -> resources::Icd10_cminference_job<'_> {
        resources::Icd10_cminference_job::new(self.provider)
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
