use range::Range;
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
        val: &str, f: F
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
                    Ok(v)
                } else {
                    Err((
                        range,
                        ParseError::Conversion(
                            format!("Expected name `{}`, found `{}`", name, n))
                    ))
                }
            }
            &(range, ref val) => { Err((
                range,
                ParseError::Conversion(
                    format!("Expected f64 `{}`, found `{:?}`", name, val))
            )) }
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
}
