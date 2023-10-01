use bevy::prelude::*;

use crate::{
    colliders::RectCollider,
    level::{Level, PIXELS_PER_METER},
    sprites::{AnimationBundle, AnimationIndexes, AnimationTimer, SpriteDirection},
};

use super::*;

pub fn spawn_player(
    mut commands: Commands,
    level: Res<Level>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query_player: Query<(), With<PlayerMarker>>,
) {
    if !query_player.is_empty() {
        return;
    }

    let spawn = level.get_random_spawn();

    if let Some((x, y)) = spawn {
        let texture_handle = asset_server.load("aseprite/base_idle.png");

        let column_number = 1;
        let row_number = 1;
        let frame_number = row_number * column_number;

        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(13.0, 21.0),
            column_number,
            row_number,
            None,
            None,
        );

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let animation_indexes = AnimationIndexes {
            first: 0,
            last: frame_number - 1,
        };

        commands.spawn(PlayerBundle {
            marker: PlayerMarker,
            actions: PlayerActions::default(),
            vertical: PlayerVerticalState::default_falling(),
            collider: RectCollider(Vec2::new(2.5, 9.5)),
            velocity: PlayerVelocityBundle::default(),
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32) * PIXELS_PER_METER,
                        (y as f32) * PIXELS_PER_METER,
                        0.0,
                    ),
                    ..default()
                },
                ..default()
            },
            animation_bundle: AnimationBundle {
                animation_indexes,
                animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            },
            direction: SpriteDirection::Right,
        });
    }
}
