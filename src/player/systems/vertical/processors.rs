use bevy::prelude::*;

use crate::{colliders::RectCollider, level::Level, time::TickerComponent};

use super::*;

pub fn process_query_coyote_time_ticker(
    ticker_option: Option<Mut<TickerComponent>>,
    commands: &mut Commands,
    player: Entity,
    jump: bool,
) -> bool {
    if let Some(mut ticker) = ticker_option {
        ticker.tick();

        if jump || ticker.is_over() {
            commands.entity(player).remove::<CoyoteTimeBundle>();

            if jump {
                swap_state_to_rising(commands, player);

                return true;
            }
        }
    }

    false
}

pub fn process_query_jump_buffer(
    ticker_option: Option<Mut<TickerComponent>>,
    commands: &mut Commands,
    player: Entity,
    jump: bool,
) -> bool {
    if let Some(mut ticker) = ticker_option {
        if jump {
            ticker.reset();
        } else {
            ticker.tick();

            if ticker.is_over() {
                commands.entity(player).remove::<JumpBufferBundle>();

                return false;
            }
        }

        true
    } else if jump {
        commands.entity(player).insert(JumpBufferBundle::new());

        true
    } else {
        false
    }
}

#[allow(clippy::too_many_arguments)]
pub fn process_wall_jump(
    level: &Level,
    commands: &mut Commands,
    player: Entity,
    transform: &Transform,
    collider: &RectCollider,
    velocity_x: &mut PlayerVelocityX,
    ticker_option: Option<Mut<TickerComponent>>,
    precomputed_touches_left_wall: bool,
    precomputed_touches_right_wall: bool,
) -> bool {
    let close_to_left_wall = {
        if precomputed_touches_right_wall {
            false
        } else {
            precomputed_touches_left_wall
                || level.touches_wall(transform, collider, PLAYER_WALL_JUMP_PAD, true)
        }
    };

    let close_to_right_wall = {
        if close_to_left_wall {
            false
        } else {
            precomputed_touches_right_wall
                || level.touches_wall(transform, collider, PLAYER_WALL_JUMP_PAD, false)
        }
    };

    if close_to_left_wall || close_to_right_wall {
        swap_state_to_rising(commands, player);

        if let Some(mut ticker) = ticker_option {
            ticker.reset();
        } else {
            commands.entity(player).insert(WallJumpLockBundle::new());
        }

        velocity_x.set_to_max(close_to_right_wall);

        true
    } else {
        false
    }
}
