use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::{colliders::RectCollider, utils::get_horizontal_signum};

use super::*;

#[derive(Resource)]
pub struct Level {
    pub width: u32,
    pub height: u32,
    pub matrix: Vec<Vec<u32>>,
    pub spawns: Vec<(u32, u32)>,
}

impl Level {
    pub fn new() -> Self {
        let walls_matrix_image =
            image::open("assets/ldtk/world/png/Level_0__IntGrid-int.png").unwrap();

        let walls_matrix_rgba = walls_matrix_image.as_rgba8().unwrap();

        let (width, height) = walls_matrix_rgba.dimensions();

        let mut matrix = vec![vec![0; height as usize]; width as usize];

        let mut spawns = vec![];

        (0..width).for_each(|x| {
            (0..height).for_each(|y| {
                let tile_present = walls_matrix_rgba.get_pixel(x, y)[3] != 0;

                if tile_present {
                    let y = height - y - 1;

                    matrix[x as usize][y as usize] = 1;

                    if x > 0 {
                        // TODO: 3 shouldn't be hard coded
                        let center = (x, y + 3);

                        // XXX
                        // XXX
                        // X.X
                        // XXX
                        // XXX
                        //  -

                        // TODO: -1 and +1 shouldn't be hard coded
                        // Use player's sprite dimensions
                        let all_above_are_emtpy = (x - 1..=x + 1)
                            .flat_map(|x| {
                                // TODO: 5 shouldn't be hard coded
                                (y + 1..=y + 5)
                                    .map(|y| {
                                        x < width
                                            && y < height
                                            && matrix[x as usize][y as usize] == 0
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .all(|empty| empty);

                        if all_above_are_emtpy {
                            spawns.push(center);
                        }
                    }
                }
            });
        });

        Self {
            width,
            height,
            matrix,
            spawns,
        }
    }

    pub fn get_random_spawn(&self) -> Option<(u32, u32)> {
        self.spawns
            .choose(&mut rand::thread_rng())
            .map(|coords| coords.to_owned())
    }

    pub fn touches_floor(&self, transform: &Transform, collider: &RectCollider) -> bool {
        !self.check_no_walls(
            transform.translation.y - collider.y - PIXEL,
            transform.translation.x,
            collider.x,
            true,
        )
    }

    pub fn touches_ceiling(&self, transform: &Transform, collider: &RectCollider) -> bool {
        !self.check_no_walls(
            transform.translation.y + collider.y + PIXEL,
            transform.translation.x,
            collider.x,
            true,
        )
    }

    pub fn touches_wall(
        &self,
        transform: &Transform,
        collider: &RectCollider,
        padding: f32,
        left: bool,
    ) -> bool {
        let signum = get_horizontal_signum(left);

        !self.check_no_walls(
            transform.translation.x + signum * (collider.x + padding + PIXEL),
            transform.translation.y,
            collider.y,
            false,
        )
    }

    // TODO: Fix rounding
    // For example touches_floor returns true when less than 8 pixels between collider and floor
    pub fn check_no_walls(
        &self,
        pixel_index: f32,
        perpendicular_pixel_index: f32,
        perpendicular_collider: f32,
        inverse_matrix_search: bool,
    ) -> bool {
        let index_desired = self.get_matrix_index_from_pixels(pixel_index);

        let perpendicular_index_max = self.get_matrix_index_from_pixels(
            perpendicular_pixel_index + perpendicular_collider - PIXEL,
        );

        let perpendicular_index_min =
            self.get_matrix_index_from_pixels(perpendicular_pixel_index - perpendicular_collider);

        (perpendicular_index_min..=perpendicular_index_max).all(|perpendicular_index| {
            if inverse_matrix_search {
                self.matrix[perpendicular_index][index_desired] == 0
            } else {
                self.matrix[index_desired][perpendicular_index] == 0
            }
        })
    }

    fn get_matrix_index_from_pixels(&self, value: f32) -> usize {
        (value / PIXELS_PER_TILE).floor() as usize
    }
}
