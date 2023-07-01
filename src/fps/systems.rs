use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use std::fmt::Write;

use super::*;

pub fn spawn_fps(mut commands: Commands) {
    commands.spawn((
        FPS,
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    style: TextStyle {
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                    ..default()
                }],
                ..default()
            },
            ..default()
        },
    ));
}

pub fn update_fps(
    diagnostics_store: Res<DiagnosticsStore>,
    mut text_query: Query<&mut Text, With<FPS>>,
) {
    if text_query.is_empty() {
        return;
    }

    let mut text = text_query.single_mut();

    let value = &mut text.sections[0].value;
    value.clear();

    if let Some(fps) = get_fps(&diagnostics_store) {
        write!(value, "{}{:.0}", STRING_FORMAT, fps).ok();
    } else {
        write!(value, "{}", STRING_MISSING).ok();
    }
}

fn get_fps(diagnostics: &Res<DiagnosticsStore>) -> Option<f64> {
    diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}
