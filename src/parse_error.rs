use std::fmt::{ Display, Formatter };
use std::fmt::Error as FormatError;

use Type;

/// Errors reporting expected values.
#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    /// Not supported.
    NotSupported,
    /// Whitespace is required.
    ExpectedWhitespace,
    /// Something is required.
    ExpectedSomething,
    /// Expected token.
    ExpectedToken(String),
    /// Expected nodes with other names.
    ExpectedNode(Vec<String>),
    /// Expected another propert type.
    ExpectedPropertyType(Type),
    /// Reaching end of node, but expected more properties.
    ExpectedMoreProperties(Vec<String>),
    /// An invalid rule.
    InvalidRule(&'static str),
}

impl Display for ParseError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        match self {
            &ParseError::NotSupported =>
                try!(fmt.write_str("This feature is not supported")),
            &ParseError::ExpectedWhitespace =>
                try!(fmt.write_str("Expected whitespace")),
            &ParseError::ExpectedSomething =>
                try!(fmt.write_str("Expected something")),
            &ParseError::ExpectedToken(ref token) =>
                try!(fmt.write_fmt(format_args!(
                    "Expected `{}`", token
                ))),
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
                    "Expected property type {}", ty
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
            &ParseError::InvalidRule(msg) =>
                try!(fmt.write_fmt(format_args!(
                    "Invalid rule `{}`", msg
                ))),
        }
        Ok(())
    }
}
