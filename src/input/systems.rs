use bevy::prelude::*;

use crate::player::PlayerActions;

pub fn process_player_inputs(
    input: Res<Input<KeyCode>>,
    mut query_player_actions: Query<&mut PlayerActions>,
) {
    if query_player_actions.is_empty() {
        return;
    }

    let mut player_actions = query_player_actions.single_mut();

    player_actions.left = input.any_pressed([KeyCode::A, KeyCode::Left]);

    player_actions.down = input.any_pressed([KeyCode::S, KeyCode::Down]);

    player_actions.up = input.any_pressed([KeyCode::W, KeyCode::Up]);

    player_actions.right = input.any_pressed([KeyCode::D, KeyCode::Right]);

    player_actions.jump = input.any_pressed([KeyCode::Space]);
}
