extern crate slog;
// extern crate specs;
extern crate gfx;
extern crate gfx_device_gl as gfx_gl;
extern crate gfx_window_glutin as gfx_window;
extern crate glutin;
extern crate rusttype;

extern crate mister_core;

use gfx::format::{DepthStencil, Rgba8};

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("MISTER".to_string());
    let (window, device, factory, rtv, stv) = gfx_window::init::<Rgba8, DepthStencil>(builder);
    println!("Main function!");

    // TODO: Make a rudimentary rendering system to quickly draw an image to the screen.

    'system: loop {
        for event in window.poll_events() {
            use glutin::Event;
            match event {
                Event::Closed => break 'system,
                _ => ()
            }
        }

        // draw everything here

        window.swap_buffers().unwrap();
    }
}
