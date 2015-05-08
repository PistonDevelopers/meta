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

impl Type {
    fn get_str(&self) -> &'static str {
        match *self {
            Type::Bool => "bool",
            Type::String => "text",
            Type::F64 => "num",
        }
    }
}

impl Display for Type {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        fmt.write_str(self.get_str())
    }
}
