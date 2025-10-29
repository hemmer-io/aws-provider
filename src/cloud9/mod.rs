//! Cloud9 Service
//!
//! Auto-generated service module for cloud9

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloud9
pub struct Cloud9Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloud9Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get environment_ec2 resource handler
    pub fn environment_ec2(&self) -> resources::Environment_ec2<'_> {
        resources::Environment_ec2::new(self.provider)
    }
    /// Get environment resource handler
    pub fn environment(&self) -> resources::Environment<'_> {
        resources::Environment::new(self.provider)
    }
    /// Get environment_status resource handler
    pub fn environment_status(&self) -> resources::Environment_status<'_> {
        resources::Environment_status::new(self.provider)
    }
    /// Get environment_memberships resource handler
    pub fn environment_memberships(&self) -> resources::Environment_memberships<'_> {
        resources::Environment_memberships::new(self.provider)
    }
    /// Get environment_membership resource handler
    pub fn environment_membership(&self) -> resources::Environment_membership<'_> {
        resources::Environment_membership::new(self.provider)
    }
    /// Get environments resource handler
    pub fn environments(&self) -> resources::Environments<'_> {
        resources::Environments::new(self.provider)
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
