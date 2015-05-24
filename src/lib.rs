#![deny(missing_docs)]

//! Meta parsing and encoding for data oriented design

extern crate read_token;
extern crate range;

pub use whitespace::Whitespace;
pub use parse_error_handler::{ ParseErrorHandler, ParseStdErr };
pub use parse_error::ParseError;
pub use ty::Type;
pub use token::Token;
pub use select::Select;
pub use node::{ Node, NodeRef, NodeVisit };
pub use optional::Optional;
pub use sequence::Sequence;
pub use until_any_or_whitespace::UntilAnyOrWhitespace;
pub use text::Text;
pub use number::Number;
pub use rule::Rule;
pub use meta_reader::MetaReader;
pub use tokenizer::{ Tokenizer, TokenizerState };

use std::rc::Rc;

mod parse_error;
mod parse_error_handler;
mod ty;
mod token;
mod whitespace;
mod select;
mod node;
mod optional;
mod sequence;
mod until_any_or_whitespace;
mod text;
mod number;
mod rule;
mod meta_reader;
mod tokenizer;

/// Represents meta data.
#[derive(PartialEq, Clone, Debug)]
pub enum MetaData {
    /// Starts node.
    StartNode(Rc<String>),
    /// Ends node.
    EndNode,
    /// Sets bool property.
    Bool(Rc<String>, bool),
    /// Sets f64 property.
    F64(Rc<String>, f64),
    /// Sets string property.
    String(Rc<String>, String),
}

#[inline(always)]
fn update<'a>(range: range::Range, chars: &mut &'a [char], offset: &mut usize) {
    let next_offset = range.next_offset();
    *chars = &chars[next_offset - *offset..];
    *offset = next_offset;
}
