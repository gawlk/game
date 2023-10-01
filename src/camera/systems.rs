use bevy::prelude::*;

use crate::player::PlayerMarker;

use super::*;

pub fn spawn_camera(mut commands: Commands) {
    let mut camera2d_bundle = Camera2dBundle::default();

    camera2d_bundle.projection.scale = CAMERA_SCALE;

    commands.spawn(camera2d_bundle);
}

pub fn move_camera(
    mut query_camera_transform: Query<&mut Transform, (With<Camera>, Without<PlayerMarker>)>,
    query_player_transform: Query<&Transform, With<PlayerMarker>>,
) {
    if query_player_transform.is_empty() || query_camera_transform.is_empty() {
        return;
    }

    let player_translation = query_player_transform.single().translation;

    let mut camera_transform = query_camera_transform.single_mut();

    camera_transform.translation.x = player_translation.x;
    camera_transform.translation.y = player_translation.y;
}
