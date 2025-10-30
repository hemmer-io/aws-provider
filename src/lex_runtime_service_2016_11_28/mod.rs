//! Lex_runtime_service_2016_11_28 Service
//!
//! Auto-generated service module for lex_runtime_service_2016_11_28

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for lex_runtime_service_2016_11_28
pub struct Lex_runtime_service_2016_11_28Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Lex_runtime_service_2016_11_28Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get session resource handler
    pub fn session(&self) -> resources::Session<'_> {
        resources::Session::new(self.provider)
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
