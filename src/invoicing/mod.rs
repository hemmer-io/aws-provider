//! Invoicing Service
//!
//! Auto-generated service module for invoicing

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for invoicing
pub struct InvoicingService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> InvoicingService<'a> {
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
