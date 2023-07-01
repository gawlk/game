pub fn step_to(from: f32, to: f32, step: f32) -> f32 {
    if from != to {
        if from >= to {
            (from - step).max(to)
        } else {
            (from + step).min(to)
        }
    } else {
        from
    }
}
