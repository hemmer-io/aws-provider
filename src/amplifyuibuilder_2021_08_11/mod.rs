//! Amplifyuibuilder_2021_08_11 Service
//!
//! Auto-generated service module for amplifyuibuilder_2021_08_11

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for amplifyuibuilder_2021_08_11
pub struct Amplifyuibuilder_2021_08_11Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Amplifyuibuilder_2021_08_11Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get metadata_flag resource handler
    pub fn metadata_flag(&self) -> resources::Metadata_flag<'_> {
        resources::Metadata_flag::new(self.provider)
    }
    /// Get metadata resource handler
    pub fn metadata(&self) -> resources::Metadata<'_> {
        resources::Metadata::new(self.provider)
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
