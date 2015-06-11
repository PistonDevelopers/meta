use std::fmt::{ Display, Formatter };
use std::fmt::Error as FormatError;
use std::num::ParseFloatError;
use read_token::ParseStringError;
use std::rc::Rc;

use DebugId;

/// Errors reporting expected values.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Whitespace is required.
    ExpectedWhitespace(DebugId),
    /// New line is required.
    ExpectedNewLine(DebugId),
    /// Something is required.
    ExpectedSomething(DebugId),
    /// Expected number.
    ExpectedNumber(DebugId),
    /// Error when parsing float.
    ParseFloatError(ParseFloatError, DebugId),
    /// Expected text.
    ExpectedText(DebugId),
    /// Empty text not allowed.
    EmptyTextNotAllowed(DebugId),
    /// Invalid string format.
    ParseStringError(ParseStringError, DebugId),
    /// Expected token.
    ExpectedToken(Rc<String>, DebugId),
    /// An invalid rule.
    InvalidRule(&'static str, DebugId),
    /// No rules are specified.
    NoRules,
    /// Expected to reach the end.
    ExpectedEnd,
}

impl Display for ParseError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        match self {
            &ParseError::ExpectedWhitespace(debug_id) =>
                try!(write!(fmt, "#{}, Expected whitespace",
                    debug_id)),
            &ParseError::ExpectedNewLine(debug_id) =>
                try!(write!(fmt, "#{}, Expected new line",
                    debug_id)),
            &ParseError::ExpectedSomething(debug_id) =>
                try!(write!(fmt, "#{}, Expected something",
                    debug_id)),
            &ParseError::ExpectedNumber(debug_id) =>
                try!(write!(fmt, "#{}, Expected number", debug_id)),
            &ParseError::ParseFloatError(ref err, debug_id) =>
                try!(write!(fmt, "#{}, Invalid number format: {}",
                    debug_id, err)),
            &ParseError::ExpectedToken(ref token, debug_id) =>
                try!(write!(fmt, "#{}, Expected: `{}`", debug_id, token)),
            &ParseError::ExpectedText(debug_id) =>
                try!(write!(fmt, "#{}, Expected text", debug_id)),
            &ParseError::EmptyTextNotAllowed(debug_id) =>
                try!(write!(fmt, "#{}, Empty text not allowed", debug_id)),
            &ParseError::ParseStringError(err, debug_id) =>
                try!(write!(fmt, "#{}, Invalid string format: {}",
                    debug_id, err)),
            &ParseError::InvalidRule(msg, debug_id) =>
                try!(write!(fmt, "#{}, Invalid rule: {}", debug_id, msg)),
            &ParseError::NoRules =>
                try!(write!(fmt, "No rules are specified")),
            &ParseError::ExpectedEnd =>
                try!(write!(fmt, "Expected end")),
        }
        Ok(())
    }
}
