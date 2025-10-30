//! Migrationhuborchestrator_2021_08_28 Service
//!
//! Auto-generated service module for migrationhuborchestrator_2021_08_28

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for migrationhuborchestrator_2021_08_28
pub struct Migrationhuborchestrator_2021_08_28Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Migrationhuborchestrator_2021_08_28Service<'a> {
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
