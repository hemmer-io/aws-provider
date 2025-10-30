//! Resource modules

pub mod resource_api_key;
pub use resource_api_key::Resource_api_key;
pub mod workload_access_token;
pub use workload_access_token::Workload_access_token;
pub mod workload_access_token_for_user_id;
pub use workload_access_token_for_user_id::Workload_access_token_for_user_id;
pub mod workload_access_token_for_jwt;
pub use workload_access_token_for_jwt::Workload_access_token_for_jwt;
pub mod resource_oauth2_token;
pub use resource_oauth2_token::Resource_oauth2_token;

