//! Voice_id_2021_09_27 Service
//!
//! Auto-generated service module for voice_id_2021_09_27

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for voice_id_2021_09_27
pub struct Voice_id_2021_09_27Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Voice_id_2021_09_27Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get fraudster resource handler
    pub fn fraudster(&self) -> resources::Fraudster<'_> {
        resources::Fraudster::new(self.provider)
    }
    /// Get watchlist resource handler
    pub fn watchlist(&self) -> resources::Watchlist<'_> {
        resources::Watchlist::new(self.provider)
    }
    /// Get speaker resource handler
    pub fn speaker(&self) -> resources::Speaker<'_> {
        resources::Speaker::new(self.provider)
    }
    /// Get speaker_enrollment_job resource handler
    pub fn speaker_enrollment_job(&self) -> resources::Speaker_enrollment_job<'_> {
        resources::Speaker_enrollment_job::new(self.provider)
    }
    /// Get fraudster_registration_job resource handler
    pub fn fraudster_registration_job(&self) -> resources::Fraudster_registration_job<'_> {
        resources::Fraudster_registration_job::new(self.provider)
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
