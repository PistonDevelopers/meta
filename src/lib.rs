#![deny(missing_docs)]

//! A DSL parsing library for human readable text documents

extern crate read_token;
extern crate range;

pub use parse_error_handler::{
    stderr_unwrap,
    ParseErrorHandler
};
pub use parse_error::ParseError;
pub use meta_rules::{ parse, Rule };

/// The type of debug id used to track down errors in rules.
pub type DebugId = usize;

use std::rc::Rc;
use std::fs::File;
use std::path::Path;
use range::Range;

pub mod bootstrap;
pub mod json;
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

/// Convenience method for loading data.
/// Panics if there is an error, and writes error message to
/// standard error output.
pub fn load_syntax_data<A, B>(
    syntax_path: A,
    data_path: B
) -> Vec<(Range, MetaData)>
    where A: AsRef<Path>, B: AsRef<Path>
{
    use std::io::Read;

    let mut syntax_file = File::open(syntax_path).unwrap();
    let mut s = String::new();
    syntax_file.read_to_string(&mut s).unwrap();
    let rules = stderr_unwrap(&s, syntax(&s));

    let mut data_file = File::open(data_path).unwrap();
    let mut d = String::new();
    data_file.read_to_string(&mut d).unwrap();
    stderr_unwrap(&d, parse(&rules, &d))
}
