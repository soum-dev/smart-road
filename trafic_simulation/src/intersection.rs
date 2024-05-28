use crate::vehicle::Vehicle;
use sdl2::render::Texture;

pub struct IntersectionManager {
    pub vehicles: Vec<Vehicle>, // Make vehicles public so it can be accessed from main
    stats: crate::stats::Statistics,
}

impl IntersectionManager {
    pub fn new() -> Self {
        Self {
            vehicles: Vec::new(),
            stats: crate::stats::Statistics::new(),
        }
    }

    pub fn add_vehicle(&mut self, position: (f32, f32)) {
        let id = self.vehicles.len() as u32 + 1;
        let vehicle = Vehicle::new(id, position);
        self.vehicles.push(vehicle);
    }

    pub fn update(&mut self) {
        for vehicle in &mut self.vehicles {
            vehicle.update();
            // Check for collisions and adjust velocities
        }
        self.update_stats();
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, vehicle_texture: &Texture) {
        crate::animation::render_vehicles(canvas, &self.vehicles, vehicle_texture);
    }

    fn update_stats(&mut self) {
        // Update statistics based on vehicle states
    }

    pub fn display_stats(&self) {
        self.stats.display();
    }
}
