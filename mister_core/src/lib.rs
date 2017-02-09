#![warn(missing_docs)]
// NOTE: Once we reach version 1.0, change from warn to deny
extern crate byteorder;
extern crate palette;

pub mod image; // Where all image-storing stuff goes
pub mod format;

pub use self::image::{Channel, WrappedImage, Image};

// How will we support a "palette-only" mode. For those kinds of things, we turn to palette, as
// one main feature of image is to return a Color object (according to palette, it's technically an Alpha<Color>)
// NOTE to self: When implementing, convert to sRGB for optimal display (and provide option to turn that off.)
