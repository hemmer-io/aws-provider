//! Internetmonitor_2021_06_03 Service
//!
//! Auto-generated service module for internetmonitor_2021_06_03

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for internetmonitor_2021_06_03
pub struct Internetmonitor_2021_06_03Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Internetmonitor_2021_06_03Service<'a> {
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
