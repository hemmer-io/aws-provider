//! Resource modules

pub mod attendee;
pub use attendee::Attendee;
pub mod attendee_capabilities;
pub use attendee_capabilities::Attendee_capabilities;
pub mod meeting;
pub use meeting::Meeting;
pub mod meeting_with_attendees;
pub use meeting_with_attendees::Meeting_with_attendees;

