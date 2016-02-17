use std::sync::Arc;
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
    Tag,
    UntilAny,
    UntilAnyOrWhitespace,
    Whitespace,
};
use MetaData;
use Syntax;

/// Stores state when converting from meta data.
#[derive(Copy, Clone, Debug)]
pub struct Convert<'a> {
    data: &'a [Range<MetaData>],
    offset: usize,
}

impl<'a> Convert<'a> {
    /// Creates a new `Convert`.
    pub fn new(data: &'a [Range<MetaData>]) -> Convert<'a> {
        Convert {
            data: data,
            offset: 0,
        }
    }

    /// Returns the length of remaining data.
    #[inline(always)]
    pub fn remaining_data_len(&self) -> usize {
        self.data.len()
    }

    /// Returns the difference in offset.
    #[inline(always)]
    pub fn subtract(self, rhs: Convert) -> Range {
        Range::new(rhs.offset, self.offset - rhs.offset)
    }

    /// Returns the subtracted range in source (union of meta data ranges).
    pub fn source(self, rhs: Convert) -> Option<Range> {
        if rhs.data.len() == 0 || self.offset <= rhs.offset { return None; }
        let start = rhs.data[0].offset;
        let end = rhs.data[self.offset - rhs.offset - 1].next_offset();
        Some(Range::new(start, end - start))
    }

    /// Updates with parsed range.
    pub fn update(&mut self, range: Range) {
        let next_offset = range.next_offset();
        self.data = &self.data[next_offset - self.offset..];
        self.offset = next_offset;
    }

    /// Reads start node.
    pub fn start_node(&self, name: &str) -> Result<Range, ()> {
        if self.data.len() == 0 { return Err(()); }
        match self.data[0].data {
            MetaData::StartNode(ref n) if &**n == name => {
                Ok(Range::new(self.offset, 1))
            }
            _ => Err(())
        }
    }

    /// Reads end node.
    pub fn end_node(&self, name: &str) -> Result<Range, ()> {
        if self.data.len() == 0 { return Err(()); }
        match self.data[0].data {
            MetaData::EndNode(ref n) if &**n == name => {
                Ok(Range::new(self.offset, 1))
            }
            _ => Err(())
        }
    }

    /// Ignores next item.
    /// If this is the start of a node, it ignores all items to the end node.
    pub fn ignore(&self) -> Range {
        let mut acc: usize = 0;
        let mut len = 0;
        for item in self.data.iter() {
            match &item.data {
                &MetaData::StartNode(_) => acc += 1,
                &MetaData::EndNode(_) => acc -= 1,
                _ => {}
            }
            len += 1;
            if acc == 0 { break; }
        }
        Range::new(self.offset, len)
    }

    /// Reads string.
    pub fn meta_string(&self, name: &str) -> Result<(Range, Arc<String>), ()> {
        if self.data.len() == 0 { return Err(()); }
        match self.data[0].data {
            MetaData::String(ref n, ref val) if &**n == name => {
                Ok((Range::new(self.offset, 1), val.clone()))
            }
            _ => Err(())
        }
    }

    /// Reads f64.
    pub fn meta_f64(&self, name: &str) -> Result<(Range, f64), ()> {
        if self.data.len() == 0 { return Err(()); }
        match self.data[0].data {
            MetaData::F64(ref n, ref val) if &**n == name => {
                Ok((Range::new(self.offset, 1), *val))
            }
            _ => Err(())
        }
    }

    /// Reads bool.
    pub fn meta_bool(&self, name: &str) -> Result<(Range, bool), ()> {
        if self.data.len() == 0 { return Err(()); }
        match self.data[0].data {
            MetaData::Bool(ref n, ref val) if &**n == name => {
                Ok((Range::new(self.offset, 1), *val))
            }
            _ => Err(())
        }
    }
}

/// Converts meta data to rules.
pub fn convert(
    data: &[Range<MetaData>],
    ignored: &mut Vec<Range>
) -> Result<Syntax, ()> {

    fn read_string(mut convert: Convert)
    -> Result<(Range, (Arc<String>, Arc<String>)), ()> {
        let start = convert.clone();
        let range = try!(convert.start_node("string"));
        convert.update(range);
        let mut name = None;
        let mut text = None;
        loop {
            if let Ok((range, val)) = convert.meta_string("name") {
                name = Some(val);
                convert.update(range);
            } else if let Ok((range, val)) = convert.meta_string("text") {
                text = Some(val);
                convert.update(range);
            } else if let Ok(range) = convert.end_node("string") {
                convert.update(range);
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
        Ok((convert.subtract(start), (name, text)))
    }

    fn read_sequence(
        debug_id: &mut usize,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "sequence";
        let range = try!(convert.start_node(node));
        convert.update(range);
        let mut args: Vec<Rule> = vec![];
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", convert, strings, ignored
            ) {
                convert.update(range);
                args.push(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        *debug_id += 1;
        Ok((convert.subtract(start), Rule::Sequence(Sequence {
            debug_id: *debug_id,
            args: args
        })))
    }

    fn find_string(val: &str, strings: &[(Arc<String>, Arc<String>)]) -> Option<Arc<String>> {
        strings.iter().find(|&&(ref s, _)| &**s == val).map(|&(_, ref s)| s.clone())
    }

    fn read_set(
        property: &str,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)]
    ) -> Result<(Range, Arc<String>), ()> {
        let start = convert.clone();
        let range = try!(convert.start_node(property));
        convert.update(range);
        let mut text = None;
        loop {
            if let Ok(range) = convert.end_node(property) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = convert.meta_string("ref") {
                convert.update(range);
                text = find_string(&val, strings);
            } else if let Ok((range, val)) = convert.meta_string("value") {
                convert.update(range);
                text = Some(val);
            } else {
                return Err(())
            }
        }
        match text {
            None => Err(()),
            Some(text) => Ok((convert.subtract(start), text))
        }
    }

    fn read_until_any_or_whitespace(
        debug_id: &mut usize,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "until_any_or_whitespace";
        let range = try!(convert.start_node(node));
        convert.update(range);
        let mut any_characters = None;
        let mut optional = None;
        let mut property = None;
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = read_set("any_characters", convert, strings) {
                convert.update(range);
                any_characters = Some(val);
            } else if let Ok((range, val)) = convert.meta_bool("optional") {
                convert.update(range);
                optional = Some(val);
            } else if let Ok((range, val)) = read_set("property", convert, strings) {
                convert.update(range);
                property = Some(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        let optional = optional.unwrap_or(false);
        match any_characters {
            Some(any) => {
                *debug_id += 1;
                Ok((convert.subtract(start),
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
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "until_any";
        let range = try!(convert.start_node(node));
        convert.update(range);
        let mut any_characters = None;
        let mut optional = None;
        let mut property = None;
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = read_set("any_characters", convert, strings) {
                convert.update(range);
                any_characters = Some(val);
            } else if let Ok((range, val)) = convert.meta_bool("optional") {
                convert.update(range);
                optional = Some(val);
            } else if let Ok((range, val)) = read_set("property", convert, strings) {
                convert.update(range);
                property = Some(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        let optional = optional.unwrap_or(false);
        match any_characters {
            Some(any) => {
                *debug_id += 1;
                Ok((convert.subtract(start),
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

    fn read_tag(
        debug_id: &mut usize,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "tag";
        let range = try!(convert.start_node(node));
        convert.update(range);

        let mut text = None;
        let mut property = None;
        let mut not = None;
        let mut inverted = None;
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = read_set("text", convert, strings) {
                convert.update(range);
                text = Some(val);
            } else if let Ok((range, val)) = read_set("property", convert, strings) {
                convert.update(range);
                property = Some(val);
            } else if let Ok((range, val)) = convert.meta_bool("not") {
                convert.update(range);
                not = Some(val);
            } else if let Ok((range, val)) = convert.meta_bool("inverted") {
                convert.update(range);
                inverted = Some(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        let not = not.unwrap_or(false);
        let inverted = inverted.unwrap_or(false);
        match text {
            Some(text) => {
                *debug_id += 1;
                Ok((convert.subtract(start),
                Rule::Tag(Tag {
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

    fn read_whitespace(debug_id: &mut usize, mut convert: Convert)
    -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let range = try!(convert.start_node("whitespace"));
        convert.update(range);
        let (range, optional) = try!(convert.meta_bool("optional"));
        convert.update(range);
        let range = try!(convert.end_node("whitespace"));
        convert.update(range);
        *debug_id += 1;
        Ok((convert.subtract(start),
        Rule::Whitespace(Whitespace {
            debug_id: *debug_id,
            optional: optional,
        })))
    }

    fn read_text(
        debug_id: &mut usize,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "text";
        let range = try!(convert.start_node(node));
        convert.update(range);
        let mut allow_empty = None;
        let mut property = None;
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = convert.meta_bool("allow_empty") {
                convert.update(range);
                allow_empty = Some(val);
            } else if let Ok((range, val)) = read_set("property", convert, strings) {
                convert.update(range);
                property = Some(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        let allow_empty = allow_empty.unwrap_or(true);
        *debug_id += 1;
        Ok((convert.subtract(start),
        Rule::Text(Text {
            debug_id: *debug_id,
            allow_empty: allow_empty,
            property: property,
        })))
    }

    fn read_number(
        debug_id: &mut usize,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "number";
        let range = try!(convert.start_node(node));
        convert.update(range);

        let mut property = None;
        let mut underscore = None;
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = read_set("property", convert, strings) {
                convert.update(range);
                property = Some(val);
            } else if let Ok((range, val)) = convert.meta_bool("underscore") {
                convert.update(range);
                underscore = Some(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        let underscore = underscore.unwrap_or(false);
        *debug_id += 1;
        Ok((convert.subtract(start),
        Rule::Number(Number {
            debug_id: *debug_id,
            property: property,
            allow_underscore: underscore,
        })))
    }

    fn read_reference(
        debug_id: &mut usize,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "reference";
        let range = try!(convert.start_node(node));
        convert.update(range);

        let mut name = None;
        let mut property = None;
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = convert.meta_string("name") {
                convert.update(range);
                name = Some(val);
            } else if let Ok((range, val)) = read_set("property", convert, strings) {
                convert.update(range);
                property = Some(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        match name {
            Some(name) => {
                *debug_id += 1;
                Ok((convert.subtract(start),
                Rule::Node(Node {
                    debug_id: *debug_id,
                    name: name,
                    property: property,
                    index: None,
                })))
            }
            None => Err(())
        }
    }

    fn read_select(
        debug_id: &mut usize,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "select";
        let range = try!(convert.start_node(node));
        convert.update(range);
        let mut args: Vec<Rule> = vec![];
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", convert, strings, ignored
            ) {
                convert.update(range);
                args.push(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        *debug_id += 1;
        Ok((convert.subtract(start),
        Rule::Select(Select {
            debug_id: *debug_id,
            args: args
        })))
    }

    fn read_optional(
        debug_id: &mut usize,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "optional";
        let range = try!(convert.start_node(node));
        convert.update(range);
        let (range, rule) = try!(read_rule(
            debug_id, "rule", convert, strings, ignored
        ));
        convert.update(range);
        let range = try!(convert.end_node(node));
        convert.update(range);
        *debug_id += 1;
        Ok((convert.subtract(start),
        Rule::Optional(Box::new(Optional {
            debug_id: *debug_id,
            rule: rule,
        }))))
    }

    fn read_separated_by(
        debug_id: &mut usize,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "separated_by";
        let range = try!(convert.start_node(node));
        convert.update(range);
        let mut optional = None;
        let mut allow_trail = None;
        let mut by = None;
        let mut rule = None;
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = convert.meta_bool("optional") {
                convert.update(range);
                optional = Some(val);
            } else if let Ok((range, val)) = convert.meta_bool("allow_trail") {
                convert.update(range);
                allow_trail = Some(val);
            } else if let Ok((range, val)) = read_rule(
                debug_id, "by", convert, strings, ignored
            ) {
                convert.update(range);
                by = Some(val);
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", convert, strings, ignored
            ) {
                convert.update(range);
                rule = Some(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        let optional = optional.unwrap_or(true);
        let allow_trail = allow_trail.unwrap_or(true);
        match (by, rule) {
            (Some(by), Some(rule)) => {
                *debug_id += 1;
                Ok((convert.subtract(start),
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
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let range = try!(convert.start_node("lines"));
        convert.update(range);
        let (range, rule) = try!(read_rule(
            debug_id, "rule", convert, strings, ignored
        ));
        convert.update(range);
        let range = try!(convert.end_node("lines"));
        convert.update(range);
        *debug_id += 1;
        Ok((convert.subtract(start),
        Rule::Lines(Box::new(Lines {
            debug_id: *debug_id,
            rule: rule,
        }))))
    }

    fn read_repeat(
        debug_id: &mut usize,
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let node = "repeat";
        let range = try!(convert.start_node(node));
        convert.update(range);
        let mut rule = None;
        let mut optional = None;
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", convert, strings, ignored
            ) {
                convert.update(range);
                rule = Some(val);
            } else if let Ok((range, val)) = convert.meta_bool("optional") {
                convert.update(range);
                optional = Some(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        match (rule, optional) {
            (Some(rule), Some(optional)) => {
                *debug_id += 1;
                Ok((convert.subtract(start),
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
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Rule), ()> {
        let start = convert.clone();
        let range = try!(convert.start_node(property));
        convert.update(range);

        let mut rule = None;
        if let Ok((range, val)) = read_sequence(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_until_any_or_whitespace(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_until_any(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_tag(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_whitespace(debug_id, convert) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_text(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_number(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_reference(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_select(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_optional(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_separated_by(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_lines(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        } else if let Ok((range, val)) = read_repeat(
            debug_id, convert, strings, ignored
        ) {
            convert.update(range);
            rule = Some(val);
        }

        if let Some(rule) = rule {
            let range = try!(convert.end_node(property));
            convert.update(range);
            Ok((convert.subtract(start), rule))
        } else {
            Err(())
        }
    }

    fn read_node(
        mut convert: Convert,
        strings: &[(Arc<String>, Arc<String>)],
        ignored: &mut Vec<Range>
    ) -> Result<(Range, (Arc<String>, Rule)), ()> {
        let start = convert.clone();
        let node = "node";
        let range = try!(convert.start_node(node));
        convert.update(range);
        let mut id = None;
        let mut name = None;
        let mut rule = None;
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = convert.meta_f64("id") {
                id = Some(val as usize);
                convert.update(range);
            } else if let Ok((range, val)) = convert.meta_string("name") {
                name = Some(val);
                convert.update(range);
            } else if let Ok((range, val)) = read_rule(
                &mut (id.unwrap_or(0) * 1000), "rule",
                convert, strings, ignored
            ) {
                rule = Some(val);
                convert.update(range);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }
        match (name, rule) {
            (Some(name), Some(rule)) => {
                Ok((convert.subtract(start), (name, rule)))
            }
            _ => Err(())
        }
    }

    let mut strings: Vec<(Arc<String>, Arc<String>)> = vec![];
    let mut convert = Convert::new(data);
    loop {
        if let Ok((range, val)) = read_string(convert) {
            strings.push(val);
            convert.update(range);
        } else {
            break;
        }
    }
    let mut res = Syntax::new();
    loop {
        if let Ok((range, val)) = read_node(convert, &strings, ignored) {
            convert.update(range);
            res.push(val.0, val.1);
        } else if convert.remaining_data_len() > 0 {
            return Err(());
        } else {
            break;
        }
    }
    update_refs(&mut res);
    Ok(res)
}
