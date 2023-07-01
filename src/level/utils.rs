use bevy::prelude::*;

use super::*;

pub fn check_no_walls(
    level: &Res<Level>,
    pixel_index: f32,
    perpendicular_pixel_index: f32,
    perpendicular_collider: f32,
    inverse_matrix_search: bool,
) -> bool {
    let index_desired = from_pixels_to_matrix_index(pixel_index);

    let perpendicular_index_max =
        from_pixels_to_matrix_index(perpendicular_pixel_index + perpendicular_collider - PIXEL);

    let perpendicular_index_min =
        from_pixels_to_matrix_index(perpendicular_pixel_index - perpendicular_collider);

    (perpendicular_index_min..=perpendicular_index_max).all(|perpendicular_index| {
        if inverse_matrix_search {
            level.matrix[perpendicular_index][index_desired] == 0
        } else {
            level.matrix[index_desired][perpendicular_index] == 0
        }
    })
}

fn from_pixels_to_matrix_index(value: f32) -> usize {
    (value / PIXELS_PER_TILE).floor() as usize
}
