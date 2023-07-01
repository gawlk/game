use bevy::prelude::*;

use super::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (process_player_inputs,));
    }
}
