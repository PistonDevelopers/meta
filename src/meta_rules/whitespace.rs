use read_token;
use range::Range;

use {
    DebugId,
    ParseError,
};

/// Stores information about whitespace.
#[derive(Clone, Debug, PartialEq)]
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
}

#[cfg(test)]
mod tests {
    use all::*;
    use meta_rules::Whitespace;
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
