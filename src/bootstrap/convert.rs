use std::rc::Rc;
use std::cell::Cell;
use range::Range;

use meta_rules::{
    update_refs,
    Lines,
    Optional,
    Node,
    Number,
    Repeat,
    Rule,
    Sequence,
    Select,
    SeparateBy,
    Text,
    Token,
    UntilAny,
    UntilAnyOrWhitespace,
    Whitespace,
};
use MetaData;

/// Updates with parsed range.
pub fn update(range: Range, data: &mut &[(Range, MetaData)], offset: &mut usize) {
    let next_offset = range.next_offset();
    *data = &data[next_offset - *offset..];
    *offset = next_offset;
}

/// Reads start node.
pub fn start_node(name: &str, data: &[(Range, MetaData)], offset: usize)
-> Result<Range, ()> {
    if data.len() == 0 { return Err(()); }
    match &data[0].1 {
        &MetaData::StartNode(ref n) if &**n == name => {
            Ok(Range::new(offset, 1))
        }
        _ => Err(())
    }
}

/// Reads end node.
pub fn end_node(name: &str, data: &[(Range, MetaData)], offset: usize)
-> Result<Range, ()> {
    if data.len() == 0 { return Err(()); }
    match &data[0].1 {
        &MetaData::EndNode(ref n) if &**n == name => {
            Ok(Range::new(offset, 1))
        }
        _ => Err(())
    }
}

/// Ignores next item.
/// If this is the start of a node, it ignores all items to the end node.
pub fn ignore(data: &[(Range, MetaData)], offset: usize)
-> Range {
    let mut acc: usize = 0;
    let mut len = 0;
    for item in data.iter() {
        match &item.1 {
            &MetaData::StartNode(_) => acc += 1,
            &MetaData::EndNode(_) => acc -= 1,
            _ => {}
        }
        len += 1;
        if acc == 0 { break; }
    }
    Range::new(offset, len)
}

/// Reads string.
pub fn meta_string(name: &str, data: &[(Range, MetaData)], offset: usize)
-> Result<(Range, Rc<String>), ()> {
    if data.len() == 0 { return Err(()); }
    match &data[0].1 {
        &MetaData::String(ref n, ref val) if &**n == name => {
            Ok((Range::new(offset, 1), val.clone()))
        }
        _ => Err(())
    }
}

/// Reads f64.
pub fn meta_f64(name: &str, data: &[(Range, MetaData)], offset: usize)
-> Result<(Range, f64), ()> {
    if data.len() == 0 { return Err(()); }
    match &data[0].1 {
        &MetaData::F64(ref n, ref val) if &**n == name => {
            Ok((Range::new(offset, 1), *val))
        }
        _ => Err(())
    }
}

/// Reads bool.
pub fn meta_bool(name: &str, data: &[(Range, MetaData)], offset: usize)
-> Result<(Range, bool), ()> {
    if data.len() == 0 { return Err(()); }
    match &data[0].1 {
        &MetaData::Bool(ref n, ref val) if &**n == name => {
            Ok((Range::new(offset, 1), *val))
        }
        _ => Err(())
    }
}

