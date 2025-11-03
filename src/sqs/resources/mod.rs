//! Resource modules

pub mod queue_url;
pub use queue_url::Queue_url;
pub mod message;
pub use message::Message;
pub mod queue_attributes;
pub use queue_attributes::Queue_attributes;
pub mod queue;
pub use queue::Queue;
pub mod message_batch;
pub use message_batch::Message_batch;

