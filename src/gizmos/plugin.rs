use bevy::prelude::*;

use super::*;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                // gizmos_origin,
                // gizmos_static_collisions,
                gizmos_player,
            ),
        );
    }
}
