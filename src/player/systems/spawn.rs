use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::{
    level::{Level, PIXELS_PER_TILE},
    physics::{RectCollider, Velocity},
    sprites::{AnimationBundle, AnimationIndexes, AnimationTimer},
};

use super::*;

pub fn spawn_player(
    mut commands: Commands,
    query_player: Query<(), With<Player>>,
    level: Res<Level>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !query_player.is_empty() {
        return;
    }

    let coords = level
        .spawns
        .choose(&mut rand::thread_rng())
        .map(|coords| coords.to_owned());

    if let Some((x, y)) = coords {
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
            player: Player,
            actions: PlayerActions::default(),
            vertical: PlayerVerticalState::Falling(PLAYER_FALL_HALF_TIME),
            collider: RectCollider(Vec2::new(2.5, 9.5)),
            velocity: Velocity::default(),
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32) * PIXELS_PER_TILE,
                        (y as f32) * PIXELS_PER_TILE,
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
        });
    }
}
