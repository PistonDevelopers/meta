extern crate piston_meta;

use piston_meta::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let rules = bootstrap::rules();
    let self_syntax: PathBuf = "assets/self-syntax.txt".into();
    let mut file_h = File::open(self_syntax).unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    let mut res = vec![];
    stderr_unwrap(&source, parse(&rules, &source, &mut res));
    let mut ignored1 = vec![];
    let rules1 = bootstrap::convert(&res, &mut ignored1).unwrap();
    println!("ignored1 {:?}", ignored1.len());
    let mut res = vec![];
    stderr_unwrap(&source, parse(&rules1, &source, &mut res));
    let mut ignored2 = vec![];
    let rules2 = bootstrap::convert(&res, &mut ignored2).unwrap();
    println!("ignored2 {:?}", ignored2.len());
    stderr_unwrap(&source, parse(&rules2, &source, &mut vec![]));
    assert_eq!(rules1, rules2);
    println!("Bootstrapping succeeded!");
}
