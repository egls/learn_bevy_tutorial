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
            //.add_startup_system(spawn_enemies)
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State System
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
