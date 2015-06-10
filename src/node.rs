use std::rc::Rc;
use std::cell::Cell;

use DebugId;

/// A node reference.
#[derive(Clone)]
pub struct NodeRef {
    /// Name of rule.
    pub name: Rc<String>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
    /// The index to the rule reference.
    pub index: Cell<Option<usize>>,
    /// Whether the reference has been visited.
    pub node_visit: Cell<NodeVisit>,
}

/// Tells whether a node is visited when updated.
#[derive(Copy, Clone)]
pub enum NodeVisit {
    /// The node is not being visited.
    Unvisited,
    /// The node is being visited.
    Visited
}
