use bevy::prelude::*;

use crate::{
    time::FIXED_UPDATE_DELTA_TIME,
    utils::{get_horizontal_signum, step_to},
};

use super::*;

#[derive(Component, Default)]
pub struct PlayerVelocityX {
    si: f32, // m/s
}

impl PlayerVelocityX {
    pub fn compute(&mut self, left: bool, right: bool, grounded: bool) {
        let is_zero = self.is_zero();

        if left || right {
            self.accelerate(grounded, left, is_zero);
        } else if !is_zero {
            self.decelerate(grounded);
        } else {
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        self.set(0.0);
    }

    pub fn set_to_max(&mut self, to_left: bool) {
        let signum = get_horizontal_signum(to_left);

        self.set(signum * PLAYER_WALL_JUMP_SPEED);
    }

    pub fn get_fixed(&self) -> f32 {
        self.si * FIXED_UPDATE_DELTA_TIME
    }

    fn is_zero(&self) -> bool {
        self.si == 0.0
    }

    fn accelerate(&mut self, grounded: bool, to_left: bool, is_zero: bool) {
        let max_speed;
        let acceleration;
        let turn_speed;

        if grounded {
            max_speed = PLAYER_RUN_MAX_SPEED;
            acceleration = PLAYER_RUN_ACCELERATION;
            turn_speed = PLAYER_RUN_TURN_SPEED;
        } else {
            max_speed = PLAYER_AIR_MAX_SPEED;
            acceleration = PLAYER_AIR_ACCELERATION;
            turn_speed = PLAYER_AIR_TURN_SPEED;
        }

        let signum = get_horizontal_signum(to_left);

        let max_speed = max_speed * signum;

        let is_same_direction = self.si.signum() == max_speed.signum();

        let step = if is_zero || is_same_direction {
            acceleration
        } else {
            turn_speed
        };

        self.step_to(max_speed, step);
    }

    fn decelerate(&mut self, grounded: bool) {
        let step = if grounded {
            PLAYER_RUN_DECELERATION
        } else {
            PLAYER_AIR_DECELERATION
        };

        self.step_to(0.0, step);
    }

    fn step_to(&mut self, to: f32, step: f32) {
        self.set(step_to(self.si, to, step));
    }

    fn set(&mut self, value: f32) {
        self.si = value;
    }
}
