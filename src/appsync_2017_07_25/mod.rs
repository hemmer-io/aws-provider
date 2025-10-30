//! Appsync_2017_07_25 Service
//!
//! Auto-generated service module for appsync_2017_07_25

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for appsync_2017_07_25
pub struct Appsync_2017_07_25Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Appsync_2017_07_25Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get type resource handler
    pub fn type(&self) -> resources::Type<'_> {
        resources::Type::new(self.provider)
    }
    /// Get api_association resource handler
    pub fn api_association(&self) -> resources::Api_association<'_> {
        resources::Api_association::new(self.provider)
    }
    /// Get schema_creation_status resource handler
    pub fn schema_creation_status(&self) -> resources::Schema_creation_status<'_> {
        resources::Schema_creation_status::new(self.provider)
    }
    /// Get api_cache resource handler
    pub fn api_cache(&self) -> resources::Api_cache<'_> {
        resources::Api_cache::new(self.provider)
    }
    /// Get introspection_schema resource handler
    pub fn introspection_schema(&self) -> resources::Introspection_schema<'_> {
        resources::Introspection_schema::new(self.provider)
    }
    /// Get function resource handler
    pub fn function(&self) -> resources::Function<'_> {
        resources::Function::new(self.provider)
    }
    /// Get domain_name resource handler
    pub fn domain_name(&self) -> resources::Domain_name<'_> {
        resources::Domain_name::new(self.provider)
    }
    /// Get api_key resource handler
    pub fn api_key(&self) -> resources::Api_key<'_> {
        resources::Api_key::new(self.provider)
    }
    /// Get graphql_api_environment_variables resource handler
    pub fn graphql_api_environment_variables(&self) -> resources::Graphql_api_environment_variables<'_> {
        resources::Graphql_api_environment_variables::new(self.provider)
    }
    /// Get data_source resource handler
    pub fn data_source(&self) -> resources::Data_source<'_> {
        resources::Data_source::new(self.provider)
    }
    /// Get graphql_api resource handler
    pub fn graphql_api(&self) -> resources::Graphql_api<'_> {
        resources::Graphql_api::new(self.provider)
    }
    /// Get data_source_introspection resource handler
    pub fn data_source_introspection(&self) -> resources::Data_source_introspection<'_> {
        resources::Data_source_introspection::new(self.provider)
    }
    /// Get resolver resource handler
    pub fn resolver(&self) -> resources::Resolver<'_> {
        resources::Resolver::new(self.provider)
    }
    /// Get api resource handler
    pub fn api(&self) -> resources::Api<'_> {
        resources::Api::new(self.provider)
    }
    /// Get source_api_association resource handler
    pub fn source_api_association(&self) -> resources::Source_api_association<'_> {
        resources::Source_api_association::new(self.provider)
    }
    /// Get channel_namespace resource handler
    pub fn channel_namespace(&self) -> resources::Channel_namespace<'_> {
        resources::Channel_namespace::new(self.provider)
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
