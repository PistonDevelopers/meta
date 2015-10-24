use range::Range;
use read_token::ReadToken;
use std::sync::Arc;

use super::{
    ret_err,
    update,
    ParseResult,
};
use {
    DebugId,
    MetaData,
    ParseError,
    Rule,
};
use tokenizer::{ read_data, TokenizerState };

/// A node reference.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    /// Name of rule.
    pub name: Arc<String>,
    /// The property to set.
    pub property: Option<Arc<String>>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
    /// The index to the rule reference.
    pub index: Option<usize>,
}

impl Node {
    /// Parses node.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken,
        refs: &[Rule]
    ) -> ParseResult<TokenizerState> {
        let start = read_token;
        let mut read_token = *start;
        let index = match self.index {
            None => {
                return Err(
                    read_token.start().wrap(
                        ParseError::InvalidRule(
                            "Node rule is not updated to reference",
                            self.debug_id
                        )
                    ));
            }
            Some(i) => i
        };
        let mut state = if let Some(ref prop) = self.property {
            read_data(
                tokens,
                read_token.start().wrap(MetaData::StartNode(prop.clone())),
                state
            )
        } else {
            state.clone()
        };
        let mut opt_error = None;
        state = match refs[index].parse(
            tokens, &state, &read_token, refs
        ) {
            Err(err) => { return Err(ret_err(err, opt_error)); }
            Ok((range, state, err)) => {
                update(range, err, &mut read_token, &mut opt_error);
                state
            }
        };
        let range = read_token.subtract(start);
        Ok((
            range,
            if let Some(ref prop) = self.property {
                read_data(
                    tokens,
                    range.wrap(MetaData::EndNode(prop.clone())),
                    &state
                )
            } else {
                state.clone()
            },
            opt_error
        ))
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use meta_rules::{ update_refs, Node, Number, Optional, Sequence,
        Whitespace };
    use std::sync::Arc;

    #[test]
    fn node_ref() {
        // Create a node rule the refers to itself.
        let foo: Arc<String> = Arc::new("foo".into());
        let num: Arc<String> = Arc::new("num".into());
        let node = Rule::Sequence(Sequence {
            debug_id: 1,
            args: vec![
                Rule::Number(Number {
                    debug_id: 2,
                    property: Some(num.clone()),
                    allow_underscore: false,
                }),
                Rule::Optional(Box::new(Optional {
                    debug_id: 3,
                    rule: Rule::Sequence(Sequence {
                        debug_id: 4,
                        args: vec![
                            Rule::Whitespace(Whitespace {
                                debug_id: 3,
                                optional: false
                            }),
                            Rule::Node(Node {
                                name: foo.clone(),
                                property: Some(foo.clone()),
                                debug_id: 3,
                                index: None,
                            }),
                        ]
                    }),
                })),
            ],
        });

        // Replace self referencing names with direct references.
        let rule = Rule::Node(Node {
            name: foo.clone(),
            property: Some(foo.clone()),
            debug_id: 0,
            index: None,
        });
        let mut rules = Syntax::new();
        rules.push(foo.clone(), node);
        rules.push(Arc::new("".into()), rule);
        update_refs(&mut rules);

        let text = "1 2 3";
        let mut data = vec![];
        assert_eq!(parse(&rules, text, &mut data), Ok(()));
        assert_eq!(data.len(), 9);
        assert_eq!(&data[0].data, &MetaData::StartNode(foo.clone()));
        assert_eq!(&data[1].data, &MetaData::F64(num.clone(), 1.0));
        assert_eq!(&data[2].data, &MetaData::StartNode(foo.clone()));
        assert_eq!(&data[3].data, &MetaData::F64(num.clone(), 2.0));
        assert_eq!(&data[4].data, &MetaData::StartNode(foo.clone()));
        assert_eq!(&data[5].data, &MetaData::F64(num.clone(), 3.0));
        assert_eq!(&data[6].data, &MetaData::EndNode(foo.clone()));
        assert_eq!(&data[7].data, &MetaData::EndNode(foo.clone()));
        assert_eq!(&data[8].data, &MetaData::EndNode(foo.clone()));
    }
}
