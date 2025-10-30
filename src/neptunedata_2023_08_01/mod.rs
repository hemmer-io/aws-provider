//! Neptunedata_2023_08_01 Service
//!
//! Auto-generated service module for neptunedata_2023_08_01

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for neptunedata_2023_08_01
pub struct Neptunedata_2023_08_01Service<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> Neptunedata_2023_08_01Service<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get ml_data_processing_job resource handler
    pub fn ml_data_processing_job(&self) -> resources::Ml_data_processing_job<'_> {
        resources::Ml_data_processing_job::new(self.provider)
    }
    /// Get sparql_statistics resource handler
    pub fn sparql_statistics(&self) -> resources::Sparql_statistics<'_> {
        resources::Sparql_statistics::new(self.provider)
    }
    /// Get propertygraph_stream resource handler
    pub fn propertygraph_stream(&self) -> resources::Propertygraph_stream<'_> {
        resources::Propertygraph_stream::new(self.provider)
    }
    /// Get ml_model_transform_job resource handler
    pub fn ml_model_transform_job(&self) -> resources::Ml_model_transform_job<'_> {
        resources::Ml_model_transform_job::new(self.provider)
    }
    /// Get engine_status resource handler
    pub fn engine_status(&self) -> resources::Engine_status<'_> {
        resources::Engine_status::new(self.provider)
    }
    /// Get sparql_stream resource handler
    pub fn sparql_stream(&self) -> resources::Sparql_stream<'_> {
        resources::Sparql_stream::new(self.provider)
    }
    /// Get propertygraph_statistics resource handler
    pub fn propertygraph_statistics(&self) -> resources::Propertygraph_statistics<'_> {
        resources::Propertygraph_statistics::new(self.provider)
    }
    /// Get propertygraph_summary resource handler
    pub fn propertygraph_summary(&self) -> resources::Propertygraph_summary<'_> {
        resources::Propertygraph_summary::new(self.provider)
    }
    /// Get gremlin_query_status resource handler
    pub fn gremlin_query_status(&self) -> resources::Gremlin_query_status<'_> {
        resources::Gremlin_query_status::new(self.provider)
    }
    /// Get ml_model_training_job resource handler
    pub fn ml_model_training_job(&self) -> resources::Ml_model_training_job<'_> {
        resources::Ml_model_training_job::new(self.provider)
    }
    /// Get ml_endpoint resource handler
    pub fn ml_endpoint(&self) -> resources::Ml_endpoint<'_> {
        resources::Ml_endpoint::new(self.provider)
    }
    /// Get loader_job_status resource handler
    pub fn loader_job_status(&self) -> resources::Loader_job_status<'_> {
        resources::Loader_job_status::new(self.provider)
    }
    /// Get open_cypher_query_status resource handler
    pub fn open_cypher_query_status(&self) -> resources::Open_cypher_query_status<'_> {
        resources::Open_cypher_query_status::new(self.provider)
    }
    /// Get rdf_graph_summary resource handler
    pub fn rdf_graph_summary(&self) -> resources::Rdf_graph_summary<'_> {
        resources::Rdf_graph_summary::new(self.provider)
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
