#![deny(missing_docs)]

//! # Piston-Meta
//! A DSL parsing library for human readable text documents
//!
//! ### Introduction
//!
//! Piston-Meta makes it easy to write parsers for human readable text documents.
//! It can be used for language design, custom formats and data driven development.
//!
//! Meta parsing is a development technique that goes back to the first modern computer.
//! The idea is to turn pieces of a computer program into a programmable pipeline,
//! and thereby accelerating development.
//! An important, but surprisingly reusable part across projects, is the concept of generating
//! structured data from text, since text is easy to modify and reason about.
//!
//! Most programs that work with text use the following pipeline:
//!
//! ```ignore
//! f : text -> data
//! ```
//!
//! The problem with this approach is that `f` changes from project to project,
//! and the task of transforming text into a data structure can get very complex.
//! For example, to create a parser for the syntax of a programming language,
//! one might need several thousands lines of code.
//! This slows down development and increases the chance of making errors.
//!
//! Meta parsing is a technique where `f` gets splitted into two steps:
//!
//! ```ignore
//! f <=> f2 . f1
//! f1 : text -> meta data
//! f2 : meta data -> data
//! ```
//!
//! The first step `f1` takes text and converts it into meta data.
//! A DSL (Domain Specific Language) is used to describe how this transformation happens.
//! The second step `f2` converts meta data into data, and this is often written as code.
//!
//! ### Rules
//!
//! The meta language is used to describe how to read other documents.
//! First you define some strings to reuse, then some node rules.
//! The last node is used to read the entire document.
//!
//! `20 document = [.l(string:"string") .l(node:"node") .w?]`
//!
//! Strings starts with underscore and can be reused among the rules:
//!
//! `_opt: "optional"`
//!
//! Nodes start with a number that gets multiplied with 1000 and used as debug id.
//! If you get an error `#4003`, then it was caused by a rule in the node starting with 4.
//!
//! |Rule|Description|
//! |----|-----------|
//! |.l(rule)|Separates sub rule with lines.|
//! |.r?(rule)|Repeats sub rule until it fails, allows zero repetitions.|
//! |.r!(rule)|Repeats sub rule until it fails, requires at least one repetition.|
//! |...any_characters?:name|Reads a string until any characters, allows zero characters. Name is optional.|
//! |...any_characters!:name|Reads a string until any characters, requires at least one character. Name is optional.|
//! |..any_characters?:name|Reads a string until any characters or whitespace, allows zero characters. Name is optional.|
//! |..any_characters!:name|Reads a string until any characters or whitespace, requires at least one character. Name is optional.|
//! |.w?|Reads whitespace. The whitespace is optional.|
//! |.w!|Reads whitespace. The whitespace is required.|
//! |?rule|Makes the rule optional.|
//! |"token":name|Expects a token, sets name to `true`. Name is optional.|
//! |"token":!name|Expects a token, sets name to `false`. Name is required.|
//! |!"token":name|Fails if token is read, sets name to `true` if it is not read. Name is optional.|
//! |!"token":!name|Fails if token is read, sets name to `false` if it is not read. Name is required.|
//! |!rule|Fails if rule is read.|
//! |.s?(by_rule rule)|Separates rule by another rule, allows zero repetitions.|
//! |.s!(by_rule rule)|Separates rule by another rule, requires at least one repetition.|
//! |.s?.(by_rule rule)|Separates rule by another rule, allows trailing.|
//! |{rules}|Selects a rule. Tries the first rule, then the second etc. Rules are separated by whitespace.|
//! |[rules]|A sequence of rules. Rules are separated by whitespace.|
//! |node|Uses a node without a name. The read data is put in the current node.|
//! |node:name|Uses a node with a name. The read data is put in a new node with the name.|
//! |.t?:name|Reads a JSON string with a name. The string can be empty. Name is optional.|
//! |.t!:name|Reads a JSON string with a name. The string can not be empty. Name is optional.|
//! |.$:name|Reads a number with a name. The name is optional.|
//! |.$_:name|Reads a number with underscore as visible separator, for example `10_000`. The name is optional.|
//!
//! ### "Hello world" in Piston-Meta
//!
//! ```rust
//! extern crate piston_meta;
//!
//! use piston_meta::*;
//!
//! fn main() {
//!     let text = r#"hi James!"#;
//!     let rules = r#"
//!         1 say_hi = ["hi" .w? {"James":"james" "Peter":"peter"} "!"]
//!         2 document = say_hi
//!     "#;
//!     // Parse rules with meta language and convert to rules for parsing text.
//!     let rules = match syntax_errstr(rules) {
//!         Err(err) => {
//!             println!("{}", err);
//!             return;
//!         }
//!         Ok(rules) => rules
//!     };
//!     let mut data = vec![];
//!     match parse_errstr(&rules, text, &mut data) {
//!         Err(err) => {
//!             println!("{}", err);
//!             return;
//!         }
//!         Ok(()) => {}
//!     };
//!     json::print(&data);
//! }
//! ```
//!
//! ### Bootstrapping
//!
//! When the meta language changes, bootstrapping is used to host old meta syntax in the new meta syntax. Here is how it works:
//!
//! 1. Piston-Meta contains composable rules that can parse many human readable text formats.
//! 2. Piston-Meta knows how to parse and convert to its own rules, known as "bootstrapping".
//! 3. Therefore, you can tell Piston-Meta how to parse other text formats using a meta language!
//! 4. Including the text format describing how to parse its own syntax, which generates equivalent rules to the ones hard coded in Rust.
//! 5. New versions of the meta language can describe older versions to keep backwards compatibility, by changing the self syntax slightly, so it can read an older version of itself.
//!

