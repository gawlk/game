use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct PlayerActions {
    pub left: bool,
    pub down: bool,
    pub up: bool,
    pub right: bool,
    pub jump: bool,
    pub shoot: bool,
    pub dash: bool,
}

#[derive(Component)]
pub enum PlayerVerticalState {
    Grounded,
    Rising(f32),
    Falling(f32),
}
