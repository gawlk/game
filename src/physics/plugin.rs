use bevy::prelude::*;

use super::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(FIXED_UPDATE_DELTA_TIME));
    }
}
