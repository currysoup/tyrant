#![allow(dead_code)]

extern crate time;

extern crate backend_sdl;

use frametime::FrameTime;
use system::EntitySystem;

mod controller;
mod frametime;
mod math;
mod system;

fn main() {
    let mut window = backend_sdl::Window::new("Tyrant", 800, 600);
    let mut system = EntitySystem::new();
    let mut time = FrameTime::new();

    let test_ent = system.new_entity().transformable(math::Vector3::unit_z()).build();


    loop {
        window.process_events();
        if window.should_close() {
            break;
        }

        time.update();

        let player_vel = controller::process();

        system.set_velocity(test_ent, player_vel);

        system.update(&time);
        println!("lol: {}", system.get_position(test_ent));


        window.present();
    }
}
