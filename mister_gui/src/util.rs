use ::na::{Point2};
use ::na;

/// Represents a piece of screen
#[derive(Clone, Copy, Debug)]
pub struct ScreenRect {
    bottom_left: na::Point2<f32>,
    top_right: na::Point2<f32>
}
