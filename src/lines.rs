use range::Range;

use {
    ret_err,
    err_update,
    update,
    DebugId,
    ParseError,
    ParseResult,
    Rule,
    Tokenizer,
    TokenizerState,
};

/// Stores information about lines.
#[derive(Clone)]
pub struct Lines {
    /// The rule to read lines.
    /// This can be a multi-line rule.
    pub rule: Rule,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Lines {
    /// Parses rule separated by one or more lines.
    /// Ignores lines that only contain whitespace characters.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        mut chars: &[char],
        start_offset: usize
    ) -> ParseResult<TokenizerState> {
        let mut offset = start_offset;
        let mut state = state.clone();
        let mut opt_error = None;
        let mut new_lines = true;
        loop {
            let len = chars.iter()
                .take_while(|&c| *c != '\n' && c.is_whitespace())
                .count();
            if len == chars.len() {
                offset += len;
                break;
            } else if chars[len] == '\n' {
                chars = &chars[len + 1..];
                offset += len + 1;
                new_lines |= true;
            } else {
                if new_lines {
                    state = match self.rule.parse(tokenizer, &state, chars, offset) {
                        Err(err) => {
                            err_update(Some(err), &mut opt_error);
                            break;
                        }
                        Ok((range, state, err)) => {
                            // Find whether a new line occured at the end.
                            // If it did, we do not require a new line before
                            // reading the rule again.
                            let end = range.next_offset() - offset;
                            let last_new_line = chars[..end].iter()
                                .rev()
                                .take_while(|&c| *c != '\n' && c.is_whitespace())
                                .count();
                            new_lines =
                                if end < last_new_line + 1
                                || chars[end - last_new_line - 1] != '\n' { false }
                                else { true };
                            update(range, err, &mut chars, &mut offset, &mut opt_error);
                            state
                        }
                    };
                } else {
                    let err = (Range::new(offset, 0),
                        ParseError::ExpectedNewLine(self.debug_id));
                    return Err(ret_err(err, opt_error));
                }
            }
        }
        Ok((Range::new(start_offset, offset - start_offset), state, opt_error))
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;
    use std::rc::Rc;

    #[test]
    fn fail() {
        let text = "
1
2

3


\"error\"
4
        ";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: None,
                allow_underscore: false,
            }),
        };
        let res = lines.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Ok((Range::new(0, 10), s,
            Some((Range::new(10, 0), ParseError::ExpectedNumber(1))))));
    }

    #[test]
    fn fails_same_line() {
        let text = "
1
2

3 4

5
 ";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let val: Rc<String> = Rc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Sequence(Sequence {
                debug_id: 1,
                args: vec![
                    Rule::Number(Number {
                        debug_id: 1,
                        property: Some(val.clone()),
                        allow_underscore: false,
                    }),
                    Rule::Whitespace(Whitespace {
                        debug_id: 2,
                        optional: true,
                    })
                ]
            }),
        };
        let res = lines.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Err((Range::new(8, 0), ParseError::ExpectedNewLine(0))));
    }

    #[test]
    fn success() {
        let text = "
1
2

3


4
 ";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let val: Rc<String> = Rc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: Some(val.clone()),
                allow_underscore: false,
            }),
        };
        let res = lines.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Ok((Range::new(0, 13), TokenizerState(4), None)));
    }

    #[test]
    fn sequence() {
        let text = "
1
2
3
\"one\"
\"two\"
\"three\"
        ";
        let num: Rc<String> = Rc::new("num".into());
        let tex: Rc<String> = Rc::new("tex".into());
        let rules = Rule::Sequence(Sequence {
            debug_id: 0,
            args: vec![
                Rule::Lines(Box::new(Lines {
                    debug_id: 1,
                    rule: Rule::Number(Number {
                        debug_id: 2,
                        allow_underscore: true,
                        property: Some(num.clone()),
                    })
                })),
                Rule::Lines(Box::new(Lines {
                    debug_id: 3,
                    rule: Rule::Text(Text {
                        debug_id: 4,
                        allow_empty: false,
                        property: Some(tex.clone()),
                    })
                }))
            ]
        });
        let res = parse(&rules, text);
        assert_eq!(res, Ok(vec![
            (Range::new(1, 1), MetaData::F64(num.clone(), 1.0)),
            (Range::new(3, 1), MetaData::F64(num.clone(), 2.0)),
            (Range::new(5, 1), MetaData::F64(num.clone(), 3.0)),
            (Range::new(7, 5), MetaData::String(tex.clone(), "one".into())),
            (Range::new(13, 5), MetaData::String(tex.clone(), "two".into())),
            (Range::new(19, 7), MetaData::String(tex.clone(), "three".into()))
        ]));
    }
}
