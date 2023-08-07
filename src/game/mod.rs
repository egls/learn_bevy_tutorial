pub mod enemy;
mod player;
pub mod score;
mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorepPlugin;
use star::StarPlugin;
use systems::*;

use crate::events::GameOver;
use crate::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // On Enter Systems
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            // Plugin
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorepPlugin)
            .add_plugins(StarPlugin)
            // Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // On Exit System
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
