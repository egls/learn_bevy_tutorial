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
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // Plugin
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorepPlugin)
            .add_plugin(StarPlugin)
            // Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // On Exit System
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
