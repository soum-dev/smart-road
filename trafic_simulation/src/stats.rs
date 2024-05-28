pub struct Statistics {
    max_vehicles: usize,
    max_velocity: f32,
    min_velocity: f32,
    max_time: f32,
    min_time: f32,
    close_calls: usize,
}



impl Statistics {
    pub fn new() -> Self {
        Self {
            max_vehicles: 0,
            max_velocity: 0.0,
            min_velocity: f32::MAX,
            max_time: 0.0,
            min_time: f32::MAX,
            close_calls: 0,
        }
    }

    pub fn update(&mut self, vehicles: &[crate::vehicle::Vehicle]) {
        // Update statistics based on vehicle states
    }

    pub fn display(&self) {
        // Display statistics in a window or console
    }
}
