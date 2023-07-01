use bevy::prelude::*;

use crate::{
    level::{check_no_walls, Level, PIXELS_PER_TILE},
    physics::{RectCollider, Velocity},
};

use super::*;

pub fn translate_player(
    level: Res<Level>,
    mut query_player: Query<(&Velocity, &mut Transform, &RectCollider), With<Player>>,
) {
    if query_player.is_empty() {
        return;
    }

    let (velocity, mut transform, rect_collider) = query_player.single_mut();

    let corrected_velocity_x = velocity.x * PIXELS_PER_TILE;
    let corrected_velocity_y = velocity.y * PIXELS_PER_TILE;

    if corrected_velocity_x == 0.0 && corrected_velocity_y == 0.0 {
        return;
    } else if corrected_velocity_x >= PIXELS_PER_TILE || corrected_velocity_y >= PIXELS_PER_TILE {
        panic!("A velocity higher than the number of pixels isn't supported");
    }

    let collider_x = rect_collider.x;
    let collider_y = rect_collider.y;

    let translation = &mut transform.translation;

    if corrected_velocity_x != 0.0 {
        translation.x = compute_translation(
            &level,
            translation.x,
            translation.y,
            collider_x,
            collider_y,
            corrected_velocity_x,
            false,
        );
    }

    if corrected_velocity_y != 0.0 {
        translation.y = compute_translation(
            &level,
            translation.y,
            translation.x,
            collider_y,
            collider_x,
            corrected_velocity_y,
            true,
        );
    }
}

fn compute_translation(
    level: &Res<Level>,
    origin: f32,
    perpendicular_origin: f32,
    collider: f32,
    perpendicular_collider: f32,
    velocity: f32,
    inverse_matrix_search: bool,
) -> f32 {
    let desired = origin + velocity;

    let sign = velocity.signum();

    let collider_directed = collider * sign;

    let no_walls = check_no_walls(
        level,
        desired + collider_directed,
        perpendicular_origin,
        perpendicular_collider,
        inverse_matrix_search,
    );

    if no_walls {
        desired
    } else {
        let pixels_from_wall = {
            let rest = (origin + collider_directed) % PIXELS_PER_TILE;

            if sign == -1.0 {
                -rest
            } else if rest != 0.0 {
                PIXELS_PER_TILE - rest
            } else {
                0.0
            }
        };

        origin + pixels_from_wall
    }
}
