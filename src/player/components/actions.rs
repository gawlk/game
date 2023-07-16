use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PlayerActions {
    pub left: bool,
    pub down: bool,
    pub up: bool,
    pub right: bool,

    pub jump: bool,
    pub rise: bool,

    pub shoot: bool,

    pub dash: bool,
}
