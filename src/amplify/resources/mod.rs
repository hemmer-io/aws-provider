//! Resource modules

pub mod deployment;
pub use deployment::Deployment;
pub mod app;
pub use app::App;
pub mod job;
pub use job::Job;
pub mod backend_environment;
pub use backend_environment::Backend_environment;
pub mod domain_association;
pub use domain_association::Domain_association;
pub mod webhook;
pub use webhook::Webhook;
pub mod branch;
pub use branch::Branch;
pub mod artifact_url;
pub use artifact_url::Artifact_url;

