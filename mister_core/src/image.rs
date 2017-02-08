// We store images as groups of channels (buffers of EQUAL SIZED DATA) and a format to interpret it.
// XXX: We don't store format anymore. Just channels of equal size.

use std::ops::{Index, IndexMut};
use std::fmt::Debug;
use std::sync::{Arc, Mutex}; // Anticipate threading

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
    size: usize,
}

impl<T: Clone + Debug> Image<T> {
    /// Creates a new Image
    pub fn new(size: usize) -> Image<T> {
        // NOTE: We start with NO CHANNELS, so something must be done...
        Image {
            channels: vec![],
            size: size
        }
    }

    pub fn create_channel(&mut self, default: T) {
        self.channels.push(Channel::new(default, self.size))
    }

    // TODO: Bounds-checking
    pub fn channel(&self, i: usize) -> &Channel<T> {
        &self.channels[i]
    }

    pub fn channel_mut(&mut self, i: usize) -> &mut Channel<T> {
        &mut self.channels[i]
    }

    /// Returns the number of channels
    pub fn count(&self) -> usize {
        self.channels.len()
    }

    pub fn len(&self) -> usize {
        self.size
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

/// Defines the bounds of an image
/// Each drawing layer should be its own Image
// #[derive(Clone, Debug)]
// pub struct Image<T: Clone + Debug> {
//     data: Arc<Mutex<Image<T>>>,
//     size: Arc<Mutex<(usize, usize)>>, // NOTE: Do we use usize to define image limits? Might work, but..., also, shared?
// }
//
// impl<T: Clone + Debug> Image<T> {
//     // An important feature? for temporaries
//     /// Creates a completely unique copy of this image (data is NOT linked between to two)
//     pub fn deep_clone(&self) -> Image<T> {
//         // QUESTION: Do we really want to use lock()? It's not very error friendly
//         // XXX: We'll use lock() until we find a case were we DON'T want to hard error out on deep clone
//         Image {
//             data: Arc::new(Mutex::new(self.data.lock().unwrap().clone())),
//             size: Arc::new(Mutex::new(self.size.lock().unwrap().clone())),
//         }
//     }
// }

pub type WrappedImage<T> = Arc<Mutex<Image<T>>>; // Placeholder

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
        // Let's change something
        new_data.channel_mut(0).write(1, 21);
        // Can also write as: new_data[0].write(1, 21) because of IndexMut impl
        assert_eq!(new_data.channel(0).iter().cloned().collect::<Vec<_>>(), vec![0,21,0,0,0]);
    }

    #[test]
    fn imagedata_double_channel() {
        let mut new_data = Image::new(5);
        // An Image is simply a grouping of channels.
        // Why choose a method like this to store data? Because this is the way I know how~
        // On a more serious note, I do plan on create color channel support, so support all the way
        // down here should help some.
        new_data.create_channel(0); // NOTE: Value passed is DEFAULT value. Argument to Image is size
        new_data.create_channel(1);
        // Let's change something
        new_data.channel_mut(0).write(1, 21);
        new_data[1].write(2, 22);
        // Can also write as: new_data[0].write(1, 21) because of IndexMut impl
        assert_eq!(new_data.channel(0).iter().cloned().collect::<Vec<_>>(), vec![0,21,0,0,0]);
        assert_eq!(new_data.channel(1).iter().cloned().collect::<Vec<_>>(), vec![1,1,22,1,1]);
    }
}
