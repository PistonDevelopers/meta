use std::rc::Rc;

use DebugId;

/// A node reference.
#[derive(Clone)]
pub enum NodeRef {
    /// Points to a node by name.
    Name(Rc<String>, DebugId),
    /// Reference to node.
    /// The `bool` flag is used to prevent multiple visits when updating.
    Ref(usize, NodeVisit),
}

/// Tells whether a node is visited when updated.
#[derive(Clone)]
pub enum NodeVisit {
    /// The node is not being visited.
    Unvisited,
    /// The node is being visited.
    Visited
}
