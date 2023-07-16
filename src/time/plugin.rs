use bevy::prelude::*;

use super::*;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(FIXED_UPDATE_DELTA_TIME));
    }
}
