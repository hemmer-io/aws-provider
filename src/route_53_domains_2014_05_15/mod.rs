//! Route_53_domains_2014_05_15 Service
//!
//! Auto-generated service module for route_53_domains_2014_05_15

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for route_53_domains_2014_05_15
pub struct Route_53_domains_2014_05_15Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Route_53_domains_2014_05_15Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get contact_reachability_status resource handler
    pub fn contact_reachability_status(&self) -> resources::Contact_reachability_status<'_> {
        resources::Contact_reachability_status::new(self.provider)
    }
    /// Get domain_suggestions resource handler
    pub fn domain_suggestions(&self) -> resources::Domain_suggestions<'_> {
        resources::Domain_suggestions::new(self.provider)
    }
    /// Get domain_contact resource handler
    pub fn domain_contact(&self) -> resources::Domain_contact<'_> {
        resources::Domain_contact::new(self.provider)
    }
    /// Get operation_detail resource handler
    pub fn operation_detail(&self) -> resources::Operation_detail<'_> {
        resources::Operation_detail::new(self.provider)
    }
    /// Get domain_nameservers resource handler
    pub fn domain_nameservers(&self) -> resources::Domain_nameservers<'_> {
        resources::Domain_nameservers::new(self.provider)
    }
    /// Get domain_detail resource handler
    pub fn domain_detail(&self) -> resources::Domain_detail<'_> {
        resources::Domain_detail::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
    }
    /// Get tags_for_domain resource handler
    pub fn tags_for_domain(&self) -> resources::Tags_for_domain<'_> {
        resources::Tags_for_domain::new(self.provider)
    }
    /// Get domain_contact_privacy resource handler
    pub fn domain_contact_privacy(&self) -> resources::Domain_contact_privacy<'_> {
        resources::Domain_contact_privacy::new(self.provider)
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
