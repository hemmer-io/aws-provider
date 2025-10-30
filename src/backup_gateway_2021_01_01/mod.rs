//! Backup_gateway_2021_01_01 Service
//!
//! Auto-generated service module for backup_gateway_2021_01_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for backup_gateway_2021_01_01
pub struct Backup_gateway_2021_01_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Backup_gateway_2021_01_01Service<'a> {
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
