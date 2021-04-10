use range::Range;
use read_token::ReadToken;

use super::{
    ret_err,
    err_update,
    IndentSettings,
    ParseResult,
};
use {
    DebugId,
    MetaData,
    ParseError,
    Rule,
};
use tokenizer::TokenizerState;

/// Stores information about lines.
#[derive(Clone, Debug, PartialEq)]
pub struct Lines {
    /// The rule to read lines.
    /// This can be a multi-line rule.
    pub rule: Rule,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
    /// Whether to increase indention.
    pub indent: bool,
}

impl Lines {
    /// Parses rule separated by one or more lines.
    /// Ignores lines that only contain whitespace characters.
    pub fn parse(
        &self,
        tokenizer: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken,
        refs: &[Rule],
        indent_settings: &mut IndentSettings,
    ) -> ParseResult<TokenizerState> {
        let mut state = state.clone();
        let mut opt_error = None;
        let mut first = true;
        if self.indent {
            match read_token.lines(|read_token| {
                let offset = read_token.offset;
                let mut read_token = *read_token;
                if self.indent {
                    let mut n = indent_settings.indent;
                    while n > 0 {
                        if let Some(range) = read_token.tag(" ") {
                            read_token = read_token.consume(range.length);
                            n -= 1;
                        } else if let Some(range) = read_token.tag("\t") {
                            read_token = read_token.consume(range.length);
                            if n <= indent_settings.tab_spaces as u32 {
                                break
                            } else {
                                n -= indent_settings.tab_spaces as u32;
                            }
                        } else {
                            return None;
                        }
                    }
                    if first && indent_settings.align_first {
                        first = false;
                        loop {
                            if let Some(range) = read_token.tag(" ") {
                                read_token = read_token.consume(range.length);
                                indent_settings.indent += 1;
                            } else if let Some(range) = read_token.tag("\t") {
                                read_token = read_token.consume(range.length);
                                indent_settings.indent += indent_settings.tab_spaces as u32;
                            } else {
                                break;
                            }
                        }
                    } else {
                        if let Some(_) = read_token.tag(" ") {
                            return None;
                        }
                    }
                }

                // Increase indent.
                let old_indent = indent_settings.indent;
                indent_settings.indent += 1;
                match self.rule.parse(tokenizer, &state, &read_token, refs, indent_settings) {
                    Err(err) => {
                        indent_settings.indent = old_indent;
                        err_update(Some(err), &mut opt_error);
                        None
                    }
                    Ok((mut range, new_state, err)) => {
                        let end = range.offset + range.length;
                        range.length = end - offset;
                        range.offset = offset;
                        indent_settings.indent = old_indent;
                        err_update(err, &mut opt_error);
                        state = new_state;
                        Some(range)
                    }
                }
            }) {
                Err(range) => {
                    let err = range.wrap(
                        ParseError::ExpectedNewLine(self.debug_id));
                    Err(ret_err(err, opt_error))
                }
                Ok(range) => {
                    Ok((range, state, opt_error))
                }
            }
        } else {
            match read_token.lines(|read_token| {
                match self.rule.parse(tokenizer, &state, &read_token, refs, indent_settings) {
                    Err(err) => {
                        err_update(Some(err), &mut opt_error);
                        None
                    }
                    Ok((range, new_state, err)) => {
                        err_update(err, &mut opt_error);
                        state = new_state;
                        Some(range)
                    }
                }
            }) {
                Err(range) => {
                    let err = range.wrap(
                        ParseError::ExpectedNewLine(self.debug_id));
                    Err(ret_err(err, opt_error))
                }
                Ok(range) => {
                    Ok((range, state, opt_error))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use all::tokenizer::*;
    use meta_rules::{ IndentSettings, Lines, Number, Sequence, Text, Whitespace };
    use range::Range;
    use read_token::ReadToken;
    use std::sync::Arc;

    #[test]
    fn fail() {
        let text = "
1
2

3


\"error\"
4
        ";
        let ref mut indent_settings = IndentSettings::default();
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: None,
                allow_underscore: false,
            }),
            indent: false,
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[], indent_settings);
        assert_eq!(res, Ok((Range::new(0, 10), s,
            Some(Range::new(10, 0).wrap(ParseError::ExpectedNumber(1))))));
    }

    #[test]
    fn fails_same_line() {
        let text = "
1
2

3 4

5
 ";
        let ref mut ident_settings = IndentSettings::default();
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let val: Arc<String> = Arc::new("val".into());
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
            indent: false,
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[], ident_settings);
        assert_eq!(res, Err(Range::new(8, 0).wrap(
            ParseError::ExpectedNewLine(0))));
    }

