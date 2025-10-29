//! Account Service
//!
//! Auto-generated service module for account

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for account
pub struct AccountService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> AccountService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
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
