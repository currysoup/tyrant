#![allow(dead_code)]

extern crate gl;
extern crate image;
extern crate sdl2;
extern crate time;

use assets::{AssetManager, Texture};
use frametime::FrameTime;
use gfx::{Mesh, Window};
use gfx::vertex::VertexPositionColor;
use math::{Color, Vector3};
use system::EntitySystem;

mod assets;
mod controller;
mod frametime;
mod gfx;
mod math;
mod system;

fn main() {
    let mut window = Window::new("Tyrant", 800, 600);
    let mut system = EntitySystem::new();
    let mut time = FrameTime::new();

    let assets = AssetManager::new("assets");


    let verts: Vec<VertexPositionColor> = vec![
        VertexPositionColor::new(Vector3::new(0.0, 0.5, 0.0), Color::new(1.0, 0.0, 0.0, 1.0)),
        VertexPositionColor::new(Vector3::new(0.5, -0.5, 0.0), Color::new(0.0, 1.0, 0.0, 1.0)),
        VertexPositionColor::new(Vector3::new(-0.5, -0.5, 0.0), Color::new(0.0, 0.0, 1.0, 1.0)),
        ];

    let texture: Texture = assets.load("hal.png");
    let effect: Effect = assets.load("test.effect");

    let mesh = Mesh::new(effect, verts);

    loop {
        window.process_events();
        if window.should_close() {
            break;
        }

        time.update();

        mesh.draw();
        //system.update(&time);

        window.present();
    }
}
