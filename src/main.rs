// Do not rexport
extern crate backend_sdl;

pub mod math;

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
