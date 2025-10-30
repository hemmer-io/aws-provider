//! Eks_auth_2023_11_26 Service
//!
//! Auto-generated service module for eks_auth_2023_11_26

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for eks_auth_2023_11_26
pub struct Eks_auth_2023_11_26Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Eks_auth_2023_11_26Service<'a> {
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
