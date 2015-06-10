use range::Range;
use std::rc::Rc;
use std::cell::RefCell;

use {
    Whitespace,
    Token,
    UntilAny,
    UntilAnyOrWhitespace,
    Text,
    Number,
    NodeRef,
    NodeVisit,
    ParseError,
    ParseResult,
    Select,
    SeparatedBy,
    Repeat,
    Lines,
    Sequence,
    Optional,
    Tokenizer,
    TokenizerState,
};

/// A rule describes how some section of a document should be parsed.
#[derive(Clone)]
pub enum Rule {
    /// Read whitespace.
    Whitespace(Whitespace),
    /// Match against a token.
    Token(Token),
    /// Reads until any character.
    UntilAny(UntilAny),
    /// Read until any character or whitespace.
    UntilAnyOrWhitespace(UntilAnyOrWhitespace),
    /// Read text.
    Text(Text),
    /// Read number.
    Number(Number),
    /// Select one of the sub rules.
    /// If the first one does not succeed, try another and so on.
    /// If all sub rules fail, then the rule fails.
    Select(Select),
    /// Run each sub rule in sequence.
    /// If any sub rule fails, the rule fails.
    Sequence(Sequence),
    /// Repeat rule separated by another rule.
    SeparatedBy(Box<SeparatedBy>),
    /// Repeat rule.
    Repeat(Box<Repeat>),
    /// Repeat rule separated by one or more lines.
    Lines(Box<Lines>),
    /// Read node.
    Node(NodeRef),
    /// Read optional.
    Optional(Box<Optional>),
}

impl Rule {
    /// Parses rule.
    pub fn parse(
        &self,
        tokenizer: &mut Tokenizer,
        state: &TokenizerState,
        chars: &[char],
        offset: usize,
        refs: &[(Rc<String>, Rc<RefCell<Rule>>)]
    ) -> ParseResult<TokenizerState> {
        match self {
            &Rule::Whitespace(ref w) => {
                w.parse(chars, offset).map(|r| (r, state.clone(), None))
            }
            &Rule::Token(ref t) => {
                t.parse(tokenizer, state, chars, offset)
            }
            &Rule::UntilAny(ref u) => {
                u.parse(tokenizer, state, chars, offset)
            }
            &Rule::UntilAnyOrWhitespace(ref u) => {
                u.parse(tokenizer, state, chars, offset)
            }
            &Rule::Text(ref t) => {
                t.parse(tokenizer, state, chars, offset)
            }
            &Rule::Number(ref n) => {
                n.parse(tokenizer, state, chars, offset)
            }
            &Rule::Select(ref s) => {
                s.parse(tokenizer, state, chars, offset, refs)
            }
            &Rule::Sequence(ref s) => {
                s.parse(tokenizer, state, chars, offset, refs)
            }
            &Rule::SeparatedBy(ref s) => {
                s.parse(tokenizer, state, chars, offset, refs)
            }
            &Rule::Repeat(ref r) => {
                r.parse(tokenizer, state, chars, offset, refs)
            }
            &Rule::Lines(ref l) => {
                l.parse(tokenizer, state, chars, offset, refs)
            }
            &Rule::Node(ref p) => {
                match p {
                    &NodeRef::Name(_, debug_id) => {
                        Err((
                            Range::empty(offset),
                            ParseError::InvalidRule(
                                "Node rule is not updated to reference", debug_id)
                        ))
                    }
                    &NodeRef::Ref(i, _) => {
                        refs[i].1.borrow().parse(
                            tokenizer, state, chars, offset, refs
                        )
                    }
                }
            }
            &Rule::Optional(ref o) => {
                Ok(o.parse(tokenizer, state, chars, offset, refs))
            }
        }
    }

    /// Updates replacing names with the references.
    ///
    /// The references contains the name,
    /// but this can not be borrowed as when the same reference is updated.
    pub fn update_refs(&mut self, refs: &[(Rc<String>, Rc<RefCell<Rule>>)]) {
        match self {
            &mut Rule::Node(ref mut p) => {
                use std::cell::BorrowState;

                *p = match p {
                    &mut NodeRef::Name(ref name, _) => {
                        // Look through references and update if correct name
                        // is found.
                        let mut found: Option<usize> = None;
                        for (i, r) in refs.iter().enumerate() {
                            if &**name == &*r.0 {
                                found = Some(i);
                                break;
                            }
                        }
                        match found {
                            None => { return; }
                            Some(i) => {
                                NodeRef::Ref(i, NodeVisit::Unvisited)
                            }
                        }
                    }
                    &mut NodeRef::Ref(i, ref mut visited) => {
                        // Update the sub rules of the reference,
                        // but only if it has not been visited.
                        if let NodeVisit::Unvisited = *visited {
                            *visited = NodeVisit::Visited;
                            let p = &refs[i].1;
                            if p.borrow_state() == BorrowState::Unused {
                                p.borrow_mut().update_refs(refs);
                            }
                        }
                        return;
                    }
                };
                // Make sure to visit the referenced rule when replacing a name.
                if let &mut NodeRef::Ref(i, ref mut visited) = p {
                    // Update the sub rules of the reference,
                    // but only if it has not been visited.
                    if let NodeVisit::Unvisited = *visited {
                        *visited = NodeVisit::Visited;
                        let p = &refs[i].1;
                        if p.borrow_state() == BorrowState::Unused {
                            p.borrow_mut().update_refs(refs);
                        }
                    }
                    return;
                }
            }
            &mut Rule::Whitespace(_) => {}
            &mut Rule::Token(_) => {}
            &mut Rule::UntilAny(_) => {}
            &mut Rule::UntilAnyOrWhitespace(_) => {}
            &mut Rule::Text(_) => {}
            &mut Rule::Number(_) => {}
            &mut Rule::Select(ref mut s) => {
                for sub_rule in &mut s.args {
                    sub_rule.update_refs(refs);
                }
            }
            &mut Rule::Sequence(ref mut s) => {
                for sub_rule in &mut s.args {
                    sub_rule.update_refs(refs);
                }
            }
            &mut Rule::SeparatedBy(ref mut s) => {
                s.rule.update_refs(refs);
                s.by.update_refs(refs);
            }
            &mut Rule::Repeat(ref mut r) => {
                r.rule.update_refs(refs);
            }
            &mut Rule::Lines(ref mut l) => {
                l.rule.update_refs(refs);
            }
            &mut Rule::Optional(ref mut o) => {
                o.rule.update_refs(refs);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn node_ref() {
        // Create a node rule the refers to itself.
        let foo: Rc<String> = Rc::new("foo".into());
        let num: Rc<String> = Rc::new("num".into());
        let node = Rc::new(RefCell::new(Rule::Sequence(Sequence {
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
                            Rule::Node(NodeRef::Name(foo.clone(), 3)),
                        ]
                    }),
                })),
            ],
        })));

        // Replace self referencing names with direct references.
        let refs = vec![(foo.clone(), node.clone())];
        let mut rules = Rule::Node(NodeRef::Name(foo.clone(), 0));
        rules.update_refs(&refs);

        let text = "1 2 3";
        let data = parse(&rules, &refs, text).unwrap();
        assert_eq!(data.len(), 3);
        assert_eq!(&data[0].1, &MetaData::F64(num.clone(), 1.0));
        assert_eq!(&data[1].1, &MetaData::F64(num.clone(), 2.0));
        assert_eq!(&data[2].1, &MetaData::F64(num.clone(), 3.0));
    }
}
