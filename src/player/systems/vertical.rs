use bevy::prelude::*;

use crate::{
    level::{check_no_walls, Level, PIXEL},
    physics::{RectCollider, Velocity, FIXED_UPDATE_DELTA_TIME},
};

use super::*;

pub fn player_vertical(
    level: Res<Level>,
    mut commands: Commands,
    query: Query<(Entity, &PlayerActions, &Transform, &RectCollider)>,
    mut query_mut: Query<(&mut Velocity, &mut PlayerVerticalState), With<Player>>,
) {
    if query.is_empty() || query_mut.is_empty() {
        return;
    }

    let (player, &PlayerActions { jump, down, .. }, transform, collider) = query.single();

    let (mut velocity, mut vertical) = query_mut.single_mut();

    let delta = FIXED_UPDATE_DELTA_TIME;

    let rise_gravity: f32 = compute_gravity(PLAYER_RISE_HALF_TIME);
    let fall_gravity: f32 = compute_gravity(PLAYER_FALL_HALF_TIME);

    // Also equals to = gravity * timeToJumpApex;
    let rise_init_velocity_y: f32 = compute_init_velocity_y(PLAYER_RISE_HALF_TIME);
    let fall_init_velocity_y: f32 = compute_init_velocity_y(PLAYER_FALL_HALF_TIME);

    let rise_time_at_jump_minus_min_jump: f32 =
        compute_time_at_height(rise_init_velocity_y, rise_gravity);
    let fall_time_at_jump_minus_min_jump: f32 =
        compute_time_at_height(fall_init_velocity_y, fall_gravity);

    let is_grounded = !check_no_walls(
        &level,
        transform.translation.x,
        transform.translation.y - PIXEL,
        collider.y,
        false,
    );

    match &mut *vertical {
        PlayerVerticalState::Grounded => {
            info!("grounded");

            if jump {
                commands.entity(player).remove::<PlayerVerticalState>();

                commands
                    .entity(player)
                    .insert(PlayerVerticalState::Rising(0.0));
            } else if !is_grounded {
                commands.entity(player).remove::<PlayerVerticalState>();

                commands
                    .entity(player)
                    .insert(PlayerVerticalState::Falling(PLAYER_FALL_HALF_TIME));
            }
        }
        PlayerVerticalState::Rising(t_ref) => {
            info!("rising");

            *t_ref += delta;

            let t = *t_ref;

            let y_at_current_frame = compute_y(t, rise_gravity, rise_init_velocity_y);

            // TODO: Save previous computation instead redoing it every time
            let y_at_last_frame = compute_y(t - delta, rise_gravity, rise_init_velocity_y);

            velocity.y = y_at_current_frame - y_at_last_frame;

            let timer_over = t >= PLAYER_RISE_HALF_TIME;

            let early_release = !jump && t <= rise_time_at_jump_minus_min_jump;

            if timer_over || early_release {
                commands.entity(player).remove::<PlayerVerticalState>();

                let new_t = {
                    if early_release {
                        fall_time_at_jump_minus_min_jump
                    } else {
                        t - PLAYER_RISE_HALF_TIME + PLAYER_FALL_HALF_TIME
                    }
                };

                commands
                    .entity(player)
                    .insert(PlayerVerticalState::Falling(new_t));
            }
        }
        PlayerVerticalState::Falling(t_ref) => {
            if is_grounded {
                commands.entity(player).remove::<PlayerVerticalState>();

                commands
                    .entity(player)
                    .insert(PlayerVerticalState::Grounded);
                return;
            }

            *t_ref = (*t_ref + delta).min(PLAYER_FALL_HALF_TIME * 2.0);

            let t = *t_ref;
            dbg!("falling", t);

            let y_at_current_frame = compute_y(t, fall_gravity, fall_init_velocity_y);
            let y_at_last_frame = compute_y(t - delta, fall_gravity, fall_init_velocity_y);

            let max_speed = if down {
                PLAYER_FAST_FALL_MAX_SPEED
            } else {
                PLAYER_FALL_MAX_SPEED
            };

            velocity.y = (y_at_current_frame - y_at_last_frame).min(-max_speed);
        }
    }
}

fn compute_y(t: f32, gravity: f32, init_velocity: f32) -> f32 {
    0.5 * gravity * t * t + init_velocity * t
}

fn compute_gravity(half_time: f32) -> f32 {
    (-2.0 * PLAYER_JUMP_HEIGHT) / (half_time * half_time)
}

fn compute_init_velocity_y(half_time: f32) -> f32 {
    (2.0 * PLAYER_JUMP_HEIGHT) / half_time
}

fn compute_time_at_height(init_velocity_y: f32, gravity: f32) -> f32 {
    (-init_velocity_y
        + (init_velocity_y * init_velocity_y
            - 4.0 * 0.5 * gravity * (PLAYER_MIN_JUMP_HEIGHT - PLAYER_JUMP_HEIGHT))
            .sqrt())
        / (2.0 * 0.5 * gravity)
}
