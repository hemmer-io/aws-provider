//! Gamelift Service
//!
//! Auto-generated service module for gamelift

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for gamelift
pub struct GameliftService<'a> {
    provider: &'a crate::AwsProvider,
}

impl<'a> GameliftService<'a> {
    pub(crate) fn new(provider: &'a crate::AwsProvider) -> Self {
        Self { provider }
    }

    /// Get vpc_peering_connections resource handler
    pub fn vpc_peering_connections(&self) -> resources::Vpc_peering_connections<'_> {
        resources::Vpc_peering_connections::new(self.provider)
    }
    /// Get vpc_peering_connection resource handler
    pub fn vpc_peering_connection(&self) -> resources::Vpc_peering_connection<'_> {
        resources::Vpc_peering_connection::new(self.provider)
    }
    /// Get game_session resource handler
    pub fn game_session(&self) -> resources::Game_session<'_> {
        resources::Game_session::new(self.provider)
    }
    /// Get container_group_definition resource handler
    pub fn container_group_definition(&self) -> resources::Container_group_definition<'_> {
        resources::Container_group_definition::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get game_session_queue resource handler
    pub fn game_session_queue(&self) -> resources::Game_session_queue<'_> {
        resources::Game_session_queue::new(self.provider)
    }
    /// Get player_session resource handler
    pub fn player_session(&self) -> resources::Player_session<'_> {
        resources::Player_session::new(self.provider)
    }
    /// Get compute resource handler
    pub fn compute(&self) -> resources::Compute<'_> {
        resources::Compute::new(self.provider)
    }
    /// Get instances resource handler
    pub fn instances(&self) -> resources::Instances<'_> {
        resources::Instances::new(self.provider)
    }
    /// Get game_session_log_url resource handler
    pub fn game_session_log_url(&self) -> resources::Game_session_log_url<'_> {
        resources::Game_session_log_url::new(self.provider)
    }
    /// Get fleet_location_capacity resource handler
    pub fn fleet_location_capacity(&self) -> resources::Fleet_location_capacity<'_> {
        resources::Fleet_location_capacity::new(self.provider)
    }
    /// Get instance_access resource handler
    pub fn instance_access(&self) -> resources::Instance_access<'_> {
        resources::Instance_access::new(self.provider)
    }
    /// Get fleet_deployment resource handler
    pub fn fleet_deployment(&self) -> resources::Fleet_deployment<'_> {
        resources::Fleet_deployment::new(self.provider)
    }
    /// Get matchmaking_rule_sets resource handler
    pub fn matchmaking_rule_sets(&self) -> resources::Matchmaking_rule_sets<'_> {
        resources::Matchmaking_rule_sets::new(self.provider)
    }
    /// Get scaling_policy resource handler
    pub fn scaling_policy(&self) -> resources::Scaling_policy<'_> {
        resources::Scaling_policy::new(self.provider)
    }
    /// Get fleet_port_settings resource handler
    pub fn fleet_port_settings(&self) -> resources::Fleet_port_settings<'_> {
        resources::Fleet_port_settings::new(self.provider)
    }
    /// Get scaling_policies resource handler
    pub fn scaling_policies(&self) -> resources::Scaling_policies<'_> {
        resources::Scaling_policies::new(self.provider)
    }
    /// Get fleet_utilization resource handler
    pub fn fleet_utilization(&self) -> resources::Fleet_utilization<'_> {
        resources::Fleet_utilization::new(self.provider)
    }
    /// Get matchmaking_configuration resource handler
    pub fn matchmaking_configuration(&self) -> resources::Matchmaking_configuration<'_> {
        resources::Matchmaking_configuration::new(self.provider)
    }
    /// Get fleet_location_attributes resource handler
    pub fn fleet_location_attributes(&self) -> resources::Fleet_location_attributes<'_> {
        resources::Fleet_location_attributes::new(self.provider)
    }
    /// Get matchmaking_configurations resource handler
    pub fn matchmaking_configurations(&self) -> resources::Matchmaking_configurations<'_> {
        resources::Matchmaking_configurations::new(self.provider)
    }
    /// Get ec2_instance_limits resource handler
    pub fn ec2_instance_limits(&self) -> resources::Ec2_instance_limits<'_> {
        resources::Ec2_instance_limits::new(self.provider)
    }
    /// Get compute_auth_token resource handler
    pub fn compute_auth_token(&self) -> resources::Compute_auth_token<'_> {
        resources::Compute_auth_token::new(self.provider)
    }
    /// Get game_session_details resource handler
    pub fn game_session_details(&self) -> resources::Game_session_details<'_> {
        resources::Game_session_details::new(self.provider)
    }
    /// Get fleet_attributes resource handler
    pub fn fleet_attributes(&self) -> resources::Fleet_attributes<'_> {
        resources::Fleet_attributes::new(self.provider)
    }
    /// Get game_sessions resource handler
    pub fn game_sessions(&self) -> resources::Game_sessions<'_> {
        resources::Game_sessions::new(self.provider)
    }
    /// Get matchmaking_rule_set resource handler
    pub fn matchmaking_rule_set(&self) -> resources::Matchmaking_rule_set<'_> {
        resources::Matchmaking_rule_set::new(self.provider)
    }
    /// Get runtime_configuration resource handler
    pub fn runtime_configuration(&self) -> resources::Runtime_configuration<'_> {
        resources::Runtime_configuration::new(self.provider)
    }
    /// Get fleet resource handler
    pub fn fleet(&self) -> resources::Fleet<'_> {
        resources::Fleet::new(self.provider)
    }
    /// Get game_server resource handler
    pub fn game_server(&self) -> resources::Game_server<'_> {
        resources::Game_server::new(self.provider)
    }
    /// Get game_server_instances resource handler
    pub fn game_server_instances(&self) -> resources::Game_server_instances<'_> {
        resources::Game_server_instances::new(self.provider)
    }
    /// Get vpc_peering_authorization resource handler
    pub fn vpc_peering_authorization(&self) -> resources::Vpc_peering_authorization<'_> {
        resources::Vpc_peering_authorization::new(self.provider)
    }
    /// Get script resource handler
    pub fn script(&self) -> resources::Script<'_> {
        resources::Script::new(self.provider)
    }
    /// Get fleet_locations resource handler
    pub fn fleet_locations(&self) -> resources::Fleet_locations<'_> {
        resources::Fleet_locations::new(self.provider)
    }
    /// Get player_sessions resource handler
    pub fn player_sessions(&self) -> resources::Player_sessions<'_> {
        resources::Player_sessions::new(self.provider)
    }
    /// Get container_fleet resource handler
    pub fn container_fleet(&self) -> resources::Container_fleet<'_> {
        resources::Container_fleet::new(self.provider)
    }
    /// Get fleet_location_utilization resource handler
    pub fn fleet_location_utilization(&self) -> resources::Fleet_location_utilization<'_> {
        resources::Fleet_location_utilization::new(self.provider)
    }
    /// Get game_session_placement resource handler
    pub fn game_session_placement(&self) -> resources::Game_session_placement<'_> {
        resources::Game_session_placement::new(self.provider)
    }
    /// Get matchmaking resource handler
    pub fn matchmaking(&self) -> resources::Matchmaking<'_> {
        resources::Matchmaking::new(self.provider)
    }
    /// Get build resource handler
    pub fn build(&self) -> resources::Build<'_> {
        resources::Build::new(self.provider)
    }
    /// Get fleet_capacity resource handler
    pub fn fleet_capacity(&self) -> resources::Fleet_capacity<'_> {
        resources::Fleet_capacity::new(self.provider)
    }
    /// Get compute_access resource handler
    pub fn compute_access(&self) -> resources::Compute_access<'_> {
        resources::Compute_access::new(self.provider)
    }
    /// Get alias resource handler
    pub fn alias(&self) -> resources::Alias<'_> {
        resources::Alias::new(self.provider)
    }
    /// Get fleet_events resource handler
    pub fn fleet_events(&self) -> resources::Fleet_events<'_> {
        resources::Fleet_events::new(self.provider)
    }
    /// Get game_session_queues resource handler
    pub fn game_session_queues(&self) -> resources::Game_session_queues<'_> {
        resources::Game_session_queues::new(self.provider)
    }
    /// Get vpc_peering_authorizations resource handler
    pub fn vpc_peering_authorizations(&self) -> resources::Vpc_peering_authorizations<'_> {
        resources::Vpc_peering_authorizations::new(self.provider)
    }
    /// Get game_server_group resource handler
    pub fn game_server_group(&self) -> resources::Game_server_group<'_> {
        resources::Game_server_group::new(self.provider)
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