    #[test]
    fn success() {
        let text = "
1
2

3


4
 ";
        let ref mut ident_settings = IndentSettings::default();
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let val: Arc<String> = Arc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: Some(val.clone()),
                allow_underscore: false,
            }),
            indent: false,
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[], ident_settings);
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
        let num: Arc<String> = Arc::new("num".into());
        let tex: Arc<String> = Arc::new("tex".into());
        let rule = Rule::Sequence(Sequence {
            debug_id: 0,
            args: vec![
                Rule::Lines(Box::new(Lines {
                    debug_id: 1,
                    rule: Rule::Number(Number {
                        debug_id: 2,
                        allow_underscore: true,
                        property: Some(num.clone()),
                    }),
                    indent: false,
                })),
                Rule::Lines(Box::new(Lines {
                    debug_id: 3,
                    rule: Rule::Text(Text {
                        debug_id: 4,
                        allow_empty: false,
                        property: Some(tex.clone()),
                    }),
                    indent: false,
                }))
            ]
        });

        let mut syntax = Syntax::new();
        syntax.push(Arc::new("".into()), rule);
        let mut res = vec![];
        assert_eq!(parse(&syntax, text, &mut res), Ok(()));
        assert_eq!(res, vec![
            Range::new(1, 1).wrap(MetaData::F64(num.clone(), 1.0)),
            Range::new(3, 1).wrap(MetaData::F64(num.clone(), 2.0)),
            Range::new(5, 1).wrap(MetaData::F64(num.clone(), 3.0)),
            Range::new(7, 5).wrap(
                MetaData::String(tex.clone(), Arc::new("one".into()))),
            Range::new(13, 5).wrap(
                MetaData::String(tex.clone(), Arc::new("two".into()))),
            Range::new(19, 7).wrap(
                MetaData::String(tex.clone(), Arc::new("three".into())))
        ]);
    }

    #[test]
    fn indent_success() {
        let text = "
1
2

3


4
 ";
        let ref mut ident_settings = IndentSettings::default();
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let val: Arc<String> = Arc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: Some(val.clone()),
                allow_underscore: false,
            }),
            indent: true,
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[], ident_settings);
        assert_eq!(res, Ok((Range::new(0, 13), TokenizerState(4), None)));
    }

    #[test]
    fn indent_1_2() {
        let text = "
1
2
 3
4
 ";
        let ref mut ident_settings = IndentSettings::default();
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let val: Arc<String> = Arc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: Some(val.clone()),
                allow_underscore: false,
            }),
            indent: true,
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[], ident_settings);
        assert_eq!(res, Ok((Range::new(0, 5), TokenizerState(2), None)));
    }

    #[test]
    fn indent_align() {
        let text = "
    1
    2
";
        let ref mut ident_settings = IndentSettings::default();
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let val: Arc<String> = Arc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: Some(val.clone()),
                allow_underscore: false,
            }),
            indent: true,
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[], ident_settings);
        assert_eq!(res, Ok((Range::new(0, 13), TokenizerState(2), None)));
    }

    #[test]
    fn indent_align_tabs() {
        let text = "
\t1
    2
";
        let ref mut ident_settings = IndentSettings::default();
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let val: Arc<String> = Arc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: Some(val.clone()),
                allow_underscore: false,
            }),
            indent: true,
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[], ident_settings);
        assert_eq!(res, Ok((Range::new(0, 10), TokenizerState(2), None)));
    }

    #[test]
    fn indent_align_tabs2() {
        let text = "
    1
\t2
";
        let ref mut ident_settings = IndentSettings::default();
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let val: Arc<String> = Arc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: Some(val.clone()),
                allow_underscore: false,
            }),
            indent: true,
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[], ident_settings);
        assert_eq!(res, Ok((Range::new(0, 10), TokenizerState(2), None)));
    }
}
