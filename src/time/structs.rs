use super::*;

#[derive(Debug)]
pub struct Ticker {
    current: f32,
    max: Option<f32>,
}

impl Ticker {
    pub fn new(current: Option<f32>, max: Option<f32>) -> Self {
        Self {
            current: current.unwrap_or(0.0),
            max,
        }
    }

    pub fn get(&self) -> f32 {
        self.current
    }

    pub fn tick(&mut self) {
        self.current += FIXED_UPDATE_DELTA_TIME;
    }

    pub fn reset(&mut self) {
        self.current = 0.0;
    }

    pub fn is_over(&self) -> bool {
        if let Some(max) = self.max {
            self.current >= max
        } else {
            false
        }
    }
}
