use std::rc::Rc;
use std::cell::Cell;

use DebugId;

/// A node reference.
#[derive(Clone)]
pub enum NodeRef {
    /// Points to a node by name.
    Name(Rc<String>, DebugId),
    /// Reference to node.
    /// The `NodeVisit` flag is used to prevent multiple visits when updating.
    Ref(Cell<usize>, Cell<NodeVisit>),
}

/// Tells whether a node is visited when updated.
#[derive(Copy, Clone)]
pub enum NodeVisit {
    /// The node is not being visited.
    Unvisited,
    /// The node is being visited.
    Visited
}
