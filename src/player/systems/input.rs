use bevy::prelude::*;

use crate::player::PlayerActions;

pub fn process_player_inputs(
    input: Res<Input<KeyCode>>,
    mut query_player_actions: Query<&mut PlayerActions>,
) {
    if query_player_actions.is_empty() {
        return;
    }

    let key_codes_left = [KeyCode::A, KeyCode::Left];
    let key_codes_down = [KeyCode::S, KeyCode::Down];
    let key_codes_up = [KeyCode::W, KeyCode::Up];
    let key_codes_right = [KeyCode::D, KeyCode::Right];
    let key_codes_jump = [KeyCode::Space];

    let mut player_actions = query_player_actions.single_mut();

    let left_pressed = input.any_pressed(key_codes_left);
    let down_pressed = input.any_pressed(key_codes_down);
    let up_pressed = input.any_pressed(key_codes_up);
    let right_pressed = input.any_pressed(key_codes_right);

    player_actions.left = left_pressed && !right_pressed;
    player_actions.down = down_pressed && !up_pressed;
    player_actions.up = up_pressed && !down_pressed;
    player_actions.right = right_pressed && !left_pressed;

    player_actions.jump = input.any_just_pressed(key_codes_jump);

    player_actions.rise = input.any_pressed(key_codes_jump);

    if player_actions.jump {
        println!("Jump pressed")
    }
}
