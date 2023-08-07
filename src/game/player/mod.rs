use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::game::AppState;
use crate::game::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure System Sets
            .configure_set(Update, MovementSystemSet.before(ConfinementSystemSet))
            // On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_player)
            // Systems
            .add_systems(Update, player_movement.in_set(MovementSystemSet).run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            .add_systems(Update, confine_player_movement.in_set(ConfinementSystemSet).run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            .add_systems(Update, enemy_hit_player.run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            .add_systems(Update, player_hit_star.run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
