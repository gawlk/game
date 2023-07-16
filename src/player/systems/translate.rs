use bevy::prelude::*;

use crate::{
    colliders::RectCollider,
    level::{Level, PIXELS_PER_METER, PIXELS_PER_TILE},
};

use super::*;

pub fn translate_player(
    level: Res<Level>,
    mut query: Query<(
        &PlayerVelocityX,
        &PlayerVelocityY,
        &RectCollider,
        &mut Transform,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (velocity_x, velocity_y, rect_collider, mut transform) = query.single_mut();

    let fixed_velocity_x = velocity_x.get_fixed() * PIXELS_PER_METER;

    let fixed_velocity_y = velocity_y.get_fixed() * PIXELS_PER_METER;

    if fixed_velocity_x == 0.0 && fixed_velocity_y == 0.0 {
        return;
    } else if fixed_velocity_x >= PIXELS_PER_TILE || fixed_velocity_y >= PIXELS_PER_TILE {
        panic!("A velocity higher than the number of pixels in a tile isn't supported");
    }

    let collider_x = rect_collider.x;
    let collider_y = rect_collider.y;

    let translation = &mut transform.translation;

    if fixed_velocity_x != 0.0 {
        translation.x = compute_translation(
            &level,
            translation.x,
            translation.y,
            collider_x,
            collider_y,
            fixed_velocity_x,
            false,
        );
    }

    if fixed_velocity_y != 0.0 {
        translation.y = compute_translation(
            &level,
            translation.y,
            translation.x,
            collider_y,
            collider_x,
            fixed_velocity_y,
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

    let no_walls = level.check_no_walls(
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
