use rand::Rng;

pub enum Route {
    Left,
    Straight,
    Right,
}

pub struct Vehicle {
    pub id: u32,
    pub position: (f32, f32),
    pub velocity: f32,
    pub route: Route,
}

impl Vehicle {
    pub fn new(id: u32, position: (f32, f32)) -> Self {
        let route = Self::random_route();
        Self {
            id,
            position,
            velocity: 0.0,
            route,
        }
    }

    fn random_route() -> Route {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Route::Left,
            1 => Route::Straight,
            _ => Route::Right,
        }
    }

    pub fn update(&mut self) {
        // Update the position based on velocity and direction
        match self.route {
            Route::Left => self.position.0 -= self.velocity,
            Route::Straight => self.position.1 += self.velocity,
            Route::Right => self.position.0 += self.velocity,
        }
    }
}
