use bevy::prelude::*;

use super::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera,))
            .add_systems(PostUpdate, (move_camera,));
    }
}
