/*
Bootstraps meta language to a better syntax
*/

extern crate piston_meta;

use piston_meta::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn load_file<P: AsRef<Path>>(path: P) -> String {
    let mut file_h = File::open(path.as_ref()).unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    source
}

fn main() {
    // Load the better syntax.
    let source = load_file("assets/better-syntax.txt");
    let better_rules = stderr_unwrap(&source, syntax(&source));

    // Use the better rules to read better self syntax.
    let source = load_file("assets/better-self-syntax.txt");
    let res = stderr_unwrap(&source, parse(&better_rules, &source));
    let mut ignored1 = vec![];
    let rules1 = bootstrap::convert(&res, &mut ignored1).unwrap();
    println!("better-self-syntax.txt: ignored1 {:?}", ignored1.len());

    // Read better self syntax again, using its own rules.
    let res = stderr_unwrap(&source, parse(&rules1, &source));
    let mut ignored2 = vec![];
    let rules2 = bootstrap::convert(&res, &mut ignored2).unwrap();
    println!("better-self-syntax.txt: ignored2 {:?}", ignored2.len());
    let _ = stderr_unwrap(&source, parse(&rules2, &source));
    assert_eq!(rules1, rules2);
    println!("Bootstrapping better syntax succeeded!");
}
