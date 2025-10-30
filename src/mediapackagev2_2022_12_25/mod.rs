//! Mediapackagev2_2022_12_25 Service
//!
//! Auto-generated service module for mediapackagev2_2022_12_25

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for mediapackagev2_2022_12_25
pub struct Mediapackagev2_2022_12_25Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Mediapackagev2_2022_12_25Service<'a> {
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
