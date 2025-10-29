//! Resource modules

pub mod campaign;
pub use campaign::Campaign;
pub mod instance_onboarding_job;
pub use instance_onboarding_job::Instance_onboarding_job;
pub mod campaign_state_batch;
pub use campaign_state_batch::Campaign_state_batch;
pub mod connect_instance_config;
pub use connect_instance_config::Connect_instance_config;
pub mod campaign_outbound_call_config;
pub use campaign_outbound_call_config::Campaign_outbound_call_config;
pub mod dial_request_batch;
pub use dial_request_batch::Dial_request_batch;
pub mod campaign_state;
pub use campaign_state::Campaign_state;
pub mod campaign_dialer_config;
pub use campaign_dialer_config::Campaign_dialer_config;
pub mod campaign_name;
pub use campaign_name::Campaign_name;
pub mod instance_onboarding_job_status;
pub use instance_onboarding_job_status::Instance_onboarding_job_status;

