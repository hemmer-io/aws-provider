//! Omics_2022_11_28 Service
//!
//! Auto-generated service module for omics_2022_11_28

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for omics_2022_11_28
pub struct Omics_2022_11_28Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Omics_2022_11_28Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get s3_access_policy resource handler
    pub fn s3_access_policy(&self) -> resources::S3_access_policy<'_> {
        resources::S3_access_policy::new(self.provider)
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
