//! Synthetics_2017_10_11 Service
//!
//! Auto-generated service module for synthetics_2017_10_11

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for synthetics_2017_10_11
pub struct Synthetics_2017_10_11Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Synthetics_2017_10_11Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
    }
    /// Get canaries_last_run resource handler
    pub fn canaries_last_run(&self) -> resources::Canaries_last_run<'_> {
        resources::Canaries_last_run::new(self.provider)
    }
    /// Get canary resource handler
    pub fn canary(&self) -> resources::Canary<'_> {
        resources::Canary::new(self.provider)
    }
    /// Get canaries resource handler
    pub fn canaries(&self) -> resources::Canaries<'_> {
        resources::Canaries::new(self.provider)
    }
    /// Get canary_runs resource handler
    pub fn canary_runs(&self) -> resources::Canary_runs<'_> {
        resources::Canary_runs::new(self.provider)
    }
    /// Get runtime_versions resource handler
    pub fn runtime_versions(&self) -> resources::Runtime_versions<'_> {
        resources::Runtime_versions::new(self.provider)
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
