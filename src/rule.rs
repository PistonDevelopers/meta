use range::Range;
use std::rc::Rc;
use std::cell::RefCell;

use {
    Whitespace,
    Token,
    UntilAnyOrWhitespace,
    Text,
    Number,
    Parameter,
    ParameterRef,
    ParameterVisit,
    MetaReader,
    ParseError,
    Select,
    Sequence,
    Optional,
};

/// A rule describes how some section of a document should be parsed.
pub enum Rule {
    /// Read whitespace.
    Whitespace(Whitespace),
    /// Match against a token.
    Token(Token),
    /// Read until any or whitespace.
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
    /// Read parameter.
    Parameter(ParameterRef),
    /// Read optional.
    Optional(Optional),
}

impl Rule {
    /// Parses rule.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        chars: &[char],
        offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        match self {
            &Rule::Whitespace(ref w) => {
                w.parse(chars, offset).map(|r| (r, state.clone()))
            }
            &Rule::Token(ref t) => {
                t.parse(meta_reader, state, chars, offset)
            }
            &Rule::UntilAnyOrWhitespace(ref u) => {
                u.parse(meta_reader, state, chars, offset)
            }
            &Rule::Text(ref t) => {
                t.parse(meta_reader, state, chars, offset)
            }
            &Rule::Number(ref n) => {
                n.parse(meta_reader, state, chars, offset)
            }
            &Rule::Select(ref s) => {
                s.parse(meta_reader, state, chars, offset)
            }
            &Rule::Sequence(ref s) => {
                s.parse(meta_reader, state, chars, offset)
            }
            &Rule::Parameter(ref p) => {
                match p {
                    &ParameterRef::Name(_) => {
                        Err((
                            Range::empty(offset),
                            ParseError::InvalidRule(
                                "Parameter rule is not updated to reference")
                        ))
                    }
                    &ParameterRef::Ref(ref p, _) => {
                        p.borrow().parse(meta_reader, state, chars, offset)
                    }
                }
            }
            &Rule::Optional(ref o) => {
                Ok(o.parse(meta_reader, state, chars, offset))
            }
        }
    }

    /// Updates replacing names with the references.
    ///
    /// The references contains the name,
    /// but this can not be borrowed as when the same reference is updated.
    pub fn update_refs(&mut self, refs: &[(Rc<String>, Rc<RefCell<Parameter>>)]) {
        match self {
            &mut Rule::Parameter(ref mut p) => {
                *p = {
                    match p {
                        &mut ParameterRef::Name(ref name) => {
                            // Look through references and update if correct name
                            // is found.
                            let mut found: Option<Rc<RefCell<Parameter>>> = None;
                            for r in refs {
                                if &**name == &*r.0 {
                                    found = Some(r.1.clone());
                                }
                            }
                            match found {
                                None => { return; }
                                Some(r) =>
                                    ParameterRef::Ref(r, ParameterVisit::Unvisited)
                            }
                        }
                        &mut ParameterRef::Ref(ref mut p, ref mut visited) => {
                            // Update the sub rules of the reference,
                            // but only if it has not been visited.
                            if let ParameterVisit::Unvisited = *visited {
                                *visited = ParameterVisit::Visited;
                                for sub_rule in &mut p.borrow_mut().body {
                                    sub_rule.update_refs(refs);
                                }
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
            &mut Rule::Optional(ref mut o) => {
                for sub_rule in &mut o.args {
                    sub_rule.update_refs(refs);
                }
            }
        }
    }
}
