use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;

use crate::vehicle::{Vehicle, Route};

pub fn render_vehicles(canvas: &mut Canvas<Window>, vehicles: &[Vehicle], vehicle_texture: &Texture) {
    for vehicle in vehicles {
        let rect = Rect::new(
            vehicle.position.0 as i32,
            vehicle.position.1 as i32,
            40, // Width of the vehicle
            20, // Height of the vehicle
        );

        // Rotate the vehicle texture based on the route
        let angle = match vehicle.route {
            Route::Left => 90.0,
            Route::Straight => 0.0,
            Route::Right => 270.0,
        };

        // Render the vehicle texture with the correct orientation
        canvas.copy_ex(vehicle_texture, None, rect, angle, None, false, false).unwrap();
    }
}

pub fn render_background(canvas: &mut Canvas<Window>, background_texture: &Texture) {
    // Clear the canvas with a black color
    canvas.clear();

    // Render the background texture
    canvas.copy(background_texture, None, None).unwrap();
}
