use bevy::prelude::*;

use super::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, (spawn_player,))
            .add_systems(PreUpdate, (process_player_inputs,))
            .add_systems(
                FixedUpdate,
                (
                    player_lock,
                    player_horizontal,
                    player_vertical,
                    // player_diagonal,
                    translate_player,
                )
                    .chain(),
            );
    }
}
