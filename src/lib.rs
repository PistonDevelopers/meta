#![deny(missing_docs)]

//! Meta parsing and encoding for data oriented design

extern crate read_token;
extern crate range;

pub use whitespace::{ Whitespace, WHITESPACE };
pub use parse_error_handler::{ ParseErrorHandler, ParseStdErr };
pub use parse_error::ParseError;
pub use ty::Type;
pub use token::Token;
pub use select::Select;
pub use parameter::Parameter;
pub use optional::Optional;
pub use until_any_or_whitespace::UntilAnyOrWhitespace;
pub use rule::Rule;

mod parse_error;
mod parse_error_handler;
mod ty;
mod token;
mod whitespace;
mod select;
mod parameter;
mod optional;
mod until_any_or_whitespace;
mod rule;

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
    fn set_as_str(&mut self, name: &str, val: String, state: &Self::State) ->
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

#[inline(always)]
fn update<'a>(range: range::Range, chars: &mut &'a [char], offset: &mut usize) {
    let next_offset = range.next_offset();
    *chars = &chars[next_offset - *offset..];
    *offset = next_offset;
}
