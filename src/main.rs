extern crate slog;
// extern crate specs;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl as gfx_gl;
extern crate gfx_window_glutin as gfx_window;
extern crate glutin;
extern crate rusttype;
extern crate rand;
extern crate palette;

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
        picture: gfx::TextureSampler<[f32; 4]> = "Texture",
        out: gfx::RenderTarget<Rgba8> = "Color",
    }
}

const VERTEX_SRC: &'static [u8] = include_bytes!("test.vert");
const FRGMNT_SRC: &'static [u8] = include_bytes!("test.frag");

fn main() {
    use gfx::traits::{FactoryExt};
    use gfx::{Factory};
    let builder = glutin::WindowBuilder::new()
        .with_title("MISTER".to_string());
    let (window, mut device, mut factory, rtv, stv) = gfx_window::init::<Rgba8, DepthStencil>(builder);
    println!("Main function!");

    // TODO: Make a rudimentary rendering system to quickly draw an image to the screen.

    let verticies = vec![
        Vertex{ pos: [-1.0, 1.0], tex: [0.0, 0.0] },
        Vertex{ pos: [1.0, 1.0], tex: [1.0, 0.0] },
        Vertex{ pos: [1.0, -1.0], tex: [1.0, 1.0] },
        Vertex{ pos: [-1.0, -1.0], tex: [0.0, 1.0] },
    ];
    let indices: &[u16] = &[0, 1, 2, 0, 2, 3];
    let (buf, slice) = factory.create_vertex_buffer_with_slice(&verticies, indices);
    // let program = factory.link_program(VERTEX_SRC, FRGMNT_SRC).unwrap();
    let pso = factory.create_pipeline_simple(VERTEX_SRC, FRGMNT_SRC, rgb::new()).unwrap();

    let (WIDTH, HEIGHT) = (640, 480);

    let mut image = mister_core::Image::new(WIDTH * HEIGHT);
    image.create_channel(0.0f32); // R
    image.create_channel(0.0f32); // G
    image.create_channel(0.0f32); // B
    image.create_channel(1.0f32); // A

    // Generate some sample image data
    // NOTE: API to be formalized
    let mut rng = rand::thread_rng();
    for i in 0..3 {
        use rand::Rng;
        let channel = image.channel_mut(i);
        let mut value = rng.gen(); // Be slightly faster, We just want A value. We don't actually care what.
        for x in 0..channel.len() {
            channel[x] = value;
            if (x % WIDTH == 0) { value = rng.gen(); }
        }
    }

    // Convert channels into pixels
    let mut colors = vec![];
    for i in 0..image.len() {
        colors.push(palette::Rgba::new(image.channel(0)[i], image.channel(1)[i], image.channel(2)[i], image.channel(3)[i]));
    }

    // Convert linear pixels into gamma-corrected pixels
    let colors: Vec<palette::pixel::Srgb> = colors.into_iter().map(|x| x.into()).collect::<Vec<_>>();

    use gfx::texture::{Kind, AaMode, FilterMethod, SamplerInfo, WrapMode};

    // Upload pixel data to a texture
    let data = colors.iter().flat_map(|x| vec![(x.red * 255.0) as u8, (x.green * 255.0) as u8, (x.blue * 255.0) as u8, (x.alpha * 255.0) as u8]).collect::<Vec<_>>();
    let (tex, texview) = factory.create_texture_immutable_u8::<Rgba8>(Kind::D2(WIDTH as u16, HEIGHT as u16, AaMode::Single), &[&data]).unwrap();
    println!("LOD COLOR");
    let sampler = factory.create_sampler(SamplerInfo::new(FilterMethod::Scale, WrapMode::Clamp));

    // Create data to draw
    let data = rgb::Data {
        vbuf: buf,
        out: rtv.clone(),
        picture: (texview, sampler)
    };

    'system: loop {
        use gfx::{Device};
        // use gfx::traits::{FactoryExt};

        for event in window.poll_events() {
            use glutin::Event;
            match event {
                Event::Closed => break 'system,
                _ => ()
            }
        }

        let mut enc: gfx::Encoder<gfx_gl::Resources, _> = factory.create_command_buffer().into();

        // draw everything here
        enc.clear(&rtv, [0.0, 0.0, 0.0, 1.0]);
        // enc.
        enc.draw(&slice, &pso, &data);

        enc.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
