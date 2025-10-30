//! Oam_2022_06_10 Service
//!
//! Auto-generated service module for oam_2022_06_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for oam_2022_06_10
pub struct Oam_2022_06_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Oam_2022_06_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get link resource handler
    pub fn link(&self) -> resources::Link<'_> {
        resources::Link::new(self.provider)
    }
    /// Get sink_policy resource handler
    pub fn sink_policy(&self) -> resources::Sink_policy<'_> {
        resources::Sink_policy::new(self.provider)
    }
    /// Get sink resource handler
    pub fn sink(&self) -> resources::Sink<'_> {
        resources::Sink::new(self.provider)
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
