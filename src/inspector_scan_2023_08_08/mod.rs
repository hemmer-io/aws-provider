//! Inspector_scan_2023_08_08 Service
//!
//! Auto-generated service module for inspector_scan_2023_08_08

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for inspector_scan_2023_08_08
pub struct Inspector_scan_2023_08_08Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Inspector_scan_2023_08_08Service<'a> {
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
