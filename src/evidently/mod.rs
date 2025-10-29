//! Evidently Service
//!
//! Auto-generated service module for evidently

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for evidently
pub struct EvidentlyService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> EvidentlyService<'a> {
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
