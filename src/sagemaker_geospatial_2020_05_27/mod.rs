//! Sagemaker_geospatial_2020_05_27 Service
//!
//! Auto-generated service module for sagemaker_geospatial_2020_05_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for sagemaker_geospatial_2020_05_27
pub struct Sagemaker_geospatial_2020_05_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Sagemaker_geospatial_2020_05_27Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
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
