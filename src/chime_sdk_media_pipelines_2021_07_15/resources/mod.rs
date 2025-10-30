//! Resource modules

pub mod media_capture_pipeline;
pub use media_capture_pipeline::Media_capture_pipeline;
pub mod speaker_search_task;
pub use speaker_search_task::Speaker_search_task;
pub mod media_pipeline_kinesis_video_stream_pool;
pub use media_pipeline_kinesis_video_stream_pool::Media_pipeline_kinesis_video_stream_pool;
pub mod media_stream_pipeline;
pub use media_stream_pipeline::Media_stream_pipeline;
pub mod media_insights_pipeline_configuration;
pub use media_insights_pipeline_configuration::Media_insights_pipeline_configuration;
pub mod media_insights_pipeline;
pub use media_insights_pipeline::Media_insights_pipeline;
pub mod media_concatenation_pipeline;
pub use media_concatenation_pipeline::Media_concatenation_pipeline;
pub mod media_pipeline;
pub use media_pipeline::Media_pipeline;
pub mod voice_tone_analysis_task;
pub use voice_tone_analysis_task::Voice_tone_analysis_task;
pub mod media_insights_pipeline_status;
pub use media_insights_pipeline_status::Media_insights_pipeline_status;
pub mod media_live_connector_pipeline;
pub use media_live_connector_pipeline::Media_live_connector_pipeline;

