#![allow(clippy::type_complexity)]

use bevy::prelude::*;

mod processors;
mod swappers;

pub use super::*;
use crate::{colliders::RectCollider, level::Level, time::TickerComponent};
use processors::*;
use swappers::*;

pub fn player_vertical(
    level: Res<Level>,
    mut commands: Commands,
    query: Query<(Entity, &PlayerActions, &Transform, &RectCollider)>,
    mut query_mut: Query<(
        &mut PlayerVelocityX,
        &mut PlayerVelocityY,
        &mut PlayerVerticalState,
    )>,
    mut query_coyote_time_opt: Query<&mut TickerComponent, With<CoyoteTimeMarker>>,
    mut query_jump_buffer_opt: Query<
        &mut TickerComponent,
        (
            With<JumpBufferMarker>,
            Without<CoyoteTimeMarker>,
            Without<WallJumpLockMarker>,
        ),
    >,
    mut query_locked_movement_opt: Query<
        &mut TickerComponent,
        (
            With<WallJumpLockMarker>,
            Without<CoyoteTimeMarker>,
            Without<JumpBufferMarker>,
        ),
    >,
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

    // TODO: Set only on new platform
    velocity_y.reset_states_variables();

    match &mut *vertical_state {
        PlayerVerticalState::Grounded => {
            // println!("Grouded");

            let buffer = query_jump_buffer_opt.get_single_mut().is_ok();

            // println!("{} {} {}", jump || buffer, jump, buffer);

            if jump || buffer {
                swap_state_to_rising(&mut commands, player);
            } else if !level.touches_floor(transform, collider) {
                swap_state_to_falling(&mut commands, player, None);
                commands.entity(player).insert(CoyoteTimeBundle::new());
            }
        }
        PlayerVerticalState::Rising(ticker) => {
            // println!("Rising");

            let wall_jump = jump
                && process_wall_jump(
                    &level,
                    &mut commands,
                    player,
                    transform,
                    collider,
                    &mut velocity_x,
                    query_locked_movement_opt.get_single_mut().ok(),
                    false,
                    false,
                );

            if wall_jump {
                return;
            }

            ticker.tick();

            let time = ticker.get();

            velocity_y.compute_rising(time);

            let button_released = !rise
                && time < velocity_y.rise_time_at_max_height_minus_min_height
                && query_locked_movement_opt.get_single_mut().is_err();

            let rise_over = ticker.is_over();

            let touches_ceiling = level.touches_ceiling(transform, collider);

            if button_released || rise_over || touches_ceiling {
                let fall_time = {
                    if button_released {
                        Some(velocity_y.fall_time_at_max_height_minus_min_height)
                    } else if rise_over {
                        Some(time - PLAYER_RISE_HALF_TIME + PLAYER_FALL_HALF_TIME)
                    } else {
                        None
                    }
                };

                swap_state_to_falling(&mut commands, player, fall_time);
            }
        }
        PlayerVerticalState::Falling(ticker) => {
            // println!("Falling");

            if level.touches_floor(transform, collider) {
                swap_state_to_grounded(&mut commands, player, &mut velocity_y);
                return;
            }

            if process_query_coyote_time_ticker(
                query_coyote_time_opt.get_single_mut().ok(),
                &mut commands,
                player,
                jump,
            ) {
                return;
            }

            process_query_jump_buffer(
                query_jump_buffer_opt.get_single_mut().ok(),
                &mut commands,
                player,
                jump,
            );

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
                    query_locked_movement_opt.get_single_mut().ok(),
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

            ticker.tick();

            velocity_y.compute_falling(ticker.get(), max_speed);
        }
    }
}
