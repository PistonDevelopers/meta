//! Write meta data as JSON.

use range::Range;
use std::io;

use MetaData;

/// Writes meta data as JSON.
pub fn write<W>(w: &mut W, data: &[Range<MetaData>]) -> Result<(), io::Error>
    where W: io::Write
{
    use std::cmp::{ min, max };

    let indent_offset = 0;

    // Start indention such that it balances off to zero.
    let starts = data.iter()
        .filter(|x| if let MetaData::StartNode(_) = x.data { true } else { false })
        .count() as u32;
    let ends = data.iter()
        .filter(|x| if let MetaData::EndNode(_) = x.data { true } else { false })
        .count() as u32;
    let mut indent: u32 = max(starts, ends) - min(starts, ends);
    let mut first = true;
    for (i, d) in data.iter().enumerate() {
        let d = &d.data;
        let is_end = if let MetaData::EndNode(_) = *d {
            indent -= 1;
            true
        } else { false };
        let is_next_end = if i < data.len() - 1 {
            match data[i + 1].data {
                MetaData::EndNode(_) => false,
                _ => true
            }
        } else { true };
        let print_comma = !first && !is_end && is_next_end;
        if print_comma {
            try!(writeln!(w, ","));
        } else if i != 0 {
            try!(writeln!(w, ""));
        }
        first = false;
        for _ in 0 .. indent_offset + indent {
            try!(write!(w, " "));
        }
        match *d {
            MetaData::StartNode(ref name) => {
                first = true;
                try!(write_string(w, name));
                try!(write!(w, ":{}", "{"));
                indent += 1;
            }
            MetaData::EndNode(_) => {
                try!(write!(w, "{}", "}"));
            }
            MetaData::Bool(ref name, val) => {
                try!(write_string(w, name));
                try!(write!(w, ":{}", val));
            }
            MetaData::F64(ref name, val) => {
                try!(write_string(w, name));
                try!(write!(w, ":{}", val));
            }
            MetaData::String(ref name, ref val) => {
                try!(write_string(w, name));
                try!(write!(w, ":"));
                try!(write_string(w, val));
            }
        }
    }
    try!(writeln!(w, ""));
    Ok(())
}

/// Writes a JSON string.
pub fn write_string<W>(w: &mut W, val: &str) -> Result<(), io::Error>
    where W: io::Write
{
    try!(write!(w, "\""));
    for c in val.chars() {
        if c == '\\' {
            try!(write!(w, "\\\\"));
        } else if c == '\"' {
            try!(write!(w, "\\\""));
        } else {
            try!(write!(w, "{}", c));
        }
    }
    try!(write!(w, "\""));
    Ok(())
}

/// Prints meta data.
pub fn print(data: &[Range<MetaData>]) {
    use std::io::stdout;

    write(&mut stdout(), data).unwrap();
}
