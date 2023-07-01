use bevy::prelude::*;

use crate::physics::{Velocity, FIXED_UPDATE_DELTA_TIME};

use super::*;

#[allow(dead_code)]
pub fn player_diagonal(
    query: Query<&PlayerActions>,
    mut query_mut: Query<&mut Velocity, With<Player>>,
) {
    if query.is_empty() || query_mut.is_empty() {
        return;
    }

    let mut velocity = query_mut.single_mut();

    let speed = PLAYER_RUN_MAX_SPEED * FIXED_UPDATE_DELTA_TIME;

    dbg!(speed);

    let &PlayerActions {
        left,
        down,
        up,
        right,
        ..
    } = query.single();

    velocity.x = 0.0;

    if left || right {
        if left {
            velocity.x -= speed;
        }
        if right {
            velocity.x += speed;
        }
    }

    velocity.y = 0.0;

    if down || up {
        if down {
            velocity.y -= speed;
        }
        if up {
            velocity.y += speed;
        }
    }
}
