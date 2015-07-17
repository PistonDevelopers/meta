#![deny(missing_docs)]

//! A DSL parsing library for human readable text documents

extern crate read_token;
extern crate range;

pub use parse_error_handler::{ stderr_unwrap, ParseErrorHandler, ParseStdErr };
pub use parse_error::ParseError;
pub use meta_rules::{ parse, Rule };
pub use search::Search;

/// The type of debug id used to track down errors in rules.
pub type DebugId = usize;

use std::rc::Rc;
use range::Range;

pub mod bootstrap;
pub mod json;
pub mod meta_rules;
pub mod tokenizer;

mod parse_error;
mod parse_error_handler;
mod search;

mod all {
    pub use super::*;
}

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
    String(Rc<String>, Rc<String>),
}

/// Reads syntax from text.
pub fn syntax(rules: &str) -> Result<Vec<(Rc<String>, Rule)>, (Range, ParseError)> {
    match bootstrap::convert(
        &try!(parse(&bootstrap::rules(), rules)),
        &mut vec![] // Ignored meta data
    ) {
        Ok(res) => Ok(res),
        Err(()) => Err((Range::empty(0), ParseError::Conversion(
            format!("Bootstrapping rules are incorrect"))))
    }
}

