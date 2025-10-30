//! Ssm_contacts_2021_05_03 Service
//!
//! Auto-generated service module for ssm_contacts_2021_05_03

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for ssm_contacts_2021_05_03
pub struct Ssm_contacts_2021_05_03Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Ssm_contacts_2021_05_03Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get contact_policy resource handler
    pub fn contact_policy(&self) -> resources::Contact_policy<'_> {
        resources::Contact_policy::new(self.provider)
    }
    /// Get rotation resource handler
    pub fn rotation(&self) -> resources::Rotation<'_> {
        resources::Rotation::new(self.provider)
    }
    /// Get engagement resource handler
    pub fn engagement(&self) -> resources::Engagement<'_> {
        resources::Engagement::new(self.provider)
    }
    /// Get contact resource handler
    pub fn contact(&self) -> resources::Contact<'_> {
        resources::Contact::new(self.provider)
    }
    /// Get rotation_override resource handler
    pub fn rotation_override(&self) -> resources::Rotation_override<'_> {
        resources::Rotation_override::new(self.provider)
    }
    /// Get page resource handler
    pub fn page(&self) -> resources::Page<'_> {
        resources::Page::new(self.provider)
    }
    /// Get contact_channel resource handler
    pub fn contact_channel(&self) -> resources::Contact_channel<'_> {
        resources::Contact_channel::new(self.provider)
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
