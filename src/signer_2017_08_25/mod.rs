//! Signer_2017_08_25 Service
//!
//! Auto-generated service module for signer_2017_08_25

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for signer_2017_08_25
pub struct Signer_2017_08_25Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Signer_2017_08_25Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get signing_profile resource handler
    pub fn signing_profile(&self) -> resources::Signing_profile<'_> {
        resources::Signing_profile::new(self.provider)
    }
    /// Get signing_platform resource handler
    pub fn signing_platform(&self) -> resources::Signing_platform<'_> {
        resources::Signing_platform::new(self.provider)
    }
    /// Get signing_job resource handler
    pub fn signing_job(&self) -> resources::Signing_job<'_> {
        resources::Signing_job::new(self.provider)
    }
    /// Get revocation_status resource handler
    pub fn revocation_status(&self) -> resources::Revocation_status<'_> {
        resources::Revocation_status::new(self.provider)
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
