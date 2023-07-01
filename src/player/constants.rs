// All speeds are in SI (meters/second)

pub const PLAYER_RUN_MAX_SPEED: f32 = 15.0;
pub const PLAYER_RUN_ACCELERATION: f32 = PLAYER_RUN_MAX_SPEED / 6.0;
pub const PLAYER_RUN_DECELERATION: f32 = PLAYER_RUN_ACCELERATION / 3.0;
pub const PLAYER_RUN_TURN_SPEED: f32 = PLAYER_RUN_MAX_SPEED / 2.0;

pub const PLAYER_AIR_MAX_SPEED: f32 = PLAYER_RUN_MAX_SPEED;
pub const PLAYER_AIR_ACCELERATION: f32 = PLAYER_AIR_MAX_SPEED / 6.0;
pub const PLAYER_AIR_DECELERATION: f32 = PLAYER_AIR_MAX_SPEED / 3.0;
pub const PLAYER_AIR_TURN_SPEED: f32 = PLAYER_AIR_MAX_SPEED / 2.0;

pub const PLAYER_RISE_GRAVITY_MULTIPLIER: f32 = 1.0;
pub const PLAYER_FALL_GRAVITY_MULTIPLIER: f32 = 1.5;

pub const PLAYER_FALL_MAX_SPEED: f32 = 20.0;
pub const PLAYER_FAST_FALL_MAX_SPEED: f32 = PLAYER_FALL_MAX_SPEED * 1.5;
pub const PLAYER_FALL_ACCELERATION: f32 =
    (PLAYER_FAST_FALL_MAX_SPEED - PLAYER_FALL_MAX_SPEED) / 6.0;
pub const PLAYER_FALL_DECELERATION: f32 =
    (PLAYER_FAST_FALL_MAX_SPEED - PLAYER_FALL_MAX_SPEED) / 3.0;

pub const PLAYER_JUMP_HEIGHT: f32 = 4.5;
pub const PLAYER_MIN_JUMP_HEIGHT: f32 = 0.5;
pub const PLAYER_JUMP_TIME: f32 = 0.6;

pub const PLAYER_RISE_HALF_TIME: f32 = PLAYER_JUMP_TIME * PLAYER_FALL_GRAVITY_MULTIPLIER
    / (PLAYER_RISE_GRAVITY_MULTIPLIER + PLAYER_FALL_GRAVITY_MULTIPLIER);
pub const PLAYER_FALL_HALF_TIME: f32 = PLAYER_JUMP_TIME - PLAYER_RISE_HALF_TIME;