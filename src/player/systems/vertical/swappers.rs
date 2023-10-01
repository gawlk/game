use super::*;

pub fn swap_state_to_grounded(
    commands: &mut Commands,
    player: Entity,
    velocity_y: &mut PlayerVelocityY,
) {
    println!("Swap to ground");

    velocity_y.reset();

    commands.entity(player).remove::<CoyoteTimeBundle>();
    commands.entity(player).remove::<WallJumpLockBundle>();

    commands
        .entity(player)
        .insert(PlayerVerticalState::Grounded);
}

pub fn swap_state_to_rising(commands: &mut Commands, player: Entity) {
    println!("Swap to rise");

    commands.entity(player).remove::<CoyoteTimeBundle>();
    commands.entity(player).remove::<JumpBufferBundle>();

    commands
        .entity(player)
        .insert(PlayerVerticalState::default_rising());
}

pub fn swap_state_to_falling(commands: &mut Commands, player: Entity, time: Option<f32>) {
    println!("Swap to fall");

    let state = {
        if let Some(time) = time {
            PlayerVerticalState::new_falling(time)
        } else {
            PlayerVerticalState::default_falling()
        }
    };

    commands.entity(player).insert(state);
}
