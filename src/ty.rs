use std::fmt::{ Display, Formatter };
use std::fmt::Error as FormatError;

/// Types of properties.
#[derive(Debug)]
pub enum Type {
    /// Either true or false.
    Bool,
    /// Text.
    String,
    /// Number.
    F64,
}

impl Display for Type {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        match self {
            &Type::Bool => try!(fmt.write_str("bool")),
            &Type::String => try!(fmt.write_str("text")),
            &Type::F64 => try!(fmt.write_str("num")),
        }
        Ok(())
    }
}
