//! Resource modules

pub mod finding_v2;
pub use finding_v2::Finding_v2;
pub mod generated_policy;
pub use generated_policy::Generated_policy;
pub mod finding_recommendation;
pub use finding_recommendation::Finding_recommendation;
pub mod analyzed_resource;
pub use analyzed_resource::Analyzed_resource;
pub mod access_preview;
pub use access_preview::Access_preview;
pub mod finding;
pub use finding::Finding;
pub mod findings;
pub use findings::Findings;
pub mod findings_statistics;
pub use findings_statistics::Findings_statistics;

