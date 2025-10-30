//! Lex_runtime Service
//!
//! Auto-generated service module for lex_runtime

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for lex_runtime
pub struct Lex_runtimeService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Lex_runtimeService<'a> {
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
