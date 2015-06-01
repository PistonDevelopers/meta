use read_token;
use std::rc::Rc;

use {
    DebugId,
    MetaData,
    ParseError,
    ParseResult,
    Tokenizer,
    TokenizerState,
};

/// Stores information about reading until whitespace or any of some character.
pub struct UntilAnyOrWhitespace {
    /// The characters to stop at.
    pub any_characters: Rc<String>,
    /// Whether empty data is accepted or not.
    pub optional: bool,
    /// The property to store read text.
    pub property: Option<Rc<String>>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl UntilAnyOrWhitespace {
    /// Parses until whitespace or any specified characters.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        chars: &[char],
        offset: usize
    ) -> ParseResult<TokenizerState> {
        let (range, _) = read_token::until_any_or_whitespace(
            &self.any_characters, chars, offset);
        if range.length == 0 && !self.optional {
            Err((range, ParseError::ExpectedSomething(self.debug_id)))
        } else {
            if let Some(ref property) = self.property {
                let mut text = String::with_capacity(range.length);
                for c in chars.iter().take(range.length) {
                    text.push(*c);
                }
                Ok((range, tokenizer.data(
                    MetaData::String(property.clone(), text),
                    state,
                    range
                ), None))
            } else {
                Ok((range, state.clone(), None))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;
    use std::rc::Rc;

    #[test]
    fn required() {
        let text = "fn ()";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let name = UntilAnyOrWhitespace {
            debug_id: 0,
            any_characters: Rc::new("(".into()),
            optional: false,
            property: None
        };
        let res = name.parse(&mut tokenizer, &s, &chars[3..], 3);
        assert_eq!(res, Err((Range::new(3, 0),
            ParseError::ExpectedSomething(0))));
    }

    #[test]
    fn successful() {
        let text = "fn foo()";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let function_name: Rc<String> = Rc::new("function_name".into());
        let name = UntilAnyOrWhitespace {
            debug_id: 0,
            any_characters: Rc::new("(".into()),
            optional: false,
            property: Some(function_name.clone())
        };
        let res = name.parse(&mut tokenizer, &s, &chars[3..], 3);
        assert_eq!(res, Ok((Range::new(3, 3), TokenizerState(1), None)));
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(&tokenizer.tokens[0].0,
            &MetaData::String(function_name.clone(), "foo".into()));
    }
}
