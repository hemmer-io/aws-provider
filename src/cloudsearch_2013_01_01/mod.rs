//! Cloudsearch_2013_01_01 Service
//!
//! Auto-generated service module for cloudsearch_2013_01_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudsearch_2013_01_01
pub struct Cloudsearch_2013_01_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudsearch_2013_01_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get index_field resource handler
    pub fn index_field(&self) -> resources::Index_field<'_> {
        resources::Index_field::new(self.provider)
    }
    /// Get expression resource handler
    pub fn expression(&self) -> resources::Expression<'_> {
        resources::Expression::new(self.provider)
    }
    /// Get analysis_scheme resource handler
    pub fn analysis_scheme(&self) -> resources::Analysis_scheme<'_> {
        resources::Analysis_scheme::new(self.provider)
    }
    /// Get expressions resource handler
    pub fn expressions(&self) -> resources::Expressions<'_> {
        resources::Expressions::new(self.provider)
    }
    /// Get domain resource handler
    pub fn domain(&self) -> resources::Domain<'_> {
        resources::Domain::new(self.provider)
    }
    /// Get index_fields resource handler
    pub fn index_fields(&self) -> resources::Index_fields<'_> {
        resources::Index_fields::new(self.provider)
    }
    /// Get analysis_schemes resource handler
    pub fn analysis_schemes(&self) -> resources::Analysis_schemes<'_> {
        resources::Analysis_schemes::new(self.provider)
    }
    /// Get availability_options resource handler
    pub fn availability_options(&self) -> resources::Availability_options<'_> {
        resources::Availability_options::new(self.provider)
    }
    /// Get domains resource handler
    pub fn domains(&self) -> resources::Domains<'_> {
        resources::Domains::new(self.provider)
    }
    /// Get suggesters resource handler
    pub fn suggesters(&self) -> resources::Suggesters<'_> {
        resources::Suggesters::new(self.provider)
    }
    /// Get domain_endpoint_options resource handler
    pub fn domain_endpoint_options(&self) -> resources::Domain_endpoint_options<'_> {
        resources::Domain_endpoint_options::new(self.provider)
    }
    /// Get suggester resource handler
    pub fn suggester(&self) -> resources::Suggester<'_> {
        resources::Suggester::new(self.provider)
    }
    /// Get service_access_policies resource handler
    pub fn service_access_policies(&self) -> resources::Service_access_policies<'_> {
        resources::Service_access_policies::new(self.provider)
    }
    /// Get scaling_parameters resource handler
    pub fn scaling_parameters(&self) -> resources::Scaling_parameters<'_> {
        resources::Scaling_parameters::new(self.provider)
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
