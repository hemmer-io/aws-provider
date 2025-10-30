//! Cloudtrail_data_2021_08_11 Service
//!
//! Auto-generated service module for cloudtrail_data_2021_08_11

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudtrail_data_2021_08_11
pub struct Cloudtrail_data_2021_08_11Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudtrail_data_2021_08_11Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get audit_events resource handler
    pub fn audit_events(&self) -> resources::Audit_events<'_> {
        resources::Audit_events::new(self.provider)
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
