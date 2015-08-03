extern crate time;

extern crate backend_sdl;

mod frametime;
mod math;
mod system;

fn main() {
    let mut window = backend_sdl::Window::new("Tyrant", 800, 600);

    loop {
        window.process_events();
        if window.should_close() {
            break;
        }

        window.present();
    }
}
