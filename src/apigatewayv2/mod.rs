//! Apigatewayv2 Service
//!
//! Auto-generated service module for apigatewayv2

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apigatewayv2
pub struct Apigatewayv2Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Apigatewayv2Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get routes resource handler
    pub fn routes(&self) -> resources::Routes<'_> {
        resources::Routes::new(self.provider)
    }
    /// Get routing_rule resource handler
    pub fn routing_rule(&self) -> resources::Routing_rule<'_> {
        resources::Routing_rule::new(self.provider)
    }
    /// Get api resource handler
    pub fn api(&self) -> resources::Api<'_> {
        resources::Api::new(self.provider)
    }
    /// Get models resource handler
    pub fn models(&self) -> resources::Models<'_> {
        resources::Models::new(self.provider)
    }
    /// Get integration resource handler
    pub fn integration(&self) -> resources::Integration<'_> {
        resources::Integration::new(self.provider)
    }
    /// Get deployments resource handler
    pub fn deployments(&self) -> resources::Deployments<'_> {
        resources::Deployments::new(self.provider)
    }
    /// Get model_template resource handler
    pub fn model_template(&self) -> resources::Model_template<'_> {
        resources::Model_template::new(self.provider)
    }
    /// Get domain_name resource handler
    pub fn domain_name(&self) -> resources::Domain_name<'_> {
        resources::Domain_name::new(self.provider)
    }
    /// Get domain_names resource handler
    pub fn domain_names(&self) -> resources::Domain_names<'_> {
        resources::Domain_names::new(self.provider)
    }
    /// Get vpc_link resource handler
    pub fn vpc_link(&self) -> resources::Vpc_link<'_> {
        resources::Vpc_link::new(self.provider)
    }
    /// Get api_mappings resource handler
    pub fn api_mappings(&self) -> resources::Api_mappings<'_> {
        resources::Api_mappings::new(self.provider)
    }
    /// Get apis resource handler
    pub fn apis(&self) -> resources::Apis<'_> {
        resources::Apis::new(self.provider)
    }
    /// Get route_settings resource handler
    pub fn route_settings(&self) -> resources::Route_settings<'_> {
        resources::Route_settings::new(self.provider)
    }
    /// Get integration_response resource handler
    pub fn integration_response(&self) -> resources::Integration_response<'_> {
        resources::Integration_response::new(self.provider)
    }
    /// Get cors_configuration resource handler
    pub fn cors_configuration(&self) -> resources::Cors_configuration<'_> {
        resources::Cors_configuration::new(self.provider)
    }
    /// Get api_mapping resource handler
    pub fn api_mapping(&self) -> resources::Api_mapping<'_> {
        resources::Api_mapping::new(self.provider)
    }
    /// Get authorizer resource handler
    pub fn authorizer(&self) -> resources::Authorizer<'_> {
        resources::Authorizer::new(self.provider)
    }
    /// Get vpc_links resource handler
    pub fn vpc_links(&self) -> resources::Vpc_links<'_> {
        resources::Vpc_links::new(self.provider)
    }
    /// Get integration_responses resource handler
    pub fn integration_responses(&self) -> resources::Integration_responses<'_> {
        resources::Integration_responses::new(self.provider)
    }
    /// Get stage resource handler
    pub fn stage(&self) -> resources::Stage<'_> {
        resources::Stage::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get route resource handler
    pub fn route(&self) -> resources::Route<'_> {
        resources::Route::new(self.provider)
    }
    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
    }
    /// Get route_request_parameter resource handler
    pub fn route_request_parameter(&self) -> resources::Route_request_parameter<'_> {
        resources::Route_request_parameter::new(self.provider)
    }
    /// Get stages resource handler
    pub fn stages(&self) -> resources::Stages<'_> {
        resources::Stages::new(self.provider)
    }
    /// Get integrations resource handler
    pub fn integrations(&self) -> resources::Integrations<'_> {
        resources::Integrations::new(self.provider)
    }
    /// Get authorizers resource handler
    pub fn authorizers(&self) -> resources::Authorizers<'_> {
        resources::Authorizers::new(self.provider)
    }
    /// Get access_log_settings resource handler
    pub fn access_log_settings(&self) -> resources::Access_log_settings<'_> {
        resources::Access_log_settings::new(self.provider)
    }
    /// Get route_response resource handler
    pub fn route_response(&self) -> resources::Route_response<'_> {
        resources::Route_response::new(self.provider)
    }
    /// Get route_responses resource handler
    pub fn route_responses(&self) -> resources::Route_responses<'_> {
        resources::Route_responses::new(self.provider)
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
