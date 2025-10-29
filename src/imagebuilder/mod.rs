//! Imagebuilder Service
//!
//! Auto-generated service module for imagebuilder

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for imagebuilder
pub struct ImagebuilderService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> ImagebuilderService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get component resource handler
    pub fn component(&self) -> resources::Component<'_> {
        resources::Component::new(self.provider)
    }
    /// Get image_recipe_policy resource handler
    pub fn image_recipe_policy(&self) -> resources::Image_recipe_policy<'_> {
        resources::Image_recipe_policy::new(self.provider)
    }
    /// Get lifecycle_execution resource handler
    pub fn lifecycle_execution(&self) -> resources::Lifecycle_execution<'_> {
        resources::Lifecycle_execution::new(self.provider)
    }
    /// Get component_policy resource handler
    pub fn component_policy(&self) -> resources::Component_policy<'_> {
        resources::Component_policy::new(self.provider)
    }
    /// Get infrastructure_configuration resource handler
    pub fn infrastructure_configuration(&self) -> resources::Infrastructure_configuration<'_> {
        resources::Infrastructure_configuration::new(self.provider)
    }
    /// Get image_recipe resource handler
    pub fn image_recipe(&self) -> resources::Image_recipe<'_> {
        resources::Image_recipe::new(self.provider)
    }
    /// Get image_policy resource handler
    pub fn image_policy(&self) -> resources::Image_policy<'_> {
        resources::Image_policy::new(self.provider)
    }
    /// Get image_pipeline resource handler
    pub fn image_pipeline(&self) -> resources::Image_pipeline<'_> {
        resources::Image_pipeline::new(self.provider)
    }
    /// Get lifecycle_policy resource handler
    pub fn lifecycle_policy(&self) -> resources::Lifecycle_policy<'_> {
        resources::Lifecycle_policy::new(self.provider)
    }
    /// Get workflow_execution resource handler
    pub fn workflow_execution(&self) -> resources::Workflow_execution<'_> {
        resources::Workflow_execution::new(self.provider)
    }
    /// Get container_recipe resource handler
    pub fn container_recipe(&self) -> resources::Container_recipe<'_> {
        resources::Container_recipe::new(self.provider)
    }
    /// Get image resource handler
    pub fn image(&self) -> resources::Image<'_> {
        resources::Image::new(self.provider)
    }
    /// Get workflow_step_execution resource handler
    pub fn workflow_step_execution(&self) -> resources::Workflow_step_execution<'_> {
        resources::Workflow_step_execution::new(self.provider)
    }
    /// Get marketplace_resource resource handler
    pub fn marketplace_resource(&self) -> resources::Marketplace_resource<'_> {
        resources::Marketplace_resource::new(self.provider)
    }
    /// Get workflow resource handler
    pub fn workflow(&self) -> resources::Workflow<'_> {
        resources::Workflow::new(self.provider)
    }
    /// Get distribution_configuration resource handler
    pub fn distribution_configuration(&self) -> resources::Distribution_configuration<'_> {
        resources::Distribution_configuration::new(self.provider)
    }
    /// Get container_recipe_policy resource handler
    pub fn container_recipe_policy(&self) -> resources::Container_recipe_policy<'_> {
        resources::Container_recipe_policy::new(self.provider)
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
