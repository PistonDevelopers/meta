use read_token;
use range::Range;

use {
    MetaReader,
    ParseError,
};

/// Stores information about text.
pub struct Text<'a> {
    /// Whether to allow empty string.
    pub allow_empty: bool,
    /// Which property to set if text is read.
    pub property: Option<&'a str>,
}

impl<'a> Text<'a> {
    /// Parses text.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        chars: &[char],
        offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        if let Some(range) = read_token::string(chars, offset) {
            if !self.allow_empty && range.length == 2 {
                Err((range, ParseError::EmptyTextNotAllowed))
            } else {
                match read_token::parse_string(
                    chars, offset, range.next_offset()) {
                    // Focus range to invalid string format.
                    Err(err) => Err((err.range(),
                        ParseError::ParseStringError(err))),
                    Ok(text) => {
                        if let Some(property) = self.property {
                            match meta_reader.set_as_string(property, text ,state) {
                                Err(err) => Err((range, err)),
                                Ok(state) => Ok((range, state)),
                            }
                        } else {
                            Ok((range, state.clone()))
                        }
                    }
                }
            }
        } else {
            Err((Range::new(offset, 0), ParseError::ExpectedText))
        }
    }
}
