//! Taxsettings Service
//!
//! Auto-generated service module for taxsettings

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for taxsettings
pub struct TaxsettingsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> TaxsettingsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get tax_registration resource handler
    pub fn tax_registration(&self) -> resources::Tax_registration<'_> {
        resources::Tax_registration::new(self.provider)
    }
    /// Get supplemental_tax_registration resource handler
    pub fn supplemental_tax_registration(&self) -> resources::Supplemental_tax_registration<'_> {
        resources::Supplemental_tax_registration::new(self.provider)
    }
    /// Get tax_exemption_types resource handler
    pub fn tax_exemption_types(&self) -> resources::Tax_exemption_types<'_> {
        resources::Tax_exemption_types::new(self.provider)
    }
    /// Get tax_inheritance resource handler
    pub fn tax_inheritance(&self) -> resources::Tax_inheritance<'_> {
        resources::Tax_inheritance::new(self.provider)
    }
    /// Get tax_registration_document resource handler
    pub fn tax_registration_document(&self) -> resources::Tax_registration_document<'_> {
        resources::Tax_registration_document::new(self.provider)
    }
    /// Get tax_exemption resource handler
    pub fn tax_exemption(&self) -> resources::Tax_exemption<'_> {
        resources::Tax_exemption::new(self.provider)
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
