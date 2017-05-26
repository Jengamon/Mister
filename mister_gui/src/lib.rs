extern crate palette;
extern crate nalgebra as na;
extern crate cassowary;
extern crate gfx_core;
extern crate petgraph;

mod util;
#[cfg(test)] mod tests;

use palette::{Colora};
use util::{ScreenRect};
use std::error::Error;

// These are the things that our little GUI produces that are drawn.
pub enum Primitive {
    Text { text: String, font_size: u32 },
    Rectangle { color: Colora },
    PixelData { pixel_scale: u32, data: Vec<Colora>, dimensions: (u32, u32) }
}

pub type PrimitiveList = Vec<(Primitive, (f32, f32, f32, f32))>;
pub type MappedList = Vec<(Primitive, (u32, u32, u32, u32))>;

pub trait Widget {
    fn draw(&self) -> PrimitiveList; // Rectangle is expressed by (l, b, r, t) [0.0, 1.0] rel to parent
}

struct Button {
    color: Colora
}

impl Widget for Button {
    fn draw(&self) -> PrimitiveList {
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

/// (Helper?) struct that maps the float point system of primitive lists into actual coordinates that the Realizer can easily use (assumes OpenGL style coordinates)
struct Mapper {
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

    pub fn map(&self, list: PrimitiveList) -> MappedList {
        let operation = |(prim, rect)| {
            let (l, b, r, t) = rect;
            let (sw, sh) = self.screen_space;
            let (ox, oy) = self.offset;
            let (l, b, r, t) = ((l*(sw as f32) as u32)+ox, (b*(sh as f32)) as u32+oy, (r*(sw as f32) as u32)+ox, (t*(sh as f32)) as u32+oy);
            (prim, (l, b, r, t))
        };
        list.into_iter().map(operation).collect()
    }
}

/// This collects all primitives of a widget, and draws them.
pub trait Realizer {
    type Error: Error;
    fn realize(&mut self, MappedList) -> Result<(), Self::Error>;
}
