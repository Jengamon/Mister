extern crate palette;
extern crate nalgebra as na;
extern crate cassowary;
extern crate gfx_core;

mod util;
#[cfg(test)] mod tests;

use palette::{Colora};
use util::{ScreenRect};

// Meant for a GUI application
// meaning we should have control of the program loop

/// A virtual interface that is used to draw things
pub trait Painter {
    fn draw_color_rect(&mut self, Colora, ScreenRect);

    fn flip(&mut self);
}

//struct GfxPainter<R: gfx_core::Resources> {
//    enc:
//}

use std::rc::Rc;
type WidgetPtr = Rc<Box<Widget>>;
type ContainerPtr = Rc<Box<Container>>;

// Widgets can only have 1 child
/// All widgets implement this trait
trait Widget {
    fn draw(&self, &mut Painter, ScreenRect); // TODO Add transform to arguments
    fn add_child(&mut self, WidgetPtr); // Sets the child to this widget
}

// Containers are widgets that can contain multiple children
trait Container {
    fn draw_children(&mut self, &mut Painter, ScreenRect);
    fn add_to_children(&mut self, WidgetPtr); // Adds a child to the child list
}

// struct GfxPainter {
//     enc:
// }

// impl GfxPainter {
//     fn draw_color_rect(&mut self, Colora, ScreenRect) {
//         unimplemented!()
//     }
// }

// Mother of all things widget-y
// IMPL NOTE: The PARENT sizes the children when drawn
struct Window {
    // painter: Box<Painter>,
    // TODO Store transform
}

impl Window {
    fn create() -> Window {
        Window {
            // painter: p,
        }
    }

    fn draw<P: Painter>(&self, painter: &mut P) {
        unimplemented!();
    }
}
