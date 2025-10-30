//! Cloudtrail_2013_11_01 Service
//!
//! Auto-generated service module for cloudtrail_2013_11_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for cloudtrail_2013_11_01
pub struct Cloudtrail_2013_11_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Cloudtrail_2013_11_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get dashboard resource handler
    pub fn dashboard(&self) -> resources::Dashboard<'_> {
        resources::Dashboard::new(self.provider)
    }
    /// Get trail_status resource handler
    pub fn trail_status(&self) -> resources::Trail_status<'_> {
        resources::Trail_status::new(self.provider)
    }
    /// Get insight_selectors resource handler
    pub fn insight_selectors(&self) -> resources::Insight_selectors<'_> {
        resources::Insight_selectors::new(self.provider)
    }
    /// Get resource_policy resource handler
    pub fn resource_policy(&self) -> resources::Resource_policy<'_> {
        resources::Resource_policy::new(self.provider)
    }
    /// Get event_selectors resource handler
    pub fn event_selectors(&self) -> resources::Event_selectors<'_> {
        resources::Event_selectors::new(self.provider)
    }
    /// Get query resource handler
    pub fn query(&self) -> resources::Query<'_> {
        resources::Query::new(self.provider)
    }
    /// Get trail resource handler
    pub fn trail(&self) -> resources::Trail<'_> {
        resources::Trail::new(self.provider)
    }
    /// Get event_configuration resource handler
    pub fn event_configuration(&self) -> resources::Event_configuration<'_> {
        resources::Event_configuration::new(self.provider)
    }
    /// Get event_data_store resource handler
    pub fn event_data_store(&self) -> resources::Event_data_store<'_> {
        resources::Event_data_store::new(self.provider)
    }
    /// Get import resource handler
    pub fn import(&self) -> resources::Import<'_> {
        resources::Import::new(self.provider)
    }
    /// Get trails resource handler
    pub fn trails(&self) -> resources::Trails<'_> {
        resources::Trails::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
    }
    /// Get query_results resource handler
    pub fn query_results(&self) -> resources::Query_results<'_> {
        resources::Query_results::new(self.provider)
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
