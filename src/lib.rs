#![deny(missing_docs)]

//! Meta parsing and encoding for data oriented design

extern crate read_token;

pub use whitespace::{ Whitespace, WHITESPACE };

mod whitespace;

/// Types of properties.
pub enum Type {
    /// Either true or false.
    Bool,
    /// Text.
    String,
    /// Number.
    F64,
}

/// Errors reporting expected values.
pub enum Error<'a> {
    /// Not supported.
    NotSupported,
    /// Expected nodes with other names.
    ExpectedNode(&'a [&'a str]),
    /// Expected another propert type.
    ExpectedPropertyType(Type),
    /// Reaching end of node, but expected more properties.
    ExpectedMoreProperties(&'a [&'a str]),
}

/// Implemented by meta readers.
///
/// A meta reader contains an internal state that corresponds to a virtual
/// tree structure. The meta parser communicates with the meta reader such
/// that parsing is interrupted if any error happens.
pub trait MetaReader {
    /// The state that points to a location in the parsed structure.
    type State;

    /// Starts parsing a node.
    fn start_node(&mut self, name: &str, state: &Self::State) ->
        Result<Self::State, Error>;
    /// Ends parsing a node.
    fn end_node(&mut self, state: &Self::State) -> Result<Self::State, Error>;
    /// Starts optional section of a node.
    fn start_optional(&mut self, state: &Self::State) ->
        Result<Self::State, Error>;
    /// Ends optional section of a node.
    fn end_optional(&mut self, state: &Self::State) ->
        Result<Self::State, Error>;
    /// Starts selection.
    fn start_select(&mut self, state: &Self::State) ->
        Result<Self::State, Error>;
    /// Ends selection.
    fn end_select(&mut self, state: &Self::State) ->
        Result<Self::State, Error>;
    /// Sets a bool property of the node.
    fn set_as_bool(&mut self, name: &str, val: bool, state: &Self::State) ->
        Result<Self::State, Error>;
    /// Sets a string property of the node.
    fn set_as_str(&mut self, name: &str, val: &str, state: &Self::State) ->
        Result<Self::State, Error>;
    /// Sets a f64 property of the node.
    fn set_as_f64(&mut self, name: &str, val: f64, state: &Self::State) ->
        Result<Self::State, Error>;
}

/// Stores information about token.
pub struct Token<'a> {
    /// The text to match against.
    pub text: &'a str,
    /// Whether to set property to true or false (inverted).
    pub inverted: Option<bool>,
    /// Which predicate to set if token matches.
    /// This is the name of the property in current node.
    pub predicate: Option<&'a str>,
}

/// Stores information about a parameter.
pub struct Parameter<'a> {
    /// The name of the parameter.
    pub name: &'a str,
    /// The properties of the parameter.
    /// This is used to check the property names set by sub rules.
    /// If a property name does not match any of the arguments to the parameter,
    /// then an error is reported.
    pub args: &'a [&'a str],
    /// The property name of parent to set the value.
    pub value: Option<&'a str>,
    /// The body of the parameter.
    pub body: &'a [Rule<'a>],
}

/// A rule describes how some section of a document should be parsed.
pub enum Rule<'a> {
    /// Read whitespace.
    Whitespace(Whitespace),
    /// Match against a token.
    Token(Token<'a>),
    /// Select one of the sub rules.
    /// If the first one does not succeed, try another and so on.
    /// If all sub rules fail, then the rule fails.
    Select(&'a [Rule<'a>]),
    /// Read parameter.
    Parameter(Parameter<'a>),
}
