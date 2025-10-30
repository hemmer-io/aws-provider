//! Resource modules

pub mod latest_assessment_id;
pub use latest_assessment_id::Latest_assessment_id;
pub mod server_details;
pub use server_details::Server_details;
pub mod application_component_config;
pub use application_component_config::Application_component_config;
pub mod server_config;
pub use server_config::Server_config;
pub mod application_component_strategies;
pub use application_component_strategies::Application_component_strategies;
pub mod portfolio_summary;
pub use portfolio_summary::Portfolio_summary;
pub mod portfolio_preferences;
pub use portfolio_preferences::Portfolio_preferences;
pub mod import_file_task;
pub use import_file_task::Import_file_task;
pub mod application_component_details;
pub use application_component_details::Application_component_details;
pub mod recommendation_report_details;
pub use recommendation_report_details::Recommendation_report_details;
pub mod server_strategies;
pub use server_strategies::Server_strategies;
pub mod assessment;
pub use assessment::Assessment;

