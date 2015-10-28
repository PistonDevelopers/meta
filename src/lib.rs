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

use std::sync::Arc;
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
    StartNode(Arc<String>),
    /// Ends node.
    EndNode(Arc<String>),
    /// Sets bool property.
    Bool(Arc<String>, bool),
    /// Sets f64 property.
    F64(Arc<String>, f64),
    /// Sets string property.
    String(Arc<String>, Arc<String>),
}

/// Stores syntax.
#[derive(Clone, Debug, PartialEq)]
pub struct Syntax {
    /// Rule data.
    pub rules: Vec<Rule>,
    /// Name of rules.
    pub names: Vec<Arc<String>>,
}

impl Syntax {
    /// Creates a new syntax.
    pub fn new() -> Syntax {
        Syntax {
            rules: vec![],
            names: vec![]
        }
    }

    /// Adds a new rule.
    pub fn push(&mut self, name: Arc<String>, rule: Rule) {
        self.rules.push(rule);
        self.names.push(name);
    }
}

/// Reads syntax from text.
pub fn syntax(rules: &str) -> Result<Syntax, Range<ParseError>> {
    let mut tokens = vec![];
    try!(parse(&bootstrap::rules(), rules, &mut tokens));
    let mut ignored_meta_data = vec![];
    match bootstrap::convert(&tokens, &mut ignored_meta_data) {
        Ok(res) => Ok(res),
        Err(()) => Err((Range::empty(0).wrap(ParseError::Conversion(
            format!("Bootstrapping rules are incorrect")))))
    }
}

/// Convenience method for loading data, using the meta language.
/// Panics if there is an error, and writes error message to
/// standard error output.
pub fn load_syntax_data<A, B>(
    syntax_path: A,
    data_path: B
) -> Vec<Range<MetaData>>
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
    let mut tokens = vec![];
    stderr_unwrap(&d, parse(&rules, &d, &mut tokens));
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_thread_safe<T: Send + Sync>() {}

    #[test]
    fn meta_data_thread_safe() {
        is_thread_safe::<MetaData>();
    }

    #[test]
    fn parse_error_thread_safe() {
        is_thread_safe::<ParseError>();
    }

    #[test]
    fn rule_thread_safe() {
        is_thread_safe::<Rule>();
    }

    #[test]
    fn syntax_thread_safe() {
        is_thread_safe::<Syntax>();
    }
}
