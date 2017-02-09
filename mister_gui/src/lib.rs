#![warn(missing_docs)]
//! The goal of this crate is to provide a library to draw a user interface (UI) for MISTER.
//! We will try to abstract over the underlying graphics system (DX11, GL, Vulkan, Metal) using GFX.

extern crate gfx_core as gfx;

// NOTE: Once we reach version 1.0, change from warn to deny

// Implemented by all widgets
trait Widget {

}

// The basic button!
// Draws a rectangle of a color to the screen
struct BasicButton {

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
