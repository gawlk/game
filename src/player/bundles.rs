use bevy::prelude::*;

use crate::{
    colliders::RectCollider,
    sprites::{AnimationBundle, SpriteDirection},
};

use super::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,
    pub actions: PlayerActions,
    pub vertical: PlayerVerticalState,
    pub collider: RectCollider,
    pub velocity: PlayerVelocityBundle,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub animation_bundle: AnimationBundle,
    pub direction: SpriteDirection,
}

#[derive(Bundle, Default)]
pub struct PlayerVelocityBundle {
    pub x: PlayerVelocityX,
    pub y: PlayerVelocityY,
}
