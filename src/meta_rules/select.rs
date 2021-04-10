use range::Range;
use read_token::ReadToken;

use super::{
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

/// Stores information about select.
#[derive(Clone, Debug, PartialEq)]
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
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken,
        refs: &[Rule],
        indent_settings: &mut IndentSettings,
    ) -> ParseResult<TokenizerState> {
        let mut opt_error: Option<Range<ParseError>> = None;
        for sub_rule in &self.args {
            match sub_rule.parse(tokens, state, read_token, refs, indent_settings) {
                Ok((range, state, err)) => {
                    err_update(err, &mut opt_error);
                    return Ok((read_token.peek(range.length),
                        state, opt_error));
                }
                Err(err) => {
                    err_update(Some(err), &mut opt_error);
                }
            }
        }
        match opt_error {
            None => Err(read_token.start().wrap(
                ParseError::InvalidRule(
                "`Select` requires at least one sub rule", self.debug_id))),
            Some(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use meta_rules::{ Number, Select, Text };
    use range::Range;
    use std::sync::Arc;

    #[test]
    fn invalid_rule() {
        let text = "";
        let select = Rule::Select(Select {
            debug_id: 0,
            args: vec![]
        });
        let mut syntax = Syntax::new();
        syntax.push(Arc::new("".into()), select);
        let mut tokens = vec![];
        let res = parse(&syntax, &text, &mut tokens);
        let invalid_rule = match &res {
            &Err(ref range_err) => {
                match range_err.data {
                    ParseError::InvalidRule(_, _) => true,
                    _ => false
                }
            }
            _ => false
        };
        assert!(invalid_rule);
    }

    #[test]
    fn fail_first() {
        let text = "2";
        let num: Arc<String> = Arc::new("num".into());
        let select = Rule::Select(Select {
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
        });
        let mut syntax = Syntax::new();
        syntax.push(Arc::new("".into()), select);
        let mut tokens = vec![];
        let res = parse(&syntax, &text, &mut tokens);
        assert_eq!(res, Ok(()));
        assert_eq!(tokens, vec![
            Range::new(0, 1).wrap(MetaData::F64(num.clone(), 2.0))
        ]);
    }
}
