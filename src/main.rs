use bevy::{prelude::*, render::texture::ImageSampler, window::WindowResolution};

mod camera;
mod colliders;
mod fps;
mod gizmos;
mod level;
mod player;
mod sprites;
mod time;
mod utils;
mod walls;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: ImageSampler::nearest_descriptor(),
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1024.0, 720.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            camera::CameraPlugin,
            fps::FPSPlugin,
            gizmos::GizmosPlugin,
            level::LevelPlugin,
            player::PlayerPlugin,
            sprites::SpritesPlugin,
            time::TimePlugin,
            walls::WallsPlugin,
        ))
        .run();

    Ok(())
}
