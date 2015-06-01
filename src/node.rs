use range::Range;
use std::rc::Rc;
use std::cell::RefCell;

use {
    ret_err,
    update,
    DebugId,
    MetaData,
    ParseResult,
    Rule,
    Tokenizer,
    TokenizerState,
};

/// Stores information about a node.
pub struct Node {
    /// The name of the node.
    pub name: Rc<String>,
    /// The body of the node.
    pub body: Vec<Rule>,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Node {
    /// Parses node.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        mut chars: &[char],
        start_offset: usize
    ) -> ParseResult<TokenizerState> {
        let mut offset = start_offset;
        let mut state = tokenizer.data(
            MetaData::StartNode(self.name.clone()),
            state,
            Range::empty(offset)
        );
        let mut opt_error = None;
        for rule in &self.body {
            state = match rule.parse(tokenizer, &state, chars, offset) {
                Err(err) => { return Err(ret_err(err, opt_error)); }
                Ok((range, state, err)) => {
                    update(range, err, &mut chars, &mut offset, &mut opt_error);
                    state
                }
            }
        }
        let range = Range::new(start_offset, offset - start_offset);
        Ok((
            range,
            tokenizer.data(MetaData::EndNode(self.name.clone()), &state, range),
            opt_error
        ))
    }
}

/// A node reference.
pub enum NodeRef {
    /// Points to a node by name.
    Name(Rc<String>, DebugId),
    /// Reference to node.
    /// The `bool` flag is used to prevent multiple visits when updating.
    Ref(Rc<RefCell<Node>>, NodeVisit),
}

/// Tells whether a node is visited when updated.
pub enum NodeVisit {
    /// The node is not being visited.
    Unvisited,
    /// The node is being visited.
    Visited
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn node_ref() {
        // Create a node rule the refers to itself.
        let foo: Rc<String> = Rc::new("foo".into());
        let num: Rc<String> = Rc::new("num".into());
        let node = Rc::new(RefCell::new(Node {
            debug_id: 0,
            name: foo.clone(),
            body: vec![
                Rule::Number(Number { debug_id: 1, property: Some(num.clone()) }),
                Rule::Optional(Optional {
                    debug_id: 2,
                    args: vec![
                        Rule::Whitespace(Whitespace {
                            debug_id: 3,
                            optional: false
                        }),
                        Rule::Node(NodeRef::Name(foo.clone(), 3)),
                    ]
                })
            ]
        }));

        // Replace self referencing names with direct references.
        let refs = vec![(foo.clone(), node.clone())];
        for sub_rule in &mut node.borrow_mut().body {
            sub_rule.update_refs(&refs);
        }

        let text = "1 2 3";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let res = node.borrow().parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, Ok((Range::new(0, 5), TokenizerState(9),
            Some((Range::new(5, 0), ParseError::ExpectedWhitespace(3))))));
        assert_eq!(tokenizer.tokens.len(), 9);
        assert_eq!(&tokenizer.tokens[0].0, &MetaData::StartNode(foo.clone()));
        assert_eq!(&tokenizer.tokens[1].0, &MetaData::F64(num.clone(), 1.0));
        assert_eq!(&tokenizer.tokens[2].0, &MetaData::StartNode(foo.clone()));
        assert_eq!(&tokenizer.tokens[3].0, &MetaData::F64(num.clone(), 2.0));
        assert_eq!(&tokenizer.tokens[4].0, &MetaData::StartNode(foo.clone()));
        assert_eq!(&tokenizer.tokens[5].0, &MetaData::F64(num.clone(), 3.0));
        assert_eq!(&tokenizer.tokens[6].0, &MetaData::EndNode(foo.clone()));
        assert_eq!(&tokenizer.tokens[7].0, &MetaData::EndNode(foo.clone()));
        assert_eq!(&tokenizer.tokens[8].0, &MetaData::EndNode(foo.clone()));
    }
}
