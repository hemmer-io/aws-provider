//! Account_2021_02_01 Service
//!
//! Auto-generated service module for account_2021_02_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for account_2021_02_01
pub struct Account_2021_02_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Account_2021_02_01Service<'a> {
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
