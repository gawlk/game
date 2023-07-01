use bevy::prelude::*;

use crate::{
    physics::{RectCollider, Velocity},
    sprites::AnimationBundle,
};

use super::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub actions: PlayerActions,
    pub vertical: PlayerVerticalState,
    pub collider: RectCollider,
    pub velocity: Velocity,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub animation_bundle: AnimationBundle,
}
