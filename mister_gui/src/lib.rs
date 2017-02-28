extern crate palette;
extern crate nalgebra as na;
extern crate cassowary;

use palette::{Colora};
use na::{Point2};

//TODO Make util module and put this there
/// Represents a piece of screen
#[derive(Clone, Copy, Debug)]
pub struct ScreenRect {
    bottom_left: na::Point2<f32>,
    top_right: na::Point2<f32>
}

// Meant for a GUI application
// meaning we should have control of the program loop

/// A virtual interface that is used to draw things
pub trait Painter {
    fn draw_color_rect(&mut self, Colora, ScreenRect);
}

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

struct GfxPainter {
    enc:
}

impl GfxPainter {
    fn draw_color_rect(&mut self, Colora, ScreenRect) {
        unimplemented!()
    }
}

// Mother of all things widget-y
// IMPL NOTE: The PARENT sizes the children when drawn
struct Window {
    area: ScreenRect,
}

impl Window {
    fn draw(&self, p: &mut Painter) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
