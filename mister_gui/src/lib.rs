#![warn(missing_docs)]
// NOTE: Once we reach version 1.0, change from warn to deny
//! The goal of this crate is to provide a library to draw a user interface (UI) for MISTER.
//! We will try to abstract over the underlying graphics system (DX11, GL, Vulkan, Metal) using GFX.

// We will use pull event system, where events are pulled from elements, rather than the other way round.
// We also will try to abstract over rendering systems, but provide an implementation for gfx

extern crate gfx_core as gfx;
extern crate nalgebra as na;
#[macro_use]
extern crate mopa;
extern crate palette;
extern crate cassowary;

mod layout;

/// The struct where the current state of input is recorded.
struct InputHub {

}

/// An indication of a change in state
enum Event {
    Pressed(bool),
}

// All widgets implement this.
trait Widget: mopa::Any {
    fn maximum_size(&self) -> Option<(usize, usize)> { None }
    fn minimum_size(&self) -> (usize, usize) { (0, 0) }
    fn preferred_size(&self) -> (usize, usize) { (0, 0) }

    fn draw(&self) -> Vec<RenderCommand> { vec![] }
    fn update(&mut self, input: &InputHub);
}
mopafy!(Widget);

/// Objects that directly produce events implement this.
trait Object {
    fn events(&mut self) -> Vec<Event>;
}

use std::rc::Rc;
use std::sync::RwLock;
// TODO Implement threadsafe variant and move to "handle" module
struct WidgetHandle {
    widget: Rc<RwLock<Widget>>,
}

impl WidgetHandle {
    // fn
}

use std::ops::Deref;
impl Deref for WidgetHandle {
    type Target = RwLock<Widget>;
    fn deref(&self) -> &RwLock<Widget> {
        // NOTE Blocking!
        &self.widget
    }
}

struct Button {

}

pub struct Rect<T> {
    pub top_left: na::Point2<T>,
    pub bottom_right: na::Point2<T>
}

// TODO Theming support
/// Possible drawing commands
enum RenderCommand {
    DrawBox(Rect<isize>, palette::Colora),
}

trait ContainerStrategy: Widget {
    /// Layout a container's children
    fn layout(&mut self, children: &mut [WidgetHandle]) {}
    /// Update a container, and return if children should be updated
    fn container_update(&mut self, input: &InputHub, _: &mut [WidgetHandle]) -> bool { self.update(input); true }
}

/// A struct for items that can have children children
struct Container<CS: ContainerStrategy> {
    strategy: Box<CS>,
    children: Vec<WidgetHandle>,
}
impl<CS: ContainerStrategy> Widget for Container<CS> {
    fn maximum_size(&self) -> Option<(usize, usize)> { self.strategy.maximum_size() }
    fn minimum_size(&self) -> (usize, usize) { self.strategy.minimum_size() }
    fn preferred_size(&self) -> (usize, usize) { self.strategy.preferred_size() }

    fn draw(&self) -> Vec<RenderCommand> {
        let mut pv = self.strategy.draw();
        for child in self.children.iter() {
            pv.extend(child.read().unwrap().draw().into_iter());
        }
        pv
    }

    fn update(&mut self, input: &InputHub) {
        if self.strategy.container_update(input, &mut self.children) {
            self.children.iter_mut().map(|child| child.write().unwrap().update(input));
            // Children have been updated, so refresh the layout
            self.strategy.layout(&mut self.children);
        }
    }
}


// NOTE Following the advice of MOPA, we'll just use an enum to represent all our objects


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
