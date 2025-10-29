//! Resource modules

pub mod webhook;
pub use webhook::Webhook;
pub mod source_credentials;
pub use source_credentials::Source_credentials;
pub mod project;
pub use project::Project;
pub mod fleet;
pub use fleet::Fleet;
pub mod report_group;
pub use report_group::Report_group;
pub mod project_visibility;
pub use project_visibility::Project_visibility;
pub mod resource_policy;
pub use resource_policy::Resource_policy;
pub mod test_cases;
pub use test_cases::Test_cases;
pub mod code_coverages;
pub use code_coverages::Code_coverages;
pub mod report_group_trend;
pub use report_group_trend::Report_group_trend;
pub mod build_batch;
pub use build_batch::Build_batch;
pub mod report;
pub use report::Report;

