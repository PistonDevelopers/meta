use range::Range;
use std::rc::Rc;
use std::cell::RefCell;

use {
    err_update,
    DebugId,
    Node,
    ParseError,
    ParseResult,
    Rule,
    Tokenizer,
    TokenizerState,
};

/// Stores information about select.
#[derive(Clone)]
pub struct Select {
    /// The rules to select from.
    pub args: Vec<Rule>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Select {
    /// Parses select.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        chars: &[char],
        offset: usize,
        refs: &[(Rc<String>, Rc<RefCell<Node>>)]
    ) -> ParseResult<TokenizerState> {
        let mut opt_error: Option<(Range, ParseError)> = None;
        for sub_rule in &self.args {
            match sub_rule.parse(tokenizer, state, chars, offset, refs) {
                Ok((range, state, err)) => {
                    err_update(err, &mut opt_error);
                    return Ok((Range::new(offset, range.next_offset() - offset),
                        state, opt_error));
                }
                Err(err) => {
                    err_update(Some(err), &mut opt_error);
                }
            }
        }
        match opt_error {
            None => Err((Range::new(offset, 0), ParseError::InvalidRule(
                "`Select` requires at least one sub rule", self.debug_id))),
            Some(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;
    use std::rc::Rc;

    #[test]
    fn invalid_rule() {
        let text = "";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let select = Select {
            debug_id: 0,
            args: vec![]
        };
        let res = select.parse(&mut tokenizer, &s, &chars, 0, &[]);
        let invalid_rule = match &res {
            &Err((_, ParseError::InvalidRule(_, _))) => true,
            _ => false
        };
        assert!(invalid_rule);
    }

    #[test]
    fn fail_first() {
        let text = "2";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let num: Rc<String> = Rc::new("num".into());
        let select = Select {
            debug_id: 0,
            args: vec![
                Rule::Text(Text {
                    debug_id: 1,
                    allow_empty: true,
                    property: None
                }),
                Rule::Number(Number {
                    debug_id: 2,
                    property: Some(num.clone()),
                    allow_underscore: false,
                })
            ]
        };
        let res = select.parse(&mut tokenizer, &s, &chars, 0, &[]);
        assert_eq!(res, Ok((Range::new(0, 1), TokenizerState(1),
            Some((Range::new(0, 0), ParseError::ExpectedText(1))))));
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(&tokenizer.tokens[0].1, &MetaData::F64(num.clone(), 2.0));
    }
}
