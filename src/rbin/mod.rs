//! Rbin Service
//!
//! Auto-generated service module for rbin

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for rbin
pub struct RbinService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> RbinService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get rule resource handler
    pub fn rule(&self) -> resources::Rule<'_> {
        resources::Rule::new(self.provider)
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