/// Converts meta data to rules.
pub fn convert(
    mut data: &[(Range, MetaData)],
    ignored: &mut Vec<Range>
) -> Result<Vec<(Rc<String>, Rule)>, ()> {
    fn read_string(mut data: &[(Range, MetaData)], mut offset: usize)
    -> Result<(Range, (Rc<String>, Rc<String>)), ()> {
        let start_offset = offset;
        let range = try!(start_node("string", data, offset));
        update(range, &mut data, &mut offset);
        let mut name = None;
        let mut text = None;
        loop {
            if let Ok((range, val)) = meta_string("name", data, offset) {
                name = Some(val);
                update(range, &mut data, &mut offset);
            } else if let Ok((range, val)) = meta_string("text", data, offset) {
                text = Some(val);
                update(range, &mut data, &mut offset);
            } else if let Ok(range) = end_node("string", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else {
                return Err(())
            }
        }
        let name = match name {
            None => { return Err(()); }
            Some(x) => x
        };
        let text = match text {
            None => { return Err(()); }
            Some(x) => x
        };
        Ok((Range::new(start_offset, offset - start_offset), (name, text)))
    }

    fn read_sequence(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "sequence";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        let mut args: Vec<Rule> = vec![];
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", data, offset, strings, ignored
            ) {
                update(range, &mut data, &mut offset);
                args.push(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset), Rule::Sequence(Sequence {
            debug_id: *debug_id,
            args: args
        })))
    }

    fn find_string(val: &str, strings: &[(Rc<String>, Rc<String>)]) -> Option<Rc<String>> {
        strings.iter().find(|&&(ref s, _)| &**s == val).map(|&(_, ref s)| s.clone())
    }

    fn read_set(property: &str, mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rc<String>), ()> {
        let start_offset = offset;
        let range = try!(start_node(property, data, offset));
        update(range, &mut data, &mut offset);
        let mut text = None;
        loop {
            if let Ok(range) = end_node(property, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("ref", data, offset) {
                update(range, &mut data, &mut offset);
                text = find_string(&val, strings);
            } else if let Ok((range, val)) = meta_string("value", data, offset) {
                update(range, &mut data, &mut offset);
                text = Some(val);
            } else {
                return Err(())
            }
        }
        match text {
            None => Err(()),
            Some(text) => Ok((Range::new(start_offset, offset - start_offset), text))
        }
    }

    fn read_until_any_or_whitespace(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "until_any_or_whitespace";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        let mut any_characters = None;
        let mut optional = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("any_characters", data, offset, strings) {
                update(range, &mut data, &mut offset);
                any_characters = Some(val);
            } else if let Ok((range, val)) = meta_bool("optional", data, offset) {
                update(range, &mut data, &mut offset);
                optional = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        let optional = optional.unwrap_or(false);
        match any_characters {
            Some(any) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                    debug_id: *debug_id,
                    any_characters: any,
                    optional: optional,
                    property: property
                })))
            }
            None => Err(())
        }
    }

    fn read_until_any(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "until_any";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        let mut any_characters = None;
        let mut optional = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("any_characters", data, offset, strings) {
                update(range, &mut data, &mut offset);
                any_characters = Some(val);
            } else if let Ok((range, val)) = meta_bool("optional", data, offset) {
                update(range, &mut data, &mut offset);
                optional = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        let optional = optional.unwrap_or(false);
        match any_characters {
            Some(any) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::UntilAny(UntilAny {
                    debug_id: *debug_id,
                    any_characters: any,
                    optional: optional,
                    property: property
                })))
            }
            None => Err(())
        }
    }

    fn read_token(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "token";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);

        let mut text = None;
        let mut property = None;
        let mut not = None;
        let mut inverted = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("text", data, offset, strings) {
                update(range, &mut data, &mut offset);
                text = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else if let Ok((range, val)) = meta_bool("not", data, offset) {
                update(range, &mut data, &mut offset);
                not = Some(val);
            } else if let Ok((range, val)) = meta_bool("inverted", data, offset) {
                update(range, &mut data, &mut offset);
                inverted = Some(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        let not = not.unwrap_or(false);
        let inverted = inverted.unwrap_or(false);
        match text {
            Some(text) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::Token(Token {
                    debug_id: *debug_id,
                    text: text,
                    not: not,
                    inverted: inverted,
                    property: property,
                })))
            }
            None => Err(())
        }
    }

    fn read_whitespace(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("whitespace", data, offset));
        update(range, &mut data, &mut offset);
        let (range, optional) = try!(meta_bool("optional", data, offset));
        update(range, &mut data, &mut offset);
        let range = try!(end_node("whitespace", data, offset));
        update(range, &mut data, &mut offset);
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Whitespace(Whitespace {
            debug_id: *debug_id,
            optional: optional,
        })))
    }

    fn read_text(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "text";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        let mut allow_empty = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_bool("allow_empty", data, offset) {
                update(range, &mut data, &mut offset);
                allow_empty = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        let allow_empty = allow_empty.unwrap_or(true);
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Text(Text {
            debug_id: *debug_id,
            allow_empty: allow_empty,
            property: property,
        })))
    }

    fn read_number(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "number";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);

        let mut property = None;
        let mut underscore = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else if let Ok((range, val)) = meta_bool("underscore", data, offset) {
                update(range, &mut data, &mut offset);
                underscore = Some(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        let underscore = underscore.unwrap_or(false);
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Number(Number {
            debug_id: *debug_id,
            property: property,
            allow_underscore: underscore,
        })))
    }

    fn read_reference(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "reference";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);

        let mut name = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("name", data, offset) {
                update(range, &mut data, &mut offset);
                name = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        match name {
            Some(name) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::Node(Node {
                    debug_id: *debug_id,
                    name: name,
                    property: property,
                    index: Cell::new(None),
                })))
            }
            None => Err(())
        }
    }

    fn read_select(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "select";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        let mut args: Vec<Rule> = vec![];
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", data, offset, strings, ignored
            ) {
                update(range, &mut data, &mut offset);
                args.push(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Select(Select {
            debug_id: *debug_id,
            args: args
        })))
    }

    fn read_optional(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "optional";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        let (range, rule) = try!(read_rule(
            debug_id, "rule", data, offset, strings, ignored
        ));
        update(range, &mut data, &mut offset);
        let range = try!(end_node(node, data, offset));
        update(range, &mut data, &mut offset);
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Optional(Box::new(Optional {
            debug_id: *debug_id,
            rule: rule,
        }))))
    }

    fn read_separated_by(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "separated_by";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        let mut optional = None;
        let mut allow_trail = None;
        let mut by = None;
        let mut rule = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_bool("optional", data, offset) {
                update(range, &mut data, &mut offset);
                optional = Some(val);
            } else if let Ok((range, val)) = meta_bool("allow_trail", data, offset) {
                update(range, &mut data, &mut offset);
                allow_trail = Some(val);
            } else if let Ok((range, val)) = read_rule(
                debug_id, "by", data, offset, strings, ignored
            ) {
                update(range, &mut data, &mut offset);
                by = Some(val);
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", data, offset, strings, ignored
            ) {
                update(range, &mut data, &mut offset);
                rule = Some(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        let optional = optional.unwrap_or(true);
        let allow_trail = allow_trail.unwrap_or(true);
        match (by, rule) {
            (Some(by), Some(rule)) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::SeparateBy(Box::new(SeparateBy {
                    debug_id: *debug_id,
                    optional: optional,
                    allow_trail: allow_trail,
                    by: by,
                    rule: rule,
                }))))
            }
            _ => Err(())
        }
    }

    fn read_lines(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("lines", data, offset));
        update(range, &mut data, &mut offset);
        let (range, rule) = try!(read_rule(
            debug_id, "rule", data, offset, strings, ignored
        ));
        update(range, &mut data, &mut offset);
        let range = try!(end_node("lines", data, offset));
        update(range, &mut data, &mut offset);
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Lines(Box::new(Lines {
            debug_id: *debug_id,
            rule: rule,
        }))))
    }

    fn read_repeat(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let node = "repeat";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        let mut rule = None;
        let mut optional = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", data, offset, strings, ignored
            ) {
                update(range, &mut data, &mut offset);
                rule = Some(val);
            } else if let Ok((range, val)) = meta_bool(
                "optional", data, offset
            ) {
                update(range, &mut data, &mut offset);
                optional = Some(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        match (rule, optional) {
            (Some(rule), Some(optional)) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::Repeat(Box::new(Repeat {
                    debug_id: *debug_id,
                    rule: rule,
                    optional: optional,
                }))))
            }
            _ => Err(())
        }
    }

    fn read_rule(
        debug_id: &mut usize,
        property: &str,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node(property, data, offset));
        update(range, &mut data, &mut offset);

        let mut rule = None;
        if let Ok((range, val)) = read_sequence(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_until_any_or_whitespace(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_until_any(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_token(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_whitespace(
            debug_id, data, offset
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_text(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_number(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_reference(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_select(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_optional(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_separated_by(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_lines(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_repeat(
            debug_id, data, offset, strings, ignored
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        }

        if let Some(rule) = rule {
            let range = try!(end_node(property, data, offset));
            update(range, &mut data, &mut offset);
            Ok((Range::new(start_offset, offset - start_offset), rule))
        } else {
            Err(())
        }
    }

    fn read_node(
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, (Rc<String>, Rule)), ()> {
        let start_offset = offset;
        let node = "node";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        let mut id = None;
        let mut name = None;
        let mut rule = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_f64("id", data, offset) {
                id = Some(val as usize);
                update(range, &mut data, &mut offset);
            } else if let Ok((range, val)) = meta_string("name", data, offset) {
                name = Some(val);
                update(range, &mut data, &mut offset);
            } else if let Ok((range, val)) = read_rule(
                &mut (id.unwrap_or(0) * 1000), "rule",
                data, offset, strings, ignored
            ) {
                rule = Some(val);
                update(range, &mut data, &mut offset);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }
        match (name, rule) {
            (Some(name), Some(rule)) => {
                Ok((Range::new(start_offset, offset - start_offset), (name, rule)))
            }
            _ => Err(())
        }
    }

    let mut strings: Vec<(Rc<String>, Rc<String>)> = vec![];
    let mut offset: usize = 0;
    loop {
        if let Ok((range, val)) = read_string(data, offset) {
            strings.push(val);
            update(range, &mut data, &mut offset);
        } else {
            break;
        }
    }
    let mut res = vec![];
    loop {
        if let Ok((range, val)) = read_node(data, offset, &strings, ignored) {
            update(range, &mut data, &mut offset);
            res.push(val);
        } else if offset < data.len() {
            return Err(());
        } else {
            break;
        }
    }
    update_refs(&res);
    Ok(res)
}
