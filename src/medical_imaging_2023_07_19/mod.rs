//! Medical_imaging_2023_07_19 Service
//!
//! Auto-generated service module for medical_imaging_2023_07_19

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for medical_imaging_2023_07_19
pub struct Medical_imaging_2023_07_19Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Medical_imaging_2023_07_19Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get dicom_import_job resource handler
    pub fn dicom_import_job(&self) -> resources::Dicom_import_job<'_> {
        resources::Dicom_import_job::new(self.provider)
    }
    /// Get image_frame resource handler
    pub fn image_frame(&self) -> resources::Image_frame<'_> {
        resources::Image_frame::new(self.provider)
    }
    /// Get image_set resource handler
    pub fn image_set(&self) -> resources::Image_set<'_> {
        resources::Image_set::new(self.provider)
    }
    /// Get image_set_metadata resource handler
    pub fn image_set_metadata(&self) -> resources::Image_set_metadata<'_> {
        resources::Image_set_metadata::new(self.provider)
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
