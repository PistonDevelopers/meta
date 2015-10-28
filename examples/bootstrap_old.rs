extern crate piston_meta;

use piston_meta::*;
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn load_file<P: AsRef<Path>>(path: P) -> String {
    let mut file_h = File::open(path.as_ref()).unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    source
}

fn main() {
    // Get the old syntax in the new syntax.
    let old = load_file("assets/old-syntax.txt");
    let old_syntax = stderr_unwrap(&old, syntax(&old));

    let old_in_old = load_file("assets/old-self-syntax.txt");
    let mut old_in_old_syntax = vec![];
    stderr_unwrap(&old_in_old,
        parse(&old_syntax, &old_in_old, &mut old_in_old_syntax));
    let old_in_old_syntax = bootstrap::convert(&old_in_old_syntax, &mut vec![])
        .unwrap();

    assert_eq!(old_syntax, old_in_old_syntax);
    println!("Bootstrap succeeded!");
}
