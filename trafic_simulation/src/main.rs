extern crate sdl2;

mod vehicle;
mod intersection;
mod animation;
mod stats;
mod image_loader;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use intersection::IntersectionManager;
use sdl2::image::InitFlag;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();

    let window = video_subsystem
        .window("Smart Intersection", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let background_texture = image_loader::load_texture(&texture_creator, "assets/road.jpg")
        .expect("Could not load background image");
    let vehicle_texture = image_loader::load_texture(&texture_creator, "assets/car-down.png")
        .expect("Could not load vehicle image");

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut manager = IntersectionManager::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    manager.display_stats();
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => manager.add_vehicle((400.0, 0.0)),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => manager.add_vehicle((400.0, 600.0)),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => manager.add_vehicle((780.0, 300.0)),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => manager.add_vehicle((0.0, 300.0)),
                _ => {}
            }
        }
        manager.update();

        // Render the background image
        animation::render_background(&mut canvas, &background_texture);

        // Render the vehicles
        manager.render(&mut canvas, &vehicle_texture);

        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }
}
