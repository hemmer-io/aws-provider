//! Acm_pca_2017_08_22 Service
//!
//! Auto-generated service module for acm_pca_2017_08_22

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for acm_pca_2017_08_22
pub struct Acm_pca_2017_08_22Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Acm_pca_2017_08_22Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get certificate_authority_csr resource handler
    pub fn certificate_authority_csr(&self) -> resources::Certificate_authority_csr<'_> {
        resources::Certificate_authority_csr::new(self.provider)
    }
    /// Get certificate_authority_audit_report resource handler
    pub fn certificate_authority_audit_report(&self) -> resources::Certificate_authority_audit_report<'_> {
        resources::Certificate_authority_audit_report::new(self.provider)
    }
    /// Get certificate_authority_certificate resource handler
    pub fn certificate_authority_certificate(&self) -> resources::Certificate_authority_certificate<'_> {
        resources::Certificate_authority_certificate::new(self.provider)
    }
    /// Get permission resource handler
    pub fn permission(&self) -> resources::Permission<'_> {
        resources::Permission::new(self.provider)
    }
    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
    }
    /// Get certificate resource handler
    pub fn certificate(&self) -> resources::Certificate<'_> {
        resources::Certificate::new(self.provider)
    }
    /// Get certificate_authority resource handler
    pub fn certificate_authority(&self) -> resources::Certificate_authority<'_> {
        resources::Certificate_authority::new(self.provider)
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
