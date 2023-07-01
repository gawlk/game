use bevy::prelude::*;

use super::{AnimationIndexes, AnimationTimer};

// https://bevyengine.org/examples/2d/sprite-sheet/
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndexes,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indixes, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            sprite.index = if sprite.index == indixes.last {
                indixes.first
            } else {
                sprite.index + 1
            };
        }
    }
}
