//! Support_2013_04_15 Service
//!
//! Auto-generated service module for support_2013_04_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for support_2013_04_15
pub struct Support_2013_04_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Support_2013_04_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get trusted_advisor_check_refresh_statuses resource handler
    pub fn trusted_advisor_check_refresh_statuses(&self) -> resources::Trusted_advisor_check_refresh_statuses<'_> {
        resources::Trusted_advisor_check_refresh_statuses::new(self.provider)
    }
    /// Get trusted_advisor_check_summaries resource handler
    pub fn trusted_advisor_check_summaries(&self) -> resources::Trusted_advisor_check_summaries<'_> {
        resources::Trusted_advisor_check_summaries::new(self.provider)
    }
    /// Get case resource handler
    pub fn case(&self) -> resources::Case<'_> {
        resources::Case::new(self.provider)
    }
    /// Get trusted_advisor_check_result resource handler
    pub fn trusted_advisor_check_result(&self) -> resources::Trusted_advisor_check_result<'_> {
        resources::Trusted_advisor_check_result::new(self.provider)
    }
    /// Get cases resource handler
    pub fn cases(&self) -> resources::Cases<'_> {
        resources::Cases::new(self.provider)
    }
    /// Get communications resource handler
    pub fn communications(&self) -> resources::Communications<'_> {
        resources::Communications::new(self.provider)
    }
    /// Get supported_languages resource handler
    pub fn supported_languages(&self) -> resources::Supported_languages<'_> {
        resources::Supported_languages::new(self.provider)
    }
    /// Get severity_levels resource handler
    pub fn severity_levels(&self) -> resources::Severity_levels<'_> {
        resources::Severity_levels::new(self.provider)
    }
    /// Get trusted_advisor_checks resource handler
    pub fn trusted_advisor_checks(&self) -> resources::Trusted_advisor_checks<'_> {
        resources::Trusted_advisor_checks::new(self.provider)
    }
    /// Get services resource handler
    pub fn services(&self) -> resources::Services<'_> {
        resources::Services::new(self.provider)
    }
    /// Get create_case_options resource handler
    pub fn create_case_options(&self) -> resources::Create_case_options<'_> {
        resources::Create_case_options::new(self.provider)
    }
    /// Get attachment resource handler
    pub fn attachment(&self) -> resources::Attachment<'_> {
        resources::Attachment::new(self.provider)
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
