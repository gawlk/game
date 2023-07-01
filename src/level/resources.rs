use bevy::prelude::*;

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
}
