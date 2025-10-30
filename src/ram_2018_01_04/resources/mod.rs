//! Resource modules

pub mod permission_version;
pub use permission_version::Permission_version;
pub mod resource_policies;
pub use resource_policies::Resource_policies;
pub mod resource_share_invitations;
pub use resource_share_invitations::Resource_share_invitations;
pub mod resource_share;
pub use resource_share::Resource_share;
pub mod resource_shares;
pub use resource_shares::Resource_shares;
pub mod resource_share_associations;
pub use resource_share_associations::Resource_share_associations;
pub mod permission;
pub use permission::Permission;

