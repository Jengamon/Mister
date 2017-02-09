// We store images as groups of channels (buffers of EQUAL SIZED DATA) and a format to interpret it.
// XXX: We don't store format anymore. Just channels of equal size.

use std::ops::{Index, IndexMut};
use std::fmt::Debug;

// QUESTION: Do we need a constrait on T?
/// This represent a set of data values for one color.
#[derive(Clone, Debug)]
pub struct Channel<T: Clone + Debug> {
    // TODO: Maybe look for a fixed length, but resizeable, array structure
    // NOTE: data must guarantee that replacements will keep data @ data.len
    data: Vec<T>,

    default: T, // Hold onto this, we need it for resizing.
}

impl<T: Clone + Debug> Channel<T> {
    /// Creates a new Channel
    pub fn new(default: T, x: usize) -> Channel<T> {
        Channel {
            data: vec![default.clone(); x],
            default: default
        }
    }

    // Note this is the size of data w/o reallocation
    /// Get the amount of data this channel can hold.
    /// Note that this is equal to its length
    #[deprecated(since="0.0.1", note="Use len() instead")]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    // This is the actual size of data inside the channel
    /// Get the length of data in this channel
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // NOTE: Changing this to "write", but we may switch back, IDK.
    /// Change value at index `i` to `data`
    pub fn write(&mut self, i: usize, data: T) {
        // TODO: Bounds checking
        self.data.remove(i);
        self.data.insert(i, data);
    }

    /// Retrieve value at index `i`
    pub fn index(&self, i: usize) -> &T {
        &self.data[i]
    }

    /// Retrieve value at index `i` mutably
    pub fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }

    /// Retrieve value at index `i` as a clone (non-reference)
    #[deprecated(since="0.0.1", note="Prefer index(i).clone() instead")]
    pub fn index_clone(&self, i: usize) -> T {
        self.data[i].clone()
    }

    /// Resize channel to `new_len`
    pub fn resize(&mut self, new_len: usize) {
        self.data.truncate(new_len);
        if self.len() < new_len {
            let data_len = new_len - self.data.len();
            let default = self.default.clone();
            self.data.extend_from_slice(&vec![default; data_len])
        }
    }

    /// Create an iterator over the values of this channel
    pub fn iter(&self) -> ChannelIterator<T> {
        ChannelIterator {
            chan: self,
            at: 0
        }
    }
}

impl<T: Clone + Debug> Index<usize> for Channel<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        self.index(i)
    }
}

impl<T: Clone + Debug> IndexMut<usize> for Channel<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        self.index_mut(i)
    }
}

/// Iterates over the data of a channel
pub struct ChannelIterator<'a, T: Clone + Debug + 'a> {
    chan: &'a Channel<T>,
    at: usize
}

impl<'a, T: Clone + Debug + 'a> Iterator for ChannelIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.at += 1;
        if self.at - 1 >= self.chan.len() {
            None
        } else {
            Some(&self.chan[self.at-1])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.chan.len(), Some(self.chan.len()))
    }
}
impl<'a, T: Clone + Debug + 'a> ExactSizeIterator for ChannelIterator<'a, T> {}


// TODO A mutable iterator

/// A collection of channels to be interpreted in a certain way.
// NOTE: We DON'T assign a type here. That's MISTER's job...
#[derive(Clone, Debug)]
pub struct Image<T: Clone + Debug> {
    /// NOTE: At this point, we aren't going to even assign a color model, just a configuation of channels
    channels: Vec<Channel<T>>,
    /// The size that all channels *must* be.
    len: usize,
}

impl<T: Clone + Debug> Image<T> {
    /// Creates a new Image
    pub fn new(len: usize) -> Image<T> {
        // NOTE: We start with NO CHANNELS, so something must be done...
        Image {
            channels: vec![],
            len: len
        }
    }

    /// Creates a channel
    // TODO Add specifics (same with Channel::new)
    pub fn create_channel(&mut self, default: T) {
        self.channels.push(Channel::new(default, self.len))
    }

    // TODO: Bounds-checking
    /// Access channel at index `i`
    pub fn channel(&self, i: usize) -> &Channel<T> {
        &self.channels[i]
    }

    /// Access channel at index `i` mutably
    pub fn channel_mut(&mut self, i: usize) -> &mut Channel<T> {
        &mut self.channels[i]
    }

    /// Get the number of channels
    pub fn count(&self) -> usize {
        self.channels.len()
    }

    /// Get the length of image, which is the length of each channel
    pub fn len(&self) -> usize {
        self.len
    }

    /// Resize image to length `new_len`
    pub fn resize(&mut self, new_len: usize) {
        self.len = new_len;
        for c in self.channels.iter_mut() {
            c.resize(new_len);
        }
    }
}

impl<T: Clone + Debug> Index<usize> for Image<T> {
    type Output = Channel<T>;
    fn index(&self, i: usize) -> &Channel<T> {
        self.channel(i)
    }
}

impl<T: Clone + Debug> IndexMut<usize> for Image<T> {
    fn index_mut(&mut self, i: usize) -> &mut Channel<T> {
        self.channel_mut(i)
    }
}

use palette;
use std::marker;

/// An interface that all Image interpreters must use.
pub trait ImageFormat<T: Clone + Debug> {
    /// Gets the underlying image mutably
    fn image_mut(&mut self) -> &mut Image<T>;
    /// Gets the underlying image
    fn image(&self) -> &Image<T>;
    /// Gets the size of the image (in width and height)
    /// ## Example:
    /// ```rust
    /// let (width, height) = format.size();
    /// ```
    fn size(&self) -> (usize, usize);

