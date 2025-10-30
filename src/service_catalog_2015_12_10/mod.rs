//! Service_catalog_2015_12_10 Service
//!
//! Auto-generated service module for service_catalog_2015_12_10

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for service_catalog_2015_12_10
pub struct Service_catalog_2015_12_10Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Service_catalog_2015_12_10Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get copy_product_status resource handler
    pub fn copy_product_status(&self) -> resources::Copy_product_status<'_> {
        resources::Copy_product_status::new(self.provider)
    }
    /// Get provisioning_parameters resource handler
    pub fn provisioning_parameters(&self) -> resources::Provisioning_parameters<'_> {
        resources::Provisioning_parameters::new(self.provider)
    }
    /// Get product_view resource handler
    pub fn product_view(&self) -> resources::Product_view<'_> {
        resources::Product_view::new(self.provider)
    }
    /// Get constraint resource handler
    pub fn constraint(&self) -> resources::Constraint<'_> {
        resources::Constraint::new(self.provider)
    }
    /// Get portfolio_share_status resource handler
    pub fn portfolio_share_status(&self) -> resources::Portfolio_share_status<'_> {
        resources::Portfolio_share_status::new(self.provider)
    }
    /// Get portfolio_share resource handler
    pub fn portfolio_share(&self) -> resources::Portfolio_share<'_> {
        resources::Portfolio_share::new(self.provider)
    }
    /// Get provisioned_product_plan resource handler
    pub fn provisioned_product_plan(&self) -> resources::Provisioned_product_plan<'_> {
        resources::Provisioned_product_plan::new(self.provider)
    }
    /// Get tag_option resource handler
    pub fn tag_option(&self) -> resources::Tag_option<'_> {
        resources::Tag_option::new(self.provider)
    }
    /// Get product_as_admin resource handler
    pub fn product_as_admin(&self) -> resources::Product_as_admin<'_> {
        resources::Product_as_admin::new(self.provider)
    }
    /// Get portfolio_shares resource handler
    pub fn portfolio_shares(&self) -> resources::Portfolio_shares<'_> {
        resources::Portfolio_shares::new(self.provider)
    }
    /// Get provisioning_artifact resource handler
    pub fn provisioning_artifact(&self) -> resources::Provisioning_artifact<'_> {
        resources::Provisioning_artifact::new(self.provider)
    }
    /// Get provisioned_product resource handler
    pub fn provisioned_product(&self) -> resources::Provisioned_product<'_> {
        resources::Provisioned_product::new(self.provider)
    }
    /// Get aws_organizations_access_status resource handler
    pub fn aws_organizations_access_status(&self) -> resources::Aws_organizations_access_status<'_> {
        resources::Aws_organizations_access_status::new(self.provider)
    }
    /// Get provisioned_product_properties resource handler
    pub fn provisioned_product_properties(&self) -> resources::Provisioned_product_properties<'_> {
        resources::Provisioned_product_properties::new(self.provider)
    }
    /// Get service_action resource handler
    pub fn service_action(&self) -> resources::Service_action<'_> {
        resources::Service_action::new(self.provider)
    }
    /// Get portfolio resource handler
    pub fn portfolio(&self) -> resources::Portfolio<'_> {
        resources::Portfolio::new(self.provider)
    }
    /// Get product resource handler
    pub fn product(&self) -> resources::Product<'_> {
        resources::Product::new(self.provider)
    }
    /// Get provisioned_product_outputs resource handler
    pub fn provisioned_product_outputs(&self) -> resources::Provisioned_product_outputs<'_> {
        resources::Provisioned_product_outputs::new(self.provider)
    }
    /// Get service_action_execution_parameters resource handler
    pub fn service_action_execution_parameters(&self) -> resources::Service_action_execution_parameters<'_> {
        resources::Service_action_execution_parameters::new(self.provider)
    }
    /// Get record resource handler
    pub fn record(&self) -> resources::Record<'_> {
        resources::Record::new(self.provider)
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
