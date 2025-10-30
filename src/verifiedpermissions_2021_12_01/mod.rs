//! Verifiedpermissions_2021_12_01 Service
//!
//! Auto-generated service module for verifiedpermissions_2021_12_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for verifiedpermissions_2021_12_01
pub struct Verifiedpermissions_2021_12_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Verifiedpermissions_2021_12_01Service<'a> {
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
