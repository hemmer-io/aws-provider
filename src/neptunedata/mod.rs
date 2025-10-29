//! Neptunedata Service
//!
//! Auto-generated service module for neptunedata

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for neptunedata
pub struct NeptunedataService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> NeptunedataService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get gremlin_query_status resource handler
    pub fn gremlin_query_status(&self) -> resources::Gremlin_query_status<'_> {
        resources::Gremlin_query_status::new(self.provider)
    }
    /// Get mlendpoint resource handler
    pub fn mlendpoint(&self) -> resources::Mlendpoint<'_> {
        resources::Mlendpoint::new(self.provider)
    }
    /// Get propertygraph_stream resource handler
    pub fn propertygraph_stream(&self) -> resources::Propertygraph_stream<'_> {
        resources::Propertygraph_stream::new(self.provider)
    }
    /// Get loader_job_status resource handler
    pub fn loader_job_status(&self) -> resources::Loader_job_status<'_> {
        resources::Loader_job_status::new(self.provider)
    }
    /// Get rdfgraph_summary resource handler
    pub fn rdfgraph_summary(&self) -> resources::Rdfgraph_summary<'_> {
        resources::Rdfgraph_summary::new(self.provider)
    }
    /// Get sparql_statistics resource handler
    pub fn sparql_statistics(&self) -> resources::Sparql_statistics<'_> {
        resources::Sparql_statistics::new(self.provider)
    }
    /// Get mlmodel_training_job resource handler
    pub fn mlmodel_training_job(&self) -> resources::Mlmodel_training_job<'_> {
        resources::Mlmodel_training_job::new(self.provider)
    }
    /// Get open_cypher_query_status resource handler
    pub fn open_cypher_query_status(&self) -> resources::Open_cypher_query_status<'_> {
        resources::Open_cypher_query_status::new(self.provider)
    }
    /// Get sparql_stream resource handler
    pub fn sparql_stream(&self) -> resources::Sparql_stream<'_> {
        resources::Sparql_stream::new(self.provider)
    }
    /// Get mldata_processing_job resource handler
    pub fn mldata_processing_job(&self) -> resources::Mldata_processing_job<'_> {
        resources::Mldata_processing_job::new(self.provider)
    }
    /// Get propertygraph_statistics resource handler
    pub fn propertygraph_statistics(&self) -> resources::Propertygraph_statistics<'_> {
        resources::Propertygraph_statistics::new(self.provider)
    }
    /// Get engine_status resource handler
    pub fn engine_status(&self) -> resources::Engine_status<'_> {
        resources::Engine_status::new(self.provider)
    }
    /// Get propertygraph_summary resource handler
    pub fn propertygraph_summary(&self) -> resources::Propertygraph_summary<'_> {
        resources::Propertygraph_summary::new(self.provider)
    }
    /// Get mlmodel_transform_job resource handler
    pub fn mlmodel_transform_job(&self) -> resources::Mlmodel_transform_job<'_> {
        resources::Mlmodel_transform_job::new(self.provider)
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
