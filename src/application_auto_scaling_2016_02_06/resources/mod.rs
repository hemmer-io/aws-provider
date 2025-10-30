//! Resource modules

pub mod scaling_policy;
pub use scaling_policy::Scaling_policy;
pub mod scheduled_actions;
pub use scheduled_actions::Scheduled_actions;
pub mod scheduled_action;
pub use scheduled_action::Scheduled_action;
pub mod scalable_targets;
pub use scalable_targets::Scalable_targets;
pub mod predictive_scaling_forecast;
pub use predictive_scaling_forecast::Predictive_scaling_forecast;
pub mod scaling_activities;
pub use scaling_activities::Scaling_activities;
pub mod scaling_policies;
pub use scaling_policies::Scaling_policies;

