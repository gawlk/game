use bevy::prelude::*;

use super::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_walls,));
    }
}
