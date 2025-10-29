//! Clouddirectory Service
//!
//! Auto-generated service module for clouddirectory

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for clouddirectory
pub struct ClouddirectoryService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ClouddirectoryService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get object resource handler
    pub fn object(&self) -> resources::Object<'_> {
        resources::Object::new(self.provider)
    }
    /// Get schema resource handler
    pub fn schema(&self) -> resources::Schema<'_> {
        resources::Schema::new(self.provider)
    }
    /// Get typed_link_facet_information resource handler
    pub fn typed_link_facet_information(&self) -> resources::Typed_link_facet_information<'_> {
        resources::Typed_link_facet_information::new(self.provider)
    }
    /// Get object_attributes resource handler
    pub fn object_attributes(&self) -> resources::Object_attributes<'_> {
        resources::Object_attributes::new(self.provider)
    }
    /// Get index resource handler
    pub fn index(&self) -> resources::Index<'_> {
        resources::Index::new(self.provider)
    }
    /// Get link_attributes resource handler
    pub fn link_attributes(&self) -> resources::Link_attributes<'_> {
        resources::Link_attributes::new(self.provider)
    }
    /// Get facet resource handler
    pub fn facet(&self) -> resources::Facet<'_> {
        resources::Facet::new(self.provider)
    }
    /// Get schema_as_json resource handler
    pub fn schema_as_json(&self) -> resources::Schema_as_json<'_> {
        resources::Schema_as_json::new(self.provider)
    }
    /// Get object_information resource handler
    pub fn object_information(&self) -> resources::Object_information<'_> {
        resources::Object_information::new(self.provider)
    }
    /// Get schema_from_json resource handler
    pub fn schema_from_json(&self) -> resources::Schema_from_json<'_> {
        resources::Schema_from_json::new(self.provider)
    }
    /// Get typed_link_facet resource handler
    pub fn typed_link_facet(&self) -> resources::Typed_link_facet<'_> {
        resources::Typed_link_facet::new(self.provider)
    }
    /// Get directory resource handler
    pub fn directory(&self) -> resources::Directory<'_> {
        resources::Directory::new(self.provider)
    }
    /// Get applied_schema_version resource handler
    pub fn applied_schema_version(&self) -> resources::Applied_schema_version<'_> {
        resources::Applied_schema_version::new(self.provider)
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
