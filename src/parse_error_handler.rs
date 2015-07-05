use range::Range;

use ParseError;

/// Implemented by error handlers.
pub trait ParseErrorHandler {
    /// Report an error.
    fn error(&mut self, range: Range, error: ParseError);
}

/// Reports error to standard error output.
pub struct ParseStdErr<'a> {
    lines: Vec<(Range, &'a str)>,
}

impl<'a> ParseStdErr<'a> {
    /// Creates a new error handler for standard error output.
    pub fn new(text: &'a str) -> ParseStdErr<'a> {
        let mut start = 0;
        let mut lines = vec![];
        for line in text.split('\n') {
            let length = line.len();
            lines.push((Range::new(start, length), line));
            // Lines are separated by '\n'.
            start += length + 1;
        }

        ParseStdErr {
            lines: lines,
        }
    }
}

impl<'b> ParseErrorHandler for ParseStdErr<'b> {
    fn error(&mut self, range: Range, error: ParseError) {
        use std::io::{ stderr, Write };

        // Gets the first line of error message.
        fn first_line(
            err_handler: &ParseStdErr,
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

        let mut stderr = stderr();
        writeln!(&mut stderr, "Error {}", error).unwrap();
        if let &ParseError::ExpectedToken(_, _) = &error {
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
                    writeln!(&mut stderr, "{}: {}",
                        i + prev_line + 1, text).unwrap();
                }
            }
        }
        for (i, &(r, text)) in self.lines.iter().enumerate() {
            if let Some(intersect) = range.ends_intersect(&r) {
                if intersect.offset >= r.offset {
                    let j = intersect.offset - r.offset;
                    let s = if j > 75 { j - 50 } else { 0 };
                    let e = ::std::cmp::min(s + 100, r.length);
                    write!(&mut stderr, "{},{}: ", i + 1, j).unwrap();
                    for c in text.chars().skip(s).take(e - s) {
                        write!(&mut stderr, "{}", c).unwrap();
                    }
                    writeln!(&mut stderr, "").unwrap();
                    write!(&mut stderr, "{},{}: ", i + 1, j).unwrap();
                    for c in text.chars().skip(s).take(j - s) {
                        match c {
                            '\t' => {
                                write!(&mut stderr, "\t").unwrap();
                            }
                            _ => {
                                write!(&mut stderr, " ").unwrap();
                            }
                        }
                    }
                    writeln!(&mut stderr, "^").unwrap();
                }
            }
        }
    }
}
