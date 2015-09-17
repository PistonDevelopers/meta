/*
Bootstraps meta language to a better syntax
*/

extern crate piston_meta;

use piston_meta::{ bootstrap, parse, stderr_unwrap };
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    // Get the rules built into piston-meta.
    let rules = bootstrap::rules();

    // Load the better syntax.
    let better_syntax: PathBuf = "assets/better-syntax.txt".into();
    let mut file_h = File::open(better_syntax).unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    let res = stderr_unwrap(&source, parse(&rules, &source));
    let mut ignored1 = vec![];
    let better_rules = bootstrap::convert(&res, &mut ignored1).unwrap();
    println!("better-syntax.txtx: ignored {:?}", ignored1.len());

    // Use the better rules to read better self syntax.
    let better_self_syntax: PathBuf = "assets/better-self-syntax.txt".into();
    let mut file_h = File::open(better_self_syntax).unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
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