    /// Returns the pixel at the specified location
    fn pixel(&self, x: usize, y: usize) -> Option<palette::Colora>;
    /// Returns an iterator over all pixels of this image
    fn pixels(&self) -> PixelIterator<T> where Self: marker::Sized { PixelIterator { source: self, index: 0 } }
}

/// An interface for image formats that allows for modification
pub trait MutableImageFormat<T: Clone + Debug>: ImageFormat<T> {
    /// The format that the format uses to set its channels
    type InternalFormat; // TODO Find a better way to do these
    /// Type returned on error
    type Error;
    /// Returns the pixel at the specified location mutably
    fn pixel_mut(&mut self, x: usize, y: usize) -> Option<Box<PixelWrapper<T, Self::InternalFormat, Self::Error>>>;
}

/// Provides access to a pixel for both reading and writing
// IMPL NOTE: Might use RwLock
pub trait PixelWrapper<'a, T: Clone + Debug, F, E> {
    /// Read the pixel
    fn read(&self) -> palette::Colora;
    /// Write to the pixel
    fn write(&mut self, f: F) -> Result<(), E>;
}

/// Iterates over the pixels of an image
pub struct PixelIterator<'a, T: 'a + Clone + Debug> {
    source: &'a ImageFormat<T>,
    index: usize,
    // phantom: marker::PhantomData<T>
}

impl<'a, T: Clone + Debug> Iterator for PixelIterator<'a, T> {
    type Item = palette::Colora;
    fn next(&mut self) -> Option<palette::Colora> {
        let (width, height) = self.source.size();
        let (x, y) = (self.index / width, self.index % width);
        if y > height {
            return None
        } else if x > width {
            unreachable!("Math is now wonky!");
        }

        self.index += 1;
        self.source.pixel(x, y)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (width, height) = self.source.size();
        let size = width * height;
        (size, Some(size))
    }
}
impl<'a, T: Clone + Debug + 'a> ExactSizeIterator for PixelIterator<'a, T> {}


#[cfg(test)]
mod tests {
    use super::{Channel, Image};
    // TODO: Move these tests and Image, Channel and ImagaData into separate module
    #[test]
    fn channel_capacity() {
        let new_channel = Channel::new(0, 10);
        assert_eq!(new_channel.capacity(), 10);
    }

    #[test]
    fn channel_len() {
        let new_channel = Channel::new(0, 10);
        assert_eq!(new_channel.len(), 10);
    }

    #[test]
    fn channel_resize() {
        let mut new_channel = Channel::new(0, 10);
        // Ok, capacity is the amount of data a channel CAN hold WITHOUT REALLOCATING,
        // but its size is the amount of data it SHOULD hold.
        // Resizing may change the capacity, but it MUST change the size.
        new_channel.resize(60);
        assert_eq!(new_channel.len(), 60);
    }

    #[test]
    fn channel_write() {
        let mut new_channel = Channel::new(0u8, 10);
        // So writing is simple, but we can only do it 1 item at a time.
        // TODO: Make it so that blocks can be written to a channel
        let len = new_channel.len();
        new_channel.write(4, 21);
        assert_eq!(len, new_channel.len()); // length cannot change with write!
        assert_eq!(new_channel.iter().cloned().collect::<Vec<_>>(), vec![0,0,0,0,21,0,0,0,0,0]);
    }

    #[test]
    fn imagedata_single_channel() {
        let mut new_data = Image::new(5);
        // An Image is simply a grouping of channels.
        // Why choose a method like this to store data? Because this is the way I know how~
        // On a more serious note, I do plan on create color channel support, so support all the way
        // down here should help some.
        new_data.create_channel(0); // NOTE: Value passed is DEFAULT value. Argument to Image is size
        assert_eq!(new_data.count(), 1);
        // Let's change something
        new_data.channel_mut(0).write(1, 21);
        // Can also write as: new_data[0].write(1, 21) because of IndexMut impl
        assert_eq!(new_data.channel(0).iter().cloned().collect::<Vec<_>>(), vec![0,21,0,0,0]);
    }

    #[test]
    fn imagedata_double_channel() {
        let mut new_data = Image::new(5);
        new_data.create_channel(0); // NOTE: Value passed is DEFAULT value. Argument to Image is size
        new_data.create_channel(1);
        // Let's change something
        assert_eq!(new_data.count(), 2);
        new_data.channel_mut(0).write(1, 21);
        new_data[1].write(2, 22);
        // Can also write as: new_data[0].write(1, 21) because of IndexMut impl
        assert_eq!(new_data.channel(0).iter().cloned().collect::<Vec<_>>(), vec![0,21,0,0,0]);
        assert_eq!(new_data.channel(1).iter().cloned().collect::<Vec<_>>(), vec![1,1,22,1,1]);
    }

    #[test]
    fn imagedata_resize() {
        let mut new_data = Image::new(5);
        new_data.create_channel(0); // NOTE: Value passed is DEFAULT value. Argument to Image is size
        new_data.create_channel(1);
        // resize the channel
        new_data.resize(3);
        assert_eq!(new_data.len(), 3);
    }

    #[test]
    fn imagedata_channel_length() {
        let new_data = Image::new(5);
        new_data.create_channel(());
        new_data.create_channel(());

        assert_eq!(new_data.len(), new_data.channel(0).len());
        assert_eq!(new_data.len(), new_data.channel(1).len());
    }
}
