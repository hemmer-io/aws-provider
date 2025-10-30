//! Emr_serverless_2021_07_13 Service
//!
//! Auto-generated service module for emr_serverless_2021_07_13

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for emr_serverless_2021_07_13
pub struct Emr_serverless_2021_07_13Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Emr_serverless_2021_07_13Service<'a> {
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
