use bevy::prelude::*;

use super::*;

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate_sprite,));
    }
}
