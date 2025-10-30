//! Resource modules

pub mod language_model;
pub use language_model::Language_model;
pub mod vocabulary;
pub use vocabulary::Vocabulary;
pub mod medical_scribe_job;
pub use medical_scribe_job::Medical_scribe_job;
pub mod call_analytics_category;
pub use call_analytics_category::Call_analytics_category;
pub mod vocabulary_filter;
pub use vocabulary_filter::Vocabulary_filter;
pub mod call_analytics_job;
pub use call_analytics_job::Call_analytics_job;
pub mod medical_transcription_job;
pub use medical_transcription_job::Medical_transcription_job;
pub mod medical_vocabulary;
pub use medical_vocabulary::Medical_vocabulary;
pub mod transcription_job;
pub use transcription_job::Transcription_job;

