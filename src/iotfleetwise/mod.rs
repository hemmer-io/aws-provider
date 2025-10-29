//! Iotfleetwise Service
//!
//! Auto-generated service module for iotfleetwise

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for iotfleetwise
pub struct IotfleetwiseService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> IotfleetwiseService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get logging_options resource handler
    pub fn logging_options(&self) -> resources::Logging_options<'_> {
        resources::Logging_options::new(self.provider)
    }
    /// Get vehicle_status resource handler
    pub fn vehicle_status(&self) -> resources::Vehicle_status<'_> {
        resources::Vehicle_status::new(self.provider)
    }
    /// Get register_account_status resource handler
    pub fn register_account_status(&self) -> resources::Register_account_status<'_> {
        resources::Register_account_status::new(self.provider)
    }
    /// Get encryption_configuration resource handler
    pub fn encryption_configuration(&self) -> resources::Encryption_configuration<'_> {
        resources::Encryption_configuration::new(self.provider)
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
