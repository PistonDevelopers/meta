use range::Range;
use read_token::ReadToken;

use super::{
    ret_err,
    err_update,
    update,
    ParseResult,
};
use {
    DebugId,
    MetaData,
    Rule,
};
use tokenizer::TokenizerState;

/// Stores inforamtion about separated by.
#[derive(Clone, Debug, PartialEq)]
pub struct SeparateBy {
    /// The rule to separate.
    pub rule: Rule,
    /// The rule to separate by.
    pub by: Rule,
    /// Whether the rule must occur at least once.
    pub optional: bool,
    /// Whether the rule can end with separator.
    pub allow_trail: bool,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl SeparateBy {
    /// Parses rule repeatedly separated by another rule.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken,
        refs: &[Rule]
    ) -> ParseResult<TokenizerState> {
        let start = read_token;
        let mut read_token = *start;
        let mut state = state.clone();
        let mut first = true;
        let mut opt_error = None;
        loop {
            state = match self.rule.parse(tokens, &state, &read_token, refs) {
                Err(err) => {
                    match (first, self.optional, self.allow_trail) {
                          (true, false, _)
                        | (false, _, false) => {
                            return Err(ret_err(err, opt_error));
                        }
                          (true, true, _)
                        | (false, _, true) => {
                            err_update(Some(err), &mut opt_error);
                            break;
                        }
                    }
                }
                Ok((range, state, err)) => {
                    update(range, err, &mut read_token, &mut opt_error);
                    state
                }
            };
            state = match self.by.parse(
                tokens, &state, &read_token, refs
            ) {
                Err(err) => {
                    err_update(Some(err), &mut opt_error);
                    break;
                }
                Ok((range, state, err)) => {
                    update(range, err, &mut read_token, &mut opt_error);
                    state
                }
            };
            first = false;
        }
        Ok((read_token.subtract(start), state, opt_error))
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use all::tokenizer::*;
    use meta_rules::{ SeparateBy, Tag, UntilAnyOrWhitespace };
    use std::sync::Arc;
    use range::Range;
    use read_token::ReadToken;

    #[test]
    fn required() {
        let text = "foo()";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let sep = SeparateBy {
            debug_id: 0,
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1,
                any_characters: Arc::new(",)".into()),
                optional: false,
                property: None,
            }),
            by: Rule::Tag(Tag {
                debug_id: 2,
                text: Arc::new(",".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            optional: false,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokens, &s,
            &ReadToken::new(&text[4..], 4), &[]);
        assert_eq!(res, Err(Range::new(4, 0).wrap(
            ParseError::ExpectedSomething(1))));
    }

    #[test]
    fn optional() {
        let text = "foo()";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let sep = SeparateBy {
            debug_id: 0,
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1,
                any_characters: Arc::new(",)".into()),
                optional: false,
                property: None,
            }),
            by: Rule::Tag(Tag {
                debug_id: 2,
                text: Arc::new(",".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokens, &s,
            &ReadToken::new(&text[4..], 4), &[]);
        assert_eq!(res, Ok((Range::new(4, 0), s,
            Some(Range::new(4, 0).wrap(ParseError::ExpectedSomething(1))))));
    }

    #[test]
    fn disallow_trail() {
        let text = "foo(a,b,c,)";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let arg: Arc<String> = Arc::new("arg".into());
        let sep = SeparateBy {
            debug_id: 0,
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1,
                any_characters: Arc::new(",)".into()),
                optional: false,
                property: Some(arg.clone()),
            }),
            by: Rule::Tag(Tag {
                debug_id: 2,
                text: Arc::new(",".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokens, &s,
            &ReadToken::new(&text[4..], 4), &[]);
        assert_eq!(res, Err(Range::new(10, 0).wrap(
            ParseError::ExpectedSomething(1))));
    }

    #[test]
    fn allow_trail() {
        let text = "foo(a,b,c,)";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let arg: Arc<String> = Arc::new("arg".into());
        let sep = SeparateBy {
            debug_id: 0,
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1,
                any_characters: Arc::new(",)".into()),
                optional: false,
                property: Some(arg.clone()),
            }),
            by: Rule::Tag(Tag {
                debug_id: 2,
                text: Arc::new(",".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: true,
        };
        let res = sep.parse(&mut tokens, &s,
            &ReadToken::new(&text[4..], 4), &[]);
        assert_eq!(res, Ok((Range::new(4, 6), TokenizerState(3),
            Some(Range::new(10, 0).wrap(ParseError::ExpectedSomething(1))))));
        assert_eq!(tokens.len(), 3);
        assert_eq!(&tokens[0].data,
            &MetaData::String(arg.clone(), Arc::new("a".into())));
        assert_eq!(&tokens[1].data,
            &MetaData::String(arg.clone(), Arc::new("b".into())));
        assert_eq!(&tokens[2].data,
            &MetaData::String(arg.clone(), Arc::new("c".into())));
    }

    #[test]
    fn successful() {
        let text = "foo(a,b,c)";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let arg: Arc<String> = Arc::new("arg".into());
        let sep = SeparateBy {
            debug_id: 0,
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1,
                any_characters: Arc::new(",)".into()),
                optional: false,
                property: Some(arg.clone()),
            }),
            by: Rule::Tag(Tag {
                debug_id: 2,
                text: Arc::new(",".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokens, &s,
            &ReadToken::new(&text[4..], 4), &[]);
        assert_eq!(res, Ok((Range::new(4, 5), TokenizerState(3),
            Some(Range::new(9, 0).wrap(
                ParseError::ExpectedTag(Arc::new(",".into()), 2))))));
        assert_eq!(tokens.len(), 3);
        assert_eq!(&tokens[0].data,
            &MetaData::String(arg.clone(), Arc::new("a".into())));
        assert_eq!(&tokens[1].data,
            &MetaData::String(arg.clone(), Arc::new("b".into())));
        assert_eq!(&tokens[2].data,
            &MetaData::String(arg.clone(), Arc::new("c".into())));
    }

    #[test]
    fn nested() {
        let text = "a,b,c;d,e,f;";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let arg: Arc<String> = Arc::new("arg".into());
        let sep = SeparateBy {
            debug_id: 0,
            rule: Rule::SeparateBy(Box::new(SeparateBy {
                debug_id: 1,
                rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                    debug_id: 2,
                    any_characters: Arc::new(",;".into()),
                    optional: false,
                    property: Some(arg.clone()),
                }),
                by: Rule::Tag(Tag {
                    debug_id: 3,
                    text: Arc::new(",".into()),
                    not: false,
                    inverted: false,
                    property: None,
                }),
                optional: false,
                allow_trail: true,
            })),
            by: Rule::Tag(Tag {
                debug_id: 4,
                text: Arc::new(";".into()),
                not: false,
                inverted: false,
                property: None,
            }),
            optional: false,
            allow_trail: true,
        };
        let res = sep.parse(&mut tokens, &s, &ReadToken::new(&text, 0), &[]);
        assert_eq!(res, Ok((Range::new(0, 12), TokenizerState(6),
            Some(Range::new(12, 0).wrap(
                ParseError::ExpectedSomething(2))))));
        assert_eq!(tokens.len(), 6);
        assert_eq!(&tokens[0].data,
            &MetaData::String(arg.clone(), Arc::new("a".into())));
        assert_eq!(&tokens[1].data,
            &MetaData::String(arg.clone(), Arc::new("b".into())));
        assert_eq!(&tokens[2].data,
            &MetaData::String(arg.clone(), Arc::new("c".into())));
        assert_eq!(&tokens[3].data,
            &MetaData::String(arg.clone(), Arc::new("d".into())));
        assert_eq!(&tokens[4].data,
            &MetaData::String(arg.clone(), Arc::new("e".into())));
    }
}
