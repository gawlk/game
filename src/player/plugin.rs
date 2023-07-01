use bevy::prelude::*;

use super::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, (spawn_player,)).add_systems(
            FixedUpdate,
            (
                player_horizontal,
                player_vertical,
                // player_diagonal,
                translate_player,
            )
                .chain(),
        );
    }
}
