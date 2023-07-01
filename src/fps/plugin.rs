use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use super::*;

pub struct FPSPlugin;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, (spawn_fps,))
            .add_systems(Update, (update_fps,));
    }
}
