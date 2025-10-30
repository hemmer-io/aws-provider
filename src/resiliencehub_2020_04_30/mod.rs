//! Resiliencehub_2020_04_30 Service
//!
//! Auto-generated service module for resiliencehub_2020_04_30

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for resiliencehub_2020_04_30
pub struct Resiliencehub_2020_04_30Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Resiliencehub_2020_04_30Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get metrics_export resource handler
    pub fn metrics_export(&self) -> resources::Metrics_export<'_> {
        resources::Metrics_export::new(self.provider)
    }
    /// Get app_version resource handler
    pub fn app_version(&self) -> resources::App_version<'_> {
        resources::App_version::new(self.provider)
    }
    /// Get draft_app_version_resources_import_status resource handler
    pub fn draft_app_version_resources_import_status(&self) -> resources::Draft_app_version_resources_import_status<'_> {
        resources::Draft_app_version_resources_import_status::new(self.provider)
    }
    /// Get app_version_app_component resource handler
    pub fn app_version_app_component(&self) -> resources::App_version_app_component<'_> {
        resources::App_version_app_component::new(self.provider)
    }
    /// Get app_version_template resource handler
    pub fn app_version_template(&self) -> resources::App_version_template<'_> {
        resources::App_version_template::new(self.provider)
    }
    /// Get resiliency_policy resource handler
    pub fn resiliency_policy(&self) -> resources::Resiliency_policy<'_> {
        resources::Resiliency_policy::new(self.provider)
    }
    /// Get recommendation_template resource handler
    pub fn recommendation_template(&self) -> resources::Recommendation_template<'_> {
        resources::Recommendation_template::new(self.provider)
    }
    /// Get app_assessment resource handler
    pub fn app_assessment(&self) -> resources::App_assessment<'_> {
        resources::App_assessment::new(self.provider)
    }
    /// Get app_version_resources_resolution_status resource handler
    pub fn app_version_resources_resolution_status(&self) -> resources::App_version_resources_resolution_status<'_> {
        resources::App_version_resources_resolution_status::new(self.provider)
    }
    /// Get app_input_source resource handler
    pub fn app_input_source(&self) -> resources::App_input_source<'_> {
        resources::App_input_source::new(self.provider)
    }
    /// Get app_version_resource resource handler
    pub fn app_version_resource(&self) -> resources::App_version_resource<'_> {
        resources::App_version_resource::new(self.provider)
    }
    /// Get draft_app_version_template resource handler
    pub fn draft_app_version_template(&self) -> resources::Draft_app_version_template<'_> {
        resources::Draft_app_version_template::new(self.provider)
    }
    /// Get resource_grouping_recommendation_task resource handler
    pub fn resource_grouping_recommendation_task(&self) -> resources::Resource_grouping_recommendation_task<'_> {
        resources::Resource_grouping_recommendation_task::new(self.provider)
    }
    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
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
