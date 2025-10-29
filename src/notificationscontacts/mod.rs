//! Notificationscontacts Service
//!
//! Auto-generated service module for notificationscontacts

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for notificationscontacts
pub struct NotificationscontactsService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> NotificationscontactsService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
