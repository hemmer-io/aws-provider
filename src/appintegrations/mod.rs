//! Appintegrations Service
//!
//! Auto-generated service module for appintegrations

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appintegrations
pub struct AppintegrationsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> AppintegrationsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get data_integration_association resource handler
    pub fn data_integration_association(&self) -> resources::Data_integration_association<'_> {
        resources::Data_integration_association::new(self.provider)
    }
    /// Get event_integration resource handler
    pub fn event_integration(&self) -> resources::Event_integration<'_> {
        resources::Event_integration::new(self.provider)
    }
    /// Get data_integration resource handler
    pub fn data_integration(&self) -> resources::Data_integration<'_> {
        resources::Data_integration::new(self.provider)
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
