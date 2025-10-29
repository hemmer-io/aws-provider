//! Appstream Service
//!
//! Auto-generated service module for appstream

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appstream
pub struct AppstreamService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> AppstreamService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get software_associations resource handler
    pub fn software_associations(&self) -> resources::Software_associations<'_> {
        resources::Software_associations::new(self.provider)
    }
    /// Get directory_configs resource handler
    pub fn directory_configs(&self) -> resources::Directory_configs<'_> {
        resources::Directory_configs::new(self.provider)
    }
    /// Get fleets resource handler
    pub fn fleets(&self) -> resources::Fleets<'_> {
        resources::Fleets::new(self.provider)
    }
    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
    }
    /// Get user resource handler
    pub fn user(&self) -> resources::User<'_> {
        resources::User::new(self.provider)
    }
    /// Get app_blocks resource handler
    pub fn app_blocks(&self) -> resources::App_blocks<'_> {
        resources::App_blocks::new(self.provider)
    }
    /// Get usage_report_subscriptions resource handler
    pub fn usage_report_subscriptions(&self) -> resources::Usage_report_subscriptions<'_> {
        resources::Usage_report_subscriptions::new(self.provider)
    }
    /// Get app_block_builder_app_block_associations resource handler
    pub fn app_block_builder_app_block_associations(&self) -> resources::App_block_builder_app_block_associations<'_> {
        resources::App_block_builder_app_block_associations::new(self.provider)
    }
    /// Get streaming_url resource handler
    pub fn streaming_url(&self) -> resources::Streaming_url<'_> {
        resources::Streaming_url::new(self.provider)
    }
    /// Get user_stack_associations resource handler
    pub fn user_stack_associations(&self) -> resources::User_stack_associations<'_> {
        resources::User_stack_associations::new(self.provider)
    }
    /// Get image_builder_streaming_url resource handler
    pub fn image_builder_streaming_url(&self) -> resources::Image_builder_streaming_url<'_> {
        resources::Image_builder_streaming_url::new(self.provider)
    }
    /// Get directory_config resource handler
    pub fn directory_config(&self) -> resources::Directory_config<'_> {
        resources::Directory_config::new(self.provider)
    }
    /// Get application resource handler
    pub fn application(&self) -> resources::Application<'_> {
        resources::Application::new(self.provider)
    }
    /// Get applications resource handler
    pub fn applications(&self) -> resources::Applications<'_> {
        resources::Applications::new(self.provider)
    }
    /// Get image_builders resource handler
    pub fn image_builders(&self) -> resources::Image_builders<'_> {
        resources::Image_builders::new(self.provider)
    }
    /// Get app_block_builders resource handler
    pub fn app_block_builders(&self) -> resources::App_block_builders<'_> {
        resources::App_block_builders::new(self.provider)
    }
    /// Get images resource handler
    pub fn images(&self) -> resources::Images<'_> {
        resources::Images::new(self.provider)
    }
    /// Get app_license_usage resource handler
    pub fn app_license_usage(&self) -> resources::App_license_usage<'_> {
        resources::App_license_usage::new(self.provider)
    }
    /// Get app_block resource handler
    pub fn app_block(&self) -> resources::App_block<'_> {
        resources::App_block::new(self.provider)
    }
    /// Get fleet resource handler
    pub fn fleet(&self) -> resources::Fleet<'_> {
        resources::Fleet::new(self.provider)
    }
    /// Get stack resource handler
    pub fn stack(&self) -> resources::Stack<'_> {
        resources::Stack::new(self.provider)
    }
    /// Get usage_report_subscription resource handler
    pub fn usage_report_subscription(&self) -> resources::Usage_report_subscription<'_> {
        resources::Usage_report_subscription::new(self.provider)
    }
    /// Get stacks resource handler
    pub fn stacks(&self) -> resources::Stacks<'_> {
        resources::Stacks::new(self.provider)
    }
    /// Get image_permissions resource handler
    pub fn image_permissions(&self) -> resources::Image_permissions<'_> {
        resources::Image_permissions::new(self.provider)
    }
    /// Get entitlements resource handler
    pub fn entitlements(&self) -> resources::Entitlements<'_> {
        resources::Entitlements::new(self.provider)
    }
    /// Get app_block_builder_streaming_url resource handler
    pub fn app_block_builder_streaming_url(&self) -> resources::App_block_builder_streaming_url<'_> {
        resources::App_block_builder_streaming_url::new(self.provider)
    }
    /// Get entitlement resource handler
    pub fn entitlement(&self) -> resources::Entitlement<'_> {
        resources::Entitlement::new(self.provider)
    }
    /// Get updated_image resource handler
    pub fn updated_image(&self) -> resources::Updated_image<'_> {
        resources::Updated_image::new(self.provider)
    }
    /// Get application_fleet_associations resource handler
    pub fn application_fleet_associations(&self) -> resources::Application_fleet_associations<'_> {
        resources::Application_fleet_associations::new(self.provider)
    }
    /// Get users resource handler
    pub fn users(&self) -> resources::Users<'_> {
        resources::Users::new(self.provider)
    }
    /// Get image_builder resource handler
    pub fn image_builder(&self) -> resources::Image_builder<'_> {
        resources::Image_builder::new(self.provider)
    }
    /// Get sessions resource handler
    pub fn sessions(&self) -> resources::Sessions<'_> {
        resources::Sessions::new(self.provider)
    }
    /// Get theme_for_stack resource handler
    pub fn theme_for_stack(&self) -> resources::Theme_for_stack<'_> {
        resources::Theme_for_stack::new(self.provider)
    }
    /// Get app_block_builder resource handler
    pub fn app_block_builder(&self) -> resources::App_block_builder<'_> {
        resources::App_block_builder::new(self.provider)
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
