//! Codecatalyst Service
//!
//! Auto-generated service module for codecatalyst

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for codecatalyst
pub struct CodecatalystService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> CodecatalystService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get user_details resource handler
    pub fn user_details(&self) -> resources::User_details<'_> {
        resources::User_details::new(self.provider)
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
