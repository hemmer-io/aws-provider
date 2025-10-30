//! Invoicing_2024_12_01 Service
//!
//! Auto-generated service module for invoicing_2024_12_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for invoicing_2024_12_01
pub struct Invoicing_2024_12_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Invoicing_2024_12_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get invoice_unit resource handler
    pub fn invoice_unit(&self) -> resources::Invoice_unit<'_> {
        resources::Invoice_unit::new(self.provider)
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
