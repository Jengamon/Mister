extern crate palette;
extern crate nalgebra as na;
extern crate cassowary;

use palette::{Colora};
use na::{Point2};

mod gui_solver {
    use cassowary::{Solver, Variable, Constraint};
    use cassowary::WeightedRelation::*;
    use cassowary::strength::{ WEAK, MEDIUM, STRONG, REQUIRED };
    use std::collections::HashMap;
    type VariablePair<T> = (Variable, T);
    type BoxId = i32; // Used to identify boxes for retreival
    /// A box the solver can solve for
    pub struct SolveBox {
        pub bottom: VariablePair<f32>,
        pub left: VariablePair<f32>,
        pub top: VariablePair<f32>,
        pub right: VariablePair<f32>,
    }

    /// Sets up constraints for a single box
    pub trait BoxPreferences {
        fn apply_preference(&self, &mut SolveBox) -> Vec<Constraint>;
    }

    pub trait BoxPreferencesExt {
        fn non_negative(self) -> NonNegativeBox {
            NonNegativeBox::new(self)
        }
    }

    impl<T: BoxPreferences> BoxPreferencesExt for T {}

    /// Sets up constraint between boxes
    pub trait ContainerPreferences {
        fn apply_preference(&self, width: &Variable, height: &Variable, &[&mut SolveBox]) -> Vec<Constraint>;
    }

    /// Guarantees a non-negative solvebox
    pub struct NonNegativeBox {
        other: Box<BoxPreferences>,
    }

    impl NonNegativeBox {
        fn new<T: BoxPreferences>(t: T) -> NonNegativeBox {
            NonNegativeBox {
                other: t
            }
        }
    }

    impl BoxPreferences for NonNegativeBox {
        fn apply_preference(&self, bx: &mut SolveBox) -> Vec<Constraint> {
            let mut v = vec![
                bx.top |GE(REQUIRED)| bx.bottom,
                bx.right |GE(REQUIRED)| bx.left,
            ];
            v.extend(self.other.apply_preference(bx).into_iter());
            v
        }
    }

    /// Fit all boxes horizontally, extending to bottom
    pub struct HorizontalFit;

    /// Represents a container w/ children
    pub struct LayoutSolver {
        // TODO Stop using "SolveBox"
        // Switch to using a HashMap to store a variable's current value,
        // then allow for registering IDs to store such variables.
        boxes: HashMap<BoxId, SolveBox>,
        box_prefs: Vec<(BoxId, Box<BoxPreferences>)>,
        container_pref: Box<ContainerPreferences>,

        container_width: Variable,
        container_height: Variable,

        solver: Solver
    }

    impl LayoutSolver {
        pub fn new<C: ContainerPreferences>(c: C) -> LayoutSolver {
            LayoutSolver {
                boxes: HashMap::new(),
                box_prefs: vec![],
                container_pref: Box::new(c),

                container_width: Variable::new(),
                container_height: Variable::new(),

                solver: Solver::new()
            }
        }

        /// Update all SolveBoxs
        pub fn layout(&mut self, width: f32, height: f32) {
            self.solver.reset(); // Clear out the solver

        }
    }
}

//TODO Make util module and put this there
/// Represents a piece of screen
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

/// All widgets implement this trait
trait Widget {
    fn draw(&self, &mut Painter); // TODO Add transform to arguments
}

struct WidgetChild {

}

// Mother of all things widget-y
struct Window {
    area: ScreenRect,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
