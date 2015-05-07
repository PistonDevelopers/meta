#![deny(missing_docs)]

//! Meta parsing and encoding for data oriented design

extern crate read_token;
extern crate range;

pub use whitespace::{ Whitespace, WHITESPACE };
pub use parse_error_handler::{ ParseErrorHandler, ParseStdErr };
pub use parse_error::ParseError;
pub use ty::Type;
pub use token::Token;

mod parse_error;
mod parse_error_handler;
mod ty;
mod token;
mod whitespace;

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
        Result<Self::State, ParseError>;
    /// Ends parsing a node.
    fn end_node(&mut self, state: &Self::State) ->
        Result<Self::State, ParseError>;
    /// Sets a bool property of the node.
    fn set_as_bool(&mut self, name: &str, val: bool, state: &Self::State) ->
        Result<Self::State, ParseError>;
    /// Sets a string property of the node.
    fn set_as_str(&mut self, name: &str, val: &str, state: &Self::State) ->
        Result<Self::State, ParseError>;
    /// Sets a f64 property of the node.
    fn set_as_f64(&mut self, name: &str, val: f64, state: &Self::State) ->
        Result<Self::State, ParseError>;
}

/// Implemented by meta writers.
pub trait MetaWriter {
    /// Starts encoding a node.
    fn start_node(&mut self, name: &str);
    /// Ends encoding a node.
    fn end_node(&mut self, name: &str);
    /// Get bool property.
    fn get_as_bool(&mut self, name: &str) -> Option<bool>;
    /// Get str property.
    fn get_as_str<F, U>(&mut self, name: &str, f: F) -> Option<U>
        where F: FnOnce(&str) -> U;
    /// Get f64 property.
    fn get_as_f64(&mut self, name: &str) -> Option<f64>;
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

/*
Might be useful for later.
#[inline(always)]
fn update<'a>(range: range::Range, chars: &'a [char], offset: usize) ->
    (&'a [char], usize) {
    let next_offset = range.next_offset();
    (&chars[next_offset - offset..], next_offset)
}
*/
