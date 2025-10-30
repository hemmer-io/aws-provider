//! Ssm_guiconnect_2021_05_01 Service
//!
//! Auto-generated service module for ssm_guiconnect_2021_05_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ssm_guiconnect_2021_05_01
pub struct Ssm_guiconnect_2021_05_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ssm_guiconnect_2021_05_01Service<'a> {
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
