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
        let print_comma = !first && !is_end;
        if print_comma {
            writeln!(w, ",")?;
        } else if i != 0 {
            writeln!(w, "")?;
        }
        first = false;
        for _ in 0 .. indent_offset + indent {
            write!(w, " ")?;
        }
        match *d {
            MetaData::StartNode(ref name) => {
                first = true;
                write_string(w, name)?;
                write!(w, ":{}", "{")?;
                indent += 1;
            }
            MetaData::EndNode(_) => {
                write!(w, "{}", "}")?;
            }
            MetaData::Bool(ref name, val) => {
                write_string(w, name)?;
                write!(w, ":{}", val)?;
            }
            MetaData::F64(ref name, val) => {
                write_string(w, name)?;
                write!(w, ":{}", val)?;
            }
            MetaData::String(ref name, ref val) => {
                write_string(w, name)?;
                write!(w, ":")?;
                write_string(w, val)?;
            }
        }
    }
    writeln!(w, "")?;
    Ok(())
}

/// Writes a JSON string.
pub fn write_string<W>(w: &mut W, val: &str) -> Result<(), io::Error>
    where W: io::Write
{
    write!(w, "\"")?;
    for c in val.chars() {
        if c == '\\' {
            write!(w, "\\\\")?;
        } else if c == '\"' {
            write!(w, "\\\"")?;
        } else {
            write!(w, "{}", c)?;
        }
    }
    write!(w, "\"")?;
    Ok(())
}

/// Prints meta data.
pub fn print(data: &[Range<MetaData>]) {
    use std::io::stdout;

    write(&mut stdout(), data).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use MetaData;
    use range::Range;
    use std::sync::Arc;

    #[test]
    fn numbers() {
        let mut s: Vec<u8> = vec![];
        let r = Range::empty(0);
        write(&mut s, &[
            r.wrap(MetaData::F64(Arc::new("x".into()), 1.0)),
            r.wrap(MetaData::F64(Arc::new("y".into()), 2.0)),
            r.wrap(MetaData::F64(Arc::new("z".into()), 3.0))
        ]).unwrap();
        assert_eq!(String::from_utf8(s).unwrap(),
            "\"x\":1,\n\"y\":2,\n\"z\":3\n");
    }

    #[test]
    fn node() {
        let mut s: Vec<u8> = vec![];
        let r = Range::empty(0);
        write(&mut s, &[
            r.wrap(MetaData::StartNode(Arc::new("pos".into()))),
            r.wrap(MetaData::F64(Arc::new("x".into()), 1.0)),
            r.wrap(MetaData::F64(Arc::new("y".into()), 2.0)),
            r.wrap(MetaData::F64(Arc::new("z".into()), 3.0)),
            r.wrap(MetaData::EndNode(Arc::new("pos".into())))
        ]).unwrap();
        assert_eq!(String::from_utf8(s).unwrap(),
            "\"pos\":{\n \"x\":1,\n \"y\":2,\n \"z\":3\n}\n");
    }

    #[test]
    fn node2() {
        let mut s: Vec<u8> = vec![];
        let r = Range::empty(0);
        write(&mut s, &[
            r.wrap(MetaData::StartNode(Arc::new("pos".into()))),
            r.wrap(MetaData::F64(Arc::new("x".into()), 1.0)),
            r.wrap(MetaData::EndNode(Arc::new("pos".into()))),
            r.wrap(MetaData::StartNode(Arc::new("pos".into()))),
            r.wrap(MetaData::F64(Arc::new("x".into()), 1.0)),
            r.wrap(MetaData::EndNode(Arc::new("pos".into())))
        ]).unwrap();
        assert_eq!(String::from_utf8(s).unwrap(),
            "\"pos\":{\n \"x\":1\n},\n\"pos\":{\n \"x\":1\n}\n");
    }
}
