extern crate palette;
extern crate nalgebra as na;
extern crate cassowary;
extern crate petgraph;
#[macro_use]
extern crate gfx;

mod util;
#[cfg(test)] mod tests;

use palette::{Colora};
use util::{ScreenRect};
use std::error::Error;

// These are the things that our little GUI produces that are drawn.
pub enum Primitive {
    Text { text: String, font_size: u32 },
    Rectangle { color: Colora },
    PixelData { data: Vec<Colora>, dimensions: (u32, u32) }
}

pub type Primitives = Vec<(Primitive, (f32, f32, f32, f32))>;
pub type MappedPrimitives = Vec<(Primitive, (u32, u32, u32, u32))>;

pub trait Widget {
    fn draw(&self) -> Primitives; // Rectangle is expressed by (l, b, r, t) [0.0, 1.0] rel to parent
}

struct Button {
    color: Colora
}

impl Widget for Button {
    fn draw(&self) -> Primitives {
        vec![
            (Primitive::Rectangle { color: self.color.clone() }, (0.0, 0.0, 1.0, 1.0))
        ]
    }
}

impl Button {
    pub fn new() -> Button {
        Button {
            color: Colora::rgb_u8(216, 178, 95, 256)
        }
    }
}

/// Helper struct that maps the float point system of primitive lists into actual coordinates (assumes OpenGL style coordinates)
pub struct Mapper {
    screen_space: (u32, u32),
    offset: (u32, u32),
}

impl Mapper {
    pub fn new(scsp: (u32, u32), offset: (u32, u32)) -> Mapper {
        Mapper {
            screen_space: scsp,
            offset: offset
        }
    }

    pub fn map(&self, list: Primitives) -> MappedPrimitives {
        let operation = |(prim, rect)| {
            let (l, b, r, t) = rect;
            let (sw, sh) = self.screen_space;
            let (ox, oy) = self.offset;
            let (l, b, r, t) = ((l*(sw as f32)) as u32+ox, (b*(sh as f32)) as u32+oy, (r*(sw as f32)) as u32+ox, (t*(sh as f32)) as u32+oy);
            (prim, (l, b, r, t))
        };
        list.into_iter().map(operation).collect()
    }
}

/// This collects all primitives of a widget, and draws them.
pub trait Realizer {
    // type Error: Error;
    type Error; // Temp
    fn realize(&mut self, Primitives) -> Result<(), Self::Error>;
}

mod gfxr {
    use super::{Realizer, Mapper, Primitive, Primitives};
    use palette::{Colora};
    use gfx::{Resources, CommandBuffer, Encoder, VertexBuffer, Global, RenderTarget, Rgba8};

    gfx_defines!{
        vertex Vertex {
            pos: [f32; 2] = "pos",
            color: [f32; 3] = "color",
        }

        pipeline rect {
            vbuf: gfx::VertexBuffer<Vertex> = (),
            projection: gfx::Global<[[f32; 4]; 4]> = "projection",
            modelview: gfx::Global<[[f32; 3]; 3]> = "modelview",
            out: gfx::RenderTarget<Rgba8> = "Target0",
        }
    }

    /// A realizer that relies on gfx
    struct GfxRealizer<'a, R: Resources, B: 'a + CommandBuffer<R>> {
        map: Mapper,
        enc: &'a mut Encoder<R, B>,
        
    }

    enum GfxRealizerError {}

    type Rect<T> = (T, T, T, T);
    type Point<T> = (T, T);

    impl<'a, R: Resources, B: 'a+CommandBuffer<R>> GfxRealizer<'a, R, B> {
        pub fn new(pos: Point<u32>, size: (u32, u32), enc: &'a mut Encoder<R, B>) -> GfxRealizer<'a, R, B> {
            GfxRealizer {
                map: Mapper::new(size, pos),
                enc
            }
        }

        fn realize_text(&mut self, text: String, font_size: u32, space: Rect<f32>) {
            // TODO
        }

        fn realize_rect(&mut self, color: Colora, space: Rect<f32>) {

        }

        fn realize_pixeldata(&mut self, data: Vec<Colora>, dims: (u32, u32), space: Rect<f32>) {

        }
    }

    impl<'a, R: Resources, B: CommandBuffer<R>> Realizer for GfxRealizer<'a, R, B> {
        type Error = GfxRealizerError;
        fn realize(&mut self, list: Primitives) -> Result<(), Self::Error> {
            for (prim, space) in list.into_iter() {
                match prim {
                    Primitive::Text {text, font_size} => self.realize_text(text, font_size, space),
                    Primitive::Rectangle {color} => self.realize_rect(color, space),
                    Primitive::PixelData {data, dimensions} => self.realize_pixeldata(data, dimensions, space)
                }
            }
            Ok(())
        }
    }
}
