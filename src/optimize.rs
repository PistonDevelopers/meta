//! Helper methods for syntax rule optimization.

use meta_rules::{Select, Rule};

/// Creates a unique table from select rule.
///
/// Uses a unique byte rule: A unique byte determines which sub-rule to process.
///
/// Returns a table mapping from next byte to sub-rule index,
/// and an index for which the unique byte rule no longer holds.
///
/// Supports only up to 255 sub-rules, but this is sufficient for most cases.
pub fn unique_table_from_select(select: &Select, refs: &[Rule]) -> ([u8; 256], usize) {
    let mut data: [u8; 256] = [255; 256];
    let mut unique_up_to = 0;
    for (i, r) in select.args.iter().enumerate() {
        if i >= 255 {break};
        if let Some(b) = unique_byte(r, refs) {
            if data[b as usize] == 255 {
                data[b as usize] = i as u8;
                unique_up_to = i + 1;
            } else {break}
        } else {break}
    }
    (data, unique_up_to)
}

/// Finds a unique byte from rule that determines whether
/// the rule will fail if it does not equals the next byte.
pub fn unique_byte(rule: &Rule, refs: &[Rule]) -> Option<u8> {
    match *rule {
        Rule::Whitespace(_) => None,
        Rule::UntilAny(_) => None,
        Rule::UntilAnyOrWhitespace(_) => None,
        Rule::Number(_) => None,
        Rule::Lines(_) => None,
        Rule::Optional(_) => None,
        Rule::FastSelect(_) => None,
        Rule::Not(ref not) => {
            if let Rule::Not(ref r) = not.rule {unique_byte(&r.rule, refs)}
            else {None}
        }
        Rule::Text(_) => Some(0x22),
        Rule::Select(ref sel) => {
            if sel.args.len() == 1 {unique_byte(&sel.args[0], refs)}
            else {None}
        }
        Rule::SeparateBy(ref sep) => {
            if sep.optional {None}
            else {unique_byte(&sep.rule, refs)}
        }
        Rule::Sequence(ref seq) => {
            if seq.args.len() == 0 {None}
            else {unique_byte(&seq.args[0], refs)}
        }
        Rule::Repeat(ref rep) => {
            if rep.optional {None}
            else {unique_byte(&rep.rule, refs)}
        }
        Rule::Node(ref node) => {
            if let Some(index) = node.index {unique_byte(&refs[index], refs)}
            else {None}
        }
        Rule::Tag(ref tag) => {
            if tag.not {None}
            else {
                if let Some(ch) = tag.text.chars().next() {
                    let mut buf = [0; 4];
                    ch.encode_utf8(&mut buf);
                    Some(buf[0])
                } else {None}
            }
        }
    }
}

