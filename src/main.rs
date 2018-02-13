extern crate slog;
// extern crate specs;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl as gfx_gl;
extern crate gfx_window_glutin as gfx_window;
extern crate glutin;
//extern crate rusttype;
extern crate rand;
extern crate palette;
//extern crate conrod;
extern crate nalgebra as na;
extern crate mister_core;
extern crate mister_gui;

use glutin::{GlContext, GlRequest};
use glutin::Api::OpenGl;

use gfx::format::{DepthStencil, Rgba8, Srgba8};
use gfx::texture::Mipmap;

// Our rudimentary rendering system. Hurrah for laziness.
gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "position",
        tex: [f32; 2] = "tex_coords",
    }

    pipeline rgb {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        picture: gfx::TextureSampler<[f32; 4]> = "Texture",
        // mvp: gfx::Global<[[f32; 3]; 3]> = "MVP",
        out: gfx::RenderTarget<Srgba8> = "Color",
    }
}

const VERTEX_SRC: &'static [u8] = include_bytes!("test.vert");
const FRGMNT_SRC: &'static [u8] = include_bytes!("test.frag");

trait ToU32 {
    fn to_u32(self) -> u32;
}

impl ToU32 for f32 {
    fn to_u32(self) -> u32 {
        unsafe { std::mem::transmute::<f32, u32>(self) }
    }
}

fn main() {
    use gfx::traits::{FactoryExt};
    use gfx::{Factory};
    let mut events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("MISTER".to_string())
        .with_dimensions(640, 480);
    let contextbuilder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl,(3,2)))
        .with_vsync(true);
    let (mut window, mut device, mut factory, mut rtv, mut stv) = 
        gfx_window::init::<Srgba8, DepthStencil>(builder, contextbuilder, &events_loop);
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

    let (WIDTH, HEIGHT) = (1, 480);

    // Here, we cheat. Let me explain.
    // We originally were generating a pixel per screen pixel (640*480 pixels), which is a lot.
    // We then switched to generating pixels by row, and only changing the value then.
    //
    // But I realized that our shader streches "pixels" to fit the screen, so why not just render 1 pixel,
    // and stretch it across the screen. Shader stuff works, because it works in screen space,
    // not pixel, so hurrah! We got it!

    use mister_core::ImageFormat;
    use palette::Colora;
    let mut image = mister_core::RgbaImage::new(WIDTH, HEIGHT);

    // Generate some sample image data
    // NOTE: API to be formalized
    let mut rng = rand::thread_rng();
    for i in 0..WIDTH*HEIGHT {
        use rand::Rng;
        use palette::{Colora, Rgba};
        use palette::pixel::Srgb;
        let (y, x) = (i/WIDTH, i%WIDTH);
        let (r, g, b, a) = rng.gen();
        /*
         Old buggy behavior (as in OOOOLD): Treat sRGB values as RGB values, and sRGB 'em again
        let rgb: Srgb = Rgba::new(r, g, b, a).into();
        image.set_pixel(x, y, Colora::rgb(rgb.red, rgb.green,rgb.blue, rgb.alpha)).unwrap();
        */
       image.set_pixel(x, y, Colora::rgb(r, g, b, a)).unwrap()
    }

    // // Convert channels into pixels
    // let mut colors = vec![];
    // for i in 0..image.len() {
    //     colors.push(palette::Rgba::new(image[0][i], image[1][i], image[2][i], image[3][i]));
    // }
    //
    // // Convert linear pixels into gamma-corrected pixels
    // let colors: Vec<palette::pixel::Srgb> = colors.into_iter().map(|x| x.into()).collect::<Vec<_>>();
    // TODO Create an SRGBImage convertion and format, for sRGB stuff

    use gfx::texture::{Kind, AaMode, FilterMethod, SamplerInfo, WrapMode};
    use gfx::format::Rgba32F;

    // Upload pixel data to a texture
    // let data: Vec<[u32; 4]> = colors.iter().map(|x| [x.red.to_u32(), x.green.to_u32(), x.blue.to_u32(), x.alpha.to_u32()]).collect::<Vec<_>>();
    let data: Vec<[u32; 4]> = image.data().iter().map(|x| [x[0].to_u32(), x[1].to_u32(), x[2].to_u32(), x[3].to_u32()]).collect();
    // let data: Vec<[u32; 4]> = colors.iter().map(|x| [(x.red.to_u32() + x.green.to_u32() + x.blue.to_u32()) / 3, 0, 0, x.alpha.to_u32()]).collect::<Vec<_>>();
    // let (tex, texview) = factory.create_texture_immutable_u8::<Rgba8>(Kind::D2(WIDTH as u16, HEIGHT as u16, AaMode::Single), &[&data]).unwrap();
    let (tex, texview) = factory.create_texture_immutable::<Rgba32F>(Kind::D2(WIDTH as u16, HEIGHT as u16, AaMode::Single), Mipmap::Allocated, &[&data]).unwrap();
    let sampler = factory.create_sampler(SamplerInfo::new(FilterMethod::Scale, WrapMode::Clamp));

    // TODO Add model-view-projection matrix

    // Create data to draw
    let mut data = rgb::Data {
        vbuf: buf.clone(),
        out: rtv.clone(),
        // mvp:
        picture: (texview.clone(), sampler.clone())
    };

    let mut view_update = false;
    let mut is_running = true;
    let mut enc: gfx::Encoder<gfx_gl::Resources, _> = factory.create_command_buffer().into();
    while is_running {
        use gfx::{Device};
        // use gfx::traits::{FactoryExt};

        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Closed |
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        }, ..
                    } => is_running = false,
                    glutin::WindowEvent::Resized(_, _) => {
                        //println!("REC");
                        view_update = true;
                    },
                    _ => {}
                }
            }
        });

        if view_update {
            gfx_window::update_views(&mut window, &mut rtv, &mut stv);
            data.out = rtv.clone();
            view_update = false;
        }

        // draw everything here
        enc.clear(&rtv, [0.0, 0.0, 0.0, 1.0]);
        // enc.
        enc.draw(&slice, &pso, &data);

        enc.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
