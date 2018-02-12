//! This modules wraps various image models, using image::Image as its backing.
use super::{Channel, Image};
use palette::Colora; // Use Colora as a generic color.
use std::fmt::Debug;
use std::error::Error;

mod rgba;
mod hsla;

pub use self::rgba::{RgbaImage, RgbaImageError, RgbaChannel};

// TODO error_chain this!
/// Indicates errors for image formats
#[derive(Clone, Debug, Copy)]
pub enum ImageFormatError<T> {
    /// The requested pixel location was outside the image
    OutOfBounds(usize, usize),
    /// This channel doesn't have a value at that location
    MissingData(T, usize, usize),
}

/// Describes a general interface for formatted images
pub trait ImageFormat<T: Clone + Debug> {
    /// A struct that can describe the channels available to this image
    type ChannelName;
    /// A struct that describes errors in validating the image
    type ValidationError: Error;
    // TODO Use assoc. type defaults when they are stable
    // /// The type of error accessing a pixel can return
    // type PixelError = ImageFormatError<Self::ChannelName>;
    /// The number of channels this image uses
    fn channel_count(&self) -> usize;
    // NOTE Confuing name QUESTION How do we fix?
    /// Enables/disables the specified channel
    fn set_channel_visible(&mut self, &Self::ChannelName, bool);
    /// Gets the "visibility" of the specified channel
    fn is_channel_visible(&self, &Self::ChannelName) -> bool;
    /// Gets an underlying channel
    fn channel(&self, &Self::ChannelName) -> &Channel<T>;
    /// Gets an underlying channel mutably
    fn channel_mut(&mut self, &Self::ChannelName) -> &mut Channel<T>;

    /// Gets the width of the image
    fn width(&self) -> usize;
    /// Gets the height of the image
    fn height(&self) -> usize;

    /// Gets color at (x, y)
    fn pixel(&self, x: usize, y: usize) -> Result<Colora, ImageFormatError<Self::ChannelName>>;
    /// Sets pixel at (x, y)
    fn set_pixel(&mut self, x: usize, y: usize, c: Colora) -> Result<(), ImageFormatError<Self::ChannelName>>;

    // Checks if the data contained within the image is valid
    fn validate(&self) -> Result<(), Self::ValidationError>;

    // /// Gets color at (x, y)
    // fn pixel(&self, x: usize, y: usize) -> Result<Colora, Self::PixelError>;
    // /// Sets pixel at (x, y)
    // fn set_pixel(&mut self, x: usize, y: usize, c: Colora) -> Result<(), Self::PixelError>;
    /// Exposes the image as chunks of data
    fn data(&self) -> Vec<Vec<T>>;
    /// Flat maps all the data
    fn flat_data(&self) -> Vec<T> {
        self.data().iter().flat_map(|x| x).cloned().collect()
    }
}
