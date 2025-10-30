//! Resource modules

pub mod id_mapping_job;
pub use id_mapping_job::Id_mapping_job;
pub mod schema_mapping;
pub use schema_mapping::Schema_mapping;
pub mod id_namespace;
pub use id_namespace::Id_namespace;
pub mod id_mapping_workflow;
pub use id_mapping_workflow::Id_mapping_workflow;
pub mod policy;
pub use policy::Policy;
pub mod provider_service;
pub use provider_service::Provider_service;
pub mod matching_job;
pub use matching_job::Matching_job;
pub mod policy_statement;
pub use policy_statement::Policy_statement;
pub mod match_id;
pub use match_id::Match_id;
pub mod matching_workflow;
pub use matching_workflow::Matching_workflow;

