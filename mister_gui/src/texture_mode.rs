// Instead of having a draw deal with primitives at the end, all components are provided with a TextureProvider
// which they display in their own unique way!

use std::sync::mpsc::{Sender, Receiver};
use palette::{Colora};

enum TextureUpdate<PosType> {
    SinglePixel { x: PosType, y: PosType, c: Colora },
    BoxPixel { l: PosType, t: PosType, w: PosType, h: PosType, c: Colora }
}

impl<P> TextureUpdate<P> {
    fn single(x: P, y: P, c: Colora) -> TextureUpdate<P> {
        TextureUpdate::SinglePixel {
            x, y, c
        }
    }

    fn rect(l: P, t: P, w: P, h: P, c: Colora) -> TextureUpdate<P> {
        TextureUpdate::BoxPixel {
            l, t, w, h, c
        }
    }

    fn square(l: P, t: P, s: P, c: Colora) -> TextureUpdate<P>
        where P: Copy + Clone 
    {
        TextureUpdate::rect(l, t, s, s, c)
    }
}

struct TextureProvider {
    b: Receiver<TextureUpdate<u32>>,

}