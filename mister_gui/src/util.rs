use na::{Point2};
use na;

/// Represents a piece of screen
#[derive(Clone, Copy, Debug)]
pub struct ScreenRect {
    bottom_left: na::Point2<f32>,
    top_right: na::Point2<f32>
}

impl ScreenRect {
    fn width_section(&self) -> Section<f32> {
        Section {
            low: self.bottom_left.x,
            high: self.top_right.x
        }
    }
}

use std::fmt::Debug;
#[derive(Clone, Debug, Copy)]
pub struct Section<T: Clone + Debug + Copy> {
    pub low: T,
    pub high: T
}
