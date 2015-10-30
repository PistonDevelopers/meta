//! Building blocks for meta rules.

pub use self::lines::Lines;
pub use self::node::Node;
pub use self::number::Number;
pub use self::optional::Optional;
pub use self::repeat::Repeat;
pub use self::rule::Rule;
pub use self::select::Select;
pub use self::separate_by::SeparateBy;
pub use self::sequence::Sequence;
pub use self::text::Text;
pub use self::tag::Tag;
pub use self::until_any::UntilAny;
pub use self::until_any_or_whitespace::UntilAnyOrWhitespace;
pub use self::whitespace::Whitespace;

use range::Range;
use read_token::ReadToken;
use {
    MetaData,
    ParseError,
    Syntax,
};
use tokenizer::TokenizerState;

mod lines;
mod node;
mod number;
mod optional;
mod repeat;
mod rule;
mod select;
mod separate_by;
mod sequence;
mod text;
mod tag;
mod until_any;
mod until_any_or_whitespace;
mod whitespace;

/// Parses text with rules.
pub fn parse(
    rules: &Syntax,
    text: &str,
    tokens: &mut Vec<Range<MetaData>>
) -> Result<(), Range<ParseError>> {
    let s = TokenizerState(tokens.len());
    let n = match rules.rules.len() {
        0 => { return Err(Range::empty(0).wrap(ParseError::NoRules)); }
        x => x
    };
    let read_token = ReadToken::new(&text, 0);
    let res = rules.rules[n - 1].parse(tokens, &s, &read_token, &rules.rules);
    match res {
        Ok((range, s, opt_error)) => {
            // Report error if did not reach the end of text.
            if range.next_offset() < text.chars().count() {
                Err(ret_err(
                    Range::empty(range.next_offset()).wrap(
                        ParseError::ExpectedEnd),
                    opt_error
                ))
            } else {
                tokens.truncate(s.0);
                Ok(())
            }
        }
        Err(range_err) => Err(range_err)
    }
}

/// Parses text with rules, formatting the error as a `String`.
pub fn parse_errstr(
    rules: &Syntax,
    text: &str,
    tokens: &mut Vec<Range<MetaData>>
) -> Result<(), String> {
    use ParseErrorHandler;

    match parse(rules, text, tokens) {
        Ok(()) => Ok(()),
        Err(range_err) => {
            let mut w: Vec<u8> = vec![];
            ParseErrorHandler::new(&text).write(&mut w, range_err).unwrap();
            Err(String::from_utf8(w).unwrap())
        }
    }
}

/// Updates the references such that they point to each other.
pub fn update_refs(&mut Syntax { ref mut rules, ref names }: &mut Syntax) {
    for r in rules {
        r.update_refs(names);
    }
}

/// A parse result succeeds with a new state,
/// plus an optional error to replace other errors if it is deeper.
/// The deepest error is likely the most useful.
pub type ParseResult<S> = Result<(Range, S, Option<Range<ParseError>>),
    Range<ParseError>>;

/// Updates the parser state.
/// Used by rules that have multiple sub rules.
#[inline(always)]
fn update<'a>(
    range: Range,
    err: Option<Range<ParseError>>,
    read_token: &mut ReadToken<'a>,
    opt_error: &mut Option<Range<ParseError>>
) {
    *read_token = read_token.consume(range.length);
    err_update(err, opt_error);
}

/// Picks deepest error, overwriting with the newest one if they are
/// equally deep.
#[inline(always)]
fn err_update(
    err: Option<Range<ParseError>>,
    opt_error: &mut Option<Range<ParseError>>
) {
    if let &mut Some(ref mut opt_error) = opt_error {
        if let Some(err) = err {
            if opt_error.next_offset() <= err.next_offset() {
                *opt_error = err;
            }
        }
    } else {
        *opt_error = err;
    };
}

/// This is used to pick the deepest error or two alternatives,
/// one from a rule that fails certainly and another that could be optional.
#[inline(always)]
fn ret_err(a: Range<ParseError>, b: Option<Range<ParseError>>)
-> Range<ParseError> {
    if let Some(b) = b {
        if b.next_offset() > a.next_offset() {
            b
        } else {
            a
        }
    } else {
        a
    }
}

#[cfg(test)]
mod tests{
    use range::Range;
    use all::*;

    #[test]
    fn no_rules() {
        assert_eq!(parse(&Syntax::new(), "", &mut vec![]),
            Err(Range::empty(0).wrap(ParseError::NoRules)));
    }
}
