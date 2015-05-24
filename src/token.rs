use range::Range;
use read_token;
use std::rc::Rc;

use {
    MetaData,
    MetaReader,
    ParseError,
};

/// Stores information about token.
pub struct Token {
    /// The text to match against.
    pub text: Rc<String>,
    /// Whether to set property to true or false (inverted).
    pub inverted: Option<bool>,
    /// Which property to set if token matches.
    pub property: Option<Rc<String>>,
}

impl Token {
    /// Parses token.
    /// If the token is linked to a property,
    /// the property will be set.
    /// If the meta reader fails setting the property the error is handled.
    /// If the token is not linked to any property,
    /// the same state will be returned.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        chars: &[char],
        offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        if let Some(range) = read_token::token(&self.text, chars, offset) {
            match (self.inverted, &self.property) {
                (Some(inverted), &Some(ref name)) => {
                    match meta_reader.data(
                        MetaData::Bool(name.clone(), !inverted),
                        &state,
                        range
                    ) {
                        Err(err) => {
                            return Err((range, err));
                        }
                        Ok(state) => {
                            return Ok((range, state));
                        }
                    }
                }
                _ => {
                    return Ok((range, state.clone()))
                }
            }
        } else {
            return Err((Range::new(offset, 0),
                ParseError::ExpectedToken((&*self.text).clone())));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::rc::Rc;
    use range::Range;

    #[test]
    fn expected_token() {
        let text = ")";
        let chars: Vec<char> = text.chars().collect();
        let start_parenthesis = Token {
            text: Rc::new("(".into()),
            inverted: None,
            property: None
        };
        let mut tokenizer = Tokenizer::new();
        let res = start_parenthesis.parse(&mut tokenizer, &0, &chars, 0);
        assert_eq!(res, Err((
            Range::new(0, 0),
            ParseError::ExpectedToken("(".into())
            ))
        );
    }

    #[test]
    fn successful() {
        let text = "fn foo()";
        let chars: Vec<char> = text.chars().collect();
        let fn_ = Token {
            text: Rc::new("fn ".into()),
            inverted: None,
            property: None
        };
        let mut tokenizer = Tokenizer::new();
        let res = fn_.parse(&mut tokenizer, &0, &chars, 0);
        assert_eq!(res, Ok((Range::new(0, 3), 0)));
        assert_eq!(tokenizer.tokens.len(), 0);

        // Set bool property.
        let mut tokenizer = Tokenizer::new();
        let has_arguments: Rc<String> = Rc::new("has_arguments".into());
        let start_parenthesis = Token {
            text: Rc::new("(".into()),
            inverted: Some(false),
            property: Some(has_arguments.clone())
        };
        let res = start_parenthesis.parse(&mut tokenizer, &0, &chars[6..], 6);
        assert_eq!(res, Ok((Range::new(6, 1), 1)));
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(&tokenizer.tokens[0].0,
            &MetaData::Bool(has_arguments.clone(), true));

        // Set inverted bool property.
        let mut tokenizer = Tokenizer::new();
        let has_arguments: Rc<String> = Rc::new("has_no_arguments".into());
        let start_parenthesis = Token {
            text: Rc::new("(".into()),
            inverted: Some(true),
            property: Some(has_arguments.clone())
        };
        let res = start_parenthesis.parse(&mut tokenizer, &0, &chars[6..], 6);
        assert_eq!(res, Ok((Range::new(6, 1), 1)));
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(&tokenizer.tokens[0].0,
            &MetaData::Bool(has_arguments.clone(), false));
    }
}
