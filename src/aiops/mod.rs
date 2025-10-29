//! Aiops Service
//!
//! Auto-generated service module for aiops

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for aiops
pub struct AiopsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> AiopsService<'a> {
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
