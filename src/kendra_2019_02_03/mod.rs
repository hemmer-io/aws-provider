//! Kendra_2019_02_03 Service
//!
//! Auto-generated service module for kendra_2019_02_03

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for kendra_2019_02_03
pub struct Kendra_2019_02_03Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Kendra_2019_02_03Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get principal_mapping resource handler
    pub fn principal_mapping(&self) -> resources::Principal_mapping<'_> {
        resources::Principal_mapping::new(self.provider)
    }
    /// Get index resource handler
    pub fn index(&self) -> resources::Index<'_> {
        resources::Index::new(self.provider)
    }
    /// Get query_suggestions_block_list resource handler
    pub fn query_suggestions_block_list(&self) -> resources::Query_suggestions_block_list<'_> {
        resources::Query_suggestions_block_list::new(self.provider)
    }
    /// Get featured_results_set resource handler
    pub fn featured_results_set(&self) -> resources::Featured_results_set<'_> {
        resources::Featured_results_set::new(self.provider)
    }
    /// Get access_control_configuration resource handler
    pub fn access_control_configuration(&self) -> resources::Access_control_configuration<'_> {
        resources::Access_control_configuration::new(self.provider)
    }
    /// Get query_suggestions_config resource handler
    pub fn query_suggestions_config(&self) -> resources::Query_suggestions_config<'_> {
        resources::Query_suggestions_config::new(self.provider)
    }
    /// Get data_source resource handler
    pub fn data_source(&self) -> resources::Data_source<'_> {
        resources::Data_source::new(self.provider)
    }
    /// Get thesaurus resource handler
    pub fn thesaurus(&self) -> resources::Thesaurus<'_> {
        resources::Thesaurus::new(self.provider)
    }
    /// Get experience resource handler
    pub fn experience(&self) -> resources::Experience<'_> {
        resources::Experience::new(self.provider)
    }
    /// Get query_suggestions resource handler
    pub fn query_suggestions(&self) -> resources::Query_suggestions<'_> {
        resources::Query_suggestions::new(self.provider)
    }
    /// Get faq resource handler
    pub fn faq(&self) -> resources::Faq<'_> {
        resources::Faq::new(self.provider)
    }
    /// Get snapshots resource handler
    pub fn snapshots(&self) -> resources::Snapshots<'_> {
        resources::Snapshots::new(self.provider)
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
