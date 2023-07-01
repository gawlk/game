use bevy::{prelude::*, render::texture::ImageSampler, window::WindowResolution};

mod camera;
mod fps;
mod gizmos;
mod input;
mod level;
mod physics;
mod player;
mod sprites;
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
            input::InputPlugin,
            level::LevelPlugin,
            player::PlayerPlugin,
            physics::PhysicsPlugin,
            sprites::SpritesPlugin,
            walls::WallsPlugin,
        ))
        .run();

    Ok(())
}
