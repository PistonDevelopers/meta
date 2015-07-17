use range::Range;
use std::rc::Rc;
use { MetaData, ParseError };

/// Used to search through meta data.
pub struct Search<'a> {
    /// The previous range of search.
    /// Used in errors if there is no meta data left.
    pub range: Option<Range>,
    /// The data to search.
    pub data: &'a [(Range, MetaData)],
}

impl<'a> Search<'a> {
    /// Creates a new search.
    pub fn new(data: &'a [(Range, MetaData)]) -> Search<'a> {
        Search {
            data: data,
            range: None
        }
    }

    /// Searches anywhere in meta data for a string.
    /// Calls closure on the first match.
    pub fn for_string<T, F>(
        &'a self,
        name: &str,
        val: &str,
        f: F
    ) -> Result<T, (Range, ParseError)>
        where F: FnOnce(Search<'a>) -> Result<T, (Range, ParseError)>
    {
        if self.data.len() == 0 {
            return Err((
                self.range.unwrap_or(Range::empty(0)),
                ParseError::Conversion(format!("Could not find string `{}`:`{}`",
                    name, val))
            ));
        }

        for (i, d) in self.data.iter().enumerate() {
            match d {
                &(range, MetaData::String(ref n, ref v)) => {
                    if &**n == name && &**v == val {
                        return f(Search {
                            data: &self.data[i + 1..],
                            range: Some(range)
                        })
                    }
                }
                _ => {}
            }
        }

        Err((
            self.range.unwrap_or(Range::empty(0)),
            ParseError::Conversion(format!("Could not find string `{}`:`{}`",
                name, val))
        ))
    }

    /// Searches anywhere in meta data for a f64.
    /// Calls closure on the first match.
    pub fn for_f64<T, F>(
        &'a self,
        name: &str,
        val: f64,
        f: F
    ) -> Result<T, (Range, ParseError)>
        where F: FnOnce(Search<'a>) -> Result<T, (Range, ParseError)>
    {
        if self.data.len() == 0 {
            return Err((
                self.range.unwrap_or(Range::empty(0)),
                ParseError::Conversion(format!("Could not find f64 `{}`:`{}`",
                    name, val))
            ));
        }

        for (i, d) in self.data.iter().enumerate() {
            match d {
                &(range, MetaData::F64(ref n, v)) => {
                    if &**n == name && v == val {
                        return f(Search {
                            data: &self.data[i + 1..],
                            range: Some(range)
                        })
                    }
                }
                _ => {}
            }
        }

        Err((
            self.range.unwrap_or(Range::empty(0)),
            ParseError::Conversion(format!("Could not find f64 `{}`:`{}`",
                name, val))
        ))
    }

    /// Searches anywhere in meta data for a bool.
    /// Calls closure on the first match.
    pub fn for_bool<T, F>(
        &'a self,
        name: &str,
        val: bool,
        f: F
    ) -> Result<T, (Range, ParseError)>
        where F: FnOnce(Search<'a>) -> Result<T, (Range, ParseError)>
    {
        if self.data.len() == 0 {
            return Err((
                self.range.unwrap_or(Range::empty(0)),
                ParseError::Conversion(format!("Could not find bool `{}`:`{}`",
                    name, val))
            ));
        }

        for (i, d) in self.data.iter().enumerate() {
            match d {
                &(range, MetaData::Bool(ref n, v)) => {
                    if &**n == name && v == val {
                        return f(Search {
                            data: &self.data[i + 1..],
                            range: Some(range)
                        })
                    }
                }
                _ => {}
            }
        }

        Err((
            self.range.unwrap_or(Range::empty(0)),
            ParseError::Conversion(format!("Could not find bool `{}`:`{}`",
                name, val))
        ))
    }

    /// Searches anywhere in meta data for a node.
    /// Calls closure on the first match.
    pub fn for_node<T, F>(
        &'a self,
        name: &str,
        f: F
    ) -> Result<T, (Range, ParseError)>
        where F: FnOnce(Search<'a>) -> Result<T, (Range, ParseError)>
    {
        if self.data.len() == 0 {
            return Err((
                self.range.unwrap_or(Range::empty(0)),
                ParseError::Conversion(format!("Could not find node `{}`", name))
            ));
        }

        for (i, d) in self.data.iter().enumerate() {
            match d {
                &(range, MetaData::StartNode(ref n)) => {
                    if &**n == name {
                        return f(Search {
                            data: &self.data[i + 1..],
                            range: Some(range)
                        })
                    }
                }
                _ => {}
            }
        }

        Err((
            self.range.unwrap_or(Range::empty(0)),
            ParseError::Conversion(format!("Could not find node `{}`", name))
        ))
    }

    /// Searches anywhere in meta data for an end node.
    /// Calls closure on the first match.
    pub fn for_end_node<T, F>(
        &'a self,
        name: &str,
        f: F
    ) -> Result<T, (Range, ParseError)>
        where F: FnOnce(Search<'a>) -> Result<T, (Range, ParseError)>
    {
        if self.data.len() == 0 {
            return Err((
                self.range.unwrap_or(Range::empty(0)),
                ParseError::Conversion(format!("Could not find end node `{}`", name))
            ));
        }

        for (i, d) in self.data.iter().enumerate() {
            match d {
                &(range, MetaData::EndNode(ref n)) => {
                    if &**n == name {
                        return f(Search {
                            data: &self.data[i + 1..],
                            range: Some(range)
                        })
                    }
                }
                _ => {}
            }
        }

        Err((
            self.range.unwrap_or(Range::empty(0)),
            ParseError::Conversion(format!("Could not find end node `{}`", name))
        ))
    }

    /// Reads next as bool value.
    pub fn bool(&mut self, name: &str) -> Result<bool, (Range, ParseError)> {
        if self.data.len() == 0 {
            return Err((
                self.range.unwrap_or(Range::empty(0)),
                ParseError::Conversion(format!("Expected bool `{}`", name))
            ));
        }
        match &self.data[0] {
            &(range, MetaData::Bool(ref n, v)) => {
                if &**n == name {
                    self.data = &self.data[1..];
                    self.range = Some(range);
                    Ok(v)
                } else {
                    Err((
                        range,
                        ParseError::Conversion(
                            format!("Expected name `{}` found `{}`", name, n))
                    ))
                }
            }
            &(range, ref val) => {
                Err((range, ParseError::Conversion(
                    format!("Expected bool `{}`, found `{:?}`", name, val))))
            }
        }
    }

    /// Reads next as f64 value.
    pub fn f64(&mut self, name: &str) -> Result<f64, (Range, ParseError)> {
        if self.data.len() == 0 {
            return Err((
                self.range.unwrap_or(Range::empty(0)),
                ParseError::Conversion(format!("Expected f64 `{}`", name))
            ));
        }
        match &self.data[0] {
            &(range, MetaData::F64(ref n, v)) => {
                if &**n == name {
                    self.data = &self.data[1..];
                    self.range = Some(range);
                    Ok(v)
                } else {
                    Err((range, ParseError::Conversion(
                        format!("Expected name `{}`, found `{}`", name, n))))
                }
            }
            &(range, ref val) => {
                Err((range, ParseError::Conversion(
                    format!("Expected f64 `{}`, found `{:?}`", name, val))))
            }
        }
    }

    /// Reads next as string value.
    pub fn string(
        &mut self,
        name: &str
    ) -> Result<Rc<String>, (Range, ParseError)> {
        if self.data.len() == 0 {
            return Err((
                self.range.unwrap_or(Range::empty(0)),
                ParseError::Conversion(format!("Expected string `{}`", name))
            ));
        }
        match &self.data[0] {
            &(range, MetaData::String(ref n, ref v)) => {
                if &**n == name {
                    self.data = &self.data[1..];
                    self.range = Some(range);
                    Ok(v.clone())
                } else {
                    Err((range, ParseError::Conversion(
                        format!("Expected name `{}`, found `{}`", name, n))))
                }
            }
            &(range, ref val) => {
                Err((range, ParseError::Conversion(
                    format!("Expected string `{}`, found `{:?}`", name, val))))
            }
        }
    }

    /// Reads next as node.
    pub fn node(&mut self, name: &str) -> Result<(), (Range, ParseError)> {
        if self.data.len() == 0 {
            return Err((
                self.range.unwrap_or(Range::empty(0)),
                ParseError::Conversion(format!("Expected node `{}`", name))
            ));
        }
        match &self.data[0] {
            &(range, MetaData::StartNode(ref n)) => {
                if &**n == name {
                    self.data = &self.data[1..];
                    self.range = Some(range);
                    Ok(())
                } else {
                    Err((range, ParseError::Conversion(
                        format!("Expected name `{}`, found `{}`", name, n))))
                }
            }
            &(range, ref val) => {
                Err((range, ParseError::Conversion(
                    format!("Expected node `{}`, found `{:?}`", name, val))))
            }
        }
    }

    /// Reads next as end node.
    pub fn end_node(&mut self, name: &str) -> Result<(), (Range, ParseError)> {
        if self.data.len() == 0 {
            return Err((
                self.range.unwrap_or(Range::empty(0)),
                ParseError::Conversion(format!("Expected end node `{}`", name))
            ));
        }
        match &self.data[0] {
            &(range, MetaData::EndNode(ref n)) => {
                if &**n == name {
                    self.data = &self.data[1..];
                    self.range = Some(range);
                    Ok(())
                } else {
                    Err((range, ParseError::Conversion(
                        format!("Expected name `{}`, found `{}`", name, n))))
                }
            }
            &(range, ref val) => {
                Err((range, ParseError::Conversion(
                    format!("Expected end node `{}`, found `{:?}`", name, val))))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use all::*;

    #[test]
    fn search_for_string() {
        let text = "a 1 b 2";
        let rules = r#"
            0 "document" r?([..""!"name" w? $"val" w?])
        "#;
        let rules = stderr_unwrap(rules, syntax(rules));
        let data = stderr_unwrap(text, parse(&rules, text));
        let s = Search::new(&data);
        let a = stderr_unwrap(text, s.for_string("name", "a", |mut s| s.f64("val")));
        assert_eq!(a, 1.0);
        let b = stderr_unwrap(text, s.for_string("name", "b", |mut s| s.f64("val")));
        assert_eq!(b, 2.0);
        let c = s.for_string("name", "c", |mut s| s.f64("val"));
        assert!(c.is_err());
    }

    #[test]
    fn search_for_f64() {
        let text = "a 1 b 2";
        let rules = r#"
            0 "document" r?([..""!"name" w? $"val" w?])
        "#;
        let rules = stderr_unwrap(rules, syntax(rules));
        let data = stderr_unwrap(text, parse(&rules, text));
        let s = Search::new(&data);
        let a = stderr_unwrap(text, s.for_f64("val", 1.0, |mut s| s.string("name")));
        assert_eq!(&**a, "b");
    }

    #[test]
    fn search_for_bool() {
        let text = "a true b false";
        let rules = r#"
            0 "document" r?([..""!"name" w? {"true""val" "false"!"val"} w?])
        "#;
        let rules = stderr_unwrap(rules, syntax(rules));
        let data = stderr_unwrap(text, parse(&rules, text));
        let s = Search::new(&data);
        let a = stderr_unwrap(text, s.for_bool("val", true, |mut s| s.string("name")));
        assert_eq!(&**a, "b");
    }

    #[test]
    fn search_for_end_node() {
        let text = "true false";
        let rules = r#"
            0 "proposition" {"true""val" "false"!"val"}
            0 "document" r?([@"proposition""proposition" w?])
        "#;
        let rules = stderr_unwrap(rules, syntax(rules));
        let data = stderr_unwrap(text, parse(&rules, text));
        let mut s = Search::new(&data);
        stderr_unwrap(text, s.node("proposition"));
        assert!(s.for_end_node("proposition", |mut s| {
            stderr_unwrap(text, s.node("proposition"));
            assert_eq!(s.bool("val"), Ok(false));
            stderr_unwrap(text, s.end_node("proposition"));
            Ok(())
        }).is_ok());
    }

    #[test]
    fn f64() {
        let text = "1 2";
        let rules = r#"
            0 "document" r?([$"val" w?])
        "#;
        let rules = stderr_unwrap(rules, syntax(rules));
        let data = stderr_unwrap(text, parse(&rules, text));
        let mut s = Search::new(&data);
        let res = (s.f64("val").unwrap(), s.f64("val").unwrap());
        assert_eq!(res, (1.0, 2.0));
    }

    #[test]
    fn bool() {
        let text = "true false";
        let rules = r#"
            0 "document" r?([{"true""val" "false"!"val"} w?])
        "#;
        let rules = stderr_unwrap(rules, syntax(rules));
        let data = stderr_unwrap(text, parse(&rules, text));
        let mut s = Search::new(&data);
        let res = (s.bool("val").unwrap(), s.bool("val").unwrap());
        assert_eq!(res, (true, false));
    }

    #[test]
    fn node() {
        let text = "true false";
        let rules = r#"
            0 "proposition" {"true""val" "false"!"val"}
            0 "document" r?([@"proposition""proposition" w?])
        "#;
        let rules = stderr_unwrap(rules, syntax(rules));
        let data = stderr_unwrap(text, parse(&rules, text));
        let mut s = Search::new(&data);
        stderr_unwrap(text, s.node("proposition"));
        assert_eq!(s.bool("val"), Ok(true));
        stderr_unwrap(text, s.end_node("proposition"));

        stderr_unwrap(text, s.node("proposition"));
        assert_eq!(s.bool("val"), Ok(false));
        stderr_unwrap(text, s.end_node("proposition"));
    }
}
