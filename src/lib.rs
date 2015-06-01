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
pub use separated_by::SeparatedBy;
pub use until_any_or_whitespace::UntilAnyOrWhitespace;
pub use text::Text;
pub use number::Number;
pub use rule::Rule;
pub use tokenizer::{ Tokenizer, TokenizerState };

/// The type of debug id used to track down errors in rules.
pub type DebugId = usize;

use std::rc::Rc;
use range::Range;

mod parse_error;
mod parse_error_handler;
mod ty;
mod token;
mod whitespace;
mod select;
mod node;
mod optional;
mod sequence;
mod separated_by;
mod until_any_or_whitespace;
mod text;
mod number;
mod rule;
mod tokenizer;

/// A parse result succeeds with a new state,
/// plus an optional error to replace other errors if it is deeper.
/// The deepest error is likely the most useful.
pub type ParseResult<S> = Result<(Range, S, Option<(Range, ParseError)>),
    (Range, ParseError)>;

/// Represents meta data.
#[derive(PartialEq, Clone, Debug)]
pub enum MetaData {
    /// Starts node.
    StartNode(Rc<String>),
    /// Ends node.
    EndNode(Rc<String>),
    /// Sets bool property.
    Bool(Rc<String>, bool),
    /// Sets f64 property.
    F64(Rc<String>, f64),
    /// Sets string property.
    String(Rc<String>, String),
}

/// Updates the parser state.
/// Used by rules that have multiple sub rules.
#[inline(always)]
fn update<'a>(
    range: range::Range,
    err: Option<(Range, ParseError)>,
    chars: &mut &'a [char],
    offset: &mut usize,
    opt_error: &mut Option<(Range, ParseError)>
) {
    let next_offset = range.next_offset();
    *chars = &chars[next_offset - *offset..];
    *offset = next_offset;
    err_update(err, opt_error);
}

/// Picks deepest error, overwriting with the newest one if they are
/// equally deep.
#[inline(always)]
fn err_update(
    err: Option<(Range, ParseError)>,
    opt_error: &mut Option<(Range, ParseError)>
) {
    if let &mut Some(ref mut opt_error) = opt_error {
        if let Some(err) = err {
            if opt_error.0.next_offset() <= err.0.next_offset() {
                *opt_error = err;
            }
        }
    } else {
        *opt_error = err;
    };
}

/// This is used to pick the deepest error or two alternatives,
/// one from a rule that fails certainly and another that could be optional.
#[inline(always)]
fn ret_err(a: (Range, ParseError), b: Option<(Range, ParseError)>) ->
    (Range, ParseError) {
    if let Some(b) = b {
        if b.0.next_offset() > a.0.next_offset() {
            b
        } else {
            a
        }
    } else {
        a
    }
}
