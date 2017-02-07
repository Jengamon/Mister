extern crate slog;
// extern crate specs;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl as gfx_gl;
extern crate gfx_window_glutin as gfx_window;
extern crate glutin;
extern crate rusttype;

extern crate mister_core;

use gfx::format::{DepthStencil, Rgba8};

// Our rudimentary rendering system. Hurrah for laziness.
gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "position",
        tex: [f32; 2] = "tex_coords",
    }

    pipeline rgb {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        picture: gfx::TextureSampler<Rgba8> = "Texture",
        out: gfx::RenderTarget<Rgba8> = "Color",
    }
}

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("MISTER".to_string());
    let (window, mut device, mut factory, rtv, stv) = gfx_window::init::<Rgba8, DepthStencil>(builder);
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

        let enc: gfx::Encoder<gfx_gl::Resources, _> = factory.create_command_buffer().into();

        // draw everything here

        enc.flush();
        
        device.cleanup();
        window.swap_buffers().unwrap();
    }
}
