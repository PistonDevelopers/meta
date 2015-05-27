use range::Range;

use {
    ret_err,
    update,
    MetaReader,
    ParseResult,
    Rule,
};

/// Stores inforamtion about separated by.
pub struct SeparatedBy {
    /// The rule to separate.
    pub rule: Rule,
    /// The rule to separate by.
    pub by: Rule,
    /// Whether the rule must occur at least once.
    pub optional: bool,
    /// Whether the rule can end with separator.
    pub allow_trail: bool,
}

impl SeparatedBy {
    /// Parses rule repeatedly separated by another rule.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        mut chars: &[char],
        start_offset: usize
    ) -> ParseResult<M::State>
        where M: MetaReader
    {
        let mut offset = start_offset;
        let mut state = state.clone();
        let mut first = true;
        let mut opt_error = None;
        loop {
            state = match self.rule.parse(meta_reader, &state, chars, offset) {
                Err(err) => {
                    match (first, self.optional, self.allow_trail) {
                          (true, false, _)
                        | (false, _, false) => {
                            return Err(ret_err(err, opt_error));
                        }
                          (true, true, _)
                        | (false, _, true) => { break; }
                    }
                }
                Ok((range, state, err)) => {
                    update(range, err, &mut chars, &mut offset, &mut opt_error);
                    state
                }
            };
            state = match self.by.parse(meta_reader, &state, chars, offset) {
                Err(err) => { opt_error = Some(err); break; }
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
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                any_characters: Rc::new(",)".into()),
                optional: false,
                property: None,
            }),
            by: Rule::Token(Token {
                text: Rc::new(",".into()),
                inverted: false,
                property: None,
            }),
            optional: false,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars[4..], 4);
        assert_eq!(res, Err((Range::new(4, 0), ParseError::ExpectedSomething)));
    }

    #[test]
    fn optional() {
        let text = "foo()";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let sep = SeparatedBy {
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                any_characters: Rc::new(",)".into()),
                optional: false,
                property: None,
            }),
            by: Rule::Token(Token {
                text: Rc::new(",".into()),
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars[4..], 4);
        assert_eq!(res, Ok((Range::new(4, 0), s, None)));
    }

    #[test]
    fn disallow_trail() {
        let text = "foo(a,b,c,)";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let arg: Rc<String> = Rc::new("arg".into());
        let sep = SeparatedBy {
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                any_characters: Rc::new(",)".into()),
                optional: false,
                property: Some(arg.clone()),
            }),
            by: Rule::Token(Token {
                text: Rc::new(",".into()),
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars[4..], 4);
        assert_eq!(res, Err((Range::new(10, 0), ParseError::ExpectedSomething)));
    }

    #[test]
    fn allow_trail() {
        let text = "foo(a,b,c,)";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let arg: Rc<String> = Rc::new("arg".into());
        let sep = SeparatedBy {
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                any_characters: Rc::new(",)".into()),
                optional: false,
                property: Some(arg.clone()),
            }),
            by: Rule::Token(Token {
                text: Rc::new(",".into()),
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: true,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars[4..], 4);
        assert_eq!(res, Ok((Range::new(4, 6), TokenizerState(3), None)));
        assert_eq!(tokenizer.tokens.len(), 3);
        assert_eq!(&tokenizer.tokens[0].0,
            &MetaData::String(arg.clone(), "a".into()));
        assert_eq!(&tokenizer.tokens[1].0,
            &MetaData::String(arg.clone(), "b".into()));
        assert_eq!(&tokenizer.tokens[2].0,
            &MetaData::String(arg.clone(), "c".into()));
    }

    #[test]
    fn successful() {
        let text = "foo(a,b,c)";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let arg: Rc<String> = Rc::new("arg".into());
        let sep = SeparatedBy {
            rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                any_characters: Rc::new(",)".into()),
                optional: false,
                property: Some(arg.clone()),
            }),
            by: Rule::Token(Token {
                text: Rc::new(",".into()),
                inverted: false,
                property: None,
            }),
            optional: true,
            allow_trail: false,
        };
        let res = sep.parse(&mut tokenizer, &s, &chars[4..], 4);
        assert_eq!(res, Ok((Range::new(4, 5), TokenizerState(3),
            Some((Range::new(9, 0), ParseError::ExpectedToken(",".into()))))));
        assert_eq!(tokenizer.tokens.len(), 3);
        assert_eq!(&tokenizer.tokens[0].0,
            &MetaData::String(arg.clone(), "a".into()));
        assert_eq!(&tokenizer.tokens[1].0,
            &MetaData::String(arg.clone(), "b".into()));
        assert_eq!(&tokenizer.tokens[2].0,
            &MetaData::String(arg.clone(), "c".into()));
    }
}
