//! Outposts Service
//!
//! Auto-generated service module for outposts

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for outposts
pub struct OutpostsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> OutpostsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get capacity_task resource handler
    pub fn capacity_task(&self) -> resources::Capacity_task<'_> {
        resources::Capacity_task::new(self.provider)
    }
    /// Get outpost resource handler
    pub fn outpost(&self) -> resources::Outpost<'_> {
        resources::Outpost::new(self.provider)
    }
    /// Get order resource handler
    pub fn order(&self) -> resources::Order<'_> {
        resources::Order::new(self.provider)
    }
    /// Get outpost_billing_information resource handler
    pub fn outpost_billing_information(&self) -> resources::Outpost_billing_information<'_> {
        resources::Outpost_billing_information::new(self.provider)
    }
    /// Get outpost_instance_types resource handler
    pub fn outpost_instance_types(&self) -> resources::Outpost_instance_types<'_> {
        resources::Outpost_instance_types::new(self.provider)
    }
    /// Get connection resource handler
    pub fn connection(&self) -> resources::Connection<'_> {
        resources::Connection::new(self.provider)
    }
    /// Get site_rack_physical_properties resource handler
    pub fn site_rack_physical_properties(&self) -> resources::Site_rack_physical_properties<'_> {
        resources::Site_rack_physical_properties::new(self.provider)
    }
    /// Get site resource handler
    pub fn site(&self) -> resources::Site<'_> {
        resources::Site::new(self.provider)
    }
    /// Get outpost_supported_instance_types resource handler
    pub fn outpost_supported_instance_types(&self) -> resources::Outpost_supported_instance_types<'_> {
        resources::Outpost_supported_instance_types::new(self.provider)
    }
    /// Get site_address resource handler
    pub fn site_address(&self) -> resources::Site_address<'_> {
        resources::Site_address::new(self.provider)
    }
    /// Get catalog_item resource handler
    pub fn catalog_item(&self) -> resources::Catalog_item<'_> {
        resources::Catalog_item::new(self.provider)
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
