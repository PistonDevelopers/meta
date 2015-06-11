use range::Range;
use std::rc::Rc;

use {
    ret_err,
    err_update,
    update,
    DebugId,
    ParseResult,
    Rule,
    Tokenizer,
    TokenizerState,
};

/// Stores inforamtion about separated by.
#[derive(Clone)]
pub struct SeparatedBy {
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

impl SeparatedBy {
    /// Parses rule repeatedly separated by another rule.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        mut chars: &[char],
        start_offset: usize,
        refs: &[(Rc<String>, Rule)]
    ) -> ParseResult<TokenizerState> {
        let mut offset = start_offset;
        let mut state = state.clone();
        let mut first = true;
        let mut opt_error = None;
        loop {
            state = match self.rule.parse(
                tokenizer, &state, chars, offset, refs
            ) {
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
                    update(range, err, &mut chars, &mut offset, &mut opt_error);
                    state
                }
            };
            state = match self.by.parse(
                tokenizer, &state, chars, offset, refs
            ) {
                Err(err) => {
                    err_update(Some(err), &mut opt_error);
                    break;
                }
                Ok((range, state, err)) => {
                    update(range, err, &mut chars, &mut offset, &mut opt_error);
                    state
                }
            };
            first = false;
        }
        Ok((Range::new(start_offset, offset - start_offset), state, opt_error))
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::rc::Rc;
    use range::Range;

    #[test]
    fn required() {
        let text = "foo()";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let sep = SeparatedBy {
            debug_id: 0,
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1,
                any_characters: Rc::new(",)".into()),
                optional: false,
                property: None,
            }),
            by: Rule::Token(Token {
                debug_id: 2,
                text: Rc::new(",".into()),
                inverted: false,
                property: None,
            }),
            optional: false,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars[4..], 4, &[]);
        assert_eq!(res, Err((Range::new(4, 0),
            ParseError::ExpectedSomething(1))));
    }

    #[test]
    fn optional() {
        let text = "foo()";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let sep = SeparatedBy {
            debug_id: 0,
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1,
                any_characters: Rc::new(",)".into()),
                optional: false,
                property: None,
            }),
            by: Rule::Token(Token {
                debug_id: 2,
                text: Rc::new(",".into()),
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars[4..], 4, &[]);
        assert_eq!(res, Ok((Range::new(4, 0), s,
            Some((Range::new(4, 0), ParseError::ExpectedSomething(1))))));
    }

    #[test]
    fn disallow_trail() {
        let text = "foo(a,b,c,)";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let arg: Rc<String> = Rc::new("arg".into());
        let sep = SeparatedBy {
            debug_id: 0,
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1,
                any_characters: Rc::new(",)".into()),
                optional: false,
                property: Some(arg.clone()),
            }),
            by: Rule::Token(Token {
                debug_id: 2,
                text: Rc::new(",".into()),
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars[4..], 4, &[]);
        assert_eq!(res, Err((Range::new(10, 0),
            ParseError::ExpectedSomething(1))));
    }

    #[test]
    fn allow_trail() {
        let text = "foo(a,b,c,)";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let arg: Rc<String> = Rc::new("arg".into());
        let sep = SeparatedBy {
            debug_id: 0,
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1,
                any_characters: Rc::new(",)".into()),
                optional: false,
                property: Some(arg.clone()),
            }),
            by: Rule::Token(Token {
                debug_id: 2,
                text: Rc::new(",".into()),
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: true,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars[4..], 4, &[]);
        assert_eq!(res, Ok((Range::new(4, 6), TokenizerState(3),
            Some((Range::new(10, 0), ParseError::ExpectedSomething(1))))));
        assert_eq!(tokenizer.tokens.len(), 3);
        assert_eq!(&tokenizer.tokens[0].1,
            &MetaData::String(arg.clone(), Rc::new("a".into())));
        assert_eq!(&tokenizer.tokens[1].1,
            &MetaData::String(arg.clone(), Rc::new("b".into())));
        assert_eq!(&tokenizer.tokens[2].1,
            &MetaData::String(arg.clone(), Rc::new("c".into())));
    }

    #[test]
    fn successful() {
        let text = "foo(a,b,c)";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let arg: Rc<String> = Rc::new("arg".into());
        let sep = SeparatedBy {
            debug_id: 0,
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1,
                any_characters: Rc::new(",)".into()),
                optional: false,
                property: Some(arg.clone()),
            }),
            by: Rule::Token(Token {
                debug_id: 2,
                text: Rc::new(",".into()),
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars[4..], 4, &[]);
        assert_eq!(res, Ok((Range::new(4, 5), TokenizerState(3),
            Some((Range::new(9, 0),
                ParseError::ExpectedToken(Rc::new(",".into()), 2))))));
        assert_eq!(tokenizer.tokens.len(), 3);
        assert_eq!(&tokenizer.tokens[0].1,
            &MetaData::String(arg.clone(), Rc::new("a".into())));
        assert_eq!(&tokenizer.tokens[1].1,
            &MetaData::String(arg.clone(), Rc::new("b".into())));
        assert_eq!(&tokenizer.tokens[2].1,
            &MetaData::String(arg.clone(), Rc::new("c".into())));
    }

    #[test]
    fn nested() {
        let text = "a,b,c;d,e,f;";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let arg: Rc<String> = Rc::new("arg".into());
        let sep = SeparatedBy {
            debug_id: 0,
            rule: Rule::SeparatedBy(Box::new(SeparatedBy {
                debug_id: 1,
                rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                    debug_id: 2,
                    any_characters: Rc::new(",;".into()),
                    optional: false,
                    property: Some(arg.clone()),
                }),
                by: Rule::Token(Token {
                    debug_id: 3,
                    text: Rc::new(",".into()),
                    inverted: false,
                    property: None,
                }),
                optional: false,
                allow_trail: true,
            })),
            by: Rule::Token(Token {
                debug_id: 4,
                text: Rc::new(";".into()),
                inverted: false,
                property: None,
            }),
            optional: false,
            allow_trail: true,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars, 0, &[]);
        assert_eq!(res, Ok((Range::new(0, 12), TokenizerState(6),
            Some((Range::new(12, 0), ParseError::ExpectedSomething(2))))));
        assert_eq!(tokenizer.tokens.len(), 6);
        assert_eq!(&tokenizer.tokens[0].1,
            &MetaData::String(arg.clone(), Rc::new("a".into())));
        assert_eq!(&tokenizer.tokens[1].1,
            &MetaData::String(arg.clone(), Rc::new("b".into())));
        assert_eq!(&tokenizer.tokens[2].1,
            &MetaData::String(arg.clone(), Rc::new("c".into())));
        assert_eq!(&tokenizer.tokens[3].1,
            &MetaData::String(arg.clone(), Rc::new("d".into())));
        assert_eq!(&tokenizer.tokens[4].1,
            &MetaData::String(arg.clone(), Rc::new("e".into())));
    }
}
