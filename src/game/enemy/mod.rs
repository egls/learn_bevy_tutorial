use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use crate::game::SimulationState;
use crate::AppState;

use resources::*;
use systems::*;

pub const NUMBER_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 240.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemySpawnTimer>()
            // Startup Systems
            .add_systems(Update, spawn_enemies.run_if(in_state(AppState::Game)))
            // Systems
            .add_systems(Update, enemy_movement.run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            .add_systems(Update, update_enemy_direction.run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            .add_systems(Update, confine_enemy_movement.run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            .add_systems(Update, tick_enemy_spawn_timer.run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            .add_systems(Update, spawn_enemies_over_time.run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)))
            // Exit State System
            //.add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
