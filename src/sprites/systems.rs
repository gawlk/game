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
    for (indexes, mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            let times_finished_this_tick = timer.times_finished_this_tick() as usize;

            sprite.index =
                (sprite.index + times_finished_this_tick) % (indexes.last + 1 - indexes.first)
        }
    }
}
