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
