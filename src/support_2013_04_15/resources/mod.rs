//! Resource modules

pub mod trusted_advisor_check_refresh_statuses;
pub use trusted_advisor_check_refresh_statuses::Trusted_advisor_check_refresh_statuses;
pub mod trusted_advisor_check_summaries;
pub use trusted_advisor_check_summaries::Trusted_advisor_check_summaries;
pub mod case;
pub use case::Case;
pub mod trusted_advisor_check_result;
pub use trusted_advisor_check_result::Trusted_advisor_check_result;
pub mod cases;
pub use cases::Cases;
pub mod communications;
pub use communications::Communications;
pub mod supported_languages;
pub use supported_languages::Supported_languages;
pub mod severity_levels;
pub use severity_levels::Severity_levels;
pub mod trusted_advisor_checks;
pub use trusted_advisor_checks::Trusted_advisor_checks;
pub mod services;
pub use services::Services;
pub mod create_case_options;
pub use create_case_options::Create_case_options;
pub mod attachment;
pub use attachment::Attachment;

