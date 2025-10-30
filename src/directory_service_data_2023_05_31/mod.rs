//! Directory_service_data_2023_05_31 Service
//!
//! Auto-generated service module for directory_service_data_2023_05_31

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for directory_service_data_2023_05_31
pub struct Directory_service_data_2023_05_31Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Directory_service_data_2023_05_31Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get group resource handler
    pub fn group(&self) -> resources::Group<'_> {
        resources::Group::new(self.provider)
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
