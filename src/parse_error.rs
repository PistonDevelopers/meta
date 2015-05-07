use std::fmt::{ Display, Formatter };
use std::fmt::Error as FormatError;

use Type;

/// Errors reporting expected values.
#[derive(Debug)]
pub enum ParseError<'a> {
    /// Not supported.
    NotSupported,
    /// Whitespace is required.
    ExpectedWhitespace,
    /// Expected nodes with other names.
    ExpectedNode(&'a [&'a str]),
    /// Expected another propert type.
    ExpectedPropertyType(Type),
    /// Reaching end of node, but expected more properties.
    ExpectedMoreProperties(&'a [&'a str]),
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        match self {
            &ParseError::NotSupported =>
                try!(fmt.write_str("This feature is not supported")),
            &ParseError::ExpectedWhitespace =>
                try!(fmt.write_str("Expected whitespace")),
            &ParseError::ExpectedNode(nodes) => {
                try!(fmt.write_str("Expected nodes: "));
                let mut tail = false;
                for node in nodes {
                    if tail {
                        try!(fmt.write_str(", "));
                    } else {
                        tail = true;
                    }
                    try!(fmt.write_str(node));
                }
            }
            &ParseError::ExpectedPropertyType(ref ty) =>
                try!(fmt.write_fmt(format_args!(
                    "Expected property type {}", ty
                ))),
            &ParseError::ExpectedMoreProperties(props) => {
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
        }
        Ok(())
    }
}
