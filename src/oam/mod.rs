//! Oam Service
//!
//! Auto-generated service module for oam

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for oam
pub struct OamService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> OamService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get link resource handler
    pub fn link(&self) -> resources::Link<'_> {
        resources::Link::new(self.provider)
    }
    /// Get sink resource handler
    pub fn sink(&self) -> resources::Sink<'_> {
        resources::Sink::new(self.provider)
    }
    /// Get sink_policy resource handler
    pub fn sink_policy(&self) -> resources::Sink_policy<'_> {
        resources::Sink_policy::new(self.provider)
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
