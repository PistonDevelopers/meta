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
    Node,
    NodeRef,
    NodeVisit,
    ParseError,
    ParseResult,
    Select,
    SeparatedBy,
    Lines,
    Sequence,
    Optional,
    Tokenizer,
    TokenizerState,
};

/// A rule describes how some section of a document should be parsed.
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
        offset: usize
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
                s.parse(tokenizer, state, chars, offset)
            }
            &Rule::Sequence(ref s) => {
                s.parse(tokenizer, state, chars, offset)
            }
            &Rule::SeparatedBy(ref s) => {
                s.parse(tokenizer, state, chars, offset)
            }
            &Rule::Lines(ref l) => {
                l.parse(tokenizer, state, chars, offset)
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
                    &NodeRef::Ref(ref p, _) => {
                        p.borrow().parse(tokenizer, state, chars, offset)
                    }
                }
            }
            &Rule::Optional(ref o) => {
                Ok(o.parse(tokenizer, state, chars, offset))
            }
        }
    }

    /// Updates replacing names with the references.
    ///
    /// The references contains the name,
    /// but this can not be borrowed as when the same reference is updated.
    pub fn update_refs(&mut self, refs: &[(Rc<String>, Rc<RefCell<Node>>)]) {
        match self {
            &mut Rule::Node(ref mut p) => {
                *p = {
                    match p {
                        &mut NodeRef::Name(ref name, _) => {
                            // Look through references and update if correct name
                            // is found.
                            let mut found: Option<Rc<RefCell<Node>>> = None;
                            for r in refs {
                                if &**name == &*r.0 {
                                    found = Some(r.1.clone());
                                }
                            }
                            match found {
                                None => { return; }
                                Some(r) =>
                                    NodeRef::Ref(r, NodeVisit::Unvisited)
                            }
                        }
                        &mut NodeRef::Ref(ref mut p, ref mut visited) => {
                            // Update the sub rules of the reference,
                            // but only if it has not been visited.
                            if let NodeVisit::Unvisited = *visited {
                                *visited = NodeVisit::Visited;
                                p.borrow_mut().rule.update_refs(refs);
                                return;
                            } else {
                                return;
                            }
                        }
                    }
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
            &mut Rule::Lines(ref mut l) => {
                l.rule.update_refs(refs);
            }
            &mut Rule::Optional(ref mut o) => {
                o.rule.update_refs(refs);
            }
        }
    }
}
