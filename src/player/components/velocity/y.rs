use bevy::prelude::*;

use crate::time::FIXED_UPDATE_DELTA_TIME;

use super::*;

#[derive(Component, Default)]
pub struct PlayerVelocityY {
    fixed: f32, // m/s * delta

    rise_gravity: f32,
    fall_gravity: f32,

    rise_initial_velocity: f32,
    fall_initial_velocity: f32,

    pub rise_time_at_max_height_minus_min_height: f32,
    pub fall_time_at_max_height_minus_min_height: f32,
}

impl PlayerVelocityY {
    pub fn get_fixed(&self) -> f32 {
        self.fixed
    }

    pub fn compute_rising(&mut self, t: f32) {
        self.set(Self::compute_velocity(
            t,
            self.rise_gravity,
            self.rise_initial_velocity,
        ));
    }

    pub fn compute_falling(&mut self, t: f32, max_speed: f32) {
        // TODO: Decelerate if speed is too high instead of instant reduction

        let velocity = Self::compute_velocity(t, self.fall_gravity, self.fall_initial_velocity);

        let max_fall_speed = -max_speed * FIXED_UPDATE_DELTA_TIME;

        dbg!(velocity, max_fall_speed);

        self.set(velocity.max(max_fall_speed));
    }

    pub fn reset(&mut self) {
        self.set(0.0);
    }

    pub fn reset_states_variables(&mut self) {
        self.rise_gravity = Self::compute_gravity(PLAYER_RISE_HALF_TIME);
        self.fall_gravity = Self::compute_gravity(PLAYER_FALL_HALF_TIME);

        self.rise_initial_velocity = Self::compute_initial_velocity(PLAYER_RISE_HALF_TIME);
        self.fall_initial_velocity = Self::compute_initial_velocity(PLAYER_FALL_HALF_TIME);

        let max_height_minus_min_height = PLAYER_JUMP_HEIGHT - PLAYER_MIN_JUMP_HEIGHT;

        self.rise_time_at_max_height_minus_min_height = Self::compute_first_time_at_height(
            self.rise_initial_velocity,
            self.rise_gravity,
            max_height_minus_min_height,
        );

        self.fall_time_at_max_height_minus_min_height = Self::compute_first_time_at_height(
            self.fall_initial_velocity,
            self.fall_gravity,
            max_height_minus_min_height,
        );
    }

    pub fn set(&mut self, value: f32) {
        self.fixed = value;
    }

    fn compute_gravity(half_time: f32) -> f32 {
        (-2.0 * PLAYER_JUMP_HEIGHT) / (half_time * half_time)
    }

    fn compute_initial_velocity(half_time: f32) -> f32 {
        // Also equals to = gravity * timeToJumpApex;
        (2.0 * PLAYER_JUMP_HEIGHT) / half_time
    }

    fn compute_first_time_at_height(init_velocity_y: f32, gravity: f32, height: f32) -> f32 {
        let a = 0.5 * gravity;
        let b = init_velocity_y;
        let c = -height;

        (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a)
    }

    fn compute_velocity(t: f32, gravity: f32, initial_velocity: f32) -> f32 {
        let y_at_current_frame = Self::compute_y(t, gravity, initial_velocity);

        let previous_t = t - FIXED_UPDATE_DELTA_TIME;

        if previous_t.is_sign_negative() {
            panic!("Shouldn't compute_y with negative time");
        }

        let y_at_last_frame = Self::compute_y(previous_t, gravity, initial_velocity);

        y_at_current_frame - y_at_last_frame
    }

    fn compute_y(t: f32, gravity: f32, init_velocity: f32) -> f32 {
        0.5 * gravity * t * t + init_velocity * t
    }
}
