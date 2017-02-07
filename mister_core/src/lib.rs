pub mod image; // Where all image-storing stuff goes

pub use self::image::{Channel, WrappedImage, Image};

// How will we support a "palette-only" mode. For those kinds of things, we turn to palette, as
// one main feature of image is to return a Color object (according to palette, it's technically an Alpha<Color>)
// NOTE to self: When implementing, convert to sRGB for optimal display (and provide option to turn that off.)
