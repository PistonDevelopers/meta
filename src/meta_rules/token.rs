use range::Range;
use read_token;
use std::rc::Rc;

use super::{
    ParseResult,
};
use {
    DebugId,
    MetaData,
    ParseError,
    Tokenizer,
    TokenizerState,
};

/// Stores information about token.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The text to match against.
    pub text: Rc<String>,
    /// Whether to set property to true or false (inverted).
    pub inverted: bool,
    /// Which property to set if token matches.
    pub property: Option<Rc<String>>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Token {
    /// Parses token.
    /// If the token is linked to a property,
    /// the property will be set.
    /// If the meta reader fails setting the property the error is handled.
    /// If the token is not linked to any property,
    /// the same state will be returned.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        chars: &[char],
        offset: usize
    ) -> ParseResult<TokenizerState> {
        if let Some(range) = read_token::token(&self.text, chars, offset) {
            match &self.property {
                &Some(ref name) => {
                    Ok((range, tokenizer.data(
                        MetaData::Bool(name.clone(), !self.inverted),
                        &state,
                        range
                    ), None))
                }
                _ => {
                    return Ok((range, state.clone(), None))
                }
            }
        } else {
            return Err((Range::new(offset, 0),
                ParseError::ExpectedToken(self.text.clone(),
                self.debug_id)));
        }
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use meta_rules::Token;
    use std::rc::Rc;
    use range::Range;

    #[test]
    fn expected_token() {
        let text = ")";
        let chars: Vec<char> = text.chars().collect();
        let start_parenthesis = Token {
            debug_id: 0,
            text: Rc::new("(".into()),
            inverted: false,
            property: None
        };
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let res = start_parenthesis.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Err((
            Range::new(0, 0),
            ParseError::ExpectedToken(Rc::new("(".into()), 0)
            ))
        );
    }

    #[test]
    fn successful() {
        let text = "fn foo()";
        let chars: Vec<char> = text.chars().collect();
        let fn_ = Token {
            debug_id: 0,
            text: Rc::new("fn ".into()),
            inverted: false,
            property: None
        };
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let res = fn_.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Ok((Range::new(0, 3), s, None)));
        assert_eq!(tokenizer.tokens.len(), 0);

        // Set bool property.
        let mut tokenizer = Tokenizer::new();
        let has_arguments: Rc<String> = Rc::new("has_arguments".into());
        let start_parenthesis = Token {
            debug_id: 0,
            text: Rc::new("(".into()),
            inverted: false,
            property: Some(has_arguments.clone())
        };
        let s = TokenizerState::new();
        let res = start_parenthesis.parse(&mut tokenizer, &s, &chars[6..], 6);
        assert_eq!(res, Ok((Range::new(6, 1), TokenizerState(1), None)));
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(&tokenizer.tokens[0].1,
            &MetaData::Bool(has_arguments.clone(), true));

        // Set inverted bool property.
        let mut tokenizer = Tokenizer::new();
        let has_arguments: Rc<String> = Rc::new("has_no_arguments".into());
        let start_parenthesis = Token {
            debug_id: 0,
            text: Rc::new("(".into()),
            inverted: true,
            property: Some(has_arguments.clone())
        };
        let s = TokenizerState::new();
        let res = start_parenthesis.parse(&mut tokenizer, &s, &chars[6..], 6);
        assert_eq!(res, Ok((Range::new(6, 1), TokenizerState(1), None)));
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(&tokenizer.tokens[0].1,
            &MetaData::Bool(has_arguments.clone(), false));
    }
}
