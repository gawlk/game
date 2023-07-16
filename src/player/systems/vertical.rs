use bevy::prelude::*;

use crate::{colliders::RectCollider, level::Level};

use super::*;

pub fn player_vertical(
    level: Res<Level>,
    mut commands: Commands,
    query: Query<(Entity, &PlayerActions, &Transform, &RectCollider)>,
    mut query_mut: Query<(
        &mut PlayerVelocityX,
        &mut PlayerVelocityY,
        &mut PlayerVerticalState,
    )>,
    query_coyote_timer_opt: Query<&mut PlayerCoyoteTimer>,
    mut query_jump_buffer_opt: Query<&mut PlayerJumpBuffer>,
    mut query_locked_movement_opt: Query<&mut PlayerLockedMovement>,
) {
    if query.is_empty() || query_mut.is_empty() {
        return;
    }

    let (
        player,
        &PlayerActions {
            down,
            jump,
            rise,
            left,
            right,
            ..
        },
        transform,
        collider,
    ) = query.single();

    let (mut velocity_x, mut velocity_y, mut vertical_state) = query_mut.single_mut();

    let grounded = level.touches_floor(transform, collider);

    // dbg!(grounded, transform.translation.y - collider.y - PIXEL);

    // TODO: Set only on new platform
    velocity_y.reset_states_variables();

    match &mut *vertical_state {
        PlayerVerticalState::Grounded => {
            let buffer = query_jump_buffer_opt.get_single_mut().is_ok();

            commands.entity(player).remove::<PlayerCoyoteTimer>();
            commands.entity(player).remove::<PlayerLockedMovement>();

            if jump || buffer {
                swap_state_to_rise(&mut commands, player);
            } else if !grounded {
                swap_state(&mut commands, player, PlayerVerticalState::default_fall());

                commands.entity(player).insert(PlayerCoyoteTimer::new());
            }
        }
        PlayerVerticalState::Rising(timer) => {
            let wall_jump = jump
                && process_wall_jump(
                    &level,
                    &mut commands,
                    player,
                    transform,
                    collider,
                    &mut velocity_x,
                    &mut query_locked_movement_opt,
                    false,
                    false,
                );

            if wall_jump {
                return;
            }

            timer.tick();

            let t = timer.get();

            velocity_y.compute_rising(t);

            dbg!(t, velocity_y.rise_time_at_max_height_minus_min_height);

            let button_released = !rise
                && t < velocity_y.rise_time_at_max_height_minus_min_height
                && query_locked_movement_opt.get_single_mut().is_err();

            let rise_over = timer.is_over();

            let touches_ceiling = level.touches_ceiling(transform, collider);

            if button_released || rise_over || touches_ceiling {
                let fall_state = {
                    if button_released {
                        PlayerVerticalState::new_fall(
                            velocity_y.fall_time_at_max_height_minus_min_height,
                        )
                    } else {
                        PlayerVerticalState::default_fall()
                    }
                };

                swap_state(&mut commands, player, fall_state);
            }
        }
        PlayerVerticalState::Falling(timer) => {
            if grounded {
                swap_state(&mut commands, player, PlayerVerticalState::Grounded);
                return;
            }

            if process_query_coyote_timer(query_coyote_timer_opt, &mut commands, player, jump) {
                return;
            }

            process_query_jump_buffer(query_jump_buffer_opt, &mut commands, player, jump);

            // TODO: Only check from origin to bottom instead of top to bottom and check if all colliding
            let touches_left_wall = level.touches_wall(transform, collider, 0.0, true);
            let touches_right_wall = level.touches_wall(transform, collider, 0.0, false);

            let wall_jump = jump
                && process_wall_jump(
                    &level,
                    &mut commands,
                    player,
                    transform,
                    collider,
                    &mut velocity_x,
                    &mut query_locked_movement_opt,
                    touches_left_wall,
                    touches_right_wall,
                );

            if wall_jump {
                return;
            }

            let slide_left = left && touches_left_wall;
            let slide_right = right && touches_right_wall;
            let slide = slide_left || slide_right;

            let max_speed = if slide {
                PLAYER_SLIDE_MAX_SPEED
            } else if down {
                PLAYER_FAST_FALL_MAX_SPEED
            } else {
                PLAYER_FALL_MAX_SPEED
            };

            timer.tick();

            velocity_y.compute_falling(timer.get(), max_speed);
        }
    }
}

fn swap_state_to_rise(commands: &mut Commands, player: Entity) {
    commands.entity(player).remove::<PlayerCoyoteTimer>();
    commands.entity(player).remove::<PlayerJumpBuffer>();

    swap_state(commands, player, PlayerVerticalState::default_rise());
}

fn swap_state(commands: &mut Commands, player: Entity, new_state: PlayerVerticalState) {
    commands.entity(player).remove::<PlayerVerticalState>();

    commands.entity(player).insert(new_state);
}

fn process_query_coyote_timer(
    mut query_coyote_timer_opt: Query<&mut PlayerCoyoteTimer>,
    commands: &mut Commands,
    player: Entity,
    jump: bool,
) -> bool {
    if let Ok(mut coyote_timer) = query_coyote_timer_opt.get_single_mut() {
        coyote_timer.tick();

        if jump || coyote_timer.is_over() {
            commands.entity(player).remove::<PlayerCoyoteTimer>();

            if jump {
                swap_state_to_rise(commands, player);

                return true;
            }
        }
    }

    false
}

fn process_query_jump_buffer(
    mut query_jump_buffer_opt: Query<&mut PlayerJumpBuffer>,
    commands: &mut Commands,
    player: Entity,
    jump: bool,
) -> bool {
    if let Ok(mut jump_buffer) = query_jump_buffer_opt.get_single_mut() {
        if jump {
            jump_buffer.reset();
        } else {
            jump_buffer.tick();

            if jump_buffer.is_over() {
                commands.entity(player).remove::<PlayerJumpBuffer>();

                return false;
            }
        }

        true
    } else if jump {
        commands.entity(player).insert(PlayerJumpBuffer::new());

        true
    } else {
        false
    }
}

#[allow(clippy::too_many_arguments)]
fn process_wall_jump(
    level: &Level,
    commands: &mut Commands,
    player: Entity,
    transform: &Transform,
    collider: &RectCollider,
    velocity_x: &mut PlayerVelocityX,
    query_locked_movement_opt: &mut Query<&mut PlayerLockedMovement>,
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
        swap_state_to_rise(commands, player);

        if let Ok(mut locked_movement) = query_locked_movement_opt.get_single_mut() {
            locked_movement.reset();
        } else {
            commands.entity(player).insert(PlayerLockedMovement::new());
        }

        velocity_x.set_to_max(close_to_right_wall);

        true
    } else {
        false
    }
}
