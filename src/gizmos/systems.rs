#![allow(dead_code)]

use bevy::prelude::*;

use crate::{
    colliders::RectCollider,
    level::{Level, PIXELS_PER_TILE},
    player::PlayerMarker,
};

pub fn gizmos_origin(mut gizmos: Gizmos) {
    gizmos.line_2d(
        Vec2::new(-PIXELS_PER_TILE, 0.0),
        Vec2::new(PIXELS_PER_TILE, 0.0),
        Color::GREEN,
    );

    gizmos.line_2d(
        Vec2::new(0.0, -PIXELS_PER_TILE),
        Vec2::new(0.0, PIXELS_PER_TILE),
        Color::GREEN,
    );
}

pub fn gizmos_static_collisions(mut gizmos: Gizmos, level: Res<Level>) {
    level.matrix.iter().enumerate().for_each(|(index_x, ys)| {
        ys.iter().enumerate().for_each(|(index_y, value)| {
            if value != &0 {
                gizmos.rect_2d(
                    Vec2::new(
                        (index_x as f32) * PIXELS_PER_TILE + PIXELS_PER_TILE / 2.0,
                        (index_y as f32) * PIXELS_PER_TILE + PIXELS_PER_TILE / 2.0,
                    ),
                    0.0,
                    Vec2::splat(PIXELS_PER_TILE),
                    Color::YELLOW,
                )
            }
        })
    });
}

pub fn gizmos_player(
    mut gizmos: Gizmos,
    query_player_transform: Query<(&Transform, &RectCollider), With<PlayerMarker>>,
) {
    if query_player_transform.is_empty() {
        return;
    }

    let (transform, collider) = query_player_transform.single();

    let translation = transform.translation.truncate();

    let ray_length = 8.0;

    gizmos.ray_2d(
        Vec2::new(translation.x - ray_length / 2.0, translation.y),
        Vec2::new(ray_length, 0.0),
        Color::GREEN,
    );

    gizmos.ray_2d(
        Vec2::new(translation.x, translation.y - ray_length / 2.0),
        Vec2::new(0.0, ray_length),
        Color::GREEN,
    );

    gizmos.rect_2d(
        translation,
        0.0,
        Vec2::new(collider.x * 2.0, collider.y * 2.0),
        Color::RED,
    );
}
