//! Bedrock_2023_04_20 Service
//!
//! Auto-generated service module for bedrock_2023_04_20

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for bedrock_2023_04_20
pub struct Bedrock_2023_04_20Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Bedrock_2023_04_20Service<'a> {
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
