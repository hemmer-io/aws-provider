//! Api_gateway_2015_07_09 Service
//!
//! Auto-generated service module for api_gateway_2015_07_09

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for api_gateway_2015_07_09
pub struct Api_gateway_2015_07_09Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Api_gateway_2015_07_09Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get usage_plan_keys resource handler
    pub fn usage_plan_keys(&self) -> resources::Usage_plan_keys<'_> {
        resources::Usage_plan_keys::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get usage_plan resource handler
    pub fn usage_plan(&self) -> resources::Usage_plan<'_> {
        resources::Usage_plan::new(self.provider)
    }
    /// Get method_response resource handler
    pub fn method_response(&self) -> resources::Method_response<'_> {
        resources::Method_response::new(self.provider)
    }
    /// Get gateway_response resource handler
    pub fn gateway_response(&self) -> resources::Gateway_response<'_> {
        resources::Gateway_response::new(self.provider)
    }
    /// Get documentation_part resource handler
    pub fn documentation_part(&self) -> resources::Documentation_part<'_> {
        resources::Documentation_part::new(self.provider)
    }
    /// Get documentation_version resource handler
    pub fn documentation_version(&self) -> resources::Documentation_version<'_> {
        resources::Documentation_version::new(self.provider)
    }
    /// Get method resource handler
    pub fn method(&self) -> resources::Method<'_> {
        resources::Method::new(self.provider)
    }
    /// Get base_path_mappings resource handler
    pub fn base_path_mappings(&self) -> resources::Base_path_mappings<'_> {
        resources::Base_path_mappings::new(self.provider)
    }
    /// Get gateway_responses resource handler
    pub fn gateway_responses(&self) -> resources::Gateway_responses<'_> {
        resources::Gateway_responses::new(self.provider)
    }
    /// Get resources resource handler
    pub fn resources(&self) -> resources::Resources<'_> {
        resources::Resources::new(self.provider)
    }
    /// Get stage resource handler
    pub fn stage(&self) -> resources::Stage<'_> {
        resources::Stage::new(self.provider)
    }
    /// Get sdk resource handler
    pub fn sdk(&self) -> resources::Sdk<'_> {
        resources::Sdk::new(self.provider)
    }
    /// Get integration_response resource handler
    pub fn integration_response(&self) -> resources::Integration_response<'_> {
        resources::Integration_response::new(self.provider)
    }
    /// Get domain_name resource handler
    pub fn domain_name(&self) -> resources::Domain_name<'_> {
        resources::Domain_name::new(self.provider)
    }
    /// Get account resource handler
    pub fn account(&self) -> resources::Account<'_> {
        resources::Account::new(self.provider)
    }
    /// Get resource resource handler
    pub fn resource(&self) -> resources::Resource<'_> {
        resources::Resource::new(self.provider)
    }
    /// Get authorizers resource handler
    pub fn authorizers(&self) -> resources::Authorizers<'_> {
        resources::Authorizers::new(self.provider)
    }
    /// Get rest_api resource handler
    pub fn rest_api(&self) -> resources::Rest_api<'_> {
        resources::Rest_api::new(self.provider)
    }
    /// Get export resource handler
    pub fn export(&self) -> resources::Export<'_> {
        resources::Export::new(self.provider)
    }
    /// Get request_validators resource handler
    pub fn request_validators(&self) -> resources::Request_validators<'_> {
        resources::Request_validators::new(self.provider)
    }
    /// Get base_path_mapping resource handler
    pub fn base_path_mapping(&self) -> resources::Base_path_mapping<'_> {
        resources::Base_path_mapping::new(self.provider)
    }
    /// Get sdk_type resource handler
    pub fn sdk_type(&self) -> resources::Sdk_type<'_> {
        resources::Sdk_type::new(self.provider)
    }
    /// Get client_certificates resource handler
    pub fn client_certificates(&self) -> resources::Client_certificates<'_> {
        resources::Client_certificates::new(self.provider)
    }
    /// Get stages resource handler
    pub fn stages(&self) -> resources::Stages<'_> {
        resources::Stages::new(self.provider)
    }
    /// Get usage_plan_key resource handler
    pub fn usage_plan_key(&self) -> resources::Usage_plan_key<'_> {
        resources::Usage_plan_key::new(self.provider)
    }
    /// Get api_keys resource handler
    pub fn api_keys(&self) -> resources::Api_keys<'_> {
        resources::Api_keys::new(self.provider)
    }
    /// Get documentation_versions resource handler
    pub fn documentation_versions(&self) -> resources::Documentation_versions<'_> {
        resources::Documentation_versions::new(self.provider)
    }
    /// Get sdk_types resource handler
    pub fn sdk_types(&self) -> resources::Sdk_types<'_> {
        resources::Sdk_types::new(self.provider)
    }
    /// Get usage_plans resource handler
    pub fn usage_plans(&self) -> resources::Usage_plans<'_> {
        resources::Usage_plans::new(self.provider)
    }
    /// Get domain_names resource handler
    pub fn domain_names(&self) -> resources::Domain_names<'_> {
        resources::Domain_names::new(self.provider)
    }
    /// Get vpc_link resource handler
    pub fn vpc_link(&self) -> resources::Vpc_link<'_> {
        resources::Vpc_link::new(self.provider)
    }
    /// Get request_validator resource handler
    pub fn request_validator(&self) -> resources::Request_validator<'_> {
        resources::Request_validator::new(self.provider)
    }
    /// Get usage resource handler
    pub fn usage(&self) -> resources::Usage<'_> {
        resources::Usage::new(self.provider)
    }
    /// Get tags resource handler
    pub fn tags(&self) -> resources::Tags<'_> {
        resources::Tags::new(self.provider)
    }
    /// Get domain_name_access_associations resource handler
    pub fn domain_name_access_associations(&self) -> resources::Domain_name_access_associations<'_> {
        resources::Domain_name_access_associations::new(self.provider)
    }
    /// Get deployments resource handler
    pub fn deployments(&self) -> resources::Deployments<'_> {
        resources::Deployments::new(self.provider)
    }
    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
    }
    /// Get vpc_links resource handler
    pub fn vpc_links(&self) -> resources::Vpc_links<'_> {
        resources::Vpc_links::new(self.provider)
    }
    /// Get rest_apis resource handler
    pub fn rest_apis(&self) -> resources::Rest_apis<'_> {
        resources::Rest_apis::new(self.provider)
    }
    /// Get integration resource handler
    pub fn integration(&self) -> resources::Integration<'_> {
        resources::Integration::new(self.provider)
    }
    /// Get api_key resource handler
    pub fn api_key(&self) -> resources::Api_key<'_> {
        resources::Api_key::new(self.provider)
    }
    /// Get domain_name_access_association resource handler
    pub fn domain_name_access_association(&self) -> resources::Domain_name_access_association<'_> {
        resources::Domain_name_access_association::new(self.provider)
    }
    /// Get model_template resource handler
    pub fn model_template(&self) -> resources::Model_template<'_> {
        resources::Model_template::new(self.provider)
    }
    /// Get authorizer resource handler
    pub fn authorizer(&self) -> resources::Authorizer<'_> {
        resources::Authorizer::new(self.provider)
    }
    /// Get client_certificate resource handler
    pub fn client_certificate(&self) -> resources::Client_certificate<'_> {
        resources::Client_certificate::new(self.provider)
    }
    /// Get documentation_parts resource handler
    pub fn documentation_parts(&self) -> resources::Documentation_parts<'_> {
        resources::Documentation_parts::new(self.provider)
    }
    /// Get models resource handler
    pub fn models(&self) -> resources::Models<'_> {
        resources::Models::new(self.provider)
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
