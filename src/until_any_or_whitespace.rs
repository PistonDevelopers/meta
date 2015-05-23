use read_token;
use range::Range;
use std::rc::Rc;

use {
    MetaData,
    MetaReader,
    ParseError,
};

/// Stores information about reading until whitespace or any of some character.
pub struct UntilAnyOrWhitespace {
    /// The characters to stop at.
    pub any_characters: Rc<String>,
    /// Whether empty data is accepted or not.
    pub optional: bool,
    /// The property to store read text.
    pub property: Option<Rc<String>>,
}

impl UntilAnyOrWhitespace {
    /// Parses until whitespace or any specified characters.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        chars: &[char],
        offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        let (range, _) = read_token::until_any_or_whitespace(
            &self.any_characters, chars, offset);
        if range.length == 0 && !self.optional {
            Err((range, ParseError::ExpectedSomething))
        } else {
            if let Some(ref property) = self.property {
                let mut text = String::with_capacity(range.length);
                for c in chars.iter().take(range.length) {
                    text.push(*c);
                }
                match meta_reader.data(
                    MetaData::String(property.clone(), text),
                    state,
                    range
                ) {
                    Err(err) => Err((range, err)),
                    Ok(state) => Ok((range, state)),
                }
            } else {
                Ok((range, state.clone()))
            }
        }
    }
}