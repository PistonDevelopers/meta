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
        for line in text.lines() {
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

        let mut stderr = stderr();
        let mut n = 0;
        writeln!(&mut stderr, "Error: {}", error).unwrap();
        for &(r, text) in &self.lines {
            if let Some(intersect) = range.intersect(&r) {
                writeln!(&mut stderr, "{}: {}", n, text).unwrap();
                if intersect.offset > r.offset {
                    write!(&mut stderr, "{}: ", n).unwrap();
                    let n = intersect.offset - r.offset;
                    for _ in 0 .. n {
                        write!(&mut stderr, " ").unwrap();
                    }
                    writeln!(&mut stderr, "^").unwrap();
                }
            }
            n += 1;
        }
    }
}