extern crate read_token;
extern crate range;
#[macro_use]
extern crate lazy_static;

pub use parse_error_handler::{
    stderr_unwrap,
    ParseErrorHandler
};
pub use parse_error::ParseError;
pub use meta_rules::{
    parse,
    parse_errstr,
    parse_errstr_with_indent,
    parse_with_indent,
    Rule
};
pub use bootstrap::Convert;

/// The type of debug id used to track down errors in rules.
pub type DebugId = usize;

use std::sync::Arc;
use std::fs::File;
use std::path::Path;
pub use range::Range;

pub mod bootstrap;
pub mod json;
pub mod meta_rules;
pub mod tokenizer;

mod parse_error;
mod parse_error_handler;
pub mod optimize;

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

    /// Optimizes syntax.
    pub fn optimize(self) -> Syntax {
        let new_rules = self.rules.iter().map(|r| optimize::optimize_rule(&r, &self.rules)).collect();
        Syntax {rules: new_rules, names: self.names}
    }
}

/// Reads syntax from text.
pub fn syntax(rules: &str) -> Result<Syntax, Range<ParseError>> {
    lazy_static! {
        static ref BOOTSTRAP_RULES: Syntax = bootstrap::rules().optimize();
    }

    let mut tokens = vec![];
    parse(&BOOTSTRAP_RULES, rules, &mut tokens)?;
    let mut ignored_meta_data = vec![];
    match bootstrap::convert(&tokens, &mut ignored_meta_data) {
        Ok(res) => Ok(res.optimize()),
        Err(()) => Err(Range::empty(0).wrap(ParseError::Conversion(
            format!("Bootstrapping rules are incorrect"))))
    }
}

/// Reads syntax from text, formatting the error as `String`.
pub fn syntax_errstr(rules: &str) -> Result<Syntax, String> {
    match syntax(rules) {
        Ok(syntax) => Ok(syntax),
        Err(range_err) => {
            let mut w: Vec<u8> = vec![];
            ParseErrorHandler::new(&rules).write(&mut w, range_err).unwrap();
            Err(String::from_utf8(w).unwrap())
        }
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
