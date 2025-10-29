//! Medical Service
//!
//! Auto-generated service module for medical

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for medical
pub struct MedicalService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> MedicalService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get image_set resource handler
    pub fn image_set(&self) -> resources::Image_set<'_> {
        resources::Image_set::new(self.provider)
    }
    /// Get image_frame resource handler
    pub fn image_frame(&self) -> resources::Image_frame<'_> {
        resources::Image_frame::new(self.provider)
    }
    /// Get dicomimport_job resource handler
    pub fn dicomimport_job(&self) -> resources::Dicomimport_job<'_> {
        resources::Dicomimport_job::new(self.provider)
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
