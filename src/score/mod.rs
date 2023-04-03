use bevy::prelude::*;

pub mod resources;
mod systems;

use systems::*;
use resources::*;

pub struct ScorepPlugin;

impl Plugin for ScorepPlugin {

    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
        .init_resource::<HighScores>()
        .add_system(update_score)
        .add_system(update_high_scores)
        .add_system(high_scores_updated);
    }
}