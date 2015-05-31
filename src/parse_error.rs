use std::fmt::{ Display, Formatter };
use std::fmt::Error as FormatError;
use std::num::ParseFloatError;
use read_token::ParseStringError;

use {
    Type,
    DebugId
};

/// Errors reporting expected values.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Not supported.
    NotSupported,
    /// Whitespace is required.
    ExpectedWhitespace(DebugId),
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
    ExpectedToken(String, DebugId),
    /// An invalid rule.
    InvalidRule(&'static str, DebugId),
    /// Expected nodes with other names.
    ExpectedNode(Vec<String>),
    /// Expected another propert type.
    ExpectedPropertyType(Type),
    /// Reaching end of node, but expected more properties.
    ExpectedMoreProperties(Vec<String>),
}

impl Display for ParseError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        match self {
            &ParseError::NotSupported =>
                try!(fmt.write_str("This feature is not supported")),
            &ParseError::ExpectedWhitespace(debug_id) =>
                try!(write!(fmt, "Expected whitespace (debug id `{}`)",
                    debug_id)),
            &ParseError::ExpectedSomething(debug_id) =>
                try!(write!(fmt, "Expected something (debug id `{}`)",
                    debug_id)),
            &ParseError::ExpectedNumber(debug_id) =>
                try!(write!(fmt, "Expected number (debug id `{}`)", debug_id)),
            &ParseError::ParseFloatError(ref err, debug_id) =>
                try!(fmt.write_fmt(format_args!(
                    "Invalid number format (debug id `{}`): {}", debug_id, err
                ))),
            &ParseError::ExpectedToken(ref token, debug_id) =>
                try!(write!(fmt, "Expected (debug id `{}`): `{}`", debug_id,
                    token)),
            &ParseError::ExpectedText(debug_id) =>
                try!(write!(fmt, "Expected text (debug id `{}`)", debug_id)),
            &ParseError::EmptyTextNotAllowed(debug_id) =>
                try!(write!(fmt, "Empty text not allowed (debug id `{}`)",
                    debug_id)),
            &ParseError::ParseStringError(err, debug_id) =>
                try!(write!(fmt, "Invalid string format (debug id `{}`): {}",
                    debug_id, err)),
            &ParseError::ExpectedNode(ref nodes) => {
                try!(fmt.write_str("Expected nodes: "));
                let mut tail = false;
                for node in nodes {
                    if tail {
                        try!(fmt.write_str(", "));
                    } else {
                        tail = true;
                    }
                    try!(fmt.write_str(&node));
                }
            }
            &ParseError::ExpectedPropertyType(ref ty) =>
                try!(fmt.write_fmt(format_args!(
                    "Expected property type: {}", ty
                ))),
            &ParseError::ExpectedMoreProperties(ref props) => {
                try!(fmt.write_str("Expected more properties: "));
                let mut tail = false;
                for prop in props {
                    if tail {
                        try!(fmt.write_str(", "));
                    } else {
                        tail = true;
                    }
                    try!(fmt.write_str(prop));
                }
            }
            &ParseError::InvalidRule(msg, debug_id) =>
                try!(fmt.write_fmt(format_args!(
                    "Invalid rule (debug id `{}`): {}", debug_id, msg
                ))),
        }
        Ok(())
    }
}
