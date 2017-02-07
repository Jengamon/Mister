// We store images as groups of channels (buffers of EQUAL SIZED DATA) and a format to interpret it.
// How will we support a "palette-only" mode. For those kinds of things, we turn to palette, as
// one main feature of image is to return a Color object (according to palette, it's technically an Alpha<Color>)
// NOTE to self: When implementing, convert to sRGB for optimal display (and provide option to turn that off.)

use std::fmt::Debug;

// QUESTION: Do we need a constrait on T?
/// This represent a set of data values for one color.
#[derive(Clone, Debug)]
struct Channel<T: Clone + Debug> {
    // TODO: Maybe look for a fixed length, but resizeable, array structure
    // NOTE: data must guarantee that replacements will keep data @ data.len
    data: Vec<T>,

    default: T, // Hold onto this, we need it for resizing.
}

impl<T: Clone + Debug> Channel<T> {
    pub fn new(default: T, x: usize) -> Channel<T> {
        Channel {
            data: vec![default.clone(); x],
            default: default
        }
    }

    // Note this is the size of data w/o reallocation
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    // This is the actual size of data inside the channel
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // NOTE: Changing this to "write", but we may switch back, IDK.
    pub fn write(&mut self, i: usize, data: T) {
        // TODO: Bounds checking
        self.data.remove(i);
        self.data.insert(i, data);
    }

    pub fn index(&self, i: usize) -> &T {
        &self.data[i]
    }

    pub fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }

    pub fn index_clone(&self, i: usize) -> T {
        self.data[i].clone()
    }

    pub fn resize(&mut self, new_len: usize) {
        self.data.truncate(new_len);
        if self.len() < new_len {
            let data_len = new_len - self.data.len();
            let default = self.default.clone();
            self.data.extend_from_slice(&vec![default; data_len])
        }
    }

    pub fn iter(&self) -> ChannelIterator<T> {
        ChannelIterator {
            chan: self,
            at: 0
        }
    }
}

use std::ops::{Index, IndexMut};
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

struct ChannelIterator<'a, T: Clone + Debug + 'a> {
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

/// A collection of channels to be interpreted in a certain way.
// NOTE: We DON'T assign a type here. That's MISTER's job...
#[derive(Clone, Debug)]
enum ImageData<T: Clone + Debug> {
    /// NOTE: All RGB storage is going to be linear, so it needs to be converted on display
    /// RGB (no alpha) - mainly used for indexed images
    RGB{red: Channel<T>, green: Channel<T>, blue: Channel<T>}, // NOTE: Invisible color is defined by palette
    /// RGBA (w/ alpha) - mainly used for non-indexed images, although indexed can support it
    RGBA{red: Channel<T>, green: Channel<T>, blue: Channel<T>, alpha: Channel<T>},
    /// HSL (no alpha) - just a color format I want to add ;)
    HSL{hue: Channel<T>, saturation: Channel<T>, lumiosity: Channel<T>},
    /// HSLA (w/ alpha) - just a color format I want to add ;)
    HSLA{hue: Channel<T>, saturation: Channel<T>, lumiosity: Channel<T>, alpha: Channel<T>}
    /// Grayscale (no alpha) - just grays, nothing else
    Grayscale{gray: Channel<T>},
    /// GrayscaleA (w/ alpha) - gray that can now be not so opaque
    GrayscaleA{gray: Channel<T>, alpha: Channel<T>},
    /// Indexed (no alpha) - straight up channel of numbers for interpretation of some kind of index image
}

impl<T: Clone + Debug> ImageData<T> {
    /// Creates a new RGB ImageData (NOTE: len is all data for width * height)
    pub fn new_rgb(default_r: T, default_g: T, default_b: T, len: usize) -> ImageData<T> {
        ImageData::RGB {
            red: Channel::new(default_r, len),
            green: Channel::new(default_g, len),
            blue: Channel::new(default_b, len),
        }
    }

    // How do we handle channel access?
    // First, you can decompose the channels yourself (I'll see the API changes needed to make that happen)
    // Or, if you require a certain channel, but don't want to decompose the entire enum, I'm providing
    // channel methods. Of course, they'll have to properly support that channel...

    /// Returns whether the ImageData is an rgb-model
    pub fn is_rgb(&self) -> bool {
        match self {
            &ImageData::RGB{ ... } => true,
            &ImageData::RGBA{ ... } => true,
            &ImageData::HSL{ ... } => false,
            &ImageData::HSLA{ ... } => false
        }
    }
}

use std::sync::{Arc, Mutex}; // Anticipate threading
/// Defines the bounds of an image
/// Each drawing layer should be its own Image
#[derive(Clone, Debug)]
struct Image<T: Clone + Debug> {
    data: Arc<Mutex<ImageData<T>>>,
    size: Arc<Mutex<(usize, usize)>>, // NOTE: Do we use usize to define image limits? Might work, but..., also, shared?
}

impl<T: Clone + Debug> Image<T> {
    // An important feature? for temporaries
    /// Creates a completely unique copy of this image (data is NOT linked between to two)
    fn deep_clone(&self) -> Image<T> {
        // QUESTION: Do we really want to use lock()? It's not very error friendly
        // XXX: We'll use lock() until we find a case were we DON'T want to hard error out on deep clone
        Image {
            data: Arc::new(Mutex::new(self.data.lock().unwrap().clone())),
            size: Arc::new(Mutex::new(self.size.lock().unwrap().clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Channel};
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
        new_channel.write(4, 21);
        assert_eq!(new_channel.iter().cloned().collect::<Vec<_>>(), vec![0,0,0,0,21,0,0,0,0,0]);
    }
}
