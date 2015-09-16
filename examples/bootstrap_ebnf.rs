/*
Bootstraps meta language to a subset of BNF syntax
*/

extern crate piston_meta;

use piston_meta::{ bootstrap, parse, stderr_unwrap };
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    // Get the rules built into piston-meta.
    let rules = bootstrap::rules();

    // Load the EBNF syntax.
    let ebnf_syntax: PathBuf = "assets/ebnf-syntax.txt".into();
    let mut file_h = File::open(ebnf_syntax).unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    let res = stderr_unwrap(&source, parse(&rules, &source));
    let mut ignored1 = vec![];
    let ebnf_rules = bootstrap::convert(&res, &mut ignored1).unwrap();
    println!("ignored EBNF {:?}", ignored1.len());

    // Use the EBNF rules to read EBNF self syntax.
    let ebnf_self_syntax: PathBuf = "assets/ebnf-self-syntax.txt".into();
    let mut file_h = File::open(ebnf_self_syntax).unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    let res = stderr_unwrap(&source, parse(&ebnf_rules, &source));
    let mut ignored1 = vec![];
    let rules1 = bootstrap::convert(&res, &mut ignored1).unwrap();
    println!("ignored1 {:?}", ignored1.len());

    // Read EBNF self syntax again, using its own rules.
    let res = stderr_unwrap(&source, parse(&rules1, &source));
    let mut ignored2 = vec![];
    let rules2 = bootstrap::convert(&res, &mut ignored2).unwrap();
    println!("ignored2 {:?}", ignored2.len());
    let _ = stderr_unwrap(&source, parse(&rules2, &source));
    assert_eq!(rules1, rules2);
    println!("Bootstrapping EBNF succeeded!");
}
