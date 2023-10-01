use bevy::prelude::*;

use super::structs::Ticker;

#[derive(Component, Deref, DerefMut)]
pub struct TickerComponent(Ticker);

impl TickerComponent {
    pub fn new(current: Option<f32>, max: Option<f32>) -> Self {
        Self(Ticker::new(current, max))
    }
}
