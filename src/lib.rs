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
pub use text::Text;
pub use number::Number;
pub use rule::Rule;
pub use meta_reader::MetaReader;

use std::rc::Rc;
use std::cell::RefCell;

mod parse_error;
mod parse_error_handler;
mod ty;
mod token;
mod whitespace;
mod select;
mod parameter;
mod optional;
mod until_any_or_whitespace;
mod text;
mod number;
mod rule;
mod meta_reader;

/// Represents a data structure to read into.
pub struct Struct<'a> {
    /// The fields of a struct.
    pub fields: &'a mut [Data<'a>],
}

/// Allocated data for reading.
pub enum Data<'a> {
    /// Hash a f64 property.
    F64(&'a str, &'a mut f64),
    /// Has a bool property.
    Bool(&'a str, &'a mut bool),
    /// Has a string property.
    String(&'a str, &'a mut String),
    /// Has a sub node.
    Node(&'a str),
}

/// Represents meta data.
pub enum MetaData<'a> {
    /// Starts node.
    StartNode(&'a str),
    /// Ends node.
    EndNode,
    /// Sets bool property.
    Bool(&'a str, bool),
    /// Sets f64 property.
    F64(&'a str, f64),
    /// Sets string property.
    String(&'a str, String),
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

/// Used by meta readers to handle or forward a state.
/// This makes it easier to write generic meta readers wrapping a sub reader.
pub enum CommandState<T, U> {
    /// Handle state.
    Handle(T),
    /// Forward command to sub meta reader.
    Forward(U),
}
