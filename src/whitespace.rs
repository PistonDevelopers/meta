use read_token;
use range::Range;
use std::rc::Rc;

use {
    DebugId,
    ParseError,
    Node,
    Rule,
    Select,
    Token,
};

/// Stores information about whitespace.
pub struct Whitespace {
    /// Whether the whitespace is optional or required.
    pub optional: bool,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Whitespace {
    /// Parse whitespace.
    /// If whitespace is required and no whitespace is found,
    /// an error will be reported.
    pub fn parse(&self, chars: &[char], offset: usize) ->
        Result<Range, (Range, ParseError)>
    {
        let range = read_token::whitespace(chars, offset);
        if range.length == 0 && !self.optional {
            Err((range, ParseError::ExpectedWhitespace(self.debug_id)))
        } else {
            Ok(range)
        }
    }

    /// Gets the rule for whitespace in the meta language.
    pub fn rule() -> Node {
        Node {
            debug_id: 0,
            name: Rc::new("whitespace".into()),
            body: vec![
                Rule::Whitespace(Whitespace { debug_id: 1, optional: true }),
                Rule::Token(Token {
                    debug_id: 2,
                    text: Rc::new("whitespace".into()),
                    inverted: false,
                    property: None
                }),
                Rule::Select(Select { debug_id: 3, args: vec![
                    Rule::Token(Token {
                        debug_id: 4,
                        text: Rc::new("?".into()),
                        inverted: false,
                        property: Some(Rc::new("optional".into())),
                    }),
                    Rule::Token(Token {
                        debug_id: 5,
                        text: Rc::new("!".into()),
                        inverted: true,
                        property: Some(Rc::new("optional".into())),
                    }),
                ]}),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;

    #[test]
    fn optional() {
        let text = "a,b, c";
        let chars: Vec<char> = text.chars().collect();
        let optional_whitespace = Whitespace { debug_id: 0, optional: true };
        assert_eq!(optional_whitespace.parse(&chars, 0),
            Ok(Range::new(0, 0)));
        assert_eq!(optional_whitespace.parse(&chars[4..], 4),
            Ok(Range::new(4, 1)));
    }

    #[test]
    fn required() {
        let text = "a,   b,c";
        let chars: Vec<char> = text.chars().collect();
        let required_whitespace = Whitespace { debug_id: 0, optional: false };
        assert_eq!(required_whitespace.parse(&chars[2..], 2),
            Ok(Range::new(2, 3)));
        // Prints an error message to standard error output.
        assert_eq!(required_whitespace.parse(&chars[7..], 7),
            Err((Range::new(7, 0), ParseError::ExpectedWhitespace(0))));
    }
}
