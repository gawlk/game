use bevy::prelude::*;

pub fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("ldtk/world/png/Level_0__IntGrid.png");

    let sprite_bundle = SpriteBundle {
        texture: texture_handle,
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..default()
        },
        ..default()
    };

    commands.spawn(sprite_bundle);
}
