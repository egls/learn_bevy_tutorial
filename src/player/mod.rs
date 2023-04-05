use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystem;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        //.configure_set(MovementSystemSet.before(ConfinementSystem))
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(player_movement.before(confine_player_movement))
            //.add_systems(
            //    (
            //        player_movement,
            //        confine_player_movement
            //    )
            // ).chain()
            //.add_system(confine_player_movement.after(player_movement))
            .add_system(confine_player_movement)
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
