use range::Range;

use {
    MetaReader,
    ParseError,
    Rule,
};

/// Stores information about select.
pub struct Select {
    /// The rules to select from.
    pub args: Vec<Rule>,
}

impl Select {
    /// Parses select.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        chars: &[char],
        offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        let mut first_error: Option<(Range, ParseError)> = None;
        for sub_rule in &self.args {
            match sub_rule.parse(meta_reader, state, chars, offset) {
                Ok((range, state)) => {
                    return Ok((Range::new(offset, range.next_offset()), state));
                }
                Err(err) => {
                    first_error = Some(err);
                }
            }
        }
        match first_error {
            None => Err((Range::new(offset, 0), ParseError::InvalidRule(
                "`Select` requires at least one sub rule"))),
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
            args: vec![]
        };
        let res = select.parse(&mut tokenizer, &s, &chars, 0);
        let invalid_rule = match &res {
            &Err((_, ParseError::InvalidRule(_))) => true,
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
            args: vec![
                Rule::Text(Text {
                    allow_empty: true,
                    property: None
                }),
                Rule::Number(Number {
                    property: Some(num.clone())
                })
            ]
        };
        let res = select.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Ok((Range::new(0, 1), TokenizerState(1))));
        assert_eq!(tokenizer.tokens.len(), 1);
        assert_eq!(&tokenizer.tokens[0].0, &MetaData::F64(num.clone(), 2.0));
    }
}
