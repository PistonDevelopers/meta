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

/// Stores information about select.
#[derive(Clone)]
pub struct FastSelect {
    /// The table used to look up sub-rule from next byte.
    pub table: [u8; 256],
    /// The rules to select from using the table.
    /// If `tail` is true, the last rule is used in table lookup fails.
    pub args: Vec<Rule>,
    /// Whether the last rule is used when table lookup fails.
    pub tail: bool,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl PartialEq for FastSelect {
    fn eq(&self, _: &Self) -> bool {false}
}

impl std::fmt::Debug for FastSelect {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl FastSelect {
    /// Parses select with lookup table.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken,
        refs: &[Rule],
        indent_settings: &mut IndentSettings,
    ) -> ParseResult<TokenizerState> {
        if self.tail {
            if let Some(ch) = read_token.src.chars().next() {
                let mut opt_error: Option<Range<ParseError>> = None;
                let mut buf = [0; 4];
                ch.encode_utf8(&mut buf);
                let ind = self.table[buf[0] as usize];
                if ind != 255 {
                    let sub_rule = &self.args[ind as usize];
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
                let sub_rule = &self.args[self.args.len()-1];
                match sub_rule.parse(tokens, state, read_token, refs, indent_settings) {
                    Ok((range, state, err)) => {
                        err_update(err, &mut opt_error);
                        Ok((read_token.peek(range.length),
                            state, opt_error))
                    }
                    Err(err) => {
                        Err(ret_err(err, opt_error))
                    }
                }
            } else {
                Err(read_token.start().wrap(
                    ParseError::ExpectedSomething(self.debug_id)))
            }
        } else {
            if let Some(ch) = read_token.src.chars().next() {
                let mut buf = [0; 4];
                ch.encode_utf8(&mut buf);
                let ind = self.table[buf[0] as usize];
                if ind != 255 {
                    let sub_rule = &self.args[ind as usize];
                    match sub_rule.parse(tokens, state, read_token, refs, indent_settings) {
                        Ok((range, state, err)) => {
                            Ok((read_token.peek(range.length),
                                state, err))
                        }
                        Err(err) => {
                            Err(err)
                        }
                    }
                } else {
                    Err(read_token.start().wrap(
                        ParseError::ExpectedSomething(self.debug_id)))
                }
            } else {
                Err(read_token.start().wrap(
                    ParseError::ExpectedSomething(self.debug_id)))
            }
        }
    }
}
