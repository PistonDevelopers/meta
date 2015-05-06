use read_token;
use range::Range;

use {
    Error,
    ErrorHandler,
    Parameter,
    Rule,
    Token,
};

/// Stores information about whitespace.
pub struct Whitespace {
    /// Whether the whitespace is optional or required.
    pub optional: bool,
}

impl Whitespace {
    /// Parse whitespace.
    /// If whitespace is required and no whitespace is found,
    /// an error will be reported.
    pub fn parse<E>(&self, error_handler: &mut E, chars: &[char], offset: usize) ->
        Option<Range>
        where E: ErrorHandler
    {
        let range = read_token::whitespace(chars, offset);
        if range.length == 0 && !self.optional {
            error_handler.error(range, Error::ExpectedWhitespace);
            None
        } else {
            Some(range)
        }
    }
}

/// A hard coded parameter rule for whitespace.
pub static WHITESPACE: Parameter<'static> = Parameter {
    name: "whitespace",
    args: &["optional"],
    value: None,
    body: &[
        Rule::Whitespace(Whitespace { optional: true }),
        Rule::Token(Token {
            text: "whitespace",
            inverted: None,
            predicate: None
        }),
        Rule::Select(&[
            Rule::Token(Token {
                text: "?",
                inverted: Some(false),
                predicate: Some("optional"),
            }),
            Rule::Token(Token {
                text: "!",
                inverted: Some(true),
                predicate: Some("optional"),
            }),
        ]),
    ],
};

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;

    #[test]
    fn optional() {
        let text = "a,b, c";
        let chars: Vec<char> = text.chars().collect();
        let optional_whitespace = Whitespace { optional: true };
        let ref mut std_err = StdErr::new(text);
        assert_eq!(optional_whitespace.parse(std_err, &chars, 0),
            Some(Range::new(0, 0)));
        assert_eq!(optional_whitespace.parse(std_err, &chars[4..], 4),
            Some(Range::new(4, 1)));
    }

    #[test]
    fn required() {
        let text = "a,   b,c";
        let chars: Vec<char> = text.chars().collect();
        let required_whitespace = Whitespace { optional: false };
        let ref mut std_err = StdErr::new(text);
        assert_eq!(required_whitespace.parse(std_err, &chars[2..], 2),
            Some(Range::new(2, 3)));
        // Prints an error message to standard error output.
        assert_eq!(required_whitespace.parse(std_err, &chars[7..], 7),
            None);
    }
}
