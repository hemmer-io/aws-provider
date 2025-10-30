//! Identitystore_2020_06_15 Service
//!
//! Auto-generated service module for identitystore_2020_06_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for identitystore_2020_06_15
pub struct Identitystore_2020_06_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Identitystore_2020_06_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get user_id resource handler
    pub fn user_id(&self) -> resources::User_id<'_> {
        resources::User_id::new(self.provider)
    }
    /// Get group_membership_id resource handler
    pub fn group_membership_id(&self) -> resources::Group_membership_id<'_> {
        resources::Group_membership_id::new(self.provider)
    }
    /// Get group_id resource handler
    pub fn group_id(&self) -> resources::Group_id<'_> {
        resources::Group_id::new(self.provider)
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
