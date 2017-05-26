extern crate palette;
extern crate nalgebra as na;
extern crate cassowary;
extern crate gfx_core;
extern crate petgraph;

mod util;
#[cfg(test)] mod tests;

use palette::{Colora};
use util::{ScreenRect};

// These are the things that our little GUI produces that are drawn.
enum Primitives {
    Text { text: String, font_size: u32 },
    Rectangle { color: Colora }, // Rectangle is expressed by (l, b, r, t) [0.0, 1.0] rel to parent
    PixelData { pixel_scale: u32, data: Vec<Colora>, dimensions: (u32, u32) }
}
