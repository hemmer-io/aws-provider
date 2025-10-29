//! Identitystore Service
//!
//! Auto-generated service module for identitystore

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for identitystore
pub struct IdentitystoreService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> IdentitystoreService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get group_membership_id resource handler
    pub fn group_membership_id(&self) -> resources::Group_membership_id<'_> {
        resources::Group_membership_id::new(self.provider)
    }
    /// Get group_id resource handler
    pub fn group_id(&self) -> resources::Group_id<'_> {
        resources::Group_id::new(self.provider)
    }
    /// Get user_id resource handler
    pub fn user_id(&self) -> resources::User_id<'_> {
        resources::User_id::new(self.provider)
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
