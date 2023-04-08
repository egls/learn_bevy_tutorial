pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // My Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Startup System
        .add_startup_system(spawn_camera)
        // Systems
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
