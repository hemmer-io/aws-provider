//! Resource modules

pub mod identities;
pub use identities::Identities;
pub mod open_id_token;
pub use open_id_token::Open_id_token;
pub mod credentials_for_identity;
pub use credentials_for_identity::Credentials_for_identity;
pub mod identity_pool;
pub use identity_pool::Identity_pool;
pub mod identity_pool_roles;
pub use identity_pool_roles::Identity_pool_roles;
pub mod identity;
pub use identity::Identity;
pub mod id;
pub use id::Id;
pub mod open_id_token_for_developer_identity;
pub use open_id_token_for_developer_identity::Open_id_token_for_developer_identity;
pub mod principal_tag_attribute_map;
pub use principal_tag_attribute_map::Principal_tag_attribute_map;

