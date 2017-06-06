extern crate palette;
extern crate nalgebra as na;
extern crate cassowary;
extern crate gfx_core;
extern crate petgraph;

mod util;
#[cfg(test)] mod tests;

use palette::{Colora};
use util::{ScreenRect};
use std::sync::mpsc::{Sender, Receiver, channel};
use petgraph::stable_graph::StableGraph;

// Containers are invisible, and position widgets
//
// Widgets are split into arbitary data structures, widget data containing pos and size,
// and a Concept which takes the data and outputs drawing instructions

/// Something the Driver can draw
enum DrawingPrimitive {
    // location type|color
    // If type is used to access the default for the rectangle color
    Rectangle(ScreenRect, Result<String, Colora>),
}

enum DriverMessage {

}

enum DriverEvent {

}

type DriverSender = Sender<DriverMessage>;
type DriverReceiver = Receiver<DriverMessage>;
type DriverESender = Sender<DriverEvent>;
type DriverEReceiver = Receiver<DriverEvent>;

use petgraph::graph::NodeIndex;
/// Communicates with the seed of a driver
struct Widget {
    ni: NodeIndex,
    send: DriverSender, // How we communicate with the driver, with respect to our node
    mess: Option<DriverEReceiver>,
}

/// Widget data on driver. Keeps track of things for a certain widget.
struct Seed {
    /// How to draw this seed.
    drawing_method: Vec<DrawingPrimitive>,
    // Location (relative to parent)
    // Theme data
}

impl Seed {
    // Does user construct seed or do we?
    pub fn new(drmt: Vec<DrawingPrimitive>) -> Seed {
        Seed {
            drawing_method: drmt,
        }
    }

    fn root() -> Seed { // A seed for the root
        Seed {
            drawing_method: vec![],
        }
    }
}

/// Takes drawing intructions and realizes them
/// Also *owns* all widgets.
struct Driver {
    tree: StableGraph<(Seed, Option<DriverESender>), String>,
    root: NodeIndex,
    recv: DriverReceiver,
    send: DriverSender, // A default instance, to clone to give to Widgets
}

impl Driver {
    pub fn new() -> Driver {
        let (tx, rx) = channel();
        let mut graph =  StableGraph::new();
        let root = graph.add_node((Seed::root(), None));
        Driver {
            tree: graph,
            root: root,
            recv: rx,
            send: tx,
        }
    }

    pub fn widget(&mut self, label: String, seed: Seed, receive_events: bool) -> Widget {
        use petgraph::algo;
        let (etx, erx) = channel();
        let index = if receive_events {
            self.tree.add_node((seed, Some(etx)))
        } else {
            self.tree.add_node((seed, None))
        };
        assert!(!algo::is_cyclic_directed(&self.tree)); // Invariant broken; report this soon TODO
        let root = self.root.clone();
        self.tree.update_edge(root, index, label);
        assert!(!algo::is_cyclic_directed(&self.tree)); // Invariant broken; report this soon TODO
        if receive_events {
            Widget {
                ni: index,
                send: self.send.clone(),
                mess: Some(erx)
            }
        } else {
            Widget {
                ni: index,
                send: self.send.clone(),
                mess: None
            }
        }
    }
}
