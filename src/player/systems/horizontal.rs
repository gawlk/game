use bevy::prelude::*;

use crate::{
    physics::{Velocity, FIXED_UPDATE_DELTA_TIME},
    utils::step_to,
};

use super::*;

pub fn player_horizontal(
    query: Query<(&PlayerActions, &PlayerVerticalState), With<Player>>,
    mut query_mut: Query<&mut Velocity, With<Player>>,
) {
    if query.is_empty() || query_mut.is_empty() {
        return;
    }

    let mut velocity = query_mut.single_mut();

    let (&PlayerActions { left, right, .. }, vertical) = query.single();

    let is_grounded = matches!(vertical, PlayerVerticalState::Grounded);

    let raw_velocity = compute_velocity_x(velocity.raw_x, left, right, is_grounded);

    velocity.raw_x = raw_velocity;
    velocity.x = raw_velocity * FIXED_UPDATE_DELTA_TIME;
}

fn compute_velocity_x(velocity_x: f32, left: bool, right: bool, is_grounded: bool) -> f32 {
    let stopped = velocity_x == 0.0;

    if left != right {
        let max_speed;
        let acceleration;
        let turn_speed;

        if is_grounded {
            max_speed = PLAYER_RUN_MAX_SPEED;
            acceleration = PLAYER_RUN_ACCELERATION;
            turn_speed = PLAYER_RUN_TURN_SPEED;
        } else {
            max_speed = PLAYER_AIR_MAX_SPEED;
            acceleration = PLAYER_AIR_ACCELERATION;
            turn_speed = PLAYER_AIR_TURN_SPEED;
        }

        let max_speed = max_speed * (if left { -1.0 } else { 1.0 });

        let step = if stopped || velocity_x.signum() == max_speed.signum() {
            acceleration
        } else {
            turn_speed
        };

        step_to(velocity_x, max_speed, step)
    } else if !stopped {
        let deceleration = if is_grounded {
            PLAYER_RUN_DECELERATION
        } else {
            PLAYER_AIR_DECELERATION
        };

        step_to(velocity_x, 0.0, deceleration)
    } else {
        0.0
    }
}
