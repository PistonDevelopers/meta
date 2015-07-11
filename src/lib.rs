#![deny(missing_docs)]

//! A DSL parsing library for human readable text documents

extern crate read_token;
extern crate range;

pub use parse_error_handler::{ ParseErrorHandler, ParseStdErr };
pub use parse_error::ParseError;
pub use meta_rules::{ parse, Rule };

/// The type of debug id used to track down errors in rules.
pub type DebugId = usize;

use std::rc::Rc;

pub mod bootstrap;
pub mod meta_rules;
pub mod tokenizer;

mod parse_error;
mod parse_error_handler;

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
