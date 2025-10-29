//! Pca Service
//!
//! Auto-generated service module for pca

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for pca
pub struct PcaService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> PcaService<'a> {
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
