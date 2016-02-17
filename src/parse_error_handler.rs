use range::Range;
use std::io::{ self, stderr, Write };

use ParseError;

/// When an error happens, reports to standard error and then panics.
pub fn stderr_unwrap<T>(source: &str, res: Result<T, Range<ParseError>>) -> T {
    match res {
        Err(range_err) => {
            ParseErrorHandler::new(source).error(range_err);
            panic!();
        }
        Ok(val) => val,
    }
}

/// Reports error.
pub struct ParseErrorHandler<'a> {
    lines: Vec<(Range, &'a str)>,
}

impl<'a> ParseErrorHandler<'a> {
    /// Creates a new error handler.
    pub fn new(text: &'a str) -> ParseErrorHandler<'a> {
        let mut start = 0;
        let mut lines = vec![];
        for line in text.split('\n') {
            let length = line.chars().count();
            lines.push((Range::new(start, length), line));
            // Lines are separated by '\n'.
            start += length + 1;
        }

        ParseErrorHandler {
            lines: lines,
        }
    }

    /// Writes message.
    pub fn write_msg<W: Write>(
        &mut self,
        w: &mut W,
        range: Range,
        msg: &str
    ) -> Result<(), io::Error> {
        try!(writeln!(w, "{}", msg));
        for (i, &(r, text)) in self.lines.iter().enumerate() {
            if let Some(intersect) = range.ends_intersect(&r) {
                if intersect.offset >= r.offset {
                    let j = intersect.offset - r.offset;
                    let s = if j > 75 { j - 50 } else { 0 };
                    let e = ::std::cmp::min(s + 100, r.length);
                    try!(write!(w, "{},{}: ", i + 1, j + 1));
                    for c in text.chars().skip(s).take(e - s) {
                        try!(write!(w, "{}", c));
                    }
                    try!(writeln!(w, ""));
                    try!(write!(w, "{},{}: ", i + 1, j + 1));
                    for c in text.chars().skip(s).take(j - s) {
                        match c {
                            '\t' => {
                                try!(write!(w, "\t"));
                            }
                            _ => {
                                try!(write!(w, " "));
                            }
                        }
                    }
                    try!(writeln!(w, "^"));
                }
            }
        }
        Ok(())
    }

    /// Writes error message.
    pub fn write<W: Write>(
        &mut self,
        w: &mut W,
        range_err: Range<ParseError>
    ) -> Result<(), io::Error> {
        // Gets the first line of error message.
        fn first_line(
            err_handler: &ParseErrorHandler,
            range: Range
        ) -> Option<(usize, Range)> {
            let mut first_line = None;
            for (i, &(r, _)) in err_handler.lines.iter().enumerate() {
                if let Some(intersect) = range.ends_intersect(&r) {
                    first_line = Some((i, intersect));
                    break;
                }
            }
            first_line
        }

        let (range, error) = range_err.decouple();

        try!(writeln!(w, "Error {}", error));
        if let &ParseError::ExpectedTag(_, _) = &error {
            // Improves the error report when forgetting a token at end of
            // a line, for example `;` after an expression.
            if let Some(first_line) = first_line(self, range) {
                let mut prev_line = 0;
                for (i, &(_, text)) in
                    self.lines[..first_line.0].iter().enumerate().rev() {
                    prev_line = i;
                    if !text.chars()
                        .all(|c| { c.is_whitespace() }) { break; }
                }
                for (i, &(_, text)) in
                    self.lines[prev_line .. first_line.0].iter().enumerate() {
                    try!(writeln!(w, "{}: {}",
                        i + prev_line + 1, text));
                }
            }
        }
        for (i, &(r, text)) in self.lines.iter().enumerate() {
            if let Some(intersect) = range.ends_intersect(&r) {
                if intersect.offset >= r.offset {
                    let j = intersect.offset - r.offset;
                    let s = if j > 75 { j - 50 } else { 0 };
                    let e = ::std::cmp::min(s + 100, r.length);
                    try!(write!(w, "{},{}: ", i + 1, j + 1));
                    for c in text.chars().skip(s).take(e - s) {
                        try!(write!(w, "{}", c));
                    }
                    try!(writeln!(w, ""));
                    try!(write!(w, "{},{}: ", i + 1, j + 1));
                    for c in text.chars().skip(s).take(j - s) {
                        match c {
                            '\t' => {
                                try!(write!(w, "\t"));
                            }
                            _ => {
                                try!(write!(w, " "));
                            }
                        }
                    }
                    try!(writeln!(w, "^"));
                }
            }
        }
        Ok(())
    }

    /// Prints error message to standard error.
    pub fn error(&mut self, range_err: Range<ParseError>) {
        self.write(&mut stderr(), range_err).unwrap()
    }
}
