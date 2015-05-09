use range::Range;

use Whitespace;
use Token;
use Parameter;
use MetaReader;
use ParseError;
use Select;

/// A rule describes how some section of a document should be parsed.
pub enum Rule<'a> {
    /// Read whitespace.
    Whitespace(Whitespace),
    /// Match against a token.
    Token(Token<'a>),
    /// Select one of the sub rules.
    /// If the first one does not succeed, try another and so on.
    /// If all sub rules fail, then the rule fails.
    Select(Select<'a>),
    /// Read parameter.
    Parameter(Parameter<'a>),
}

impl<'a> Rule<'a> {
    /// Parses rule.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        chars: &[char],
        offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        match self {
            &Rule::Whitespace(ref w) => {
                w.parse(chars, offset).map(|r| (r, state.clone()))
            }
            &Rule::Token(ref t) => {
                t.parse(meta_reader, state, chars, offset)
            }
            &Rule::Select(ref s) => {
                s.parse(meta_reader, state, chars, offset)
            }
            &Rule::Parameter(_) => unimplemented!(),

        }
    }
}
