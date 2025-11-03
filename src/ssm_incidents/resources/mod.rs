//! Resource modules

pub mod deletion_protection;
pub use deletion_protection::Deletion_protection;
pub mod replication_set;
pub use replication_set::Replication_set;
pub mod timeline_event;
pub use timeline_event::Timeline_event;
pub mod related_items;
pub use related_items::Related_items;
pub mod resource_policy;
pub use resource_policy::Resource_policy;
pub mod incident_record;
pub use incident_record::Incident_record;
pub mod response_plan;
pub use response_plan::Response_plan;
pub mod resource_policies;
pub use resource_policies::Resource_policies;

