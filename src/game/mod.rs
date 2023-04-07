pub mod enemy;
mod player;
pub mod score;
mod star;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorepPlugin;
use star::StarPlugin;

use bevy::prelude::*;
use crate::events::GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {

    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorepPlugin)
        .add_plugin(StarPlugin);
    }
}
