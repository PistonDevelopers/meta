use ParseError;

/// Implemented by meta readers.
///
/// A meta reader contains an internal state that corresponds to a virtual
/// tree structure. The meta parser communicates with the meta reader such
/// that parsing is interrupted if any error happens.
pub trait MetaReader {
    /// The state that points to a location in the parsed structure.
    type State: Clone;

    /// Starts parsing a node.
    fn start_node(&mut self, name: &str, state: &Self::State) ->
        Result<Self::State, ParseError>;
    /// Ends parsing a node.
    fn end_node(&mut self, state: &Self::State) ->
        Result<Self::State, ParseError>;
    /// Sets a bool property of the node.
    fn set_as_bool(&mut self, name: &str, val: bool, state: &Self::State) ->
        Result<Self::State, ParseError>;
    /// Sets a string property of the node.
    fn set_as_string(&mut self, name: &str, val: String, state: &Self::State) ->
        Result<Self::State, ParseError>;
    /// Sets a f64 property of the node.
    fn set_as_f64(&mut self, name: &str, val: f64, state: &Self::State) ->
        Result<Self::State, ParseError>;
}