/// Optimizes syntax rule.
pub fn optimize_rule(rule: &Rule, refs: &[Rule]) -> Rule {
    use meta_rules::*;

    match *rule {
        Rule::Whitespace(_) |
        Rule::Tag(_) |
        Rule::UntilAny(_) |
        Rule::UntilAnyOrWhitespace(_) |
        Rule::Text(_) |
        Rule::Number(_) |
        Rule::Node(_) |
        // FastSelect is already optimized.
        Rule::FastSelect(_) => rule.clone(),
        Rule::Sequence(ref seq) => {
            Rule::Sequence(Sequence {
                args: seq.args.iter().map(|r| optimize_rule(r, refs)).collect(),
                debug_id: seq.debug_id,
            })
        }
        Rule::SeparateBy(ref sep) => {
            Rule::SeparateBy(Box::new(SeparateBy {
                rule: optimize_rule(&sep.rule, refs),
                by: optimize_rule(&sep.by, refs),
                debug_id: sep.debug_id,
                allow_trail: sep.allow_trail,
                optional: sep.optional,
            }))
        }
        Rule::Repeat(ref rep) => {
            Rule::Repeat(Box::new(Repeat {
                rule: optimize_rule(&rep.rule, refs),
                debug_id: rep.debug_id,
                optional: rep.optional,
            }))
        }
        Rule::Lines(ref lines) => {
            Rule::Lines(Box::new(Lines {
                rule: optimize_rule(&lines.rule, refs),
                debug_id: lines.debug_id,
                indent: lines.indent,
            }))
        }
        Rule::Optional(ref opt) => {
            Rule::Optional(Box::new(Optional {
                rule: optimize_rule(&opt.rule, refs),
                debug_id: opt.debug_id,
            }))
        }
        Rule::Not(ref not) => {
            Rule::Not(Box::new(Not {
                rule: optimize_rule(&not.rule, refs),
                debug_id: not.debug_id,
            }))
        }
        Rule::Select(ref sel) => {
            let (table, unique_up_to) = unique_table_from_select(sel, refs);
            if unique_up_to < 2 {
                if sel.args.len() > 2 {
                    // Check if optimization on the tail is successful.
                    let try_opt = Rule::Select(Select {
                        args: sel.args.iter().skip(1).map(|r| optimize_rule(r, refs)).collect(),
                        debug_id: sel.debug_id,
                    });
                    if let Rule::FastSelect(_) = try_opt {
                        // Optimization successful, create select rule.
                        return Rule::Select(Select {
                            args: vec![
                                optimize_rule(&sel.args[0], refs),
                                try_opt,
                            ],
                            debug_id: sel.debug_id,
                        });
                    }
                }
                Rule::Select(Select {
                    args: sel.args.iter().map(|r| optimize_rule(r, refs)).collect(),
                    debug_id: sel.debug_id,
                })
            } else {
                // Replace Select with FastSelect when possible.
                let mut args: Vec<Rule> = sel.args[..unique_up_to].iter()
                    .map(|r| optimize_rule(r, refs)).collect();
                if unique_up_to < sel.args.len() {
                    let select_args = sel.args[unique_up_to..].iter()
                        .map(|r| optimize_rule(r, refs)).collect();
                    args.push(Rule::Select(Select {
                        debug_id: sel.debug_id,
                        args: select_args,
                    }));
                    Rule::FastSelect(Box::new(FastSelect {
                        table,
                        args,
                        tail: true,
                        debug_id: sel.debug_id,
                    }))
                } else {
                    Rule::FastSelect(Box::new(FastSelect {
                        table,
                        args,
                        tail: false,
                        debug_id: sel.debug_id,
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use meta_rules::*;
    use std::sync::Arc;

    fn select(args: &[&str]) -> Select {
        Select {
            debug_id: 0,
            args: args.iter().map(|&n|
                Rule::Tag(Tag {
                    debug_id: 0,
                    not: false,
                    inverted: false,
                    property: None,
                    text: Arc::new(n.into())
                })
            ).collect()
        }
    }

    #[test]
    fn test_1() {
        let sel = select(&["John", "Peter"]);
        let (table, unique_up_to) = unique_table_from_select(&sel, &[]);
        for i in 0..256 {
            if table[i] != 255 {
                println!("0x{:x}: {:?}", i, table[i]);
            }
            assert_eq!(table[i], match i {
                0x4a => 0,
                0x50 => 1,
                _ => 255,
            });
        }
        assert_eq!(unique_up_to, 2);
    }

    #[test]
    fn test_2() {
        let sel = select(&["John", "Peter", "Carl", "Johnathan"]);
        let (table, unique_up_to) = unique_table_from_select(&sel, &[]);
        for i in 0..256 {
            if table[i] != 255 {
                println!("0x{:x}: {:?}", i, table[i]);
            }
            assert_eq!(table[i], match i {
                0x43 => 2,
                0x4a => 0,
                0x50 => 1,
                _ => 255,
            });
        }
        assert_eq!(unique_up_to, 3);
    }

    #[test]
    fn test_empty() {
        let sel = select(&[]);
        let (table, unique_up_to) = unique_table_from_select(&sel, &[]);
        for i in 0..256 {assert_eq!(table[i], 255)}
        assert_eq!(unique_up_to, 0);
    }


    #[test]
    fn test_type() {
        use *;

        let rules = r#"
            100 type = {
                "f64":"f64"
                ["[" type:"arr" "]"]
                "thr":"thr_any"
            }
        "#;
        // Parse rules with meta language and convert to rules for parsing text.
        let rules = match syntax_errstr(rules) {
            Err(err) => {
                panic!("{}", err);
            }
            Ok(rules) => rules.optimize()
        };
        let text = r#"[f64]"#;
        let mut data = vec![];
        match parse_errstr(&rules, text, &mut data) {
            Err(err) => {
                panic!("{}", err);
            }
            Ok(()) => {}
        };
    }
}
