//! Cognito_identity_2014_06_30 Service
//!
//! Auto-generated service module for cognito_identity_2014_06_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cognito_identity_2014_06_30
pub struct Cognito_identity_2014_06_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cognito_identity_2014_06_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get identity_pool resource handler
    pub fn identity_pool(&self) -> resources::Identity_pool<'_> {
        resources::Identity_pool::new(self.provider)
    }
    /// Get identity resource handler
    pub fn identity(&self) -> resources::Identity<'_> {
        resources::Identity::new(self.provider)
    }
    /// Get principal_tag_attribute_map resource handler
    pub fn principal_tag_attribute_map(&self) -> resources::Principal_tag_attribute_map<'_> {
        resources::Principal_tag_attribute_map::new(self.provider)
    }
    /// Get credentials_for_identity resource handler
    pub fn credentials_for_identity(&self) -> resources::Credentials_for_identity<'_> {
        resources::Credentials_for_identity::new(self.provider)
    }
    /// Get open_id_token_for_developer_identity resource handler
    pub fn open_id_token_for_developer_identity(&self) -> resources::Open_id_token_for_developer_identity<'_> {
        resources::Open_id_token_for_developer_identity::new(self.provider)
    }
    /// Get id resource handler
    pub fn id(&self) -> resources::Id<'_> {
        resources::Id::new(self.provider)
    }
    /// Get identity_pool_roles resource handler
    pub fn identity_pool_roles(&self) -> resources::Identity_pool_roles<'_> {
        resources::Identity_pool_roles::new(self.provider)
    }
    /// Get open_id_token resource handler
    pub fn open_id_token(&self) -> resources::Open_id_token<'_> {
        resources::Open_id_token::new(self.provider)
    }
    /// Get identities resource handler
    pub fn identities(&self) -> resources::Identities<'_> {
        resources::Identities::new(self.provider)
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
